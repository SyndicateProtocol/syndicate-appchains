// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {console2} from "forge-std/console2.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";

/**
 * @title TransactionSizeLimitIntegrationTest
 * @notice Integration test for measuring transaction size limits on a forked chain
 * @dev Run with: forge test --match-contract TransactionSizeLimitIntegrationTest -vvv
 */
contract TransactionSizeLimitIntegrationTest is Test {
    MetabasedSequencerChain public sequencerChain;
    AllowlistSequencingModule public allowlistModule;
    RequireAllModule public requireAllModule;

    address public admin = address(0x1);
    uint256 public constant L3_CHAIN_ID = 1234;

    // We'll test with these increments to find the approximate limits
    // Can increase MAX_TEST_SIZE_KB if needed
    uint256 public constant MAX_TEST_SIZE_KB = 100;
    uint256 public constant INCREMENT_KB = 5;

    function setUp() public {
        // This test will be run against a forked chain (Exo)
        vm.createSelectFork("exo");

        vm.startPrank(admin);

        // Deploy the sequencer chain
        sequencerChain = new MetabasedSequencerChain(L3_CHAIN_ID);
        console2.log("Deployed MetabasedSequencerChain at:", address(sequencerChain));

        // Set up the permission modules
        allowlistModule = new AllowlistSequencingModule(admin);
        console2.log("Deployed AllowlistSequencingModule at:", address(allowlistModule));

        requireAllModule = new RequireAllModule(admin);
        console2.log("Deployed RequireAllModule at:", address(requireAllModule));

        // Add admin to allowlist
        allowlistModule.addToAllowlist(admin);

        // Add allowlist module to the requirement module
        requireAllModule.addPermissionCheck(address(allowlistModule), true);

        // Initialize the sequencer chain with the requirement module
        sequencerChain.initialize(admin, address(requireAllModule));

        vm.stopPrank();

        console2.log("Setup complete - testing against forked chain");
    }

    function testProcessTransactionSizeLimit() public {
        vm.startPrank(admin);

        uint256 lastSuccessful = 0;

        for (uint256 kb = INCREMENT_KB; kb <= MAX_TEST_SIZE_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransaction(data) {
                console2.log("Successfully processed transaction with size (KB):", kb);
                lastSuccessful = kb;
            } catch Error(string memory reason) {
                console2.log("Failed with reason:", reason);
                break;
            } catch {
                console2.log("Failed to process transaction at size (KB):", kb);
                break;
            }
        }

        console2.log("Maximum processTransaction size (KB):", lastSuccessful);
        vm.stopPrank();
    }

    function testProcessTransactionRawSizeLimit() public {
        vm.startPrank(admin);

        uint256 lastSuccessful = 0;

        for (uint256 kb = INCREMENT_KB; kb <= MAX_TEST_SIZE_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransactionRaw(data) {
                console2.log("Successfully processed raw transaction with size (KB):", kb);
                lastSuccessful = kb;
            } catch Error(string memory reason) {
                console2.log("Failed with reason:", reason);
                break;
            } catch {
                console2.log("Failed to process raw transaction at size (KB):", kb);
                break;
            }
        }

        console2.log("Maximum processTransactionRaw size (KB):", lastSuccessful);
        vm.stopPrank();
    }

    function testGasCosts() public {
        vm.startPrank(admin);

        // Test various sizes
        uint256[] memory testSizes = new uint256[](5);
        testSizes[0] = 1; // 1KB
        testSizes[1] = 5; // 5KB
        testSizes[2] = 10; // 10KB
        testSizes[3] = 20; // 20KB
        testSizes[4] = 50; // 50KB

        for (uint256 i = 0; i < testSizes.length; i++) {
            uint256 kb = testSizes[i];
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            // Test processTransaction
            try sequencerChain.processTransaction(data) {
                uint256 gasUsed = gasleft();
                sequencerChain.processTransaction(data);
                gasUsed = gasUsed - gasleft();
                console2.log("Gas used for processTransaction at", kb, "KB:", gasUsed);
            } catch {
                console2.log("Cannot measure gas: processTransaction failed at", kb, "KB");
            }

            // Test processTransactionRaw
            try sequencerChain.processTransactionRaw(data) {
                uint256 gasUsed = gasleft();
                sequencerChain.processTransactionRaw(data);
                gasUsed = gasUsed - gasleft();
                console2.log("Gas used for processTransactionRaw at", kb, "KB:", gasUsed);
            } catch {
                console2.log("Cannot measure gas: processTransactionRaw failed at", kb, "KB");
            }
        }

        vm.stopPrank();
    }

    function testBulkTransactions() public {
        vm.startPrank(admin);

        uint256 numTxs = 5;
        uint256 lastSuccessful = 0;

        for (uint256 kb = 1; kb <= MAX_TEST_SIZE_KB / numTxs; kb += 1) {
            uint256 sizeInBytes = kb * 1024;
            bytes[] memory txs = new bytes[](numTxs);

            // Generate txs of equal size
            for (uint256 i = 0; i < numTxs; i++) {
                txs[i] = generatePayload(sizeInBytes);
            }

            try sequencerChain.processBulkTransactions(txs) {
                console2.log("Successfully processed bulk of", numTxs);
                console2.log("transactions each with size (KB):", kb);
                console2.log("Total size (KB):", kb * numTxs);
                lastSuccessful = kb;
            } catch {
                console2.log("Failed to process bulk of", numTxs);
                console2.log("transactions at size (KB):", kb);
                break;
            }
        }

        console2.log("Maximum size per transaction in bulk (KB):", lastSuccessful);
        console2.log("Maximum total bulk size (KB):", lastSuccessful * numTxs);

        vm.stopPrank();
    }

    // Helper function to find max size for processTransaction
    function findMaxProcessSize() internal returns (uint256) {
        vm.startPrank(admin);

        uint256 lastSuccessful = 0;

        for (uint256 kb = INCREMENT_KB; kb <= MAX_TEST_SIZE_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransaction(data) {
                lastSuccessful = kb;
            } catch {
                break;
            }
        }

        vm.stopPrank();
        return lastSuccessful;
    }

    // Helper function to find max size for processTransactionRaw
    function findMaxProcessRawSize() internal returns (uint256) {
        vm.startPrank(admin);

        uint256 lastSuccessful = 0;

        for (uint256 kb = INCREMENT_KB; kb <= MAX_TEST_SIZE_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransactionRaw(data) {
                lastSuccessful = kb;
            } catch {
                break;
            }
        }

        vm.stopPrank();
        return lastSuccessful;
    }

    // Generate payload efficiently
    function generatePayload(uint256 size) internal pure returns (bytes memory) {
        bytes memory result = new bytes(size);

        // Create a repeating pattern to save gas
        // Fill first 256 bytes with pattern
        uint256 patternSize = size > 256 ? 256 : size;
        for (uint256 i = 0; i < patternSize; i++) {
            result[i] = bytes1(uint8(i));
        }

        // Then copy this pattern for the rest of the buffer
        if (size > 256) {
            for (uint256 i = 256; i < size; i += 256) {
                uint256 copySize = size - i < 256 ? size - i : 256;
                for (uint256 j = 0; j < copySize; j++) {
                    result[i + j] = result[j];
                }
            }
        }

        return result;
    }
}
