// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AllowlistSequencingModule} from "./AllowlistSequencingModule.sol";
import {ISyndicateSequencingChain} from "../interfaces/ISyndicateSequencingChain.sol";

/**
 * @title WalletPoolWrapperModule
 * @dev This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.
 */
contract WalletPoolWrapperModule is AllowlistSequencingModule {
    event WalletPoolWrapperTransactionSent(address indexed from, address indexed SyndicateSequencingChain);
    event WalletPoolWrapperBulkTransactionsSent(
        address indexed from, address indexed SyndicateSequencingChain, uint256 count
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
     * @param _SyndicateSequencingChain The syndicate sequencer chain address
     */
    modifier SyndicateSequencingChainNotZero(address _SyndicateSequencingChain) {
        if (_SyndicateSequencingChain == address(0)) {
            revert ZeroSequencerAddressNotAllowed();
        }
        _;
    }

    /**
     * @dev Function to process a transaction.
     * @param _SyndicateSequencingChain The syndicate sequencer chain address
     * @param data The transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processTransactionUncompressed(address _SyndicateSequencingChain, bytes calldata data)
        external
        onlyAllowed
        SyndicateSequencingChainNotZero(_SyndicateSequencingChain)
    {
        // Forward the transaction to the syndicate sequencer chain
        ISyndicateSequencingChain(_SyndicateSequencingChain).processTransactionUncompressed(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _SyndicateSequencingChain);
    }

    /**
     * @dev Function to process a raw transaction.
     * @param _SyndicateSequencingChain The syndicate sequencer chain address
     * @param data The transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processTransaction(address _SyndicateSequencingChain, bytes calldata data)
        external
        onlyAllowed
        SyndicateSequencingChainNotZero(_SyndicateSequencingChain)
    {
        // Forward the transaction to the syndicate sequencer chain
        ISyndicateSequencingChain(_SyndicateSequencingChain).processTransaction(data);

        // Emit an event indicating the transaction was sent
        emit WalletPoolWrapperTransactionSent(msg.sender, _SyndicateSequencingChain);
    }

    /**
     * @dev Function to process bulk transactions.
     * @param _SyndicateSequencingChain The syndicate sequencer chain address
     * @param data The array of transaction data to process.
     */
    //#olympix-ignore-reentrancy-events
    function processTransactionsBulk(address _SyndicateSequencingChain, bytes[] calldata data)
        external
        onlyAllowed
        SyndicateSequencingChainNotZero(_SyndicateSequencingChain)
    {
        // Forward the transactions to the syndicate sequencer chain
        ISyndicateSequencingChain(_SyndicateSequencingChain).processTransactionsBulk(data);

        // Emit an event indicating the bulk transactions were sent
        emit WalletPoolWrapperBulkTransactionsSent(msg.sender, _SyndicateSequencingChain, data.length);
    }
}
