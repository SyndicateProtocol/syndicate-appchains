// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "../../interfaces/PermissionModule.sol";
import {AgentApplication} from "./AgentApplication.sol";

/// @title AgentValidationModule
/// @notice Validates Dream agent permissions for transaction submission
/// @dev Integrates with AgentApplication contract to check if an agent is approved to submit transactions
contract AgentValidationModule is PermissionModule {
    AgentApplication public immutable agentApplication;
    address public admin;

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);

    error NotAdmin();
    error InvalidAddress();

    constructor(address _admin, address _agentApplication) {
        if (_admin == address(0) || _agentApplication == address(0)) {
            revert InvalidAddress();
        }
        admin = _admin;
        agentApplication = AgentApplication(_agentApplication);
    }

    modifier onlyAdmin() {
        if (msg.sender != admin) {
            revert NotAdmin();
        }
        _;
    }

    /// @notice Transfer admin rights to a new address
    /// @param newAdmin The address of the new admin
    function transferAdmin(address newAdmin) external onlyAdmin {
        if (newAdmin == address(0)) {
            revert InvalidAddress();
        }
        admin = newAdmin;
        emit AdminTransferred(msg.sender, newAdmin);
    }

    /// @notice Check if a transaction is allowed
    /// @param proposer The address attempting to submit the transaction
    /// @return bool indicating if the transaction is allowed
    function isAllowed(address proposer) external view override returns (bool) {
        if (proposer == address(0)) return false;
        return agentApplication.isPermittedByAddress(proposer);
    }
}
