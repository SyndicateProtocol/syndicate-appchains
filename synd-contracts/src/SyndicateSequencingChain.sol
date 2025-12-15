// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {SyndicateSequencingChainBase} from "./SyndicateSequencingChainBase.sol";

/// @title SyndicateSequencingChain
/// @notice Standard sequencing chain contract that emits events for off-chain processing
/// @dev Inherits all functionality from SyndicateSequencingChainBase.
/// Uses the default _transactionProcessed implementation which simply emits TransactionProcessed events.
contract SyndicateSequencingChain is SyndicateSequencingChainBase {
    /// @notice Constructs the SyndicateSequencingChain contract.
    /// @param _appchainId The ID of the App chain that this contract is sequencing transactions for.
    constructor(uint256 _appchainId) SyndicateSequencingChainBase(_appchainId) {}
}
