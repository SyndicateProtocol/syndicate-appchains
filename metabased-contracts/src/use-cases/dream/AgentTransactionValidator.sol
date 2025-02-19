// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RLPTxDecoder} from "./RLP/RLPTxDecoder.sol";

/// @title IMetabasedSequencerChain
/// @notice Interface for the MetabasedSequencerChain contract
interface IMetabasedSequencerChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external;
}

/// @title IAgentApplication
/// @notice Interface for agent application status checks
interface IAgentApplication {
    /// @notice Check if an agent address is permitted
    /// @param agentAddress The address to check
    /// @return bool indicating if the agent is permitted
    function isPermittedByAddress(address agentAddress) external view returns (bool);

    /// @notice Get the owner of the contract
    /// @return The owner address
    function owner() external view returns (address);
}

/// @title AgentTransactionValidator
/// @notice Middleware contract that validates transaction's 'from' address before forwarding to sequencer
/// @dev Extracts 'from' address from transaction data and validates against AgentApplication
contract AgentTransactionValidator {
    /// @notice The AgentApplication contract for checking permissions
    IAgentApplication public immutable agentApplication;

    /// @notice The MetabasedSequencerChain contract for forwarding valid transactions
    IMetabasedSequencerChain public immutable sequencerChain;

    error Unauthorized();

    constructor(address _agentApplication, address _sequencerChain) {
        agentApplication = IAgentApplication(_agentApplication);
        sequencerChain = IMetabasedSequencerChain(_sequencerChain);
    }

    /// @notice Process a transaction after validating the 'from' address
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external {
        address from = RLPTxDecoder.decodeTx(data);

        // Check if the from address is permitted
        if (!agentApplication.isPermittedByAddress(from)) {
            // then must be owner of the AgentApplication contract, otherwise revert
            if (from != agentApplication.owner()) revert Unauthorized();
        }

        // Forward the transaction to the sequencer chain
        sequencerChain.processTransaction(data);
    }
}
