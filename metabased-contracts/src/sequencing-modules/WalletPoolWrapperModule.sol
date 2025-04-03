// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AllowlistSequencingModule} from "./AllowlistSequencingModule.sol";
import {IMetabasedSequencerChain} from "../interfaces/IMetabasedSequencerChain.sol";

/**
 * @title WalletPoolWrapperModule
 * @dev This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.
 */
contract WalletPoolWrapperModule is AllowlistSequencingModule {
    /// @notice metabased sequencer chain address
    address public immutable metabasedSequencerChain;

    event WalletPoolWrapperTransactionSent(address indexed from);

    /**
     * @dev Constructor that sets the admin address.
     * @param _admin The address of the admin who can modify the allowlist.
     * @param _metabasedSequencerChain The address of the metabased sequencer chain.
     */
    constructor(address _admin, address _metabasedSequencerChain) AllowlistSequencingModule(_admin) {
        if (_metabasedSequencerChain == address(0)) {
            revert AddressNotAllowed();
        }

        metabasedSequencerChain = _metabasedSequencerChain;
    }

    /**
     * @dev Function to process a transaction.
     * @param data The transaction data to process.
     */
    function processTransaction(bytes calldata data) external {
        // Check if the sender is allowed to process the transaction
        if (!allowlist[msg.sender]) {
            revert AddressNotAllowed();
        }

        // Forward the transaction to the metabased sequencer chain
        IMetabasedSequencerChain(metabasedSequencerChain).processTransaction(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender);
    }
}
