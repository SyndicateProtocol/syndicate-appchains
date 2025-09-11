// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {GasArchive} from "./GasArchive.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

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

contract BlockHashRelayer is AccessControl {
    // see https://specs.optimism.io/protocol/predeploys.html#overview (L1Block address)
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;

    IArbInbox public immutable arbInbox;
    IERC20 public immutable syndToken;

    uint256 gasLimit = 100_000;
    uint256 maxFeePerGas = 0.1 gwei;

    error InsufficientAllowance(uint256 allowance, uint256 amount);

    /// @notice Constructs the relayer contract
    /// @param _arbInbox The Arbitrum Inbox contract for the staking appchain (on the settlement chain)
    /// @param _syndToken The SYND token contract (on the settlement chain)
    constructor(IArbInbox _arbInbox, IERC20 _syndToken, address admin) {
        arbInbox = _arbInbox;
        syndToken = _syndToken;

        //pre-approve the arbitrum bridge to take any SYND sent to this contract
        syndToken.approve(address(arbInbox), type(uint256).max);

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    /// @notice Sends Ethereum and Base block hashes to the L3 contract with default gas parameters
    function sendBlockHashes(address _gasArchive) external {
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
        uint256 blockNumber = block.number - 1;

        // Encode the call to the GasArchive contract
        bytes memory callData =
            abi.encodeCall(GasArchive.setLastKnownBlockHashes, (ethBlockHash, baseBlockHash, blockNumber));

        address destination = _gasArchive;
        uint256 l2CallValue = 0; // the value of the transaction on the rollup - 0 because we don't want to send any tokens to the target
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

    function setGasParameters(uint256 _gasLimit, uint256 _maxFeePerGas) external onlyRole(DEFAULT_ADMIN_ROLE) {
        gasLimit = _gasLimit;
        maxFeePerGas = _maxFeePerGas;
    }
}
