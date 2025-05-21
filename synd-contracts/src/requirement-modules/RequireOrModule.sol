// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {BaseRequirementModule, AddressStructuredLinkedList} from "src/requirement-modules/BaseRequirementModule.sol";

import {IPermissionModule} from "../interfaces/IPermissionModule.sol";

/**
 * @title RequireOrModule
 * @notice A module that requires ANY check to pass (OR logic)
 * @dev This contract implements permissive permission logic where any module can approve
 */
contract RequireOrModule is BaseRequirementModule {
    // Errors
    /// @notice Thrown when all permission checks fail
    /// @param msgSender The address of the sender
    error CheckFailed(address msgSender);

    /**
     * @notice Initializes the contract with an admin address
     * @param admin The address of the admin who can add/remove checks
     */
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(address admin) BaseRequirementModule(admin) {}

    /**
     * @notice Checks if a sender is allowed to submit a transaction
     * @dev Returns true if at least one check passes or if no checks exist (OR logic)
     * @param msgSender The address of the sender to check
     * @param txOrigin The address of tx.origin. Useful to know the sender originator in wrapper contracts
     * @param data The calldata to be checked
     * @return True if the sender passes at least one check, reverts otherwise
     */
    function isAllowed(address msgSender, address txOrigin, bytes calldata data)
        external
        view
        override
        returns (bool)
    {
        address currentCheck = AddressStructuredLinkedList.getHead(permissionChecks);

        // If no checks exist, allow by default
        if (currentCheck == address(0)) return true;

        while (currentCheck != address(0)) {
            //#olympix-ignore-calls-in-loop
            if (IPermissionModule(currentCheck).isAllowed(msgSender, txOrigin, data)) {
                return true;
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(permissionChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        revert CheckFailed(msgSender);
    }
}
