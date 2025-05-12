// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import "src/LinkedList/AddressStructuredLinkedList.sol";

contract AddressStructuredLinkedListTest is Test {
    using AddressStructuredLinkedList for AddressStructuredLinkedList.List;

    AddressStructuredLinkedList.List private list;

    // Test addresses
    address private constant ADDR1 = address(0x1);
    address private constant ADDR2 = address(0x2);
    address private constant ADDR3 = address(0x3);
    address private constant ADDR4 = address(0x4);
    address private constant ADDR5 = address(0x5);
    address private constant NULL_ADDR = address(0);
    address private constant HEAD = address(0);

    function test_InitialState() public view {
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());
        assertEq(list.getHead(), NULL_ADDR);
    }

    function test_PushFront() public {
        // Push one item to the front
        assertTrue(list.pushFront(ADDR1));
        assertEq(list.sizeOf(), 1);
        assertTrue(list.listExists());
        assertEq(list.getHead(), ADDR1);

        // Push another item to the front
        assertTrue(list.pushFront(ADDR2));
        assertEq(list.sizeOf(), 2);
        assertEq(list.getHead(), ADDR2);

        // Make sure you can't push the same address twice
        assertFalse(list.pushFront(ADDR2));
        assertEq(list.sizeOf(), 2);
    }

    function test_PushBack() public {
        // Push one item to the back
        assertTrue(list.pushBack(ADDR1));
        assertEq(list.sizeOf(), 1);
        assertTrue(list.listExists());
        assertEq(list.getHead(), ADDR1);

        // Push another item to the back
        assertTrue(list.pushBack(ADDR2));
        assertEq(list.sizeOf(), 2);

        // Head should still be ADDR1
        assertEq(list.getHead(), ADDR1);

        // ADDR2 should be after ADDR1
        bool exists;
        address next;
        (exists, next) = list.getNextNode(ADDR1);
        assertTrue(exists);
        assertEq(next, ADDR2);

        // Make sure you can't push the same address twice
        assertFalse(list.pushBack(ADDR2));
        assertEq(list.sizeOf(), 2);
    }

    function test_InsertAfter() public {
        // Add two items
        list.pushFront(ADDR1);
        list.pushFront(ADDR3);
        assertEq(list.sizeOf(), 2);

        // Insert ADDR2 after ADDR3
        assertTrue(list.insertAfter(ADDR3, ADDR2));
        assertEq(list.sizeOf(), 3);

        // Check order
        assertEq(list.getHead(), ADDR3);
        bool exists;
        address next;
        (exists, next) = list.getNextNode(ADDR3);
        assertTrue(exists);
        assertEq(next, ADDR2);

        (exists, next) = list.getNextNode(ADDR2);
        assertTrue(exists);
        assertEq(next, ADDR1);

        // Try to insert an address that already exists
        assertFalse(list.insertAfter(ADDR3, ADDR2));
        assertEq(list.sizeOf(), 3);

        // Try to insert after a non-existent node
        assertFalse(list.insertAfter(ADDR4, ADDR5));
        assertEq(list.sizeOf(), 3);
    }

    function test_InsertBefore() public {
        // Add two items
        list.pushFront(ADDR1);
        list.pushFront(ADDR3);
        assertEq(list.sizeOf(), 2);

        // Insert ADDR2 before ADDR1
        assertTrue(list.insertBefore(ADDR1, ADDR2));
        assertEq(list.sizeOf(), 3);

        // Check order
        assertEq(list.getHead(), ADDR3);
        bool exists;
        address next;
        (exists, next) = list.getNextNode(ADDR3);
        assertTrue(exists);
        assertEq(next, ADDR2);

        (exists, next) = list.getNextNode(ADDR2);
        assertTrue(exists);
        assertEq(next, ADDR1);

        // Try to insert an address that already exists
        assertFalse(list.insertBefore(ADDR1, ADDR2));
        assertEq(list.sizeOf(), 3);

        // Try to insert before a non-existent node
        assertFalse(list.insertBefore(ADDR4, ADDR5));
        assertEq(list.sizeOf(), 3);
    }

    function test_Remove() public {
        // Create a list with 3 elements
        list.pushFront(ADDR1);
        list.pushFront(ADDR2);
        list.pushFront(ADDR3);
        assertEq(list.sizeOf(), 3);

        // Remove the middle element
        assertEq(list.remove(ADDR2), ADDR2);
        assertEq(list.sizeOf(), 2);

        // Check that ADDR3 now connects directly to ADDR1
        bool exists;
        address next;
        (exists, next) = list.getNextNode(ADDR3);
        assertTrue(exists);
        assertEq(next, ADDR1);

        // Try to remove a non-existent element
        assertEq(list.remove(ADDR4), NULL_ADDR);
        assertEq(list.sizeOf(), 2);

        // Try to remove NULL_ADDR
        assertEq(list.remove(NULL_ADDR), NULL_ADDR);
        assertEq(list.sizeOf(), 2);
    }

    function test_PopFront() public {
        // Create a list with 3 elements
        list.pushFront(ADDR1);
        list.pushFront(ADDR2);
        list.pushFront(ADDR3);
        assertEq(list.sizeOf(), 3);

        // Pop the front element
        assertEq(list.popFront(), ADDR3);
        assertEq(list.sizeOf(), 2);
        assertEq(list.getHead(), ADDR2);

        // Pop another front element
        assertEq(list.popFront(), ADDR2);
        assertEq(list.sizeOf(), 1);
        assertEq(list.getHead(), ADDR1);

        // Pop the last element
        assertEq(list.popFront(), ADDR1);
        assertEq(list.sizeOf(), 0);
        assertEq(list.getHead(), NULL_ADDR);

        // Try to pop from an empty list
        assertEq(list.popFront(), NULL_ADDR);
    }

    function test_PopBack() public {
        // Create a list with 3 elements
        list.pushFront(ADDR1);
        list.pushFront(ADDR2);
        list.pushFront(ADDR3);
        assertEq(list.sizeOf(), 3);

        // Pop the back element
        assertEq(list.popBack(), ADDR1);
        assertEq(list.sizeOf(), 2);

        // Head should still be ADDR3
        assertEq(list.getHead(), ADDR3);

        // Pop another back element
        assertEq(list.popBack(), ADDR2);
        assertEq(list.sizeOf(), 1);
        assertEq(list.getHead(), ADDR3);

        // Pop the last element
        assertEq(list.popBack(), ADDR3);
        assertEq(list.sizeOf(), 0);
        assertEq(list.getHead(), NULL_ADDR);

        // Try to pop from an empty list
        assertEq(list.popBack(), NULL_ADDR);
    }

    function test_GetNode() public {
        // Create a list with 3 elements
        list.pushFront(ADDR1);
        list.pushFront(ADDR2);
        list.pushFront(ADDR3);

        // Get the node in the middle
        bool exists;
        address prev;
        address next;
        (exists, prev, next) = list.getNode(ADDR2);
        assertTrue(exists);
        assertEq(prev, ADDR3);
        assertEq(next, ADDR1);

        // Get the first node
        (exists, prev, next) = list.getNode(ADDR3);
        assertTrue(exists);
        assertEq(prev, HEAD);
        assertEq(next, ADDR2);

        // Get the last node
        (exists, prev, next) = list.getNode(ADDR1);
        assertTrue(exists);
        assertEq(prev, ADDR2);
        assertEq(next, HEAD);

        // Try to get a non-existent node
        (exists, prev, next) = list.getNode(ADDR4);
        assertFalse(exists);
        assertEq(prev, NULL_ADDR);
        assertEq(next, NULL_ADDR);
    }

    function test_NodeExists() public {
        // Create a list with elements
        list.pushFront(ADDR1);

        // Check existing node
        assertTrue(list.nodeExists(ADDR1));

        // Check non-existing node
        assertFalse(list.nodeExists(ADDR2));

        // Empty list case is already checked in test_InitialState
    }

    function test_ListTraversal() public {
        // Create a list with 5 elements
        list.pushBack(ADDR1);
        list.pushBack(ADDR2);
        list.pushBack(ADDR3);
        list.pushBack(ADDR4);
        list.pushBack(ADDR5);

        // Traverse forward
        address current = list.getHead();
        assertEq(current, ADDR1);

        bool exists;
        address next;
        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, ADDR2);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, ADDR3);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, ADDR4);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, ADDR5);
        current = next;

        // Last element should not have a next
        (exists, next) = list.getNextNode(current);
        assertFalse(exists);

        // Traverse backward
        current = ADDR5;

        address prev;
        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, ADDR4);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, ADDR3);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, ADDR2);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, ADDR1);
        current = prev;

        // First element should not have a previous
        (exists, prev) = list.getPreviousNode(current);
        assertFalse(exists);
    }

    function test_GetAdjacent() public {
        // Create a list with elements
        list.pushBack(ADDR1);
        list.pushBack(ADDR2);

        // Forward direction (NEXT)
        bool exists;
        address adjacent;
        (exists, adjacent) = list.getAdjacent(ADDR1, true); // true = NEXT
        assertTrue(exists);
        assertEq(adjacent, ADDR2);

        // Backward direction (PREV)
        (exists, adjacent) = list.getAdjacent(ADDR2, false); // false = PREV
        assertTrue(exists);
        assertEq(adjacent, ADDR1);

        // Non-existent node
        (exists, adjacent) = list.getAdjacent(ADDR3, true);
        assertFalse(exists);
        assertEq(adjacent, NULL_ADDR);
    }

    function test_EdgeCases() public {
        // Test with an empty list
        assertFalse(list.nodeExists(ADDR1));
        assertEq(list.sizeOf(), 0);

        // Add and remove to get back to empty state
        list.pushFront(ADDR1);
        list.remove(ADDR1);
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());

        // Insert in an empty list - should not work for insertBefore/insertAfter
        assertFalse(list.insertAfter(ADDR1, ADDR2));
        assertFalse(list.insertBefore(ADDR1, ADDR2));

        // Add one element
        list.pushFront(ADDR1);

        // The element should connect to itself in a circular way
        bool exists;
        address prev;
        address next;
        (exists, prev, next) = list.getNode(ADDR1);
        assertTrue(exists);
        assertEq(prev, HEAD);
        assertEq(next, HEAD);
    }

    function test_ComplexOperations() public {
        // Build a list with multiple operations
        assertTrue(list.pushFront(ADDR3)); // [3]
        assertTrue(list.pushBack(ADDR5)); // [3, 5]
        assertTrue(list.insertAfter(ADDR3, ADDR4)); // [3, 4, 5]
        assertTrue(list.insertBefore(ADDR3, ADDR2)); // [2, 3, 4, 5]
        assertTrue(list.insertBefore(ADDR2, ADDR1)); // [1, 2, 3, 4, 5]

        assertEq(list.sizeOf(), 5);

        // Remove from the middle
        assertEq(list.remove(ADDR3), ADDR3); // [1, 2, 4, 5]
        assertEq(list.sizeOf(), 4);

        // The connection should now be from 2 to 4
        bool exists;
        address next;
        (exists, next) = list.getNextNode(ADDR2);
        assertTrue(exists);
        assertEq(next, ADDR4);

        // Remove from the beginning
        assertEq(list.remove(ADDR1), ADDR1); // [2, 4, 5]
        assertEq(list.sizeOf(), 3);
        assertEq(list.getHead(), ADDR2);

        // Remove from the end
        assertEq(list.remove(ADDR5), ADDR5); // [2, 4]
        assertEq(list.sizeOf(), 2);

        // Last node should not have next
        (exists, next) = list.getNextNode(ADDR4);
        assertFalse(exists);

        // Pop remaining elements
        assertEq(list.popFront(), ADDR2); // [4]
        assertEq(list.popBack(), ADDR4); // []

        // List should be empty
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());
    }
}
