// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "./LinkedList/AddressStructuredLinkedList.sol";
import {IsAllowed} from "./interfaces/IsAllowed.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/**
 * @title MasterPermissionModule
 * @notice A module that handles all permission checks for the sequencer
 * @dev Implements the core permission logic previously in RequireListManager
 */
contract MasterPermissionModule is IsAllowed, Ownable {
    /// @notice A list of isAllowed checks that must pass before a batch can be sequenced
    AddressStructuredLinkedList.List public requireAllList;

    /// @notice A list of isAllowed checks where at least one check must pass
    AddressStructuredLinkedList.List public requireAnyList;

    // Events
    event RequireAllCheckAdded(address indexed check);
    event RequireAllCheckRemoved(address indexed check);
    event RequireAnyCheckAdded(address indexed check);
    event RequireAnyCheckRemoved(address indexed check);

    // Errors
    error RequireAllCheckFailed(address requireAllAddress, address batchSubmitter);
    error RequireAnyCheckFailed(address batchSubmitter);
    error InvalidAddress();
    error AddressAlreadyExistsInRequireAllList();
    error AddressDoesNotExistInRequireAllList();
    error AddressAlreadyExistsInRequireAnyList();
    error AddressDoesNotExistInRequireAnyList();

    constructor(address admin) Ownable(admin) {}

    /// @notice Implementation of the IsAllowed interface
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the address is allowed
    function isAllowed(address proposer) external view override returns (bool) {
        // Check requireAll list
        address requireAllAddress = AddressStructuredLinkedList.getHead(requireAllList);
        bool requireAllNextNodeExists;
        address requireAllNextNodeAddress;

        if (requireAllAddress != address(0)) {
            (requireAllNextNodeExists, requireAllNextNodeAddress) =
                AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);

            // Check head node
            if (!IsAllowed(requireAllAddress).isAllowed(proposer)) {
                revert RequireAllCheckFailed(requireAllAddress, proposer);
            }

            // Check subsequent nodes
            while (requireAllNextNodeExists) {
                if (!IsAllowed(requireAllAddress).isAllowed(proposer)) {
                    revert RequireAllCheckFailed(requireAllAddress, proposer);
                }

                (requireAllNextNodeExists, requireAllAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);
            }
        }

        // Check requireAny list
        address requireAnyAddress = AddressStructuredLinkedList.getHead(requireAnyList);
        bool requireAnyNextNodeExists;
        address requireAnyNextNodeAddress;

        if (requireAnyAddress != address(0)) {
            (requireAnyNextNodeExists, requireAnyNextNodeAddress) =
                AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);

            // Check head node
            if (IsAllowed(requireAnyAddress).isAllowed(proposer)) {
                return true;
            }

            // Check subsequent nodes
            while (requireAnyNextNodeExists) {
                if (IsAllowed(requireAnyAddress).isAllowed(proposer)) {
                    return true;
                }

                (requireAnyNextNodeExists, requireAnyAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);
            }

            // If we reach this point, no requireAny checks passed
            revert RequireAnyCheckFailed(proposer);
        }

        return true;
    }

    /// @notice Adds a check to the requireAll list
    /// @param _address The address of the check to add
    /// @param addToHead Whether to add the check to the head of the list
    function addRequireAllCheck(address _address, bool addToHead) external onlyOwner {
        if (_address == address(0)) {
            revert InvalidAddress();
        }
        if (AddressStructuredLinkedList.nodeExists(requireAllList, _address)) {
            revert AddressAlreadyExistsInRequireAllList();
        }

        if (addToHead) {
            AddressStructuredLinkedList.pushFront(requireAllList, _address);
        } else {
            AddressStructuredLinkedList.pushBack(requireAllList, _address);
        }

        emit RequireAllCheckAdded(_address);
    }

    /// @notice Removes a check from the requireAll list
    /// @param _address The address of the check to remove
    function removeRequireAllCheck(address _address) external onlyOwner {
        if (_address == address(0)) {
            revert InvalidAddress();
        }
        if (!AddressStructuredLinkedList.nodeExists(requireAllList, _address)) {
            revert AddressDoesNotExistInRequireAllList();
        }

        AddressStructuredLinkedList.remove(requireAllList, _address);
        emit RequireAllCheckRemoved(_address);
    }

    /// @notice Adds a check to the requireAny list
    /// @param _address The address of the check to add
    /// @param addToHead Whether to add the check to the head of the list
    function addRequireAnyCheck(address _address, bool addToHead) external onlyOwner {
        if (_address == address(0)) {
            revert InvalidAddress();
        }
        if (AddressStructuredLinkedList.nodeExists(requireAnyList, _address)) {
            revert AddressAlreadyExistsInRequireAnyList();
        }

        if (addToHead) {
            AddressStructuredLinkedList.pushFront(requireAnyList, _address);
        } else {
            AddressStructuredLinkedList.pushBack(requireAnyList, _address);
        }

        emit RequireAnyCheckAdded(_address);
    }

    /// @notice Removes a check from the requireAny list
    /// @param _address The address of the check to remove
    function removeRequireAnyCheck(address _address) external onlyOwner {
        if (_address == address(0)) {
            revert InvalidAddress();
        }
        if (!AddressStructuredLinkedList.nodeExists(requireAnyList, _address)) {
            revert AddressDoesNotExistInRequireAnyList();
        }

        AddressStructuredLinkedList.remove(requireAnyList, _address);
        emit RequireAnyCheckRemoved(_address);
    }

    /// @notice Gets all checks from either list
    /// @param requireAll True for requireAll list, false for requireAny list
    /// @return Array of addresses representing the checks
    function getAllRequirements(bool requireAll) external view returns (address[] memory) {
        AddressStructuredLinkedList.List storage allowList = requireAll ? requireAllList : requireAnyList;
        address allowAddress = AddressStructuredLinkedList.getHead(allowList);
        bool allowNextNodeExists;
        address allowNextNodeAddress;

        (allowNextNodeExists, allowNextNodeAddress) =
            AddressStructuredLinkedList.getNextNode(allowList, allowAddress);

        address[] memory allChecks = new address[](AddressStructuredLinkedList.sizeOf(allowList));

        if (allowAddress != address(0)) {
            allChecks[0] = allowAddress;

            uint256 i = 1;
            while (allowNextNodeExists) {
                allChecks[i] = allowNextNodeAddress;

                (allowNextNodeExists, allowNextNodeAddress) =
                    AddressStructuredLinkedList.getNextNode(allowList, allowNextNodeAddress);
                i++;
            }
        }

        return allChecks;
    }
}