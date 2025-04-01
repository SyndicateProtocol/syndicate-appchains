// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {IRequirementModule, ProposerPermissionModule} from "src/interfaces/IRequirementModule.sol";
import {CalldataPermissionModule} from "src/interfaces/CalldataPermissionModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title RequireAnyModule
 * @notice A module that requires at least one check to pass for either proposers or calldata
 * @dev This contract maintains two separate linked lists of check addresses: one for
 *      proposer validation and one for calldata validation
 */
// [Olympix Warning: unfuzzed variables, missing events assertion] These test-related warnings are not security critical
// as the contract uses standard unit tests and integration tests. Parameter validation is handled through Ownable.
contract RequireAnyModule is IRequirementModule, Ownable {
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
    /// @notice Thrown when all proposer permission checks fail
    /// @param batchSubmitter The address of the proposer
    error ProposerCheckFailed(address batchSubmitter);

    /// @notice Thrown when all calldata permission checks fail
    error CalldataCheckFailed();

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
    // [Olympix Warning: no parameter validation in constructor] Parameter validation is handled by OpenZeppelin's Ownable
    constructor(address admin) Ownable(admin) {}

    /**
     * @notice Checks if a proposer is allowed to submit a transaction
     * @dev Returns true if at least one check passes or if no checks exist
     * @param proposer The address of the proposer to check
     * @return True if the proposer passes at least one check, reverts otherwise
     */
    function isAllowed(address proposer) external view override returns (bool) {
        address currentCheck = AddressStructuredLinkedList.getHead(proposerChecks);

        // If no checks exist, allow by default
        if (currentCheck == address(0)) return true;

        while (currentCheck != address(0)) {
            if (ProposerPermissionModule(currentCheck).isAllowed(proposer)) {
                return true;
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(proposerChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        revert ProposerCheckFailed(proposer);
    }

    /**
     * @notice Checks if the provided calldata is allowed
     * @dev Returns true if at least one check passes or if no checks exist
     * @param data The calldata to be checked
     * @return True if the calldata passes at least one check, reverts otherwise
     */
    function isCalldataAllowed(bytes calldata data) external view override returns (bool) {
        address currentCheck = AddressStructuredLinkedList.getHead(calldataChecks);

        // If no checks exist, allow by default
        if (currentCheck == address(0)) return true;

        while (currentCheck != address(0)) {
            if (CalldataPermissionModule(currentCheck).isCalldataAllowed(data)) {
                return true;
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(calldataChecks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        revert CalldataCheckFailed();
    }

    /**
     * @notice Adds a proposer permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addProposerCheck(address _address, bool addToHead) external override onlyOwner {
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
    function removeProposerCheck(address _address) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(proposerChecks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(proposerChecks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit ProposerCheckRemoved(_address);
    }

    /**
     * @notice Gets all proposer permission check addresses
     * @return An array of all proposer check addresses
     */
    function getAllProposerChecks() external view override returns (address[] memory) {
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
     * @notice Adds a calldata permission check address to the list
     * @dev Can add to either the head or the tail of the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addCalldataCheck(address _address, bool addToHead) external override onlyOwner {
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
    function removeCalldataCheck(address _address) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(calldataChecks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(calldataChecks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit CalldataCheckRemoved(_address);
    }

    /**
     * @notice Gets all calldata permission check addresses
     * @return An array of all calldata check addresses
     */
    function getAllCalldataChecks() external view override returns (address[] memory) {
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
