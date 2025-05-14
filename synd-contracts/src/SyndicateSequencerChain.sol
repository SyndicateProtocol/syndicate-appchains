// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

/// @title SyndicateSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture
/// to determine the address that is allowed to sequence and whether the calldata is allowed.
contract SyndicateSequencerChain is SequencingModuleChecker {
    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    uint256 public immutable appChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Constructs the SyndicateSequencerChain contract.
    /// @param _appChainId The ID of the App chain that this contract is sequencing transactions for.
    constructor(uint256 _appChainId) SequencingModuleChecker() {
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(_appChainId != 0, "App chain ID cannot be 0");
        appChainId = _appChainId;
    }

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    function processTransactionRaw(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data) {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data) {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    function processBulkTransactions(bytes[] calldata data) external {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            isAllowed(msg.sender, tx.origin, data[i]);

            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _data) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
