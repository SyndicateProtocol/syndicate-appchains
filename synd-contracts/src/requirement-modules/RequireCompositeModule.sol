// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {BaseRequirementModule} from "./BaseRequirementModule.sol";
import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

/**
 * @title RequireCompositeModule
 * @notice A module that composes both AND and OR logic for flexible permission validation
 * @dev Builds upon BaseRequirementModule to add check type functionality
 */
contract RequireCompositeModule is BaseRequirementModule {
    /// @notice Types of permission checks
    enum CheckType {
        AND, // Must pass for approval (combined with all other AND checks)
        OR // Any single OR check passing grants approval

    }

    /// @notice Mapping to store check types
    mapping(address => CheckType) public checkTypes;

    // Events
    /// @notice Emitted when a permission check is added with a specific type
    //#olympix-ignore-missing-events-assertion
    event PermissionCheckAddedWithType(address indexed check, CheckType indexed checkType);

    /// @notice Emitted when a check type is updated
    //#olympix-ignore-missing-events-assertion
    event CheckTypeUpdated(address indexed check, CheckType indexed oldType, CheckType indexed newType);

    // Errors
    /// @notice Thrown when an AND permission check fails
    /// @param requireAddress The address of the check that failed
    /// @param msgSender The address of the sender
    error AndCheckFailed(address requireAddress, address msgSender);

    /// @notice Thrown when all OR permission checks fail
    /// @param msgSender The address of the sender
    error AllOrChecksFailed(address msgSender);

    /**
     * @notice Initializes the contract with an admin address
     * @param admin The address of the admin who can add/remove checks
     */
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(address admin) BaseRequirementModule(admin) {}

    /**
     * @notice Adds a permission check with the specified type
     * @dev Adds to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param checkType The type of check (AND or OR)
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addPermissionCheckWithType(address _address, CheckType checkType, bool addToHead) external onlyOwner {
        // Call the parent implementation for linked list handling
        super.addPermissionCheck(_address, addToHead);

        // Set the check type
        checkTypes[_address] = checkType;

        emit PermissionCheckAddedWithType(_address, checkType);
    }

    /**
     * @notice Overrides the base addPermissionCheck to assign AND type by default
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addPermissionCheck(address _address, bool addToHead) public override onlyOwner {
        // Call the parent implementation for linked list handling
        super.addPermissionCheck(_address, addToHead);

        // Assign AND type by default
        checkTypes[_address] = CheckType.AND;

        emit PermissionCheckAddedWithType(_address, CheckType.AND);
    }

    /**
     * @notice Overrides the base removePermissionCheck to clean up check type
     * @param _address The address of the check to remove
     */
    function removePermissionCheck(address _address) public override onlyOwner {
        // Call the parent implementation for linked list handling
        super.removePermissionCheck(_address);

        // Clean up the check type mapping
        delete checkTypes[_address];
    }

    /**
     * @notice Updates the check type for an existing permission check
     * @param _address The address of the check to update
     * @param newCheckType The new check type (AND or OR)
     */
    function updateCheckType(address _address, CheckType newCheckType) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(permissionChecks, _address)) {
            revert AddressDoesNotExist();
        }

        CheckType oldCheckType = checkTypes[_address];
        if (oldCheckType == newCheckType) return; // No change needed

        checkTypes[_address] = newCheckType;

        emit CheckTypeUpdated(_address, oldCheckType, newCheckType);
    }

    /**
     * @notice Gets all permission check addresses with their types
     * @return addresses An array of all permission check addresses
     * @return types An array of check types corresponding to the addresses
     */
    function getAllPermissionChecksWithTypes()
        external
        view
        returns (address[] memory addresses, CheckType[] memory types)
    {
        uint256 size = AddressStructuredLinkedList.sizeOf(permissionChecks);
        addresses = new address[](size);
        types = new CheckType[](size);

        address current = AddressStructuredLinkedList.getHead(permissionChecks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            addresses[i] = current;
            types[i] = checkTypes[current];

            (bool exists, address next) = AddressStructuredLinkedList.getNextNode(permissionChecks, current);
            if (!exists) break;
            current = next;
        }

        return (addresses, types);
    }

    /**
     * @notice Checks if a sender is allowed to submit a transaction using composite logic:
     * 1. All AND checks must pass
     * 2. At least one OR check must pass, if any exist
     * @param msgSender The address of the sender to check
     * @param txOrigin The address of tx.origin
     * @param data The calldata to be checked
     * @return True if permission requirements are met
     */
    function isAllowed(address msgSender, address txOrigin, bytes calldata data)
        external
        view
        override
        returns (bool)
    {
        address currentCheck = AddressStructuredLinkedList.getHead(permissionChecks);

        // No checks exist, default allow
        if (currentCheck == address(0)) return true;

        bool hasOrChecks = false;
        bool anyOrPassed = false;

        // First pass: evaluate all AND checks (all must pass)
        while (currentCheck != address(0)) {
            CheckType checkType = checkTypes[currentCheck];

            if (checkType == CheckType.AND) {
                // AND check - must pass
                if (!IPermissionModule(currentCheck).isAllowed(msgSender, txOrigin, data)) {
                    revert AndCheckFailed(currentCheck, msgSender);
                }
            } else {
                // OR check - track if we have any OR checks
                hasOrChecks = true;

                // Evaluate OR check
                if (IPermissionModule(currentCheck).isAllowed(msgSender, txOrigin, data)) {
                    anyOrPassed = true;
                }
            }

            // Move to next check
            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(permissionChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        // If we have OR checks, at least one must pass
        if (hasOrChecks && !anyOrPassed) {
            revert AllOrChecksFailed(msgSender);
        }

        // All AND checks passed, and either no OR checks or at least one OR check passed
        return true;
    }
}
