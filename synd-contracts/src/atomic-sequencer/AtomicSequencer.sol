// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AtomicSequencerImplementation} from "./AtomicSequencerImplementation.sol";
import {Proxy} from "@openzeppelin/contracts/proxy/Proxy.sol";

/// @title AtomicSequencer
/// @notice A middleware contract for atomic cross-chain transaction sequencing in the Syndicate protocol
///
/// @dev ARCHITECTURAL DESIGN - "SECURE BY MODULE DESIGN":
/// This contract intentionally has NO access control modifiers because it follows the
/// Syndicate protocol's "secure by module design" architecture where:
///
/// 1. TRANSACTION ROUTING: AtomicSequencer acts as a transaction coordinator/router
/// 2. SECURITY ENFORCEMENT: Each SyndicateSequencingChain enforces security via custom modules
/// 3. DEVELOPER RESPONSIBILITY: Module developers implement authorization logic for their use cases
///
/// SECURITY FLOW:
/// User → AtomicSequencer → SyndicateSequencingChain → PermissionModule
///                                ↓
///                        Module evaluates both:
///                        - msg.sender (AtomicSequencer address)
///                        - tx.origin (actual user address)
///                                ↓
///                        Custom logic approves/rejects transaction
///
/// This design enables sophisticated middleware patterns while maintaining security through
/// modular permission systems. Module developers must implement proper tx.origin + msg.sender
/// validation to support trusted middleware contracts like this AtomicSequencer.
///
/// @dev Uses DELEGATECALL proxy pattern to preserve msg.sender context in the implementation
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
}
