// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface IAssertionPoster {
    function postAssertion(bytes32 blockHash, bytes32 sendRoot) external;
}
