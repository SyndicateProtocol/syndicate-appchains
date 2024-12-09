// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList_ReleaseCandidate} from
    "../LinkedList/AddressStructuredLinkedList_ReleaseCandidate.sol";
import {
    IRequirementModule_ReleaseCandidate,
    PermissionModule_ReleaseCandidate
} from "../interfaces/IRequirementModule_ReleaseCandidate.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/**
 * @title RequireAnyModule
 * @notice A module that requires at least one check to pass
 */
contract RequireAnyModule_ReleaseCandidate is IRequirementModule_ReleaseCandidate, Ownable {
    AddressStructuredLinkedList_ReleaseCandidate.List private checks;

    // Events
    event CheckAdded(address indexed check);
    event CheckRemoved(address indexed check);

    // Errors
    error CheckFailed(address batchSubmitter);
    error InvalidAddress();
    error AddressAlreadyExists();
    error AddressDoesNotExist();

    constructor(address admin) Ownable(admin) {}

    function isAllowed(address proposer) external view override returns (bool) {
        address currentCheck = AddressStructuredLinkedList_ReleaseCandidate.getHead(checks);

        // If no checks exist, allow by default
        if (currentCheck == address(0)) return true;

        while (currentCheck != address(0)) {
            if (PermissionModule_ReleaseCandidate(currentCheck).isAllowed(proposer)) {
                return true;
            }

            (bool exists, address nextCheck) =
                AddressStructuredLinkedList_ReleaseCandidate.getNextNode(checks, currentCheck);
            if (!exists) break;
            currentCheck = nextCheck;
        }

        revert CheckFailed(proposer);
    }

    function addCheck(address _address, bool addToHead) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (AddressStructuredLinkedList_ReleaseCandidate.nodeExists(checks, _address)) {
            revert AddressAlreadyExists();
        }

        if (addToHead) {
            bool success = AddressStructuredLinkedList_ReleaseCandidate.pushFront(checks, _address);
            require(success, "Address not added");
        } else {
            bool success = AddressStructuredLinkedList_ReleaseCandidate.pushBack(checks, _address);
            require(success, "Address not added");
        }

        emit CheckAdded(_address);
    }

    function removeCheck(address _address) external override onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (!AddressStructuredLinkedList_ReleaseCandidate.nodeExists(checks, _address)) {
            revert AddressDoesNotExist();
        }

        address returnedAddress = AddressStructuredLinkedList_ReleaseCandidate.remove(checks, _address);
        require(returnedAddress == _address, "Address not removed");

        emit CheckRemoved(_address);
    }

    function getAllChecks() external view override returns (address[] memory) {
        uint256 size = AddressStructuredLinkedList_ReleaseCandidate.sizeOf(checks);
        address[] memory allChecks = new address[](size);

        address current = AddressStructuredLinkedList_ReleaseCandidate.getHead(checks);
        for (uint256 i = 0; i < size && current != address(0); i++) {
            allChecks[i] = current;
            (bool exists, address next) = AddressStructuredLinkedList_ReleaseCandidate.getNextNode(checks, current);
            if (!exists) break;
            current = next;
        }

        return allChecks;
    }
}
