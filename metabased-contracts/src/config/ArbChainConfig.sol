// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

/**
 * @title ArbChainConfig
 * @dev Configuration contract for settlement chain parameters
 */
contract ArbChainConfig is Initializable, Ownable {
    // Events
    event RollupOwnerUpdated(address indexed newRollupOwner);
    event DefaultSequencingChainRpcUrlUpdated(string newRpcUrl);

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

    /**
     * @dev Constructor for the implementation contract
     * This is only used when deploying the implementation contract
     * It will not be called when deploying proxies
     */
    constructor() Ownable(msg.sender) {}

    /**
     * @dev Initializer function - replaces constructor for proxy pattern
     * @param owner The address of the contract owner
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
     */
    function initialize(
        address owner,
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
        string memory appchainBlockExplorerUrl
    ) public initializer {
        // Set the configuration parameters
        require(owner != address(0), "Owner cannot be zero address");
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

        // Initialize the Ownable contract
        _transferOwnership(owner);
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
}
