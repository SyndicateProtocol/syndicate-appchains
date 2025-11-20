// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {GasArchive} from "./GasArchive.sol";

/// @notice Minimal interface for the L1Block precompile on Base/Optimism stack
interface IL1Block {
    function hash() external view returns (bytes32);
}

/// @notice Minimal interface for ArbInbox
/// @dev https://github.com/OffchainLabs/nitro-contracts/blob/0b8c04e8f5f66fe6678a4f53aa15f23da417260e/src/bridge/Inbox.sol#L261
interface IArbInbox {
    function createRetryableTicket(
        address to,
        uint256 l2CallValue,
        uint256 maxSubmissionCost,
        address excessFeeRefundAddress,
        address callValueRefundAddress,
        uint256 gasLimit,
        uint256 maxFeePerGas,
        uint256 tokenTotalFeeAmount,
        bytes calldata data
    ) external returns (uint256);
}

/**
 * @title BlockHashRelayer
 * @notice Contract for relaying Ethereum and Base block hashes to the L3 staking appchain
 * @dev This contract bridges block hash data from Base(settlement chain) to L3 (staking appchain)
 *      using Arbitrum's retryable ticket mechanism. It's essential for cross-chain data verification.
 */
contract BlockHashRelayer {
    /// @notice L1Block precompile address on Base/Optimism stack
    /// @dev See https://specs.optimism.io/protocol/predeploys.html#overview
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;

    /// @notice Arbitrum Inbox contract for creating retryable tickets
    /// @dev Used to send cross-chain messages to the staking appchain
    IArbInbox public immutable arbInbox;

    /// @notice SYND token contract for payment of cross-chain transactions
    /// @dev Users must approve this contract to spend SYND tokens
    IERC20 public immutable syndToken;

    /// @notice Error thrown when user doesn't have sufficient SYND allowance
    /// @dev User must approve this contract to spend SYND tokens
    error InsufficientAllowance(uint256 allowance, uint256 amount);

    /**
     * @notice Constructs the relayer contract
     * @param _arbInbox The Arbitrum Inbox contract for the staking appchain (on the settlement chain)
     * @param _syndToken The SYND token contract (on the settlement chain)
     */
    constructor(IArbInbox _arbInbox, IERC20 _syndToken) {
        arbInbox = _arbInbox;
        syndToken = _syndToken;
    }

    /**
     * @notice Sends Ethereum and Base block hashes to the L3 contract
     * @dev This function:
     *      1. Collects SYND tokens from caller to cover gas costs
     *      2. Gets current Ethereum block hash via L1Block precompile
     *      3. Gets current Base block hash via blockhash() function
     *      4. Creates retryable ticket to GasArchive contract on L3
     * @param gasArchive The address of the GasArchive contract on the staking appchain
     * @param gasLimit The gas limit for the retryable ticket
     * @param maxFeePerGas The maximum fee per gas for the retryable ticket
     * @custom:example User calls with 1000 SYND, gasLimit=100000, maxFeePerGas=0.1gwei
     */
    function sendBlockHashes(address gasArchive, uint256 gasLimit, uint256 maxFeePerGas) public {
        uint256 syndAmount = gasLimit * maxFeePerGas;
        syndToken.transferFrom(msg.sender, address(arbInbox), syndAmount);

        // Encode the call to the GasArchive contract
        bytes memory callData = abi.encodeCall(
            GasArchive.sendBlockHashes, (IL1Block(L1_BLOCK_ADDRESS).hash(), blockhash(block.number - 1))
        );

        arbInbox.createRetryableTicket(
            gasArchive, // destination address
            0, // tx value
            0, // max submission cost - always 0 for custom gas token chains
            msg.sender, // refund address
            msg.sender, // refund address
            gasLimit,
            maxFeePerGas,
            syndAmount,
            callData
        );
    }
}
