// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

/**
 * @title ArbChainConfig
 * @dev Configuration contract for settlement chain parameters
 */
contract ArbChainConfig is Initializable {
    // Events
    //#olympix-ignore-missing-events-assertion
    event DefaultSequencingChainWsRpcUrlUpdated(string newWsRpcUrl);
    //#olympix-ignore-missing-events-assertion
    event AppchainBlockExplorerUrlUpdated(string newUrl);
    //#olympix-ignore-missing-events-assertion
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    //#olympix-ignore-missing-events-assertion
    event Migration(
        uint256 setStartBlock,
        uint256 seqStartBlock,
        uint256 indexed batchAcc,
        uint256 batchCount,
        uint256 indexed delayedMsgsAcc,
        uint256 delayedMsgsCount,
        uint256 indexed appchainBlockHash,
        bytes genesisConfig
    );

    address public owner;

    // ======== IMMUTABLE CONFIGURATION PARAMETERS ========
    // These parameters cannot be changed after initialization
    address public INITIAL_APPCHAIN_OWNER;
    address public ARBITRUM_BRIDGE_ADDRESS;
    address public ARBITRUM_INBOX_ADDRESS;
    address public SEQUENCING_CONTRACT_ADDRESS;
    uint256 public CHAIN_ID;
    uint256 public SEQUENCING_CHAIN_ID;
    uint256 public SETTLEMENT_DELAY;
    // NOTE: SET/SEQ start blocks can be changed in the event of a migration
    uint256 public SETTLEMENT_START_BLOCK;
    uint256 public SEQUENCING_START_BLOCK;

    // ======== MUTABLE CONFIGURATION PARAMETERS ========
    // These parameters can be updated by the contract owner
    string public DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL;
    string public APPCHAIN_BLOCK_EXPLORER_URL;

    // Migration-only data
    uint256 public MIGRATED_BATCH_ACC;
    uint256 public MIGRATED_BATCH_COUNT;
    uint256 public MIGRATED_DELAYED_MSGS_ACC;
    uint256 public MIGRATED_DELAYED_MSGS_COUNT;
    uint256 public MIGRATED_APPCHAIN_BLOCK_HASH;
    bytes public MIGRATED_GENESIS_CONFIG;

    /**
     * @dev Constructor for the implementation contract
     * This is only used when deploying the implementation contract
     * It will not be called when deploying proxies
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @dev Initializer function - replaces constructor for proxy pattern
     * @param _owner The address of the contract owner
     * @param chainId The chain ID
     * @param sequencingChainId The ID of the sequencing chain
     * @param arbitrumBridgeAddress Address of the Arbitrum bridge
     * @param arbitrumInboxAddress Address of the Arbitrum inbox
     * @param settlementDelay Delay for settlement
     * @param settlementStartBlock Starting block for settlement
     * @param sequencingContractAddress Address of the sequencing contract
     * @param sequencingStartBlock Starting block for sequencing
     * @param initialAppchainOwner Initial appchain owner
     * @param sequencingChainWsRpcUrl Default RPC URL for the sequencing chain
     * @param appchainBlockExplorerUrl URL for the appchain block explorer
     */
    function initialize(
        address _owner,
        uint256 chainId,
        uint256 sequencingChainId,
        address arbitrumBridgeAddress,
        address arbitrumInboxAddress,
        uint256 settlementDelay,
        uint256 settlementStartBlock,
        address sequencingContractAddress,
        uint256 sequencingStartBlock,
        address initialAppchainOwner,
        string memory sequencingChainWsRpcUrl,
        string memory appchainBlockExplorerUrl
    ) external initializer {
        // Set the configuration parameters
        require(_owner != address(0), "Owner cannot be zero address");
        require(chainId != 0, "Chain ID cannot be zero");
        require(sequencingChainId != 0, "Sequencing chain ID cannot be zero");
        require(arbitrumBridgeAddress != address(0), "Arbitrum bridge address cannot be zero");
        require(arbitrumInboxAddress != address(0), "Arbitrum inbox address cannot be zero");
        require(sequencingContractAddress != address(0), "Sequencing contract address cannot be zero");
        require(initialAppchainOwner != address(0), "Initial appchain owner cannot be zero address");

        // Set immutable configuration parameters
        CHAIN_ID = chainId;
        SEQUENCING_CHAIN_ID = sequencingChainId;
        ARBITRUM_BRIDGE_ADDRESS = arbitrumBridgeAddress;
        ARBITRUM_INBOX_ADDRESS = arbitrumInboxAddress;
        SETTLEMENT_DELAY = settlementDelay;
        SETTLEMENT_START_BLOCK = settlementStartBlock;
        SEQUENCING_CONTRACT_ADDRESS = sequencingContractAddress;
        SEQUENCING_START_BLOCK = sequencingStartBlock;

        // Set mutable configuration parameters
        INITIAL_APPCHAIN_OWNER = initialAppchainOwner;
        DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL = sequencingChainWsRpcUrl;
        APPCHAIN_BLOCK_EXPLORER_URL = appchainBlockExplorerUrl;

        // Initialize the Ownable contract
        _transferOwnership(_owner);
    }

    /**
     * @dev Modifier to check if the caller is the owner
     */
    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    function migration(
        uint256 _setStartBlock,
        uint256 _seqStartBlock,
        uint256 _batchAcc,
        uint256 _batchCount,
        uint256 _delayedMsgsAcc,
        uint256 _delayedMsgsCount,
        uint256 _appchainBlockHash,
        bytes calldata _genesisConfig
    ) external onlyOwner {
        SETTLEMENT_START_BLOCK = _setStartBlock;
        SEQUENCING_START_BLOCK = _seqStartBlock;
        MIGRATED_BATCH_ACC = _batchAcc;
        MIGRATED_BATCH_COUNT = _batchCount;
        MIGRATED_DELAYED_MSGS_ACC = _delayedMsgsAcc;
        MIGRATED_DELAYED_MSGS_COUNT = _delayedMsgsCount;
        MIGRATED_APPCHAIN_BLOCK_HASH = _appchainBlockHash;
        MIGRATED_GENESIS_CONFIG = _genesisConfig;
        emit Migration(
            SETTLEMENT_START_BLOCK,
            SEQUENCING_START_BLOCK,
            MIGRATED_BATCH_ACC,
            MIGRATED_BATCH_COUNT,
            MIGRATED_DELAYED_MSGS_ACC,
            MIGRATED_DELAYED_MSGS_COUNT,
            MIGRATED_APPCHAIN_BLOCK_HASH,
            MIGRATED_GENESIS_CONFIG
        );
    }

    /**
     * @dev Update DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL
     * @param newWsRpcUrl The new RPC URL for sequencing chain
     */
    //#olympix-ignore-owner-single-point-of-failure
    function updateDefaultSequencingChainWsRpcUrl(string calldata newWsRpcUrl) external onlyOwner {
        DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL = newWsRpcUrl;
        emit DefaultSequencingChainWsRpcUrlUpdated(newWsRpcUrl);
    }

    /**
     * @dev Update APPCHAIN_BLOCK_EXPLORER_URL
     * @param newUrl The new URL for the appchain block explorer
     */
    //#olympix-ignore-owner-single-point-of-failure
    function updateAppchainBlockExplorerUrl(string calldata newUrl) external onlyOwner {
        APPCHAIN_BLOCK_EXPLORER_URL = newUrl;
        emit AppchainBlockExplorerUrlUpdated(newUrl);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    //#olympix-ignore-owner-single-point-of-failure
    function transferOwnership(address newOwner) public virtual onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");

        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        require(newOwner != address(0), "New owner cannot be zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}
