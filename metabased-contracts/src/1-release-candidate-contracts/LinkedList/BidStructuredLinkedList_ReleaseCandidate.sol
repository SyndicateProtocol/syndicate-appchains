// SPDX-License-Identifier: MIT

pragma solidity 0.8.25;

/**
 * @title StructuredLinkedList
 * @author Vittorio Minacori (https://github.com/vittominacori)
 * @dev An utility library for using sorted linked list data structures in your Solidity project.
 * @notice Adapted from
 * https://github.com/vittominacori/solidity-linked-list/blob/master/contracts/StructuredLinkedList.sol
 * and
 * https://github.com/Layr-Labs/eigenlayer-contracts/blob/b087c530e002861818f0eb052eaa315b64cbf506/src/contracts/libraries/StructuredLinkedList.sol
 */
library BidStructuredLinkedList_ReleaseCandidate {
    uint256 private constant _NULL = 0;
    uint256 private constant _HEAD = 0;

    bool private constant _PREV = false;
    bool private constant _NEXT = true;

    struct List {
        uint256 size;
        mapping(uint256 bidAmount => mapping(bool direction => uint256 bidAmountNeighbor)) bidList;
        mapping(uint256 bidAmount => address bidder) bidder;
    }

    /**
     * @dev Checks if the list exists
     * @param self stored linked list from contract
     * @return bool true if list exists, false otherwise
     */
    function listExists(List storage self) internal view returns (bool) {
        // if the head nodes previous or next pointers both point to itself, then there are no items in the list
        if (self.bidList[_HEAD][_PREV] != _HEAD || self.bidList[_HEAD][_NEXT] != _HEAD) {
            return true;
        } else {
            return false;
        }
    }

    /**
     * @dev Checks if the node exists
     * @param self stored linked list from contract
     * @param _node a node to search for
     * @return bool true if node exists, false otherwise
     */
    function nodeExists(List storage self, uint256 _node) internal view returns (bool) {
        if (self.bidList[_node][_PREV] == _HEAD && self.bidList[_node][_NEXT] == _HEAD) {
            if (self.bidList[_HEAD][_NEXT] == _node) {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }

    /**
     * @dev Returns the number of elements in the list
     * @param self stored linked list from contract
     * @return uint256
     */
    function sizeOf(List storage self) internal view returns (uint256) {
        return self.size;
    }

    /**
     * @dev Gets the head of the list
     * @param self stored linked list from contract
     * @return uint256 the head of the list
     */
    function getHead(List storage self) internal view returns (uint256) {
        return self.bidList[_HEAD][_NEXT];
    }

    /**
     * @dev Returns the links and data of a node as a tuple.
     * @param self stored linked list from contract
     * @param _node id of the node to get
     * @return bool, uint256, uint256, address true if node exists or false
     * otherwise, previous node, next node, bidder address
     */
    function getNode(List storage self, uint256 _node) internal view returns (bool, uint256, uint256, address) {
        if (!nodeExists(self, _node)) {
            return (false, 0, 0, address(0));
        } else {
            return (true, self.bidList[_node][_PREV], self.bidList[_node][_NEXT], self.bidder[_node]);
        }
    }

    /**
     * @dev Returns the link of a node `_node` in direction `_direction`.
     * @param self stored linked list from contract
     * @param _node id of the node to step from
     * @param _direction direction to step in
     * @return bool, uint256 true if node exists or false otherwise, node in _direction
     */
    function getAdjacent(List storage self, uint256 _node, bool _direction) internal view returns (bool, uint256) {
        if (!nodeExists(self, _node)) {
            return (false, 0);
        } else {
            uint256 adjacent = self.bidList[_node][_direction];
            return (adjacent != _HEAD, adjacent);
        }
    }

    /**
     * @dev Returns the link of a node `_node` in direction `_NEXT`.
     * @param self stored linked list from contract
     * @param _node id of the node to step from
     * @return bool, uint256 true if node exists or false otherwise, next node
     */
    function getNextNode(List storage self, uint256 _node) internal view returns (bool, uint256) {
        return getAdjacent(self, _node, _NEXT);
    }

    /**
     * @dev Returns the link of a node `_node` in direction `_PREV`.
     * @param self stored linked list from contract
     * @param _node id of the node to step from
     * @return bool, uint256 true if node exists or false otherwise, previous node
     */
    function getPreviousNode(List storage self, uint256 _node) internal view returns (bool, uint256) {
        return getAdjacent(self, _node, _PREV);
    }

    /**
     * @dev Insert node `_newBid` beside existing node `_node` in direction `_NEXT`.
     * @param self stored linked list from contract
     * @param _node existing node
     * @param _newBid new bid amount to insert
     * @param _newBidder address of the new bidder
     * @return bool true if success, false otherwise
     */
    function insertAfter(List storage self, uint256 _node, uint256 _newBid, address _newBidder)
        internal
        returns (bool)
    {
        return _insert(self, _node, _newBid, _newBidder, _NEXT);
    }

    /**
     * @dev Insert node `_newBid` beside existing node `_node` in direction `_PREV`.
     * @param self stored linked list from contract
     * @param _node existing node
     * @param _newBid new bid amount to insert
     * @param _newBidder address of the new bidder
     * @return bool true if success, false otherwise
     */
    function insertBefore(List storage self, uint256 _node, uint256 _newBid, address _newBidder)
        internal
        returns (bool)
    {
        return _insert(self, _node, _newBid, _newBidder, _PREV);
    }

    /**
     * @dev Removes an entry from the linked list
     * @param self stored linked list from contract
     * @param _node node to remove from the list
     * @return uint256 the removed node
     */
    function remove(List storage self, uint256 _node) internal returns (uint256) {
        if ((_node == _NULL) || (!nodeExists(self, _node))) {
            return 0;
        }
        _createLink(self, self.bidList[_node][_PREV], self.bidList[_node][_NEXT], _NEXT);
        delete self.bidList[_node][_PREV];
        delete self.bidList[_node][_NEXT];
        delete self.bidder[_node];

        self.size -= 1;

        return _node;
    }

    /**
     * @dev Pushes an entry to the head of the linked list
     * @param self stored linked list from contract
     * @param _newBid new bid amount to push to the head
     * @param _newBidder address of the new bidder
     * @return bool true if success, false otherwise
     */
    function pushFront(List storage self, uint256 _newBid, address _newBidder) internal returns (bool) {
        return _push(self, _newBid, _newBidder, _NEXT);
    }

    /**
     * @dev Pushes an entry to the tail of the linked list
     * @param self stored linked list from contract
     * @param _newBid new bid amount to push to the tail
     * @param _newBidder address of the new bidder
     * @return bool true if success, false otherwise
     */
    function pushBack(List storage self, uint256 _newBid, address _newBidder) internal returns (bool) {
        return _push(self, _newBid, _newBidder, _PREV);
    }

    /**
     * @dev Pops the first entry from the head of the linked list
     * @param self stored linked list from contract
     * @return uint256, address the removed bid amount and bidder address
     */
    function popFront(List storage self) internal returns (uint256, address) {
        return _pop(self, _NEXT);
    }

    /**
     * @dev Pops the first entry from the tail of the linked list
     * @param self stored linked list from contract
     * @return uint256, address the removed bid amount and bidder address
     */
    function popBack(List storage self) internal returns (uint256, address) {
        return _pop(self, _PREV);
    }

    /**
     * @dev Pushes an entry to the head of the linked list
     * @param self stored linked list from contract
     * @param _newBid new bid amount to push
     * @param _newBidder address of the new bidder
     * @param _direction push to the head (_NEXT) or tail (_PREV)
     * @return bool true if success, false otherwise
     */
    function _push(List storage self, uint256 _newBid, address _newBidder, bool _direction) private returns (bool) {
        return _insert(self, _HEAD, _newBid, _newBidder, _direction);
    }

    /**
     * @dev Pops the first entry from the linked list
     * @param self stored linked list from contract
     * @param _direction pop from the head (_NEXT) or the tail (_PREV)
     * @return uint256, address the removed bid amount and bidder address
     */
    function _pop(List storage self, bool _direction) private returns (uint256, address) {
        uint256 adj;
        (, adj) = getAdjacent(self, _HEAD, _direction);
        uint256 bidAmount = remove(self, adj);
        return (bidAmount, self.bidder[bidAmount]);
    }

    /**
     * @dev Insert node `_newBid` beside existing node `_node` in direction `_direction`.
     * @param self stored linked list from contract
     * @param _node existing node
     * @param _newBid new bid amount to insert
     * @param _newBidder address of the new bidder
     * @param _direction direction to insert node in
     * @return bool true if success, false otherwise
     */
    function _insert(List storage self, uint256 _node, uint256 _newBid, address _newBidder, bool _direction)
        private
        returns (bool)
    {
        if (!nodeExists(self, _newBid) && nodeExists(self, _node)) {
            uint256 c = self.bidList[_node][_direction];
            self.bidder[_newBid] = _newBidder;
            _createLink(self, _node, _newBid, _direction);
            _createLink(self, _newBid, c, _direction);

            self.size += 1;

            return true;
        }

        return false;
    }

    /**
     * @dev Creates a bidirectional link between two nodes on direction `_direction`
     * @param self stored linked list from contract
     * @param _node existing node
     * @param _link node to link to in the _direction
     * @param _direction direction to insert node in
     */
    function _createLink(List storage self, uint256 _node, uint256 _link, bool _direction) private {
        self.bidList[_link][!_direction] = _node;
        self.bidList[_node][_direction] = _link;
    }
}
