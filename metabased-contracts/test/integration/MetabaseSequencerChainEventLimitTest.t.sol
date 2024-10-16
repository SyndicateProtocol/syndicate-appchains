// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract MetabaseSequencerChainEventLimitTest is Test {
    using Strings for uint256;

    MetabasedSequencerChain public chain;
    address public constant METABASE_SEQUENCER_CONTRACT_ADDRESS = 0x0E3E7d53c6451D62CE9f86201743587419Dc88Be;

    uint256 public constant OPTIMISM_BLOCK_GAS_LIMIT = 30_000_000; // Optimism block gas limit
    uint256 public constant OPTIMISM_TX_GAS_LIMIT = 30_000_000; // Optimism transaction gas limit (same as block for simplicity)
    uint256 public constant TX_SIZE = 32; // Minimal transaction size for testing

    function setUp() public {
        vm.createSelectFork("base_sepolia");
        chain = MetabasedSequencerChain(METABASE_SEQUENCER_CONTRACT_ADDRESS);
    }

    function testMaxEventsPerTransaction() public {
        uint256 gasLimit = OPTIMISM_TX_GAS_LIMIT;
        uint256 eventCount = 0;
        uint256 totalGasUsed = 0;

        bytes memory txn = new bytes(TX_SIZE);
        for (uint256 i = 0; i < TX_SIZE; i++) {
            txn[i] = bytes1(uint8(i % 256));
        }

        while (totalGasUsed < gasLimit) {
            uint256 gasBefore = gasleft();
            try chain.emitTransactionProcessed(txn) {
                uint256 gasUsed = gasBefore - gasleft();
                totalGasUsed += gasUsed;
                eventCount++;

                console.log(
                    string(
                        abi.encodePacked(
                            "Emitted event ",
                            eventCount.toString(),
                            ". Gas used: ",
                            gasUsed.toString(),
                            ". Total gas: ",
                            totalGasUsed.toString()
                        )
                    )
                );

                if (totalGasUsed > gasLimit) {
                    console.log("Exceeded transaction gas limit. Adjusting count.");
                    eventCount--;
                    break;
                }
            } catch Error(string memory reason) {
                console.log(string(abi.encodePacked("Failed to emit event. Reason: ", reason)));
                break;
            } catch (bytes memory) {
                console.log("Failed to emit event. Low level error occurred.");
                break;
            }
        }

        console.log(
            string(
                abi.encodePacked(
                    "Max events per transaction: ", eventCount.toString(), ". Total gas used: ", totalGasUsed.toString()
                )
            )
        );
    }

    function testMaxEventsPerBlock() public {
        uint256 gasLimit = OPTIMISM_BLOCK_GAS_LIMIT;
        uint256 eventCount = 0;
        uint256 totalGasUsed = 0;

        bytes memory txn = new bytes(TX_SIZE);
        for (uint256 i = 0; i < TX_SIZE; i++) {
            txn[i] = bytes1(uint8(i % 256));
        }

        while (totalGasUsed < gasLimit) {
            uint256 gasBefore = gasleft();
            try chain.emitTransactionProcessed(txn) {
                uint256 gasUsed = gasBefore - gasleft();
                totalGasUsed += gasUsed;
                eventCount++;

                if (eventCount % 1000 == 0) {
                    console.log(
                        string(
                            abi.encodePacked(
                                "Emitted event ", eventCount.toString(), ". Total gas: ", totalGasUsed.toString()
                            )
                        )
                    );
                }

                if (totalGasUsed > gasLimit) {
                    console.log("Exceeded block gas limit. Adjusting count.");
                    eventCount--;
                    break;
                }
            } catch Error(string memory reason) {
                console.log(string(abi.encodePacked("Failed to emit event. Reason: ", reason)));
                break;
            } catch (bytes memory) {
                console.log("Failed to emit event. Low level error occurred.");
                break;
            }
        }

        console.log(
            string(
                abi.encodePacked(
                    "Max events per block: ", eventCount.toString(), ". Total gas used: ", totalGasUsed.toString()
                )
            )
        );
    }
}
