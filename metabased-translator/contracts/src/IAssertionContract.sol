pragma solidity ^0.8.25;

interface IAssertionPoster {
    function postAssertion(bytes32 blockHash, bytes32 sendRoot) external;
}
