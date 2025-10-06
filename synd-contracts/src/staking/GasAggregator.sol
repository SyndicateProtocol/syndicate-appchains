// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

/// @title ISequencingContract
/// @notice Interface for sequencing chain contracts to query gas usage data
/// @dev Used by the gas aggregator to collect gas usage information from appchains
interface ISequencingContract {
    /// @notice Get the total gas tokens used by this sequencing chain for a specific epoch
    /// @param epoch The epoch number to query gas usage for
    /// @return The amount of gas tokens used in the specified epoch
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);

    /// @notice Get the address that should receive emissions for this sequencing chain
    /// @return The address configured to receive emissions from this chain
    function getEmissionsReceiver() external view returns (address);
}

/// @title IAppchainFactory
/// @notice Interface for the appchain factory to get deployment information
/// @dev Used to compute deterministic addresses for sequencing chain contracts
interface IAppchainFactory {
    /// @notice Get the consistent proxy bytecode used for sequencing chain deployments
    /// @return The bytecode used to deploy sequencing chain proxies
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
                            FIXED STORAGE SLOTS
    //////////////////////////////////////////////////////////////*/

    /// SLOT 0: aggregatedEpochDataHash
    /// @notice Storage slot is 0 for aggregatedEpochDataHash in GasAggregator (see `forge inspect GasAggregator storageLayout`)
    /// @dev Stores the final hash for each completed epoch.
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Factory contract for managing appchain contracts
    /// @dev Used to get appchain addresses, proxy bytecode, and handle implementation updates
    IAppchainFactory public factory;

    /// @notice Maximum number of appchains that can be queried automatically on-chain
    /// @dev When total appchains >= this value, off-chain aggregation with challenge mechanism is required.
    ///      This prevents gas limit issues when querying too many contracts.
    uint256 public maxAppchainsToQuery;

    /// @notice Fee required to add a chain to the gas tracking registry
    /// @dev Exists as a spam-preventing measure. Paid in SYND.
    uint256 public addChainFee;

    /// @notice Cached proxy bytecode hash for deterministic address computation
    /// @dev Computed once and cached for efficiency when calculating sequencing chain addresses
    bytes32 public sequencingChainProxyBytecodeHash;

    /// @notice Registry of chains that are currently tracked for gas usage
    /// @dev Array enables enumeration of all tracked chains for automatic aggregation
    uint256[] public appchains;

    /// @notice Mapping to quickly check if a chain ID is currently being tracked
    mapping(uint256 chainID => bool tracked) public isChainTracked;

    /// @notice Mapping of chains that have been banned from gas tracking
    /// @dev Chains are banned when they upgrade to non-approved implementations
    mapping(uint256 chainID => bool banned) public bannedAppchains;

    /// @notice Mapping of sequencing chain implementations approved for gas tracking
    /// @dev Only chains running approved implementations can participate in gas aggregation
    mapping(address implementation => bool allowed) public allowedImplementations;

    /// @notice Challenge window duration for off-chain aggregation submissions
    /// @dev Time period after first submission during which new submissions can be made.
    ///      After this window expires, the data must be sealed before the next epoch aggregation can start.
    ///      Default is 24 hours to allow for global participation.
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

    /// @notice Admin-controlled overrides for appchain contract addresses
    /// @dev Allows admins to specify custom contract addresses for specific chain IDs.
    ///      Useful for legacy migrations or special cases where deterministic addresses don't apply.
    mapping(uint256 => address) public appchainContractOverrides;

    /// @notice Version of the GasAggregator contract (updatable during upgrades)
    /// @dev Semantic version string to track contract upgrades and compatibility
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

    /// @notice Error thrown when an epoch start timestamp is zero
    error ZeroEpoch();

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

    /// @notice Emitted when a chain is successfully added to the gas tracking registry
    /// @param chainId The chain ID that was added
    /// @param chainContract The contract address of the sequencing chain
    /// @param addedBy The address that paid the fee to add this chain
    event ChainAdded(uint256 indexed chainId, address indexed chainContract, address indexed addedBy);

    /// @notice Emitted when a chain is removed from the gas tracking registry
    /// @param chainId The chain ID that was removed
    event ChainRemoved(uint256 indexed chainId);

    /// @notice Emitted when the fee for adding chains is updated
    /// @param oldFee The previous fee amount
    /// @param newFee The new fee amount
    event AddChainFeeUpdated(uint256 oldFee, uint256 newFee);

    /// @notice Emitted when a chain is banned due to upgrading to a non-approved implementation
    /// @param chainId The chain ID that was banned
    /// @param newImplementation The non-approved implementation that triggered the ban
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
     * @dev Sets up the contract with factory, admin, challenge window, and initial configuration.
     *      This function can only be called once during proxy deployment.
     * @param _admin The address to be granted admin privileges (receives DEFAULT_ADMIN_ROLE)
     * @param _factory The address of the appchain factory contract
     * @param _allowedImplementation The address of the initial approved sequencing chain implementation
     */
    function initialize(address _admin, address _factory, address _allowedImplementation, uint256 _epochStart)
        external
        initializer
    {
        if (_admin == address(0)) revert ZeroAddress();
        if (_epochStart == 0) revert ZeroEpoch();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, _admin);

