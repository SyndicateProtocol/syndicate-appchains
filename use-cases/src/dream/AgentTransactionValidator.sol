// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.29;

import {RLPTxDecoder} from "./RLP/RLPTxDecoder.sol";

/// @title ISyndicateSequencingChain
/// @notice Interface for the SyndicateSequencingChain contract

interface ISyndicateSequencingChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransactionUncompressed(bytes calldata data) external;

    /// @notice Process a raw transaction
    /// @param data The transaction data to process
    function processTransactionRaw(bytes calldata data) external;

    /// @notice Process bulk transactions
    /// @param data The array of transaction data to process
    function processBulkTransactions(bytes[] calldata data) external;
}

/// @title IAgentApplication
/// @notice Interface for agent application status checks
interface IAgentApplication {
    /// @notice Check if an agent address is permitted
    /// @param agentAddress The address to check
    /// @return bool indicating if the agent is permitted
    function isPermittedByAddress(address agentAddress) external view returns (bool);

    /// @notice The admin role for the AgentApplication contract
    function ADMIN_ROLE() external view returns (bytes32);

    /// @notice Check if an agent has a role
    /// @dev Returns `true` if `account` has been granted `role`.
    function hasRole(bytes32 role, address account) external view returns (bool);
}

/// @title AgentTransactionValidator
/// @notice Middleware contract that validates transaction's 'from' address before forwarding to sequencer
/// @dev Extracts 'from' address from transaction data and validates against AgentApplication
contract AgentTransactionValidator {
    /// @notice The AgentApplication contract for checking permissions
    IAgentApplication public immutable agentApplication;

    /// @notice The SyndicateSequencingChain contract for forwarding valid transactions
    ISyndicateSequencingChain public immutable sequencerChain;

    error Unauthorized();

    constructor(address _agentApplication, address _sequencerChain) {
        agentApplication = IAgentApplication(_agentApplication);
        sequencerChain = ISyndicateSequencingChain(_sequencerChain);
    }

    /// @notice Process a transaction after validating the 'from' address
    /// @param data The transaction data to process
    function processTransactionUncompressed(bytes calldata data) external {
        address from = RLPTxDecoder.decodeTx(data);

        // Check if the from address is permitted
        if (!agentApplication.isPermittedByAddress(from)) {
            // then must be owner of the AgentApplication contract, otherwise revert
            bytes32 adminRole = agentApplication.ADMIN_ROLE();
            if (!agentApplication.hasRole(adminRole, from)) revert Unauthorized();
        }

        // Forward the transaction to the sequencer chain
        sequencerChain.processTransaction(data);
    }
}
