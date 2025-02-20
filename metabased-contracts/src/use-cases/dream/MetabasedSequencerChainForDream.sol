// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker} from "../../SequencingModuleChecker.sol";
import {RLPTxDecoder} from "./RLP/RLPTxDecoder.sol";

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

/// @title MetabasedSequencerChainForDream
/// @notice Modified MetabasedSequencerChain contract for sequencing transactions for Dream chain
contract MetabasedSequencerChainForDream is SequencingModuleChecker {
    /// @notice The AgentApplication contract for checking permissions
    IAgentApplication public immutable agentApplication;

    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    error Unauthorized();

    /// @notice Constructs the MetabasedSequencerChainForDream contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    /// @param _agentApplication The address of the AgentApplication contract
    // [Olympix Warning: no parameter validation in constructor] Admin and masterModule validation handled by SequencingModuleChecker
    constructor(uint256 _l3ChainId, address _agentApplication) SequencingModuleChecker() {
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(_l3ChainId != 0, "L3 chain ID cannot be 0");
        l3ChainId = _l3ChainId;

        require(_agentApplication != address(0), "AgentApplication address cannot be 0");
        agentApplication = IAgentApplication(_agentApplication);
    }

    /// @notice Modifier to revert if the calldata is not allowed
    /// @param data The calldata to check
    modifier revertForUnallowedCalldata(bytes calldata data) {
        _revertForUnallowedCalldata(data);
        _;
    }

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    function processTransactionRaw(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    function processTransaction(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    function processBulkTransactions(bytes[] calldata data) external onlyWhenAllowed(msg.sender) {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            _revertForUnallowedCalldata(data[i]);

            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _data) private pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }

    /// @notice Revert if the calldata is not allowed
    /// @param data The calldata to check
    function _revertForUnallowedCalldata(bytes calldata data) internal view {
        address from = RLPTxDecoder.decodeTx(data);

        // Check if the from address is permitted
        if (!agentApplication.isPermittedByAddress(from)) {
            // then must be owner of the AgentApplication contract, otherwise revert
            bytes32 adminRole = agentApplication.ADMIN_ROLE();
            if (!agentApplication.hasRole(adminRole, from)) revert Unauthorized();
        }
    }
}
