// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

/**
 * @title ArbChainConfig
 * @dev Configuration contract for settlement chain parameters
 */
contract ArbChainConfig is Initializable {
    // Events
    event RollupOwnerUpdated(address indexed newRollupOwner);
    event DefaultSequencingChainRpcUrlUpdated(string newRpcUrl);
    event AllowedSettlementAddressesUpdated(address[] newAllowedSettlementAddresses);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    address public owner;
    address public ARBITRUM_BRIDGE_ADDRESS;
    address public ARBITRUM_INBOX_ADDRESS;
    address public SEQUENCING_CONTRACT_ADDRESS;
    bool public ARBITRUM_IGNORE_DELAYED_MESSAGES;
    uint256 public CHAIN_ID;
    uint256 public SEQUENCING_CHAIN_ID;
    uint256 public SETTLEMENT_DELAY;
    uint256 public SETTLEMENT_START_BLOCK;
    uint256 public SEQUENCING_START_BLOCK;

    address public ROLLUP_OWNER;
    string public DEFAULT_SEQUENCING_CHAIN_RPC_URL;
    string public APPCHAIN_BLOCK_EXPLORER_URL;
    address[] public ALLOWED_SETTLEMENT_ADDRESSES;

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
     * @param arbitrumIgnoreDelayedMessages Whether to ignore delayed messages
     * @param settlementDelay Delay for settlement
     * @param settlementStartBlock Starting block for settlement
     * @param sequencingContractAddress Address of the sequencing contract
     * @param sequencingStartBlock Starting block for sequencing
     * @param rollupOwner Initial rollup owner
     * @param sequencingChainRpcUrl Default RPC URL for the sequencing chain
     * @param appchainBlockExplorerUrl URL for the appchain block explorer
     * @param allowedSettlementAddresses Array of addresses allowed for settlement
     */
    function initialize(
        address _owner,
        uint256 chainId,
        uint256 sequencingChainId,
        address arbitrumBridgeAddress,
        address arbitrumInboxAddress,
        bool arbitrumIgnoreDelayedMessages,
        uint256 settlementDelay,
        uint256 settlementStartBlock,
        address sequencingContractAddress,
        uint256 sequencingStartBlock,
        address rollupOwner,
        string memory sequencingChainRpcUrl,
        string memory appchainBlockExplorerUrl,
        address[] memory allowedSettlementAddresses
    ) public initializer {
        // Set the configuration parameters
        require(_owner != address(0), "Owner cannot be zero address");
        require(chainId != 0, "Chain ID cannot be zero");
        require(sequencingChainId != 0, "Sequencing chain ID cannot be zero");
        require(arbitrumBridgeAddress != address(0), "Arbitrum bridge address cannot be zero");
        require(arbitrumInboxAddress != address(0), "Arbitrum inbox address cannot be zero");
        require(sequencingContractAddress != address(0), "Sequencing contract address cannot be zero");
        require(rollupOwner != address(0), "Rollup owner cannot be zero address");

        CHAIN_ID = chainId;
        SEQUENCING_CHAIN_ID = sequencingChainId;
        ARBITRUM_BRIDGE_ADDRESS = arbitrumBridgeAddress;
        ARBITRUM_INBOX_ADDRESS = arbitrumInboxAddress;
        ARBITRUM_IGNORE_DELAYED_MESSAGES = arbitrumIgnoreDelayedMessages;
        SETTLEMENT_DELAY = settlementDelay;
        SETTLEMENT_START_BLOCK = settlementStartBlock;
        SEQUENCING_CONTRACT_ADDRESS = sequencingContractAddress;
        SEQUENCING_START_BLOCK = sequencingStartBlock;

        ROLLUP_OWNER = rollupOwner;
        DEFAULT_SEQUENCING_CHAIN_RPC_URL = sequencingChainRpcUrl;
        APPCHAIN_BLOCK_EXPLORER_URL = appchainBlockExplorerUrl;
        ALLOWED_SETTLEMENT_ADDRESSES = allowedSettlementAddresses;

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

    /**
     * @dev Update the ROLLUP_OWNER
     * @param newRollupOwner The new address for ROLLUP_OWNER
     */
    function updateRollupOwner(address newRollupOwner) external onlyOwner {
        require(newRollupOwner != address(0), "New rollup owner cannot be zero address");
        ROLLUP_OWNER = newRollupOwner;
        emit RollupOwnerUpdated(newRollupOwner);
    }

    /**
     * @dev Update DEFAULT_SEQUENCING_CHAIN_RPC_URL
     * @param newRpcUrl The new RPC URL for sequencing chain
     */
    function updateDefaultSequencingChainRpcUrl(string calldata newRpcUrl) external onlyOwner {
        DEFAULT_SEQUENCING_CHAIN_RPC_URL = newRpcUrl;
        emit DefaultSequencingChainRpcUrlUpdated(newRpcUrl);
    }

    /**
     * @dev Update ALLOWED_SETTLEMENT_ADDRESSES
     * @param newAllowedSettlementAddresses The new array of addresses allowed for settlement
     */
    function updateAllowedSettlementAddresses(address[] calldata newAllowedSettlementAddresses) external onlyOwner {
        ALLOWED_SETTLEMENT_ADDRESSES = newAllowedSettlementAddresses;
        emit AllowedSettlementAddressesUpdated(newAllowedSettlementAddresses);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");

        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}
