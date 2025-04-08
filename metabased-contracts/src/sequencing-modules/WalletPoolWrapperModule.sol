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
    event WalletPoolWrapperBulkTransactionsSent(
        address indexed from, address indexed metabasedSequencerChain, uint256 count
    );

    error ZeroSequencerAddressNotAllowed();

    /**
     * @dev Constructor that sets the admin address.
     * @param _admin The address of the admin who can modify the allowlist.
     */
    constructor(address _admin) AllowlistSequencingModule(_admin) {}

    /**
     * @dev Modifier to check if the caller is allowed to process transactions.
     */
    modifier onlyAllowed() {
        if (!allowlist[msg.sender]) {
            revert AddressNotAllowed();
        }
        _;
    }

    /**
     * @dev Modifier to check if the metabased sequencer chain address is not zero.
     * @param _metabasedSequencerChain The metabased sequencer chain address
     */
    modifier metabasedSequencerChainNotZero(address _metabasedSequencerChain) {
        if (_metabasedSequencerChain == address(0)) {
            revert ZeroSequencerAddressNotAllowed();
        }
        _;
    }

    /**
     * @dev Function to process a transaction.
     * @param _metabasedSequencerChain The metabased sequencer chain address
     * @param data The transaction data to process.
     */
    function processTransaction(address _metabasedSequencerChain, bytes calldata data)
        external
        onlyAllowed
        metabasedSequencerChainNotZero(_metabasedSequencerChain)
    {
        // Forward the transaction to the metabased sequencer chain
        IMetabasedSequencerChain(_metabasedSequencerChain).processTransaction(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _metabasedSequencerChain);
    }

    /**
     * @dev Function to process a raw transaction.
     * @param _metabasedSequencerChain The metabased sequencer chain address
     * @param data The transaction data to process.
     */
    function processTransactionRaw(address _metabasedSequencerChain, bytes calldata data)
        external
        onlyAllowed
        metabasedSequencerChainNotZero(_metabasedSequencerChain)
    {
        // Forward the transaction to the metabased sequencer chain
        IMetabasedSequencerChain(_metabasedSequencerChain).processTransactionRaw(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _metabasedSequencerChain);
    }

    /**
     * @dev Function to process bulk transactions.
     * @param _metabasedSequencerChain The metabased sequencer chain address
     * @param data The array of transaction data to process.
     */
    function processBulkTransactions(address _metabasedSequencerChain, bytes[] calldata data)
        external
        onlyAllowed
        metabasedSequencerChainNotZero(_metabasedSequencerChain)
    {
        // Forward the transactions to the metabased sequencer chain
        IMetabasedSequencerChain(_metabasedSequencerChain).processBulkTransactions(data);

        // Emit an event indicating the bulk transactions were sent
        emit WalletPoolWrapperBulkTransactionsSent(msg.sender, _metabasedSequencerChain, data.length);
    }
}
