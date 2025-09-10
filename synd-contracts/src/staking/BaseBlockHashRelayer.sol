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

contract BlockHashRelayer {
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;

    IArbInbox public immutable arbInbox;
    IERC20 public immutable syndToken;

    error InsufficientAllowance(uint256 allowance, uint256 amount);

    constructor(IArbInbox _arbInbox, IERC20 _syndToken) {
        arbInbox = _arbInbox;
        syndToken = _syndToken;

        //pre-approve the arbitrum bridge to take any SYND sent to this contract
        syndToken.approve(address(arbInbox), type(uint256).max);
    }

    /// @notice Sends Ethereum and Base block hashes to the L3 contract with default gas parameters
    function sendBlockHashes(address _gasArchive) external {
        uint256 gasLimit = 210000;
        uint256 maxFeePerGas = 0.1 gwei;
        sendBlockHashes(_gasArchive, gasLimit, maxFeePerGas);
    }

    /// @notice Sends Ethereum and Base block hashes to the L3 contract
    function sendBlockHashes(address _gasArchive, uint256 _gasLimit, uint256 _maxFeePerGas) public {
        uint256 syndAmount = _gasLimit * _maxFeePerGas;

        uint256 syndAllowance = syndToken.allowance(msg.sender, address(this));
        if (syndAllowance < syndAmount) revert InsufficientAllowance(syndAllowance, syndAmount);
        syndToken.transferFrom(msg.sender, address(this), syndAmount);

        bytes32 ethBlockHash = IL1Block(L1_BLOCK_ADDRESS).hash();
        bytes32 baseBlockHash = blockhash(block.number - 1);

        // Encode the call to the GasArchive contract
        bytes memory callData = abi.encodeCall(GasArchive.setLastKnownBlockHashes, (ethBlockHash, baseBlockHash));

        address destination = _gasArchive;
        uint256 l2CallValue = 0;
        uint256 maxSubmissionCost = 0; // Always 0 for custom gas token chains
        address refundAddress = msg.sender;

        arbInbox.createRetryableTicket(
            destination,
            l2CallValue,
            maxSubmissionCost,
            refundAddress,
            refundAddress,
            _gasLimit,
            _maxFeePerGas,
            syndAmount,
            callData
        );
    }
}
