// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

interface ISequencingContract {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
    function getEmissionsReceiver() external view returns (address);
}

interface IAppchainFactory {
    function getProxyBytecode() external view returns (bytes memory);
}

/**
 * @title GasAggregator
 * @notice Aggregates gas usage data from appchains
 * @dev This contract manages the collection and aggregation of gas usage data from multiple appchains.
 *      It supports both automatic aggregation (for small numbers of appchains) and off-chain aggregation
 *      (for larger numbers of appchains) with a challenge mechanism for data integrity.
 * @dev Inherits from EpochTracker for epoch management and AccessControlUpgradeable for admin functions
 */
contract GasAggregator is Initializable, EpochTracker, AccessControlUpgradeable, UUPSUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Factory contract for managing appchain contracts
    /// @dev Used to get appchain addresses and total count
    IAppchainFactory public factory;

    /// @notice Maximum number of appchains that can be queried automatically
    /// @dev When total appchains >= this value, off-chain aggregation is required
    uint256 public maxAppchainsToQuery;

    /// @notice Fee required to add a chain to the registry
    uint256 public addChainFee;

    /// @notice Cached proxy bytecode hash for deterministic address computation
    bytes32 public sequencingChainProxyBytecodeHash;

    /// @notice Registry of chains that can be tracked for gas usage
    uint256[] public appchains;
    mapping(uint256 chainID => ISequencingContract sequencingContract) public appchainContracts;
    mapping(uint256 chainID => bool tracked) public isChainTracked;
    mapping(uint256 chainID => bool banned) public bannedAppchains;

    /// @notice Mapping of allowed implementations
    mapping(address implementation => bool allowed) public allowedImplementations;

    /// @notice Challenge window duration for off-chain aggregation submissions
    /// @dev Time period after first submission during which new submissions can be made
    ///      After this window, the data must be sealed for the next epoch aggregation to start
    uint256 public challengeWindow;

    /// @notice Current epoch being processed for aggregation
    /// @dev Tracks which epoch is pending aggregation
    uint256 public pendingEpoch;

    /// @notice Timestamp of the first submission for the current pending epoch
    /// @dev Used to calculate challenge window expiration
    uint256 public pendingEpochFirstSubmissionTime;

    /// @notice Hash of the pending data for the current epoch
    /// @dev Stores the hash of (appchainIDs, tokens, emissionsReceivers) for verification
    bytes32 public pendingDataHash;

    /// @notice Total tokens used in the pending epoch
    /// @dev Used to ensure new submissions have higher total than previous ones
    uint256 public pendingTotalTokensUsed;

    /// @notice Mapping from epoch to aggregated data hash
    /// @dev Stores the final hash for each completed epoch (can be used for re-submissions)
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    // TODO implement this
    /// @notice Admin-controlled overrides for appchain contracts
    mapping(uint256 => address) public appchainContractOverrides;

    /// @notice Version of the GasAggregator contract (updatable during upgrades)
    string public version;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Error thrown when automatic aggregation is attempted but off-chain aggregation is required
    /// @dev Triggered when total appchains >= maxAppchainsToQuery
    error MustUseOffchainAggregation();

    /// @notice Error thrown when off-chain aggregation is attempted but automatic aggregation should be used
    /// @dev Triggered when total appchains < maxAppchainsToQuery
    error MustUseAutomaticAggregation();

    /// @notice Error thrown when new submission total is not higher than pending total
    /// @dev Ensures submissions improve upon previous ones
    error NotHigherThanPendingTotal(uint256 submitted, uint256 pending);

    /// @notice Error thrown when attempting to aggregate an epoch that hasn't ended
    /// @dev Prevents aggregation of current or future epochs
    error EpochNotOver(uint256 epoch, uint256 currentEpoch);

    /// @notice Error thrown when attempting to seal before challenge window expires
    /// @dev Ensures challenge window has elapsed before sealing
    error WindowNotOver(uint256 currentEpoch, uint256 challengeWindow);

    /// @notice Error thrown when attempting to submit after challenge window has expired
    /// @dev Prevents submissions after the challenge period
    error WindowOver(uint256 currentEpoch, uint256 challengeWindow);

    /// @notice Error thrown when appchain IDs are not submitted in ascending order
    /// @dev Ensures consistent ordering for data integrity
    error ChainIDsMustBeInAscendingOrder();

    /// @notice Error thrown when challenge window is set to zero
    /// @dev Prevents invalid configuration
    error ZeroChallengeWindow();

    /// @notice Error thrown when a zero address is provided
    /// @dev Prevents invalid contract addresses
    error ZeroAddress();

    /// @notice Error thrown when data hash is invalid
    /// @dev Ensures data integrity
    error InvalidDataHash();
    error ChainAlreadyTracked(uint256 chainId);
    error ChainNotTracked(uint256 chainId);
    error ImplementationNotAllowed(address implementation);
    error InsufficientFee(uint256 required, uint256 provided);
    error ChainNotFound(uint256 chainId);
    error ChainIsBanned(uint256 chainId);
    error OnlyChainCanNotifyUpgrade();
    error OnlyFactoryCanNotifyNewImplementation();
    error NoSequencingChainProxyBytecodeHash();

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

    /**
     * @notice Constructor that disables initializers
     * @dev Prevents direct initialization of implementation contract
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initialize the GasAggregator contract
     * @dev Sets up the contract with factory, admin, and challenge window configuration
     * @param admin The address to be granted admin privileges
     * @param _factory The address of the appchain factory
     */
    function initialize(address admin, address _factory) external initializer {
        if (admin == address(0)) revert ZeroAddress();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // consider all past epochs ignored
        pendingEpoch = getCurrentEpoch();
        version = "1.0.0";
        challengeWindow = 24 hours;
        addChainFee = 5 ether;
        factory = IAppchainFactory(_factory);
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS 
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Modifier that ensures the epoch has completed before allowing aggregation
     * @dev Prevents aggregation of current or future epochs
     * @param epoch The epoch index to check
     */
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

    /**
     * @notice Triggers automatic aggregation of appchain gas usage data
     * @dev Only usable when total appchains < maxAppchainsToQuery.
     *      Queries all appchains directly and aggregates their gas usage data.
     *      After aggregation, moves to the next epoch.
     * @custom:example If 5 appchains exist and maxAppchainsToQuery is 10, this function will work
     * @custom:example If 15 appchains exist and maxAppchainsToQuery is 10, this function will revert
     */
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

    /**
     * @notice Submit top appchains aggregated off-chain
     * @dev Used when total appchains >= maxAppchainsToQuery.
     *      Allows submission of top-performing appchains with challenge mechanism.
     *      AppchainIDs must be submitted in ascending order for data integrity.
     * @param appchainIDs The chainIDs of the top appchains for the current epoch
     * @custom:example Submit [1, 5, 10] for top 3 appchains (must be in ascending order)
     * @custom:example If challenge window is 1 hour, submissions are only allowed for 1 hour after first submission
     */
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

    /**
     * @notice Seal the pending epoch after challenge window expires
     * @dev Finalizes the off-chain aggregation by sealing the pending data.
     *      Can only be called after the challenge window has expired.
     *      Resets all pending state variables for the next epoch.
     * @custom:example If challenge window is 1 hour, can only seal after 1 hour from first submission
     */
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
        if (msg.sender != address(factory)) revert OnlyFactoryCanNotifyNewImplementation();
        allowedImplementations[newImplementation] = true;
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the deterministic address for a sequencing chain proxy
    /// @param chainId The chain ID for the sequencing chain
    /// @return The computed address of the sequencing chain proxy
    function computeSequencingChainAddress(uint256 chainId) internal returns (address) {
        return Create2.computeAddress(bytes32(chainId), getSequencingChainProxyBytecodeHash(), address(factory));
    }

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

    function getSequencingChainProxyBytecodeHash() internal returns (bytes32) {
        if (sequencingChainProxyBytecodeHash != bytes32(0)) return sequencingChainProxyBytecodeHash;
        if (address(factory) != address(0)) {
            // Cache the proxy bytecode hash for deterministic address computation
            sequencingChainProxyBytecodeHash = keccak256(factory.getProxyBytecode());
            return sequencingChainProxyBytecodeHash;
        }
        revert NoSequencingChainProxyBytecodeHash();
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Check if off-chain aggregation is required
     * @dev Returns true when total appchains >= maxAppchainsToQuery
     * @return True if off-chain aggregation is required, false for automatic aggregation
     * @custom:example If 15 appchains exist and maxAppchainsToQuery is 10, returns true
     * @custom:example If 5 appchains exist and maxAppchainsToQuery is 10, returns false
     */
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

    /**
     * @notice Set the maximum number of appchains that can be queried automatically
     * @dev When total appchains >= this value, off-chain aggregation is required
     * @param newMax The new maximum number of appchains for automatic aggregation
     * @custom:example If set to 10, automatic aggregation works for ≤10 appchains
     */
    function setMaxAppchainsToQuery(uint256 newMax) external onlyRole(DEFAULT_ADMIN_ROLE) {
        maxAppchainsToQuery = newMax;
    }

    /**
     * @notice Set the challenge window duration for off-chain aggregation
     * @dev Time period after first submission during which new submissions can be made
     * @param newChallengeWindow The new challenge window duration in seconds
     * @custom:example If set to 3600, challenge window is 1 hour
     */
    function setChallengeWindow(uint256 newChallengeWindow) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newChallengeWindow == 0) revert ZeroChallengeWindow();
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

    function setFactory(address newFactory) external onlyRole(DEFAULT_ADMIN_ROLE) {
        factory = IAppchainFactory(newFactory);
        sequencingChainProxyBytecodeHash = keccak256(factory.getProxyBytecode());
    }

    /*//////////////////////////////////////////////////////////////
                            UPGRADE AUTHORIZATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Authorizes contract upgrades
     * @dev Only admin can authorize upgrades
     * @param newImplementation Address of the new implementation
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}
}
