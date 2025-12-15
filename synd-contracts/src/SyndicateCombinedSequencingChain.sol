// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {SyndicateSequencingChainBase} from "./SyndicateSequencingChainBase.sol";

/// @title SyndicateCombinedSequencingChain
/// @notice Sequencing chain with an accumulator for trustless TEE module state proving
/// @dev Extends SyndicateSequencingChainBase by maintaining a hash chain accumulator
/// of all processed transactions. This enables the TEE module to trustlessly prove
/// the appchain state by verifying against the on-chain accumulator.
contract SyndicateCombinedSequencingChain is SyndicateSequencingChainBase {
    /// @notice The accumulator for sequencing data - a hash chain of all transactions
    bytes32[] public sequencingAccumulator;

    /// @notice Constructs the SyndicateCombinedSequencingChain contract.
    /// @param _appchainId The ID of the App chain that this contract is sequencing transactions for.
    constructor(uint256 _appchainId) SyndicateSequencingChainBase(_appchainId) {}

    /// @notice Updates the sequencingAccumulator and emits the TransactionProcessed event
    /// @dev Overrides base implementation to add accumulator tracking before emitting event
    /// @param transaction The encoded transaction data
    function _transactionProcessed(bytes memory transaction) internal override {
        uint256 count = sequencingAccumulator.length;
        bytes32 prevAcc = count > 0 ? sequencingAccumulator[count - 1] : bytes32(0);
        sequencingAccumulator.push(keccak256(abi.encodePacked(prevAcc, transaction)));
        emit TransactionProcessed(msg.sender, transaction);
    }
}
