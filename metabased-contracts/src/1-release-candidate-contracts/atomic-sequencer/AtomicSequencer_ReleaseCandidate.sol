// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AtomicSequencerImplementation_ReleaseCandidate} from "./AtomicSequencerImplementation_ReleaseCandidate.sol";
import {Proxy} from "openzeppelin-contracts/contracts/proxy/Proxy.sol";

/// @title AtomicSequencer
/// @notice A wrapper contract that sequences transactions on two MetabasedChain contracts atomically.
/// Uses DELEGATECALL to maintain the original msg.sender context.
contract AtomicSequencer_ReleaseCandidate is Proxy {
    /// @notice The implementation contract containing the sequencing logic
    address public immutable implementation;

    constructor() {
        // Deploy the implementation contract and store its address
        implementation = address(new AtomicSequencerImplementation_ReleaseCandidate());
    }

    /// @inheritdoc Proxy
    function _implementation() internal view override returns (address) {
        return implementation;
    }
}
