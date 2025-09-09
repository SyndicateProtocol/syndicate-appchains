// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {GasArchive} from "./GasArchive.sol";

/// @notice Minimal interface for the L1Block precompile on Base/Optimism stack
interface IL1Block {
    function hash() external view returns (bytes32);
}

/// @notice Minimal interface for ArbInbox
interface IArbInbox {
    function unsafeCreateRetryableTicket(
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

contract BlockHashMessenger {
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;

    IArbInbox public immutable arbBridge;
    address public immutable gasArchive;
    IERC20 public immutable syndToken;

    constructor(address _l3Target, IArbInbox _arbBridge, IERC20 _syndToken) {
        arbBridge = _arbBridge;
        gasArchive = _l3Target;
        syndToken = _syndToken;

        //pre-approve the arbitrum bridge to take any SYND sent to this contract
        syndToken.approve(address(arbBridge), type(uint256).max);
    }

    /// @notice Sends Ethereum and Base block hashes to the L3 contract
    function sendBlockHashes() external {
        syndToken.transferFrom(msg.sender, address(this), syndToken.allowance(msg.sender, address(this)));

        bytes32 ethBlockHash = IL1Block(L1_BLOCK_ADDRESS).hash();
        bytes32 baseBlockHash = blockhash(block.number - 1);

        // Encode call to L3 contract
        bytes memory callData = abi.encodeCall(GasArchive.setLastKnownBlockHashes, (ethBlockHash, baseBlockHash));

        address destination = gasArchive;
        uint256 l2CallValue = 0; // TODO check if correct
        uint256 maxSubmissionCost = 0; // Always 0 for custom gas token chains
        address refundAddress = msg.sender;
        uint256 gasLimit = 210000;
        uint256 maxFeePerGas = 1 gwei;
        uint256 amount = 0;

        arbBridge.unsafeCreateRetryableTicket(
            destination,
            l2CallValue,
            maxSubmissionCost,
            refundAddress,
            refundAddress,
            gasLimit,
            maxFeePerGas,
            amount,
            callData
        );
    }
}
