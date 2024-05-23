// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "./LinkedList/AddressStructuredLinkedList.sol";
import {IsAllowed} from "./interfaces/IsAllowed.sol";

/// @title RequireListManager
/// @notice A contract that manages a list of addresses that must pass an
/// isAllowed check before a batch can be sequenced
/// @dev This contract is used by the BasedSequencerChain contract
abstract contract RequireListManager {
    /// @notice A list of isAllowed checks that must pass before a batch can be sequenced
    /// @dev For requireAll checks, all checks must pass for the batch to be sequenced
    /// This will fail early upon the first check that fails.
    AddressStructuredLinkedList.List public requireAllList;

    /// @notice A list of isAllowed checks where at least one check must pass for the batch to be sequenced
    /// @dev This will succeed early upon the first check that passes.
    AddressStructuredLinkedList.List public requireAnyList;

    /// @notice The admin address is the only address that can add or remove from the
    /// requireAllList and requireAnyList
    address public admin;

    /// @dev Emitted when a requireAll check fails
    /// @param requireAllAddress The address of the failed requireAll check
    /// @param batchSubmitter The address of the batch submitter
    error RequireAllCheckFailed(address requireAllAddress, address batchSubmitter);

    /// @dev Emitted when all requireAny checks fail
    /// @param batchSubmitter The address of the batch submitter
    error RequireAnyCheckFailed(address batchSubmitter);

    /// @dev Constructor function
    /// @notice Sets the admin to the address that deploys the contract
    constructor() {
        admin = msg.sender;
    }

    /// @dev Modifier to restrict access to only the admin
    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can perform this action");
        _;
    }

    /// @notice Checks if all requireAll checks pass for the given batch submitter
    /// @dev While this requires a revert, someone can always check the list item by
    /// item if they want to see whether they'll pass or fail a specific check.
    /// It's not terribly useful to know which address is the first one you'll
    /// fail, compared to being able to check each address individually
    /// @param batchSubmitter The address of the batch submitter
    function requireAllAllowed(address batchSubmitter) public view {
        address requireAllAddress = AddressStructuredLinkedList.getHead(requireAllList);
        bool requireAllNextNodeExists;
        address requireAllNextNodeAddress;

        (requireAllNextNodeExists, requireAllNextNodeAddress) =
            AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);

        if (requireAllAddress != address(0)) {
            // isAllowed check for head node
            if (!IsAllowed(requireAllAddress).isAllowed()) {
                revert RequireAllCheckFailed(requireAllAddress, batchSubmitter);
            }

            // isAllowed check for all subsequent nodes
            while (requireAllNextNodeExists) {
                // isAllowed check for node
                if (!IsAllowed(requireAllAddress).isAllowed()) {
                    revert RequireAllCheckFailed(requireAllAddress, batchSubmitter);
                }

                (requireAllNextNodeExists, requireAllAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);
            }
        }
    }

    /// @notice Checks if any requireAny check passes for the given batch submitter
    /// @param batchSubmitter The address of the batch submitter
    function requireAnyAllowed(address batchSubmitter) public view {
        address requireAnyAddress = AddressStructuredLinkedList.getHead(requireAnyList);
        bool requireAnyNextNodeExists;
        address requireAnyNextNodeAddress;

        (requireAnyNextNodeExists, requireAnyNextNodeAddress) =
            AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);

        if (requireAnyAddress != address(0)) {
            // isAllowed check for head node
            if (IsAllowed(requireAnyAddress).isAllowed()) {
                return;
            }

            // isAllowed check for all subsequent nodes
            while (requireAnyNextNodeExists) {
                // isAllowed check for node
                if (IsAllowed(requireAnyAddress).isAllowed()) {
                    return;
                }

                (requireAnyNextNodeExists, requireAnyAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);
            }
        }

        // If we reach this point, then no requireAny checks passed
        revert RequireAnyCheckFailed(batchSubmitter);
    }

    /// @notice Returns all requireAll checks if requireAll is true, or all requireAny checks if requireAll is false
    /// @dev This function isn't used within the contract logic itself. It's
    /// primarily for nodes to check the sequencing criteria for a given chain
    /// @param requireAll True to get all requireAll checks, false to get all requireAny checks
    /// @return An array of addresses representing the requireAll or requireAny checks
    function getAllRequirements(bool requireAll) public view returns (address[] memory) {
        AddressStructuredLinkedList.List storage allowList = requireAll ? requireAllList : requireAnyList;
        address allowAddress = AddressStructuredLinkedList.getHead(allowList);
        bool allowNextNodeExists;
        address allowNextNodeAddress;

        (allowNextNodeExists, allowNextNodeAddress) = AddressStructuredLinkedList.getNextNode(allowList, allowAddress);

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

    /*//////////////////////////////////////////////////////////////
                            RESTRICTED FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds an address to the requireAllList
    /// @dev Only the admin can call this function
    /// @param _address The address to add to the requireAllList
    /// @param addToHead A boolean indicating whether to add the address to the head (true) or tail (false) of the list
    function addRequireAllCheck(address _address, bool addToHead) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            !AddressStructuredLinkedList.nodeExists(requireAllList, _address),
            "Address already exists in requireAllList"
        );
        if (addToHead) {
            AddressStructuredLinkedList.pushFront(requireAllList, _address);
        } else {
            AddressStructuredLinkedList.pushBack(requireAllList, _address);
        }
    }

    /// @notice Removes an address from the requireAllList
    /// @dev Only the admin can call this function
    /// @param _address The address to remove from the requireAllList
    function removeRequireAllCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            AddressStructuredLinkedList.nodeExists(requireAllList, _address), "Address does not exist in requireAllList"
        );
        AddressStructuredLinkedList.remove(requireAllList, _address);
    }

    /// @notice Adds an address to the requireAnyList
    /// @dev Only the admin can call this function
    /// @param _address The address to add to the requireAnyList
    /// @param addToHead A boolean indicating whether to add the address to the head (true) or tail (false) of the list
    function addRequireAnyCheck(address _address, bool addToHead) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            !AddressStructuredLinkedList.nodeExists(requireAnyList, _address),
            "Address already exists in requireAnyList"
        );
        if (addToHead) {
            AddressStructuredLinkedList.pushFront(requireAnyList, _address);
        } else {
            AddressStructuredLinkedList.pushBack(requireAnyList, _address);
        }
    }

    /// @notice Removes an address from the requireAnyList
    /// @dev Only the admin can call this function
    /// @param _address The address to remove from the requireAnyList
    function removeRequireAnyCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            AddressStructuredLinkedList.nodeExists(requireAnyList, _address), "Address does not exist in requireAnyList"
        );
        AddressStructuredLinkedList.remove(requireAnyList, _address);
    }
}
