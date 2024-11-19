// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "src/LinkedList/AddressStructuredLinkedList.sol";
import {IRequirementModule, PermissionModule} from "src/interfaces/IRequirementModule.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/**
 * @title RequireAllModule
 * @notice A module that requires all checks to pass
 */
contract RequireAllModule is IRequirementModule, Ownable {
    AddressStructuredLinkedList.List private checks;

    // Events
    event CheckAdded(address indexed check);
    event CheckRemoved(address indexed check);

    // Errors
    error CheckFailed(address requireAddress, address batchSubmitter);
    error InvalidAddress();
    error AddressAlreadyExists();
    error AddressDoesNotExist();

    constructor(address admin) Ownable(admin) {}

    function isAllowed(address proposer) external view override returns (bool) {
        address currentCheck = AddressStructuredLinkedList.getHead(checks);

        while (currentCheck != address(0)) {
            if (!PermissionModule(currentCheck).isAllowed(proposer)) {
                revert CheckFailed(currentCheck, proposer);
            }

            (bool exists, address nextCheck) = AddressStructuredLinkedList.getNextNode(checks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        return true;
    }

    function addCheck(address _address, bool addToHead) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList.nodeExists(checks, _address)) {
            revert AddressAlreadyExists();
        }

        if (addToHead) {
            AddressStructuredLinkedList.pushFront(checks, _address);
        } else {
            AddressStructuredLinkedList.pushBack(checks, _address);
        }

        emit CheckAdded(_address);
    }

    function removeCheck(address _address) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList.nodeExists(checks, _address)) {
            revert AddressDoesNotExist();
        }

        AddressStructuredLinkedList.remove(checks, _address);
        emit CheckRemoved(_address);
    }

    function getAllChecks() external view override returns (address[] memory) {
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
