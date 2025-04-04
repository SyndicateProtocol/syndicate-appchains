// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AllowlistSequencingModule} from "./AllowlistSequencingModule.sol";
import {IMetabasedSequencerChain} from "../interfaces/IMetabasedSequencerChain.sol";

/**
 * @title WalletPoolWrapperModule
 * @dev This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.
 */
contract WalletPoolWrapperModule is AllowlistSequencingModule {
    event WalletPoolWrapperTransactionSent(address indexed from, address indexed metabasedSequencerChain);

    /**
     * @dev Constructor that sets the admin address.
     * @param _admin The address of the admin who can modify the allowlist.
     */
    constructor(address _admin) AllowlistSequencingModule(_admin) {}

    /**
     * @dev Function to process a transaction.
     * @notice metabased sequencer chain address
     * @param data The transaction data to process.
     */
    function processTransaction(address _metabasedSequencerChain, bytes calldata data) external {
        // Check if the sender is allowed to process the transaction
        if (!allowlist[msg.sender]) {
            revert AddressNotAllowed();
        }

        if (_metabasedSequencerChain == address(0)) {
            revert AddressNotAllowed();
        }

        // Forward the transaction to the metabased sequencer chain
        IMetabasedSequencerChain(_metabasedSequencerChain).processTransaction(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _metabasedSequencerChain);
    }
}
