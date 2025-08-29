// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title ISyndicateSequencingChain
/// @notice Interface for the SyndicateSequencingChain contract
interface ISyndicateSequencingChain {
    /// @notice The ID of the App chain that this contract is sequencing transactions for
    function appchainId() external view returns (uint256);

    /// @notice Contract nonce used to calculate the request id
    function contractNonce(address sender) external view returns (uint256);

    /// @notice Offset used for address aliasing
    function OFFSET() external view returns (uint160);

    /// @notice Utility function that converts the address in the sequencing chain
    /// that submitted a tx to the inbox to the msg.sender viewed in the appchain.
    /// @param seqAddress the address in the sequencing chain that triggered the tx to appchain
    /// @return appAddress appchain address as viewed in msg.sender
    function applyAlias(address seqAddress) external view returns (address);

    /// @notice Utility function that converts the msg.sender viewed in the appchain
    /// to the address in the sequencing chain that submitted a tx to the inbox.
    /// @param appAddress appchain address as viewed in msg.sender
    /// @return seqAddress the address in the sequencing chain that triggered the tx to appchain
    function undoAlias(address appAddress) external view returns (address);

    /// @notice Send a contract transaction to the appchain using applyAlias to alias msg.sender.
    /// @param gasLimit appchain gas limit
    /// @param maxFeePerGas appchain max gas price
    /// @param to appchain destination address or zero to deploy a contract
    /// @param value appchain tx value
    /// @param data appchain tx calldata
    /// @return requestId the request id used to determine the appchain tx hash
    function sendContractTransaction(
        uint64 gasLimit,
        uint256 maxFeePerGas,
        address to,
        uint256 value,
        bytes calldata data
    ) external returns (uint256);

    /// @notice Send an unsigned transaction to the appchain using applyAlias to alias msg.sender.
    /// @param gasLimit appchain gas limit
    /// @param maxFeePerGas appchain max gas price
    /// @param to appchain destination address or zero to deploy a contract
    /// @param value appchain tx value
    /// @param data appchain tx calldata
    function sendUnsignedTransaction(
        uint64 gasLimit,
        uint256 maxFeePerGas,
        uint256 nonce,
        address to,
        uint256 value,
        bytes calldata data
    ) external;

    /// @notice Processes a compressed batch of signed transactions.
    /// @param data The compressed transaction data.
    function processTransactionsCompressed(bytes calldata data) external;

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    function processTransaction(bytes calldata data) external;

    /// @notice Processes multiple signed transactions in bulk.
    /// @param data An array of transaction data.
    function processTransactionsBulk(bytes[] calldata data) external;
}
