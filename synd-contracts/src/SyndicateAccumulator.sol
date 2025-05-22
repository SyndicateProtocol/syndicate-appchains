// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface ArbSys {
    function arbBlockNumber() external view returns (uint256);
}

ArbSys constant arbsys = ArbSys(0x0000000000000000000000000000000000000064);

contract SyndicateAccumulator {
    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    /// @custom:storage-location
    struct Accumulator {
        bytes32 acc;
    }

    bool immutable isArbChain;

    constructor() {
        isArbChain = (address(arbsys).code.length > 0);
    }

    // keccak256("syndicate.accumulator")
    bytes32 public constant ACCUMULATOR_STORAGE_LOCATION =
        0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90;

    function _getAccumulator() private pure returns (Accumulator storage $) {
        assembly {
            $.slot := ACCUMULATOR_STORAGE_LOCATION
        }
    }

    function accumulator() public view returns (bytes32) {
        return _getAccumulator().acc;
    }

    function _transactionProcessed(bytes memory data) internal {
        uint64 blockNumber;

        if (isArbChain) {
            blockNumber = uint64(arbsys.arbBlockNumber());
        } else {
            blockNumber = uint64(block.number);
        }

        Accumulator storage acc = _getAccumulator();
        acc.acc = keccak256(
            abi.encodePacked(
                acc.acc, keccak256(abi.encodePacked(msg.sender, blockNumber, uint64(block.timestamp), keccak256(data)))
            )
        );
        emit TransactionProcessed(msg.sender, data);
    }
}
