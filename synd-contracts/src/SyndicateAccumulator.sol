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
    struct TxData {
        bytes32 acc;
        uint64 count;
    }

    // keccak256("syndicate.tx.data")
    bytes32 public constant TX_DATA_STORAGE_LOCATION =
        0xbcd134af035e52869741eb0221dfc8a26900a04521f5a2d44a59b675ea20a969;

    function _getTxData() private pure returns (TxData storage $) {
        assembly {
            $.slot := TX_DATA_STORAGE_LOCATION
        }
    }

    function txCount() public view returns (uint64) {
        return _getTxData().count;
    }

    function txAcc() public view returns (bytes32) {
        return _getTxData().acc;
    }

    function _transactionProcessed(bytes memory data) internal {
        TxData storage txData = _getTxData();
        uint256 blockNumber = block.number;
        if (address(arbsys).code.length > 0) {
            try arbsys.arbBlockNumber() returns (uint256 number) {
                blockNumber = number;
            } catch {}
        }

        txData.acc = keccak256(
            abi.encodePacked(txData.acc, msg.sender, blockNumber, block.timestamp, txData.count, keccak256(data))
        );
        txData.count += 1;
        emit TransactionProcessed(msg.sender, data);
    }
}
