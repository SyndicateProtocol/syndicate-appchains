// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

/// @title SyndicateSequencingChain
/// @notice The core contract for sequencing transactions using a modular permission architecture
/// @dev Transaction Lifecycle:
/// 1. Transaction is submitted via processTransaction, processTransactionUncompressed, or processTransactionsBulk methods
/// 2. onlyWhenAllowed modifier checks if the sender and transaction are allowed via SequencingModuleChecker
/// 3. SequencingModuleChecker delegates to the configured permissionRequirementModule (RequireAll/RequireAny)
/// 4. If allowed, a TransactionProcessed event is emitted with the sender and data
/// 5. External systems observe these events to process the transactions on the application chain
/// This design uses events rather than state changes for scalability and gas efficiency
contract SyndicateSequencingChain is SequencingModuleChecker {
    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    uint256 public immutable appchainId;

    /// @notice The factory contract that handles centralized gas tracking
    address public immutable factory;

    /// @notice Emitted when a new transaction is processed
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Constructs the SyndicateSequencingChain contract.
    /// @param _appchainId The ID of the App chain that this contract is sequencing transactions for.
    //#olympix-ignore-missing-revert-reason-tests
    constructor(uint256 _appchainId) SequencingModuleChecker() {
        // chain id zero has no replay protection: https://eips.ethereum.org/EIPS/eip-3788
        require(_appchainId != 0, "App chain ID cannot be 0");
        appchainId = _appchainId;
        factory = msg.sender; // Factory is the deployer
    }

    /// @notice Modifier that tracks gas usage for a function call
    modifier trackGasUsage() {
        uint256 gasStart = gasleft();
        _;
        uint256 gasUsed = gasStart - gasleft();
        // try to track gas usage on the factory contract
        try ISyndicateFactory(factory).trackAppchainGas(appchainId, gasUsed) {
            // Gas tracking successful
        } catch {
            // Silently fail if gas tracking is disabled or fails
            // This ensures transaction processing continues even if gas tracking has issues
        }
    }

    /// @notice Processes a compressed transaction.
    /// @param data The compressed transaction data.
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender, tx.origin, data)
        trackGasUsage
    {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice Processes multiple uncompressed transactions in bulk.
    /// @param data An array of transaction data without prepended zero bytes.
    //#olympix-ignore-required-tx-origin
    function processTransactionUncompressed(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender, tx.origin, data)
        trackGasUsage
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    //#olympix-ignore
    function processTransactionsBulk(bytes[] calldata data) external trackGasUsage {
        uint256 dataCount = data.length;

        // Process all transactions
        uint256 i;
        for (i = 0; i < dataCount; i++) {
            bool isAllowed = isAllowed(msg.sender, tx.origin, data[i]); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
            }
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data with a zero byte prepended
    function prependZeroByte(bytes calldata _data) public pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}

/// @notice Interface for the factory's gas tracking function
interface ISyndicateFactory {
    function trackAppchainGas(uint256 appchainId, uint256 gasUsed) external;
}
