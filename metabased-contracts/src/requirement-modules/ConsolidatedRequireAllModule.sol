// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {ConsolidatedPermissionModule} from "src/interfaces/ConsolidatedPermissionModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ConsolidatedRequireAllModule
 * @notice A module that requires all checks to pass with a single consolidated interface
 * @dev This contract maintains two separate linked lists of check addresses but exposes
 *      a single consolidated interface for checking both proposer and calldata permissions
 */
contract ConsolidatedRequireAllModule is Ownable {
    AddressStructuredLinkedList.List private checks;

    // Events
    event CheckAdded(address indexed check);
    event CheckRemoved(address indexed check);

    // Errors
    error CheckFailed(address requireAddress, address batchSubmitter);
    error InvalidAddress();
    error AddressAlreadyExists();
    error AddressDoesNotExist();

    // [Olympix Warning: no parameter validation in constructor] Parameter validation is handled by OpenZeppelin's Ownable
    constructor(address admin) Ownable(admin) {}

    function isAllowed(address proposer, bytes calldata data) external view returns (bool) {
        address currentCheck = AddressStructuredLinkedList.getHead(checks);

        while (currentCheck != address(0)) {
            if (!ConsolidatedPermissionModule(currentCheck).isAllowed(proposer, data)) {
                revert CheckFailed(currentCheck, proposer);
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(checks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        return true;
    }

    function addCheck(address _address, bool addToHead) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList.nodeExists(checks, _address)) {
            revert AddressAlreadyExists();
        }

        if (addToHead) {
            bool success = AddressStructuredLinkedList.pushFront(checks, _address);
            require(success, "Address not added");
        } else {
            bool success = AddressStructuredLinkedList.pushBack(checks, _address);
            require(success, "Address not added");
        }

        emit CheckAdded(_address);
    }

    function removeCheck(address _address) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(checks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList.remove(checks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit CheckRemoved(_address);
    }

    function getAllChecks() external view returns (address[] memory) {
        uint256 size = AddressStructuredLinkedList.sizeOf(checks);
        address[] memory allChecks = new address[](size);

        address current = AddressStructuredLinkedList.getHead(checks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            allChecks[i] = current;
            (bool exists, address next) = AddressStructuredLinkedList.getNextNode(checks, current);
            if (!exists) break;
            current = next;
        }

        return allChecks;
    }
}
