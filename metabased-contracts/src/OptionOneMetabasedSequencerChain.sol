// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {OptionOneSequencingModuleChecker} from "./OptionOneSequencingModuleChecker.sol";

/// @title OptionOneMetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a consolidated module approach
/// to determine who is allowed to sequence and what calldata is allowed.
contract OptionOneMetabasedSequencerChain is OptionOneSequencingModuleChecker {
    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Constructs the OptionOneMetabasedSequencerChain contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    constructor(uint256 _l3ChainId) OptionOneSequencingModuleChecker() {
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(_l3ChainId != 0, "L3 chain ID cannot be 0");
        l3ChainId = _l3ChainId;
    }

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    function processTransactionRaw(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    function processTransaction(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    function processBulkTransactions(bytes[] calldata data) external onlyWhenAllowed(msg.sender) {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            _revertForUnallowedCalldata(data[i]);

            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _data) private pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
