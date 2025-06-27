// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IAssertionPoster} from "src/withdrawal/IAssertionPoster.sol";

contract DummyPoster is IAssertionPoster {
    bytes32 public blockHash;
    bytes32 public sendRoot;

    function postAssertion(bytes32 blockHash_, bytes32 sendRoot_) external {
        blockHash = blockHash_;
        sendRoot = sendRoot_;
    }
}
