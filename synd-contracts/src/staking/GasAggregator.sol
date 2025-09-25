// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

interface ISequencingContract {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
    function getEmissionsReceiver() external view returns (address);
}

interface IAppchainFactory {
    function isImplementationAllowed(address implementation) external view returns (bool);
    function computeSequencingChainAddress(uint256 chainId) external view returns (address);
    function getProxyBytecode() external view returns (bytes memory);
    function syndicateChainImpl() external view returns (address);
}

/// @title GasAggregator
/// @notice Aggregates gas usage data from appchains and pushes it to the staking appchain
contract GasAggregator is Initializable, EpochTracker, AccessControlUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    IAppchainFactory public factory;

    uint256 public maxAppchainsToQuery;

    /// @notice Fee required to add a chain to the registry
    uint256 public addChainFee;

    /// @notice Cached proxy bytecode hash for deterministic address computation
    bytes32 public proxyBytecodeHash;

    /// @notice Registry of chains that can be tracked for gas usage
    uint256[] public appchains;
    mapping(uint256 chainID => ISequencingContract sequencingContract) public appchainContracts;
    mapping(uint256 chainID => bool tracked) public isChainTracked;
    mapping(uint256 chainID => bool banned) public bannedAppchains;

    /// @notice Mapping of allowed implementations
    mapping(address implementation => bool allowed) public allowedImplementations;

    /// @notice used for the offchain aggregation mechanism.
    /// The challenge window is the time period after the first submission during which new submission can be made
    /// After the challenge window has elapsed, the data must be pushed to the staking appchain for the next epoch aggregation to start
    uint256 public challengeWindow;
    uint256 public pendingEpoch;
    uint256 public pendingEpochFirstSubmissionTime;
    bytes32 public pendingDataHash;
    uint256 public pendingTotalTokensUsed;

    /// @notice last epoch that was aggregated using the offchain mechanism (this data can be used for re-submissions)
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    /// @notice Version of the GasAggregator contract (updatable during upgrades)
    string public version;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/
    error MustUseOffchainAggregation();
    error MustUseAutomaticAggregation();
    error NotHigherThanPendingTotal(uint256 submitted, uint256 pending);
    error EpochNotOver(uint256 epoch, uint256 currentEpoch);
    error WindowNotOver(uint256 currentEpoch, uint256 challengeWindow);
    error WindowOver(uint256 currentEpoch, uint256 challengeWindow);
    error ChainIDsMustBeInAscendingOrder();
    error ZeroChallengeWindow();
    error ZeroAddress();
    error InvalidDataHash();
    error ChainAlreadyTracked(uint256 chainId);
    error ChainNotTracked(uint256 chainId);
    error ImplementationNotAllowed(address implementation);
    error InsufficientFee(uint256 required, uint256 provided);
    error ChainNotFound(uint256 chainId);
    error ChainIsBanned(uint256 chainId);
    error OnlyChainCanNotifyUpgrade();

    /*//////////////////////////////////////////////////////////////
                              EVENTS
    //////////////////////////////////////////////////////////////*/

    event ChainAdded(uint256 indexed chainId, address indexed chainContract, address indexed addedBy);
    event ChainRemoved(uint256 indexed chainId);
    event AddChainFeeUpdated(uint256 oldFee, uint256 newFee);
    event ChainBanned(uint256 chainId, address newImplementation);

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    constructor() {
        _disableInitializers();
    }

    function initialize(IAppchainFactory _factory, address admin, uint256 _challengeWindow, uint256 _addChainFee)
        external
        initializer
    {
        if (address(_factory) == address(0)) revert ZeroAddress();
        if (admin == address(0)) revert ZeroAddress();
        if (_challengeWindow == 0) revert ZeroChallengeWindow();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // consider all past epochs ignored
        pendingEpoch = getCurrentEpoch();
        version = "1.0.0";
        factory = _factory;
        challengeWindow = _challengeWindow;
        addChainFee = _addChainFee;

        // Cache the proxy bytecode hash for deterministic address computation
        proxyBytecodeHash = keccak256(_factory.getProxyBytecode());

        address syndicateChainImpl = _factory.syndicateChainImpl();
        if (syndicateChainImpl != address(0)) {
            allowedImplementations[syndicateChainImpl] = true;
        }
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS 
    //////////////////////////////////////////////////////////////*/

    modifier onlyCompletedEpoch(uint256 epoch) {
        uint256 currentEpoch = getCurrentEpoch();
        if (currentEpoch <= epoch) {
            revert EpochNotOver(epoch, currentEpoch);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a chain to the gas tracking registry
    /// @dev Anyone can call this function by paying the required fee
    /// @param chainId The chain ID to add
    function addChain(uint256 chainId) external payable {
        if (msg.value < addChainFee) {
            revert InsufficientFee(addChainFee, msg.value);
        }

        if (isChainTracked[chainId]) {
            revert ChainAlreadyTracked(chainId);
        }

        // Get the chain contract address deterministically from the factory
        address chainContract = computeSequencingChainAddress(chainId);

        // Check if there's actually a contract deployed at this address
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(chainContract)
        }
        if (codeSize == 0) {
            revert ChainNotFound(chainId);
        }

        if (bannedAppchains[chainId]) {
            revert ChainIsBanned(chainId);
        }

        // Add to registry
        appchains.push(chainId);
        appchainContracts[chainId] = ISequencingContract(chainContract);
        isChainTracked[chainId] = true;

        emit ChainAdded(chainId, chainContract, msg.sender);
    }

    /// @notice triggers automatic aggregation of the appchain gas usage data and pushes it to the staking appchain
    /// @dev only usable until there are up to `maxAppchainsToQuery` appchains, after that point the offchain aggregation mechanism must be used
    function aggregateTokensUsed() external onlyCompletedEpoch(pendingEpoch) {
        if (fallbackToOffchainAggregation()) {
            revert MustUseOffchainAggregation();
        }
        uint256[] memory tokens = new uint256[](appchains.length);
        address[] memory emissionsReceivers = new address[](appchains.length);
        for (uint256 i = 0; i < appchains.length; i++) {
            ISequencingContract seqContract = appchainContracts[appchains[i]];
            tokens[i] = seqContract.getTokensForEpoch(pendingEpoch);
            emissionsReceivers[i] = seqContract.getEmissionsReceiver();
        }
        aggregatedEpochDataHash[pendingEpoch] = keccak256(abi.encode(appchains, tokens, emissionsReceivers));
        pendingEpoch++;
        pendingEpochFirstSubmissionTime = 0;
        pendingDataHash = bytes32(0);
        pendingTotalTokensUsed = 0;
    }

    /// @notice Submission of top appchains aggregated off-chain
    /// @dev appchainIDs must be submitted in ascending order
    /// @param appchainIDs the chainIDs of the top appchains for the current epoch
    function submitOffchainTopChains(uint256[] calldata appchainIDs) external onlyCompletedEpoch(pendingEpoch) {
        if (!fallbackToOffchainAggregation()) {
            revert MustUseAutomaticAggregation();
        }
        if (pendingEpochFirstSubmissionTime != 0 && block.timestamp > pendingEpochFirstSubmissionTime + challengeWindow)
        {
            revert WindowOver(pendingEpoch, challengeWindow);
        }
        uint256 total = 0;
        uint256[] memory tokens = new uint256[](appchainIDs.length);
        address[] memory emissionsReceivers = new address[](appchainIDs.length);
        for (uint256 i = 0; i < appchainIDs.length; i++) {
            if (i > 0 && appchainIDs[i] <= appchainIDs[i - 1]) {
                revert ChainIDsMustBeInAscendingOrder();
            }
            ISequencingContract seqContract = appchainContracts[appchainIDs[i]];
            tokens[i] = seqContract.getTokensForEpoch(pendingEpoch);
            emissionsReceivers[i] = seqContract.getEmissionsReceiver();
            total += tokens[i];
        }
        if (total <= pendingTotalTokensUsed) {
            revert NotHigherThanPendingTotal(total, pendingTotalTokensUsed);
        }
        if (pendingEpochFirstSubmissionTime == 0) {
            pendingEpochFirstSubmissionTime = block.timestamp;
        }
        pendingDataHash = keccak256(abi.encode(appchainIDs, tokens, emissionsReceivers));
        pendingTotalTokensUsed = total;
    }

    function sealPendingEpoch() external onlyCompletedEpoch(pendingEpoch) {
        if (
            pendingEpochFirstSubmissionTime == 0 || block.timestamp <= pendingEpochFirstSubmissionTime + challengeWindow
        ) {
            revert WindowNotOver(pendingEpoch, challengeWindow);
        }
        aggregatedEpochDataHash[pendingEpoch] = pendingDataHash;
        pendingEpoch++;
        pendingEpochFirstSubmissionTime = 0;
        pendingDataHash = bytes32(0);
        pendingTotalTokensUsed = 0;
    }

    /// @notice Called by sequencing chains to notify about upgrades
    /// @dev Automatically bans chain from gas tracking if implementation is not allowed
    /// @param chainId The chain ID that is upgrading
    /// @param newImplementation The address of the new implementation
    function notifyChainUpgrade(uint256 chainId, address newImplementation) external {
        if (address(appchainContracts[chainId]) != msg.sender) revert OnlyChainCanNotifyUpgrade();

        if (!allowedImplementations[newImplementation]) {
            _banAppchain(chainId);
            emit ChainBanned(chainId, newImplementation);
        }
    }

    function notifyNewImplementation(address newImplementation) external {
        allowedImplementations[newImplementation] = true;
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal function to remove a chain from tracking
    /// @param chainId The chain ID to remove
    function _banAppchain(uint256 chainId) internal {
        // Remove from tracking arrays
        for (uint256 i = 0; i < appchains.length; i++) {
            if (appchains[i] == chainId) {
                appchains[i] = appchains[appchains.length - 1];
                appchains.pop();
                break;
            }
        }
        delete isChainTracked[chainId];
        delete appchainContracts[chainId];
        bannedAppchains[chainId] = true;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the address where a sequencing chain will be deployed
    /// @param chainId The chain ID to compute the address for
    /// @return The computed address
    function computeSequencingChainAddress(uint256 chainId) internal view returns (address) {
        return Create2.computeAddress(bytes32(chainId), proxyBytecodeHash, address(factory));
    }

    function fallbackToOffchainAggregation() public view returns (bool) {
        return appchains.length >= maxAppchainsToQuery;
    }

    /// @notice Get the total number of tracked chains
    function getTotalTrackedChains() external view returns (uint256) {
        return appchains.length;
    }

    /// @notice Get all tracked chain IDs
    function getTrackedChainIds() external view returns (uint256[] memory) {
        return appchains;
    }

    /// @notice Get the contract's current balance
    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice set the max number of appchains to query
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function setMaxAppchainsToQuery(uint256 newMax) external onlyRole(DEFAULT_ADMIN_ROLE) {
        maxAppchainsToQuery = newMax;
    }

    /// @notice set the challenge window
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function setChallengeWindow(uint256 newChallengeWindow) external onlyRole(DEFAULT_ADMIN_ROLE) {
        challengeWindow = newChallengeWindow;
    }

    function removeAllowedImplementation(address newImpl) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (!allowedImplementations[newImpl]) revert ImplementationNotAllowed(newImpl);
        delete allowedImplementations[newImpl];
    }

    /// @notice Updates the contract version (admin only, typically called during upgrades)
    /// @param newVersion The new version string (e.g., "1.1.0")
    function updateVersion(string calldata newVersion) external onlyRole(DEFAULT_ADMIN_ROLE) {
        version = newVersion;
    }

    /// @notice Set the fee required to add a chain to the registry
    /// @param newFee The new fee amount
    function setAddChainFee(uint256 newFee) external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 oldFee = addChainFee;
        addChainFee = newFee;
        emit AddChainFeeUpdated(oldFee, newFee);
    }

    /// @notice Withdraw collected fees (admin only)
    /// @param to Address to send the funds to
    /// @param amount Amount to withdraw (0 to withdraw all)
    function withdrawFees(address payable to, uint256 amount) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (to == address(0)) revert ZeroAddress();

        uint256 withdrawAmount = amount == 0 ? address(this).balance : amount;
        if (withdrawAmount > address(this).balance) {
            revert InsufficientFee(withdrawAmount, address(this).balance);
        }

        (bool success,) = to.call{value: withdrawAmount}("");
        require(success, "Transfer failed");
    }
}
