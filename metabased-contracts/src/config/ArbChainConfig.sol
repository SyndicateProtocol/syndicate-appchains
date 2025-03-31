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

    uint256 public CHAIN_ID;
    bytes32 public TARGET_ROLLUP_TYPE;
    bool public MINE_EMPTY_BLOCKS;
    address public ARBITRUM_BRIDGE_ADDRESS;
    address public ARBITRUM_INBOX_ADDRESS;
    bool public ARBITRUM_IGNORE_DELAYED_MESSAGES;
    uint256 public SETTLEMENT_DELAY;
    uint256 public SETTLEMENT_START_BLOCK;
    address public SEQUENCING_CONTRACT_ADDRESS;
    uint256 public SEQUENCING_START_BLOCK;

    address public ROLLUP_OWNER;
    string public DEFAULT_SEQUENCING_CHAIN_RPC_URL;

    /**
     * @dev Constructor for the implementation contract
     * This is only used when deploying the implementation contract
     * It will not be called when deploying proxies
     */
    constructor() Ownable(msg.sender) {}

    /**
     * @dev Initializer function - replaces constructor for proxy pattern
     * @param chainId The chain ID
     * @param targetRollupType The type of rollup
     * @param mineEmptyBlocks Whether to mine empty blocks
     * @param arbitrumBridgeAddress Address of the Arbitrum bridge
     * @param arbitrumInboxAddress Address of the Arbitrum inbox
     * @param arbitrumIgnoreDelayedMessages Whether to ignore delayed messages
     * @param settlementDelay Delay for settlement
     * @param settlementStartBlock Starting block for settlement
     * @param sequencingContractAddress Address of the sequencing contract
     * @param sequencingStartBlock Starting block for sequencing
     * @param rollupOwner Initial rollup owner
     * @param sequencingChainRpcUrl Default RPC URL for the sequencing chain
     */
    function initialize(
        uint256 chainId,
        bytes32 targetRollupType,
        bool mineEmptyBlocks,
        address arbitrumBridgeAddress,
        address arbitrumInboxAddress,
        bool arbitrumIgnoreDelayedMessages,
        uint256 settlementDelay,
        uint256 settlementStartBlock,
        address sequencingContractAddress,
        uint256 sequencingStartBlock,
        address rollupOwner,
        string memory sequencingChainRpcUrl
    ) public initializer {
        require(chainId != 0, "Chain ID cannot be zero");
        require(arbitrumBridgeAddress != address(0), "Arbitrum bridge address cannot be zero");
        require(arbitrumInboxAddress != address(0), "Arbitrum inbox address cannot be zero");
        require(sequencingContractAddress != address(0), "Sequencing contract address cannot be zero");
        require(rollupOwner != address(0), "Rollup owner cannot be zero address");

        CHAIN_ID = chainId;
        TARGET_ROLLUP_TYPE = targetRollupType;
        MINE_EMPTY_BLOCKS = mineEmptyBlocks;
        ARBITRUM_BRIDGE_ADDRESS = arbitrumBridgeAddress;
        ARBITRUM_INBOX_ADDRESS = arbitrumInboxAddress;
        ARBITRUM_IGNORE_DELAYED_MESSAGES = arbitrumIgnoreDelayedMessages;
        SETTLEMENT_DELAY = settlementDelay;
        SETTLEMENT_START_BLOCK = settlementStartBlock;
        SEQUENCING_CONTRACT_ADDRESS = sequencingContractAddress;
        SEQUENCING_START_BLOCK = sequencingStartBlock;

        ROLLUP_OWNER = rollupOwner;
        DEFAULT_SEQUENCING_CHAIN_RPC_URL = sequencingChainRpcUrl;

        // Initialize the Ownable contract
        _transferOwnership(msg.sender);
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
