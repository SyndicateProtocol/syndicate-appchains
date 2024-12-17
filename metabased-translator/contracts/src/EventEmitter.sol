// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

/// @title EventEmitter - Generic event emission using EVM assembly
/// @notice Provides direct access to EVM log operations for emitting events
/// @dev Uses assembly to directly call EVM log opcodes (log1, log2, log3,
/// log4). This is necessary because we want to be able to emit events that may
/// change based on their source contracts, but may not know the events in
/// advance.
///
/// Example usage for a typical event:
/// ```solidity
/// event Transfer(address indexed from, address indexed to, uint256 amount);
/// ```
/// Would use log3 with:
/// - signatureHash: keccak256("Transfer(address,address,uint256)")
/// - indexed1: from address as bytes32
/// - indexed2: to address as bytes32
/// - nonIndexed: amount as bytes32
contract EventEmitter {
    /// @notice Emits an event with one topic (signature) and one non-indexed parameter
    /// @param signatureHash The event signature hash (topic0)
    /// @param nonIndexed The non-indexed parameter to be stored in event data
    /// @dev Uses EVM log1 opcode: Suitable for events with no indexed parameters
    function emitEvent1(bytes32 signatureHash, bytes32 nonIndexed) public {
        assembly {
            mstore(0, nonIndexed)
            log1(0, 0x20, signatureHash)
        }
    }

    /// @notice Emits an event with two topics (signature + 1 indexed) and one non-indexed parameter
    /// @param signatureHash The event signature hash (topic0)
    /// @param indexed1 The first indexed parameter (topic1)
    /// @param nonIndexed The non-indexed parameter to be stored in event data
    /// @dev Uses EVM log2 opcode: Suitable for events with one indexed parameter
    function emitEvent2(bytes32 signatureHash, bytes32 indexed1, bytes32 nonIndexed) public {
        assembly {
            mstore(0, nonIndexed)
            log2(0, 0x20, signatureHash, indexed1)
        }
    }

    /// @notice Emits an event with three topics (signature + 2 indexed) and one non-indexed parameter
    /// @param signatureHash The event signature hash (topic0)
    /// @param indexed1 The first indexed parameter (topic1)
    /// @param indexed2 The second indexed parameter (topic2)
    /// @param nonIndexed The non-indexed parameter to be stored in event data
    /// @dev Uses EVM log3 opcode: Suitable for events with two indexed parameters
    function emitEvent3(bytes32 signatureHash, bytes32 indexed1, bytes32 indexed2, bytes32 nonIndexed) public {
        assembly {
            mstore(0, nonIndexed)
            log3(0, 0x20, signatureHash, indexed1, indexed2)
        }
    }

    /// @notice Emits an event with four topics (signature + 3 indexed) and one non-indexed parameter
    /// @param signatureHash The event signature hash (topic0)
    /// @param indexed1 The first indexed parameter (topic1)
    /// @param indexed2 The second indexed parameter (topic2)
    /// @param indexed3 The third indexed parameter (topic3)
    /// @param nonIndexed The non-indexed parameter to be stored in event data
    /// @dev Uses EVM log4 opcode: Suitable for events with three indexed parameters (maximum possible)
    function emitEvent4(bytes32 signatureHash, bytes32 indexed1, bytes32 indexed2, bytes32 indexed3, bytes32 nonIndexed)
        public
    {
        assembly {
            mstore(0, nonIndexed)
            log4(0, 0x20, signatureHash, indexed1, indexed2, indexed3)
        }
    }
}
