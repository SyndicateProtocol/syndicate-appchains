// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.29;

import {BaseRequirementModule, AddressStructuredLinkedList} from "./BaseRequirementModule.sol";

import {IPermissionModule} from "../interfaces/IPermissionModule.sol";

/**
 * @title RequireAllModule
 * @notice A module that requires ALL checks to pass (AND logic)
 * @dev This contract implements strict permission logic where every module must approve
 */
contract RequireAllModule is BaseRequirementModule {
    // Errors
    /// @notice Thrown when a permission check fails
    /// @param requireAddress The address of the check that failed
    /// @param msgSender The address of the sender
    error CheckFailed(address requireAddress, address msgSender);

    /**
     * @notice Initializes the contract with an admin address
     * @param admin The address of the admin who can add/remove checks
     */
    constructor(address admin) BaseRequirementModule(admin) {}

    /**
     * @notice Checks if a sender is allowed to submit a transaction
     * @dev Runs through all permission checks in the linked list - ALL must pass (AND logic)
     * @param msgSender The address of the sender to check
     * @param txOrigin The address of tx.origin. Useful to know the sender originator in wrapper contracts
     * @param data The calldata to be checked
     * @return True if the sender passes all checks, reverts otherwise
     */
    function isAllowed(address msgSender, address txOrigin, bytes calldata data)
        external
        view
        override
        returns (bool)
    {
        address currentCheck = AddressStructuredLinkedList.getHead(permissionChecks);

        while (currentCheck != address(0)) {
            if (!IPermissionModule(currentCheck).isAllowed(msgSender, txOrigin, data)) {
                revert CheckFailed(currentCheck, msgSender);
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(permissionChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        return true;
    }
}
