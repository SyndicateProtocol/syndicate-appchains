// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {IRequirementModule, IPermissionModule} from "src/interfaces/IRequirementModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title RequireAnyModule
 * @notice A module that requires at least one check to pass for either proposers, originators, or calldata
 * @dev This contract maintains a linked lists of check addresses
 */
contract RequireAnyModule is IRequirementModule, Ownable {
    /// @notice List of permission checks addresses
    AddressStructuredLinkedList.List private permissionChecks;

    // Events
    /// @notice Emitted when a permission check is added
    event PermissionCheckAdded(address indexed check);

    /// @notice Emitted when a permission check is removed
    event PermissionCheckRemoved(address indexed check);

    // Errors
    /// @notice Thrown when all permission checks fail
    /// @param proposer The address of the proposer
    error CheckFailed(address proposer);

    /// @notice Thrown when an invalid address is provided
    error InvalidAddress();

    /// @notice Thrown when attempting to add an address that already exists
    error AddressAlreadyExists();

    /// @notice Thrown when attempting to remove an address that doesn't exist
    error AddressDoesNotExist();

    /**
     * @notice Initializes the contract with an admin address
     * @param admin The address of the admin who can add/remove checks
     */
    constructor(address admin) Ownable(admin) {}

    /**
     * @notice Checks if a proposer is allowed to submit a transaction
     * @dev Returns true if at least one check passes or if no checks exist
     * @param proposer The address of the proposer to check
     * @param originator The address of tx.origin. Useful to know the sender originator in wrapper contracts
     * @param data The calldata to be checked
     * @return True if the proposer passes at least one check, reverts otherwise
     */
    function isAllowed(address proposer, address originator, bytes calldata data)
        external
        view
        override
        returns (bool)
    {
        address currentCheck = AddressStructuredLinkedList.getHead(permissionChecks);

        // If no checks exist, allow by default
        if (currentCheck == address(0)) return true;

        while (currentCheck != address(0)) {
            if (IPermissionModule(currentCheck).isAllowed(proposer, originator, data)) {
                return true;
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(permissionChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        revert CheckFailed(proposer);
    }

    /**
     * @notice Adds permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addPermissionCheck(address _address, bool addToHead) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList.nodeExists(permissionChecks, _address)) {
            revert AddressAlreadyExists();
        }

        bool success;
        if (addToHead) {
            success = AddressStructuredLinkedList.pushFront(permissionChecks, _address);
        } else {
            success = AddressStructuredLinkedList.pushBack(permissionChecks, _address);
        }

        require(success, "Address not added");
        emit PermissionCheckAdded(_address);
    }

    /**
     * @notice Removes permission check address from the list
     * @param _address The address of the check to remove
     */
    function removePermissionCheck(address _address) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(permissionChecks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(permissionChecks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit PermissionCheckRemoved(_address);
    }

    /**
     * @notice Gets all permission check addresses
     * @return An array of all proposer check addresses
     */
    function getAllPermissionChecks() external view override returns (address[] memory) {
        uint256 size = AddressStructuredLinkedList.sizeOf(permissionChecks);
        address[] memory allChecks = new address[](size);

        address current = AddressStructuredLinkedList.getHead(permissionChecks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            allChecks[i] = current;
            (bool exists, address next) = AddressStructuredLinkedList.getNextNode(permissionChecks, current);
            if (!exists) break;
            current = next;
        }

        return allChecks;
    }
}
