// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AtomicSequencerImplementation} from "./AtomicSequencerImplementation.sol";
import {Proxy} from "openzeppelin-contracts//contracts/proxy/Proxy.sol";

/// @title AtomicSequencer
/// @notice A wrapper contract that sequences transactions on two MetabasedChain contracts atomically.
/// Uses DELEGATECALL to maintain the original msg.sender context.
contract AtomicSequencer is Proxy {
    /// @notice The implementation contract containing the sequencing logic
    address public immutable implementation;

    constructor() {
        // Deploy the implementation contract and store its address
        implementation = address(new AtomicSequencerImplementation());
    }

    /// @inheritdoc Proxy
    function _implementation() internal view override returns (address) {
        return implementation;
    }

    /// @dev Prevents receiving Ether
    receive() external payable {
        revert();
    }
}
