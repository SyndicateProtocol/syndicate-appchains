// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AllowlistSequencingModule} from "./AllowlistSequencingModule.sol";
import {ISyndicateSequencerChain} from "../interfaces/ISyndicateSequencerChain.sol";

/**
 * @title WalletPoolWrapperModule
 * @dev This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.
 */
contract WalletPoolWrapperModule is AllowlistSequencingModule {
    event WalletPoolWrapperTransactionSent(address indexed from, address indexed syndicateSequencerChain);
    event WalletPoolWrapperBulkTransactionsSent(
        address indexed from, address indexed syndicateSequencerChain, uint256 count
    );

    error ZeroSequencerAddressNotAllowed();

    /**
     * @dev Constructor that sets the admin address.
     * @dev param _admin is validated in AllowlistSequencingModule constructor.
     * @param _admin The address of the admin who can modify the allowlist.
     */
    //#olympix-ignore-no-parameter-validation-in-constructor
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
     * @dev Modifier to check if the syndicate sequencer chain address is not zero.
     * @param _syndicateSequencerChain The syndicate sequencer chain address
     */
    modifier syndicateSequencerChainNotZero(address _syndicateSequencerChain) {
        if (_syndicateSequencerChain == address(0)) {
            revert ZeroSequencerAddressNotAllowed();
        }
        _;
    }

    /**
     * @dev Function to process a transaction.
     * @param _syndicateSequencerChain The syndicate sequencer chain address
     * @param data The transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processTransaction(address _syndicateSequencerChain, bytes calldata data)
        external
        onlyAllowed
        syndicateSequencerChainNotZero(_syndicateSequencerChain)
    {
        // Forward the transaction to the syndicate sequencer chain
        ISyndicateSequencerChain(_syndicateSequencerChain).processTransaction(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _syndicateSequencerChain);
    }

    /**
     * @dev Function to process a raw transaction.
     * @param _syndicateSequencerChain The syndicate sequencer chain address
     * @param data The transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processTransactionRaw(address _syndicateSequencerChain, bytes calldata data)
        external
        onlyAllowed
        syndicateSequencerChainNotZero(_syndicateSequencerChain)
    {
        // Forward the transaction to the syndicate sequencer chain
        ISyndicateSequencerChain(_syndicateSequencerChain).processTransactionRaw(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _syndicateSequencerChain);
    }

    /**
     * @dev Function to process bulk transactions.
     * @param _syndicateSequencerChain The syndicate sequencer chain address
     * @param data The array of transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processBulkTransactions(address _syndicateSequencerChain, bytes[] calldata data)
        external
        onlyAllowed
        syndicateSequencerChainNotZero(_syndicateSequencerChain)
    {
        // Forward the transactions to the syndicate sequencer chain
        ISyndicateSequencerChain(_syndicateSequencerChain).processBulkTransactions(data);

        // Emit an event indicating the bulk transactions were sent
        emit WalletPoolWrapperBulkTransactionsSent(msg.sender, _syndicateSequencerChain, data.length);
    }
}