        // Start tracking from the current epoch (ignore all past epochs)
        pendingEpoch = _epochStart;
        version = "1.0.0";
        challengeWindow = 24 hours;
        addChainFee = 5 ether;
        maxAppchainsToQuery = 100;
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
    /// @dev Anyone can call this function by paying the required fee. The chain must exist at the
    ///      deterministic address and not be banned. Successfully added chains will participate
    ///      in gas aggregation and emissions distribution.
    /// @param chainId The chain ID to add to the tracking registry
    function addChain(uint256 chainId) external payable {
        if (msg.value < addChainFee) {
            revert InsufficientFee(addChainFee, msg.value);
        }

        if (isChainTracked[chainId]) {
            revert ChainAlreadyTracked(chainId);
        }

        address chainContract = getAppchainContractAddress(chainId);

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

        appchains.push(chainId);
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
            ISequencingContract seqContract = ISequencingContract(getAppchainContractAddress(appchains[i]));
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
            ISequencingContract seqContract = ISequencingContract(getAppchainContractAddress(appchainIDs[i]));
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

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the deterministic address for a sequencing chain proxy
    /// @dev Uses CREATE2 with the factory as deployer, chain ID as salt, and cached bytecode hash
    /// @param chainId The chain ID for the sequencing chain
    /// @return The computed address of the sequencing chain proxy
    function computeSequencingChainAddress(uint256 chainId) internal returns (address) {
        return Create2.computeAddress(bytes32(chainId), getSequencingChainProxyBytecodeHash(), address(factory));
    }

    /// @notice Internal function to ban a chain from gas tracking
    /// @dev Removes the chain from all tracking data structures and marks it as banned.
    ///      Banned chains cannot be re-added to prevent abuse.
    /// @param chainId The chain ID to ban from gas tracking
    function _banAppchain(uint256 chainId) internal {
        for (uint256 i = 0; i < appchains.length; i++) {
            if (appchains[i] == chainId) {
                appchains[i] = appchains[appchains.length - 1];
                appchains.pop();
                break;
            }
        }
        delete isChainTracked[chainId];
        bannedAppchains[chainId] = true;
    }

    /// @notice Get the cached proxy bytecode hash, computing it if necessary
    /// @dev Caches the bytecode hash for gas efficiency in repeated address computations
    /// @return The keccak256 hash of the proxy bytecode used for CREATE2 deployments
    function getSequencingChainProxyBytecodeHash() internal returns (bytes32) {
        if (sequencingChainProxyBytecodeHash != bytes32(0)) return sequencingChainProxyBytecodeHash;
        if (address(factory) != address(0)) {
            sequencingChainProxyBytecodeHash = keccak256(factory.getProxyBytecode());
            return sequencingChainProxyBytecodeHash;
        }
        revert NoSequencingChainProxyBytecodeHash();
    }

    /// @notice Get the contract address for a given chain ID
    /// @dev Checks for admin overrides first, then falls back to deterministic address computation
    /// @param chainId The chain ID to get the contract address for
    /// @return The contract address for the specified chain ID
    function getAppchainContractAddress(uint256 chainId) internal returns (address) {
        address contractOverride = appchainContractOverrides[chainId];
        if (contractOverride != address(0)) {
            return contractOverride;
        }
        return computeSequencingChainAddress(chainId);
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

    /// @notice Get the total number of chains currently being tracked for gas usage
    /// @return The number of chains in the tracking registry
    function getTotalTrackedChains() external view returns (uint256) {
        return appchains.length;
    }

    /// @notice Get all chain IDs currently being tracked for gas usage
    /// @return Array of all tracked chain IDs
    function getTrackedChainIds() external view returns (uint256[] memory) {
        return appchains;
    }

    /// @notice Get the contract's current ETH balance from collected fees
    /// @return The contract's ETH balance in wei
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

    /// @notice Withdraw collected fees from the contract (admin only)
    /// @dev Allows admins to withdraw fees collected from chain additions.
    ///      Can withdraw a specific amount or the entire balance.
    /// @param to Address to send the funds to (cannot be zero address)
    /// @param amount Amount to withdraw in wei (0 to withdraw all available funds)
    function withdrawFees(address payable to, uint256 amount) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (to == address(0)) revert ZeroAddress();

        uint256 withdrawAmount = amount == 0 ? address(this).balance : amount;
        if (withdrawAmount > address(this).balance) {
            revert InsufficientFee(withdrawAmount, address(this).balance);
        }

        (bool success,) = to.call{value: withdrawAmount}("");
        require(success, "Transfer failed");
    }

    /// @notice Set a new factory contract address (admin only)
    /// @dev Updates the factory and recalculates the proxy bytecode hash for address computations.
    /// @param newFactory The address of the new factory contract
    function setFactory(address newFactory) external onlyRole(DEFAULT_ADMIN_ROLE) {
        factory = IAppchainFactory(newFactory);
        sequencingChainProxyBytecodeHash = keccak256(factory.getProxyBytecode());
    }

    /// @notice Authorize contract upgrades (admin only)
    /// @dev Required by UUPSUpgradeable. Only admins can upgrade this contract.
    /// @param newImplementation The address of the new implementation (unused but required by interface)
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}

    /// @notice Set a custom contract address override for a specific chain ID (admin only)
    /// @dev Allows admins to specify non-deterministic addresses for specific chains.
    ///      Useful for legacy chains until they are migrated to the new architecture.
    /// @param chainId The chain ID to set an override for
    /// @param contractOverride The contract address to use instead of the deterministic address
    function setChainOverride(uint256 chainId, address contractOverride) external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(contractOverride)
        }
        if (codeSize == 0) {
            revert ChainNotFound(chainId);
        }

        appchainContractOverrides[chainId] = contractOverride;
    }
}
