// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {ConsolidatedPermissionModule} from "src/interfaces/ConsolidatedPermissionModule.sol";
import {ProposerPermissionModule} from "src/interfaces/ProposerPermissionModule.sol";
import {CalldataPermissionModule} from "src/interfaces/CalldataPermissionModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ConsolidatedRequireAllModule
 * @notice A module that requires all checks to pass with a single consolidated interface
 * @dev This contract maintains two separate linked lists of check addresses but exposes
 *      a single consolidated interface for checking both proposer and calldata permissions
 */
contract ConsolidatedRequireAllModule is ConsolidatedPermissionModule, Ownable {
    /// @notice List of addresses for proposer permission checks
    AddressStructuredLinkedList.List private proposerChecks;

    /// @notice List of addresses for calldata permission checks
    AddressStructuredLinkedList.List private calldataChecks;

    // Events
    /// @notice Emitted when a proposer check is added
    event ProposerCheckAdded(address indexed check);

    /// @notice Emitted when a proposer check is removed
    event ProposerCheckRemoved(address indexed check);

    /// @notice Emitted when a calldata check is added
    event CalldataCheckAdded(address indexed check);

    /// @notice Emitted when a calldata check is removed
    event CalldataCheckRemoved(address indexed check);

    // Errors
    /// @notice Thrown when a permission check fails
    /// @param requireAddress The address of the check that failed
    /// @param batchSubmitter The address of the proposer
    error CheckFailed(address requireAddress, address batchSubmitter);

    /// @notice Thrown when a calldata permission check fails
    /// @param requireAddress The address of the check that failed
    error CalldataCheckFailed(address requireAddress);

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
     * @notice Consolidated function that checks both proposer and calldata permissions
     * @dev Runs through all relevant permission checks in the linked lists
     * @param proposer The address of the proposer to check
     * @param data The calldata to be checked
     * @return True if all checks pass, reverts otherwise
     */
    function isAllowed(address proposer, bytes calldata data) external view override returns (bool) {
        // Only check proposer if address is not zero
        if (proposer != address(0)) {
            address currentCheck = AddressStructuredLinkedList.getHead(proposerChecks);

            while (currentCheck != address(0)) {
                if (!ProposerPermissionModule(currentCheck).isAllowed(proposer)) {
                    revert CheckFailed(currentCheck, proposer);
                }

                (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(proposerChecks, currentCheck);
                if (!exists) break;
                currentCheck = nextCheck;
            }
        }

        // Only check calldata if it's not empty
        if (data.length > 0) {
            address currentCheck = AddressStructuredLinkedList.getHead(calldataChecks);

            while (currentCheck != address(0)) {
                if (!CalldataPermissionModule(currentCheck).isCalldataAllowed(data)) {
                    revert CalldataCheckFailed(currentCheck);
                }

                (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(calldataChecks, currentCheck);
                if (!exists) break;
                currentCheck = nextCheck;
            }
        }

        return true;
    }

    /**
     * @notice Adds a proposer permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addProposerCheck(address _address, bool addToHead) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList.nodeExists(proposerChecks, _address)) {
            revert AddressAlreadyExists();
        }

        bool success;
        if (addToHead) {
            success = AddressStructuredLinkedList.pushFront(proposerChecks, _address);
        } else {
            success = AddressStructuredLinkedList.pushBack(proposerChecks, _address);
        }

        require(success, "Address not added");
        emit ProposerCheckAdded(_address);
    }

    /**
     * @notice Removes a proposer permission check address from the list
     * @param _address The address of the check to remove
     */
    function removeProposerCheck(address _address) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(proposerChecks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(proposerChecks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit ProposerCheckRemoved(_address);
    }

    /**
     * @notice Adds a calldata permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addCalldataCheck(address _address, bool addToHead) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList.nodeExists(calldataChecks, _address)) {
            revert AddressAlreadyExists();
        }

        bool success;
        if (addToHead) {
            success = AddressStructuredLinkedList.pushFront(calldataChecks, _address);
        } else {
            success = AddressStructuredLinkedList.pushBack(calldataChecks, _address);
        }

        require(success, "Address not added");
        emit CalldataCheckAdded(_address);
    }

    /**
     * @notice Removes a calldata permission check address from the list
     * @param _address The address of the check to remove
     */
    function removeCalldataCheck(address _address) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(calldataChecks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(calldataChecks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit CalldataCheckRemoved(_address);
    }

    /**
     * @notice Gets all proposer permission check addresses
     * @return An array of all proposer check addresses
     */
    function getAllProposerChecks() external view returns (address[] memory) {
        uint256 size = AddressStructuredLinkedList.sizeOf(proposerChecks);
        address[] memory allChecks = new address[](size);

        address current = AddressStructuredLinkedList.getHead(proposerChecks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            allChecks[i] = current;
            (bool exists, address next) = AddressStructuredLinkedList.getNextNode(proposerChecks, current);
            if (!exists) break;
            current = next;
        }

        return allChecks;
    }

    /**
     * @notice Gets all calldata permission check addresses
     * @return An array of all calldata check addresses
     */
    function getAllCalldataChecks() external view returns (address[] memory) {
        uint256 size = AddressStructuredLinkedList.sizeOf(calldataChecks);
        address[] memory allChecks = new address[](size);

        address current = AddressStructuredLinkedList.getHead(calldataChecks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            allChecks[i] = current;
            (bool exists, address next) = AddressStructuredLinkedList.getNextNode(calldataChecks, current);
            if (!exists) break;
            current = next;
        }

        return allChecks;
    }
}
