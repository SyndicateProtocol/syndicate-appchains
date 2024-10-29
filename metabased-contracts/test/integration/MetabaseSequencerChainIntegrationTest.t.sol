// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract MetabaseSequencerChainIntegrationTest is Test {
    using Strings for uint256;

    MetabasedSequencerChain public chain;
    bool public isTxCompressed = true;

    address public constant METABASE_SEQUENCER_CONTRACT_ADDRESS = 0x73Ba7D784d13Ec0070a4Ea6F49Ff57dc007Bb48d; // deployed to base_sepolia

    uint256 public constant OPTIMISM_BLOCK_GAS_LIMIT = 30_000_000; // Optimism block gas limit
    uint256 public constant TX_SIZE = 1024; // 1 KB transaction size

    function setUp() public {
        vm.createSelectFork("base_sepolia");

        chain = MetabasedSequencerChain(METABASE_SEQUENCER_CONTRACT_ADDRESS);
    }

    function testIncreasingTransactionSize() public {
        vm.skip(true);
        uint256 startSize = 1024; // 1KB
        uint256 maxSize = 128 * 1024; // 128KB
        uint256 step = 1024; // Increase by 1KB each time

        for (uint256 size = startSize; size <= maxSize; size += step) {
            bytes memory txn = new bytes(size);
            for (uint256 i = 0; i < size; i++) {
                txn[i] = bytes1(uint8(i % 256));
            }

            try chain.processTransaction(txn, isTxCompressed) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Successfully processed transaction of size: ",
                            size.toString(),
                            " bytes. Gas used: ",
                            gasleft().toString()
                        )
                    )
                );
            } catch Error(string memory reason) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Failed to process transaction of size: ", size.toString(), " bytes. Reason: ", reason
                        )
                    )
                );
                break;
            } catch (bytes memory) /*lowLevelData*/ {
                console.log(
                    string(
                        abi.encodePacked(
                            "Failed to process transaction of size: ",
                            size.toString(),
                            " bytes. Low level error occurred"
                        )
                    )
                );
                break;
            }
        }
    }

    function testBulkTransactions() public {
        vm.skip(true);
        uint256 individualSize = 1024; // 1KB
        uint256 maxTotalSize = 128 * 1024; // 128KB
        uint256 maxTransactions = maxTotalSize / individualSize;

        for (uint256 count = 1; count <= maxTransactions; count++) {
            bytes[] memory txns = new bytes[](count);
            bool[] memory isTxsCompressed = new bool[](count);
            for (uint256 i = 0; i < count; i++) {
                bytes memory txn = new bytes(individualSize);
                for (uint256 j = 0; j < individualSize; j++) {
                    txn[j] = bytes1(uint8((i * individualSize + j) % 256));
                }
                txns[i] = txn;
                isTxsCompressed[i] = true;
            }

            try chain.processBulkTransactions(txns, isTxsCompressed) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Successfully processed bulk transactions. Count: ",
                            count.toString(),
                            " Total size: ",
                            (count * individualSize).toString(),
                            " bytes. Gas used: ",
                            gasleft().toString()
                        )
                    )
                );
            } catch Error(string memory reason) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Failed to process bulk transactions. Count: ",
                            count.toString(),
                            " Total size: ",
                            (count * individualSize).toString(),
                            " bytes. Reason: ",
                            reason
                        )
                    )
                );
                break;
            } catch (bytes memory) /*lowLevelData*/ {
                console.log(
                    string(
                        abi.encodePacked(
                            "Failed to process bulk transactions. Count: ",
                            count.toString(),
                            " Total size: ",
                            (count * individualSize).toString(),
                            " bytes. Low level error occurred"
                        )
                    )
                );
                break;
            }
        }
    }

    function testMaxTransactionsPerBlock() public {
        uint256 gasLimit = OPTIMISM_BLOCK_GAS_LIMIT;
        uint256 txCount = 0;
        uint256 totalGasUsed = 0;

        while (totalGasUsed < gasLimit) {
            bytes memory txn = new bytes(TX_SIZE);
            for (uint256 i = 0; i < TX_SIZE; i++) {
                txn[i] = bytes1(uint8(i % 256));
            }

            uint256 gasBefore = gasleft();
            try chain.processTransaction(txn, isTxCompressed) {
                uint256 gasUsed = gasBefore - gasleft();
                totalGasUsed += gasUsed;
                txCount++;

                console.log(
                    string(
                        abi.encodePacked(
                            "Processed transaction ",
                            txCount.toString(),
                            ". Gas used: ",
                            gasUsed.toString(),
                            ". Total gas: ",
                            totalGasUsed.toString()
                        )
                    )
                );

                if (totalGasUsed > gasLimit) {
                    console.log(string(abi.encodePacked("Exceeded block gas limit. Adjusting count.")));
                    txCount--;
                    break;
                }
            } catch Error(string memory reason) {
                console.log(string(abi.encodePacked("Failed to process transaction. Reason: ", reason)));
                break;
            } catch (bytes memory) {
                console.log(string(abi.encodePacked("Failed to process transaction. Low level error occurred.")));
                break;
            }
        }

        console.log(
            string(
                abi.encodePacked(
                    "Max transactions per block: ", txCount.toString(), ". Total gas used: ", totalGasUsed.toString()
                )
            )
        );
    }
}
