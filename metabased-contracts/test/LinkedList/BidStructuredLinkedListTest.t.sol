// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import "src/LinkedList/BidStructuredLinkedList.sol";

contract BidStructuredLinkedListTest is Test {
    using BidStructuredLinkedList for BidStructuredLinkedList.List;

    BidStructuredLinkedList.List private list;

    // Test addresses
    address private constant BIDDER1 = address(0x1);
    address private constant BIDDER2 = address(0x2);
    address private constant BIDDER3 = address(0x3);
    address private constant BIDDER4 = address(0x4);
    address private constant BIDDER5 = address(0x5);

    // Test bid amounts
    uint256 private constant BID1 = 100;
    uint256 private constant BID2 = 200;
    uint256 private constant BID3 = 300;
    uint256 private constant BID4 = 400;
    uint256 private constant BID5 = 500;

    uint256 private constant NULL_BID = 0;
    uint256 private constant HEAD = 0;

    function test_InitialState() public view {
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());
        assertEq(list.getHead(), NULL_BID);
    }

    function test_PushFront() public {
        // Push one item to the front
        assertTrue(list.pushFront(BID1, BIDDER1));
        assertEq(list.sizeOf(), 1);
        assertTrue(list.listExists());
        assertEq(list.getHead(), BID1);

        // Push another item to the front
        assertTrue(list.pushFront(BID2, BIDDER2));
        assertEq(list.sizeOf(), 2);
        assertEq(list.getHead(), BID2);

        // Make sure you can't push the same bid amount twice
        assertFalse(list.pushFront(BID2, BIDDER3));
        assertEq(list.sizeOf(), 2);

        // Verify the bidder at BID2 is still BIDDER2
        bool exists;
        uint256 prev;
        uint256 next;
        address bidder;
        (exists, prev, next, bidder) = list.getNode(BID2);
        assertTrue(exists);
        assertEq(bidder, BIDDER2);
    }

    function test_PushBack() public {
        // Push one item to the back
        assertTrue(list.pushBack(BID1, BIDDER1));
        assertEq(list.sizeOf(), 1);
        assertTrue(list.listExists());
        assertEq(list.getHead(), BID1);

        // Push another item to the back
        assertTrue(list.pushBack(BID2, BIDDER2));
        assertEq(list.sizeOf(), 2);

        // Head should still be BID1
        assertEq(list.getHead(), BID1);

        // BID2 should be after BID1
        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(BID1);
        assertTrue(exists);
        assertEq(next, BID2);

        // Make sure you can't push the same bid amount twice
        assertFalse(list.pushBack(BID2, BIDDER3));
        assertEq(list.sizeOf(), 2);
    }

    function test_InsertAfter() public {
        // Add two items
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID3, BIDDER3);
        assertEq(list.sizeOf(), 2);

        // Insert BID2 after BID3
        assertTrue(list.insertAfter(BID3, BID2, BIDDER2));
        assertEq(list.sizeOf(), 3);

        // Check order
        assertEq(list.getHead(), BID3);
        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(BID3);
        assertTrue(exists);
        assertEq(next, BID2);

        (exists, next) = list.getNextNode(BID2);
        assertTrue(exists);
        assertEq(next, BID1);

        // Try to insert a bid amount that already exists
        assertFalse(list.insertAfter(BID3, BID2, BIDDER4));
        assertEq(list.sizeOf(), 3);

        // Try to insert after a non-existent node
        assertFalse(list.insertAfter(BID4, BID5, BIDDER5));
        assertEq(list.sizeOf(), 3);
    }

    function test_InsertBefore() public {
        // Add two items
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID3, BIDDER3);
        assertEq(list.sizeOf(), 2);

        // Insert BID2 before BID1
        assertTrue(list.insertBefore(BID1, BID2, BIDDER2));
        assertEq(list.sizeOf(), 3);

        // Check order
        assertEq(list.getHead(), BID3);
        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(BID3);
        assertTrue(exists);
        assertEq(next, BID2);

        (exists, next) = list.getNextNode(BID2);
        assertTrue(exists);
        assertEq(next, BID1);

        // Try to insert a bid amount that already exists
        assertFalse(list.insertBefore(BID1, BID2, BIDDER4));
        assertEq(list.sizeOf(), 3);

        // Try to insert before a non-existent node
        assertFalse(list.insertBefore(BID4, BID5, BIDDER5));
        assertEq(list.sizeOf(), 3);
    }

    function test_Remove() public {
        // Create a list with 3 elements
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID2, BIDDER2);
        list.pushFront(BID3, BIDDER3);
        assertEq(list.sizeOf(), 3);

        // Remove the middle element
        assertEq(list.remove(BID2), BID2);
        assertEq(list.sizeOf(), 2);

        // Check that BID3 now connects directly to BID1
        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(BID3);
        assertTrue(exists);
        assertEq(next, BID1);

        // Verify that the bidder mapping is also removed
        bool nodeExists;
        uint256 prev;
        address bidder;
        (nodeExists, prev, next, bidder) = list.getNode(BID2);
        assertFalse(nodeExists);
        assertEq(bidder, address(0));

        // Try to remove a non-existent element
        assertEq(list.remove(BID4), 0);
        assertEq(list.sizeOf(), 2);

        // Try to remove NULL_BID
        assertEq(list.remove(NULL_BID), 0);
        assertEq(list.sizeOf(), 2);
    }

    function test_PopFront() public {
        // Create a list with 3 elements
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID2, BIDDER2);
        list.pushFront(BID3, BIDDER3);
        assertEq(list.sizeOf(), 3);

        // Pop the front element
        uint256 bid;
        address bidder;
        (bid, bidder) = list.popFront();
        assertEq(bid, BID3);
        assertEq(bidder, BIDDER3);
        assertEq(list.sizeOf(), 2);
        assertEq(list.getHead(), BID2);

        // Pop another front element
        (bid, bidder) = list.popFront();
        assertEq(bid, BID2);
        assertEq(bidder, BIDDER2);
        assertEq(list.sizeOf(), 1);
        assertEq(list.getHead(), BID1);

        // Pop the last element
        (bid, bidder) = list.popFront();
        assertEq(bid, BID1);
        assertEq(bidder, BIDDER1);
        assertEq(list.sizeOf(), 0);
        assertEq(list.getHead(), NULL_BID);

        // Try to pop from an empty list
        (bid, bidder) = list.popFront();
        assertEq(bid, 0);
        assertEq(bidder, address(0));
    }

    function test_PopBack() public {
        // Create a list with 3 elements
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID2, BIDDER2);
        list.pushFront(BID3, BIDDER3);
        assertEq(list.sizeOf(), 3);

        // Pop the back element
        uint256 bid;
        address bidder;
        (bid, bidder) = list.popBack();
        assertEq(bid, BID1);
        assertEq(bidder, BIDDER1);
        assertEq(list.sizeOf(), 2);

        // Head should still be BID3
        assertEq(list.getHead(), BID3);

        // Pop another back element
        (bid, bidder) = list.popBack();
        assertEq(bid, BID2);
        assertEq(bidder, BIDDER2);
        assertEq(list.sizeOf(), 1);
        assertEq(list.getHead(), BID3);

        // Pop the last element
        (bid, bidder) = list.popBack();
        assertEq(bid, BID3);
        assertEq(bidder, BIDDER3);
        assertEq(list.sizeOf(), 0);
        assertEq(list.getHead(), NULL_BID);

        // Try to pop from an empty list
        (bid, bidder) = list.popBack();
        assertEq(bid, 0);
        assertEq(bidder, address(0));
    }

    function test_GetNode() public {
        // Create a list with 3 elements
        list.pushFront(BID1, BIDDER1);
        list.pushFront(BID2, BIDDER2);
        list.pushFront(BID3, BIDDER3);

        // Get the node in the middle
        bool exists;
        uint256 prev;
        uint256 next;
        address bidder;
        (exists, prev, next, bidder) = list.getNode(BID2);
        assertTrue(exists);
        assertEq(prev, BID3);
        assertEq(next, BID1);
        assertEq(bidder, BIDDER2);

        // Get the first node
        (exists, prev, next, bidder) = list.getNode(BID3);
        assertTrue(exists);
        assertEq(prev, HEAD);
        assertEq(next, BID2);
        assertEq(bidder, BIDDER3);

        // Get the last node
        (exists, prev, next, bidder) = list.getNode(BID1);
        assertTrue(exists);
        assertEq(prev, BID2);
        assertEq(next, HEAD);
        assertEq(bidder, BIDDER1);

        // Try to get a non-existent node
        (exists, prev, next, bidder) = list.getNode(BID4);
        assertFalse(exists);
        assertEq(prev, 0);
        assertEq(next, 0);
        assertEq(bidder, address(0));
    }

    function test_NodeExists() public {
        // Create a list with elements
        list.pushFront(BID1, BIDDER1);

        // Check existing node
        assertTrue(list.nodeExists(BID1));

        // Check non-existing node
        assertFalse(list.nodeExists(BID2));

        // Empty list case is already checked in test_InitialState
    }

    function test_ListTraversal() public {
        // Create a list with 5 elements
        list.pushBack(BID1, BIDDER1);
        list.pushBack(BID2, BIDDER2);
        list.pushBack(BID3, BIDDER3);
        list.pushBack(BID4, BIDDER4);
        list.pushBack(BID5, BIDDER5);

        // Traverse forward
        uint256 current = list.getHead();
        assertEq(current, BID1);

        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, BID2);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, BID3);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, BID4);
        current = next;

        (exists, next) = list.getNextNode(current);
        assertTrue(exists);
        assertEq(next, BID5);
        current = next;

        // Last element should not have a next
        (exists, next) = list.getNextNode(current);
        assertFalse(exists);

        // Traverse backward
        current = BID5;

        uint256 prev;
        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, BID4);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, BID3);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, BID2);
        current = prev;

        (exists, prev) = list.getPreviousNode(current);
        assertTrue(exists);
        assertEq(prev, BID1);
        current = prev;

        // First element should not have a previous
        (exists, prev) = list.getPreviousNode(current);
        assertFalse(exists);
    }

    function test_GetAdjacent() public {
        // Create a list with elements
        list.pushBack(BID1, BIDDER1);
        list.pushBack(BID2, BIDDER2);

        // Forward direction (NEXT)
        bool exists;
        uint256 adjacent;
        (exists, adjacent) = list.getAdjacent(BID1, true); // true = NEXT
        assertTrue(exists);
        assertEq(adjacent, BID2);

        // Backward direction (PREV)
        (exists, adjacent) = list.getAdjacent(BID2, false); // false = PREV
        assertTrue(exists);
        assertEq(adjacent, BID1);

        // Non-existent node
        (exists, adjacent) = list.getAdjacent(BID3, true);
        assertFalse(exists);
        assertEq(adjacent, 0);
    }

    function test_EdgeCases() public {
        // Test with an empty list
        assertFalse(list.nodeExists(BID1));
        assertEq(list.sizeOf(), 0);

        // Add and remove to get back to empty state
        list.pushFront(BID1, BIDDER1);
        list.remove(BID1);
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());

        // Insert in an empty list - should not work for insertBefore/insertAfter
        assertFalse(list.insertAfter(BID1, BID2, BIDDER2));
        assertFalse(list.insertBefore(BID1, BID2, BIDDER2));

        // Add one element
        list.pushFront(BID1, BIDDER1);

        // The element should connect to itself in a circular way
        bool exists;
        uint256 prev;
        uint256 next;
        address bidder;
        (exists, prev, next, bidder) = list.getNode(BID1);
        assertTrue(exists);
        assertEq(prev, HEAD);
        assertEq(next, HEAD);
        assertEq(bidder, BIDDER1);
    }

    function test_ComplexOperations() public {
        // Build a list with multiple operations
        assertTrue(list.pushFront(BID3, BIDDER3)); // [3]
        assertTrue(list.pushBack(BID5, BIDDER5)); // [3, 5]
        assertTrue(list.insertAfter(BID3, BID4, BIDDER4)); // [3, 4, 5]
        assertTrue(list.insertBefore(BID3, BID2, BIDDER2)); // [2, 3, 4, 5]
        assertTrue(list.insertBefore(BID2, BID1, BIDDER1)); // [1, 2, 3, 4, 5]

        assertEq(list.sizeOf(), 5);

        // Remove from the middle
        assertEq(list.remove(BID3), BID3); // [1, 2, 4, 5]
        assertEq(list.sizeOf(), 4);

        // The connection should now be from 2 to 4
        bool exists;
        uint256 next;
        (exists, next) = list.getNextNode(BID2);
        assertTrue(exists);
        assertEq(next, BID4);

        // Remove from the beginning
        assertEq(list.remove(BID1), BID1); // [2, 4, 5]
        assertEq(list.sizeOf(), 3);
        assertEq(list.getHead(), BID2);

        // Remove from the end
        assertEq(list.remove(BID5), BID5); // [2, 4]
        assertEq(list.sizeOf(), 2);

        // Last node should not have next
        (exists, next) = list.getNextNode(BID4);
        assertFalse(exists);

        // Pop remaining elements
        uint256 bid;
        address bidder;
        (bid, bidder) = list.popFront();
        assertEq(bid, BID2);
        assertEq(bidder, BIDDER2);
        assertEq(list.sizeOf(), 1);

        (bid, bidder) = list.popBack();
        assertEq(bid, BID4);
        assertEq(bidder, BIDDER4);
        assertEq(list.sizeOf(), 0);

        // List should be empty
        assertEq(list.sizeOf(), 0);
        assertFalse(list.listExists());
    }

    function test_ErrorHandling() public {
        // Test behavior when removing an already removed node
        list.pushFront(BID1, BIDDER1);
        list.remove(BID1);
        assertEq(list.remove(BID1), 0);

        // Test behavior when trying to access adjacent nodes of non-existent node
        (bool exists, uint256 next) = list.getNextNode(BID2);
        assertFalse(exists);
        assertEq(next, 0);

        // Test _pop with empty list (this calls into getAdjacent)
        (uint256 bid, address bidder) = list.popFront();
        assertEq(bid, 0);
        assertEq(bidder, address(0));
    }

    function test_BidderMapping() public {
        // Make sure bidder mapping is properly maintained
        list.pushFront(BID1, BIDDER1);

        bool exists;
        uint256 prev;
        uint256 next;
        address bidder;
        (exists, prev, next, bidder) = list.getNode(BID1);
        assertTrue(exists);
        assertEq(bidder, BIDDER1);

        // Update a bidder by removing and re-adding
        list.remove(BID1);
        list.pushFront(BID1, BIDDER2);

        (exists, prev, next, bidder) = list.getNode(BID1);
        assertTrue(exists);
        assertEq(bidder, BIDDER2);
    }
}
