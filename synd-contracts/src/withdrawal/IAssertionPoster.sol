// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IAssertionPoster {
 /**
  * @notice Posts a new assertion to the rollup
  * @param blockHash Hash of the block
  * @param sendRoot Root of the sends
  */
 function postAssertion(bytes32 blockHash, bytes32 sendRoot) external;
}
