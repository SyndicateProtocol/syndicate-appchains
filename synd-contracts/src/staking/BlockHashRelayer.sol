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

/**
 * @title BlockHashRelayer
 * @notice Contract for relaying Ethereum and Base block hashes to the L3 staking appchain
 * @dev This contract bridges block hash data from L1 (Ethereum/Base) to L3 (staking appchain)
 *      using Arbitrum's retryable ticket mechanism. It's essential for cross-chain data verification.
 */
contract BlockHashRelayer is AccessControl {
    /// @notice L1Block precompile address on Base/Optimism stack
    /// @dev See https://specs.optimism.io/protocol/predeploys.html#overview
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;

    /// @notice Arbitrum Inbox contract for creating retryable tickets
    /// @dev Used to send cross-chain messages to the staking appchain
    IArbInbox public immutable arbInbox;

    /// @notice SYND token contract for payment of cross-chain transactions
    /// @dev Users must approve this contract to spend SYND tokens
    IERC20 public immutable syndToken;

    /// @notice Gas limit for retryable ticket creation (default: 100,000)
    /// @dev Admin can adjust this based on gas requirements
    uint256 gasLimit = 100_000;

    /// @notice Maximum fee per gas for retryable ticket (default: 0.1 gwei)
    /// @dev Admin can adjust this based on network conditions
    uint256 maxFeePerGas = 0.1 gwei;

    /// @notice Error thrown when user doesn't have sufficient SYND allowance
    /// @dev User must approve this contract to spend SYND tokens
    error InsufficientAllowance(uint256 allowance, uint256 amount);

    /**
     * @notice Constructs the relayer contract
     * @param _arbInbox The Arbitrum Inbox contract for the staking appchain (on the settlement chain)
     * @param _syndToken The SYND token contract (on the settlement chain)
     * @param admin The address to be granted admin privileges
     */
    constructor(IArbInbox _arbInbox, IERC20 _syndToken, address admin) {
        arbInbox = _arbInbox;
        syndToken = _syndToken;

        //pre-approve the arbitrum bridge to take any SYND sent to this contract
        syndToken.approve(address(arbInbox), type(uint256).max);

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    /**
     * @notice Sends Ethereum and Base block hashes to the L3 contract with default gas parameters
     * @dev Convenience function that uses the default gas parameters set by admin
     * @param _gasArchive The address of the GasArchive contract on the staking appchain
     */
    function sendBlockHashes(address _gasArchive) external {
        sendBlockHashes(_gasArchive, gasLimit, maxFeePerGas);
    }

    /**
     * @notice Sends Ethereum and Base block hashes to the L3 contract
     * @dev This function:
     *      1. Collects SYND tokens from caller to cover gas costs
     *      2. Gets current Ethereum block hash via L1Block precompile
     *      3. Gets current Base block hash via blockhash() function
     *      4. Creates retryable ticket to GasArchive contract on L3
     * @param _gasArchive The address of the GasArchive contract on the staking appchain
     * @param _gasLimit The gas limit for the retryable ticket
     * @param _maxFeePerGas The maximum fee per gas for the retryable ticket
     * @custom:example User calls with 1000 SYND, gasLimit=100000, maxFeePerGas=0.1gwei
     */
    function sendBlockHashes(address _gasArchive, uint256 _gasLimit, uint256 _maxFeePerGas) public {
        uint256 syndAmount = _gasLimit * _maxFeePerGas;
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

    /**
     * @notice Set gas parameters for retryable ticket creation
     * @dev Admin function to adjust gas costs based on network conditions
     * @param _gasLimit The new gas limit for retryable tickets
     * @param _maxFeePerGas The new maximum fee per gas for retryable tickets
     */
    function setGasParameters(uint256 _gasLimit, uint256 _maxFeePerGas) external onlyRole(DEFAULT_ADMIN_ROLE) {
        gasLimit = _gasLimit;
        maxFeePerGas = _maxFeePerGas;
    }
}
