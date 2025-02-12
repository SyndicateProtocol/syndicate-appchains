// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "../../interfaces/PermissionModule.sol";
import {AgentApplication} from "./AgentApplication.sol";

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
        for (uint256 i = 0; i < agentApplication.applicantCount(); i++) {
            (
                address agentAddress,
                bool isPermitted,
                , // name
                    // additionalData
            ) = agentApplication.getApplicant(i);

            if (agentAddress == proposer && isPermitted) {
                return true;
            }
        }
        return false;
    }
}
