// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import "@arbitrum/nitro-contracts/src/precompiles/ArbSys.sol";

// The ArbSys precompile address on Arbitrum chains
ArbSys constant arbsys = ArbSys(0x0000000000000000000000000000000000000064);

/// @title SyndicateAccumulator
/// @notice A contract that maintains a cryptographic accumulator of processed transactions
/// @dev The accumulator creates a tamper-evident chain of transaction hashes that includes:
/// - Previous accumulator state
/// - Transaction sender
/// - Block number (Arbitrum-aware)
/// - Block timestamp
/// - Transaction data hash
/// This provides a verifiable history of all processed transactions for off-chain verification
contract SyndicateAccumulator {
    /// @notice Emitted when a new transaction is processed and added to the accumulator
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Storage structure for the accumulator state
    /// @dev Uses a custom storage slot to avoid storage collisions in inheritance
    struct Accumulator {
        /// @notice The current accumulator hash value
        bytes32 acc;
    }

    /// @notice Whether this contract is deployed on an Arbitrum chain
    /// @dev Determined at construction time and affects block number retrieval
    bool public immutable isArbChain;

    /// @notice Constructs the SyndicateAccumulator contract
    /// @dev Detects if running on Arbitrum by checking for the ArbSys precompile
    /// and validates it's working correctly if present
    constructor() {
        // Check if ArbSys precompile exists (indicates Arbitrum chain)
        isArbChain = (address(arbsys).code.length > 0);

        // If on Arbitrum, verify the precompile is functional
        require(!isArbChain || arbsys.arbBlockNumber() > 0, "ArbSys precompile validation failed");
    }

    /// @notice Storage slot for the accumulator state
    /// @dev keccak256("syndicate.accumulator") - prevents storage collisions
    bytes32 public constant ACCUMULATOR_STORAGE_LOCATION =
        0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90;

    /// @notice Retrieves the accumulator storage reference
    /// @dev Uses inline assembly to access the custom storage slot
    /// @return $ Reference to the Accumulator storage struct
    function _getAccumulator() private pure returns (Accumulator storage $) {
        assembly {
            $.slot := ACCUMULATOR_STORAGE_LOCATION
        }
    }

    /// @notice Returns the current accumulator hash value
    /// @dev Public view function to read the current state of the accumulator
    /// @return The current accumulator hash as bytes32
    function accumulator() external view returns (bytes32) {
        return _getAccumulator().acc;
    }

    /// @notice Processes a transaction and updates the accumulator
    /// @dev Internal function that:
    /// 1. Gets the appropriate block number (Arbitrum-aware)
    /// 2. Creates a hash of transaction metadata
    /// 3. Updates the accumulator with chained hash
    /// 4. Emits a TransactionProcessed event
    /// @param data The transaction data to be processed and accumulated
    //#olympix-ignore-signature-replay-attacks
    function _transactionProcessed(bytes memory data) internal {
        uint64 blockNumber;

        // Use Arbitrum block number if on Arbitrum chain, otherwise use standard block number
        if (isArbChain) {
            blockNumber = uint64(arbsys.arbBlockNumber());
        } else {
            blockNumber = uint64(block.number);
        }

        // Get accumulator storage reference
        Accumulator storage acc = _getAccumulator();

        // Create transaction hash including all relevant metadata
        bytes32 transactionHash = keccak256(
            abi.encodePacked(
                msg.sender, // Who submitted the transaction
                blockNumber, // When it was submitted (block)
                uint64(block.timestamp), // When it was submitted (timestamp)
                keccak256(data) // What was submitted (data hash)
            )
        );

        // Update accumulator by chaining with previous state
        acc.acc = keccak256(abi.encodePacked(acc.acc, transactionHash));

        // Emit event for external observers
        emit TransactionProcessed(msg.sender, data);
    }
}
