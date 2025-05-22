// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title BaseRequirementModule
 * @notice Base abstract contract for requirement modules that share common functionality
 * @dev Contains shared code for RequireAndModule and RequireOrModule
 */
abstract contract BaseRequirementModule is IRequirementModule, Ownable {
    /// @notice List of permission checks addresses
    AddressStructuredLinkedList.List internal permissionChecks;

    // Events
    /// @notice Emitted when a permission check is added
    //#olympix-ignore-missing-events-assertion
    event PermissionCheckAdded(address indexed check);

    /// @notice Emitted when a permission check is removed
    //#olympix-ignore-missing-events-assertion
    event PermissionCheckRemoved(address indexed check);

    // Errors
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
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(address admin) Ownable(admin) {}

    /**
     * @notice Adds permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addPermissionCheck(address _address, bool addToHead) public virtual override onlyOwner {
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
    function removePermissionCheck(address _address) public virtual override onlyOwner {
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
     * @return An array of all permission check addresses
     */
    function getAllPermissionChecks() external view virtual override returns (address[] memory) {
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

    /**
     * @notice Checks if a sender is allowed to submit a transaction
     * @dev Must be implemented by derived contracts (AND or OR logic)
     */
    function isAllowed(address msgSender, address txOrigin, bytes calldata data)
        external
        view
        virtual
        override
        returns (bool);
}
