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

    // Size increment for testing
    uint256 public constant INCREMENT_KB = 5;

    // Safety limit to prevent infinite loops (1MB)
    uint256 public constant EMERGENCY_STOP_KB = 1024;

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
        bool limitFound = false;

        // Start with small increments and continue until a failure is encountered
        for (uint256 kb = INCREMENT_KB; !limitFound && kb <= EMERGENCY_STOP_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransaction(data) {
                console2.log("Successfully processed transaction with size (KB):", kb);
                lastSuccessful = kb;
            } catch Error(string memory reason) {
                console2.log("Failed with reason:", reason);
                limitFound = true;
            } catch {
                console2.log("Failed to process transaction at size (KB):", kb);
                limitFound = true;
            }
        }

        if (!limitFound) {
            console2.log("WARNING: Hit emergency stop without finding a limit!");
        }

        console2.log("Maximum processTransaction size (KB):", lastSuccessful);
        console2.log("Maximum processTransaction size (bytes):", lastSuccessful * 1024);
        vm.stopPrank();
    }

    function testProcessTransactionRawSizeLimit() public {
        vm.startPrank(admin);

        uint256 lastSuccessful = 0;
        bool limitFound = false;

        // Continue testing sizes until a failure is encountered
        for (uint256 kb = INCREMENT_KB; !limitFound && kb <= EMERGENCY_STOP_KB; kb += INCREMENT_KB) {
            uint256 sizeInBytes = kb * 1024;
            bytes memory data = generatePayload(sizeInBytes);

            try sequencerChain.processTransactionRaw(data) {
                console2.log("Successfully processed raw transaction with size (KB):", kb);
                lastSuccessful = kb;
            } catch Error(string memory reason) {
                console2.log("Failed with reason:", reason);
                limitFound = true;
            } catch {
                console2.log("Failed to process raw transaction at size (KB):", kb);
                limitFound = true;
            }
        }

        if (!limitFound) {
            console2.log("WARNING: Hit emergency stop without finding a limit!");
        }

        console2.log("Maximum processTransactionRaw size (KB):", lastSuccessful);
        console2.log("Maximum processTransactionRaw size (bytes):", lastSuccessful * 1024);
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

            // Calculate gas difference
            try sequencerChain.processTransaction(data) {
                uint256 gas1 = gasleft();
                sequencerChain.processTransaction(data);
                gas1 = gas1 - gasleft();

                uint256 gas2 = gasleft();
                sequencerChain.processTransactionRaw(data);
                gas2 = gas2 - gasleft();

                console2.log("Gas difference at", kb, "KB:", gas1 > gas2 ? gas1 - gas2 : gas2 - gas1);
                console2.log("Percent overhead:", ((gas1 * 100) / gas2) - 100, "%");
            } catch {
                // Skip if any of the calls fail
            }
        }

        vm.stopPrank();
    }

    function testBulkTransactions() public {
        vm.startPrank(admin);

        uint256 numTxs = 5;
        uint256 lastSuccessful = 0;
        bool limitFound = false;

        // Continue testing sizes until we find the limit
        for (uint256 kb = 1; !limitFound && kb <= EMERGENCY_STOP_KB / numTxs; kb += 1) {
            uint256 sizeInBytes = kb * 1024;
            bytes[] memory txs = new bytes[](numTxs);

            // Generate txs of equal size
            for (uint256 i = 0; i < numTxs; i++) {
                txs[i] = generatePayload(sizeInBytes);
            }

            try sequencerChain.processBulkTransactions(txs) {
                console2.log("Successfully processed bulk transactions:", numTxs);
                console2.log("Each transaction size (KB):", kb);
                console2.log("Total size (KB):", kb * numTxs);
                lastSuccessful = kb;
            } catch {
                console2.log("Failed to process bulk transactions:", numTxs);
                console2.log("At transaction size (KB):", kb);
                limitFound = true;
            }
        }

        if (!limitFound) {
            console2.log("WARNING: Hit emergency stop without finding a limit!");
        }

        console2.log("Maximum size per transaction in bulk (KB):", lastSuccessful);
        console2.log("Maximum size per transaction in bulk (bytes):", lastSuccessful * 1024);
        console2.log("Maximum total bulk size (KB):", lastSuccessful * numTxs);
        console2.log("Maximum total bulk size (bytes):", lastSuccessful * numTxs * 1024);

        vm.stopPrank();
    }

    // Test with varying numbers of transactions in bulk
    function testBulkTransactionCounts() public {
        vm.startPrank(admin);

        // Fixed size of 10KB per transaction
        uint256 fixedSizeKB = 10;
        uint256 sizeInBytes = fixedSizeKB * 1024;

        // Test from 1 to 20 transactions
        uint256 maxTxCount = 0;
        bool limitFound = false;

        for (uint256 count = 1; !limitFound && count <= 50; count++) {
            bytes[] memory txs = new bytes[](count);

            // Generate transactions of fixed size
            for (uint256 i = 0; i < count; i++) {
                txs[i] = generatePayload(sizeInBytes);
            }

            try sequencerChain.processBulkTransactions(txs) {
                console2.log("Successfully processed transactions:", count);
                console2.log("Transaction size (KB):", fixedSizeKB);
                console2.log("Total size (KB):", count * fixedSizeKB);
                maxTxCount = count;
            } catch {
                console2.log("Failed to process transactions:", count);
                console2.log("Transaction size (KB):", fixedSizeKB);
                limitFound = true;
            }
        }

        console2.log("Maximum transaction count in bulk:", maxTxCount);
        console2.log("Maximum total bulk size (KB):", maxTxCount * fixedSizeKB);
        console2.log("Maximum total bulk size (bytes):", maxTxCount * fixedSizeKB * 1024);

        vm.stopPrank();
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
