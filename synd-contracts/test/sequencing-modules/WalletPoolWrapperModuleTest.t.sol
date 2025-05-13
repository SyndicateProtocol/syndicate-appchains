// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {WalletPoolWrapperModule, AllowlistSequencingModule} from "src/sequencing-modules/WalletPoolWrapperModule.sol";
import {ISyndicateSequencerChain} from "src/interfaces/ISyndicateSequencerChain.sol";

// Mock SyndicateSequencerChain for testing
contract MockSyndicateSequencerChain is ISyndicateSequencerChain {
    bytes public lastProcessedData;
    bytes[] public lastProcessedBulkData;
    bool public shouldRevert;

    function processTransaction(bytes calldata data) external {
        if (shouldRevert) {
            revert("Sequencer error");
        }
        lastProcessedData = data;
    }

    function processTransactionRaw(bytes calldata data) external {
        if (shouldRevert) {
            revert("Sequencer error");
        }
        lastProcessedData = data;
    }

    function processBulkTransactions(bytes[] calldata data) external {
        if (shouldRevert) {
            revert("Sequencer error");
        }
        delete lastProcessedBulkData;
        for (uint256 i = 0; i < data.length; i++) {
            lastProcessedBulkData.push(data[i]);
        }
    }

    function setRevertFlag(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }
}

contract WalletPoolWrapperModuleTest is Test {
    WalletPoolWrapperModule walletPoolWrapper;
    MockSyndicateSequencerChain mockSequencerChain;

    address admin = address(0x1);
    address allowedWallet = address(0x2);
    address nonAllowedWallet = address(0x3);
    address syndicateSequencerChainAddress;

    bytes testData = abi.encode("test transaction data");
    bytes[] testBulkData;

    function setUp() public {
        // Deploy the mock sequencer chain
        mockSequencerChain = new MockSyndicateSequencerChain();
        syndicateSequencerChainAddress = address(mockSequencerChain);

        // Deploy the WalletPoolWrapperModule with admin
        vm.prank(admin);
        walletPoolWrapper = new WalletPoolWrapperModule(admin);

        // Add the allowed wallet to the allowlist
        vm.prank(admin);
        walletPoolWrapper.addToAllowlist(allowedWallet);

        // Setup bulk data for testing
        testBulkData = new bytes[](3);
        testBulkData[0] = abi.encode("data1");
        testBulkData[1] = abi.encode("data2");
        testBulkData[2] = abi.encode("data3");
    }

    function testProcessTransactionFromAllowedWallet() public {
        vm.prank(allowedWallet);

        // Expect the event to be emitted
        vm.expectEmit(true, true, false, false);
        emit WalletPoolWrapperModule.WalletPoolWrapperTransactionSent(allowedWallet, syndicateSequencerChainAddress);

        // Process the transaction
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);

        // Verify the data was forwarded to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), testData);
    }

    function testProcessTransactionFromNonAllowedWallet() public {
        vm.prank(nonAllowedWallet);

        // Expect the call to revert
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);
    }

    function testProcessTransactionWithZeroAddress() public {
        vm.prank(allowedWallet);

        // Expect the call to revert with different error
        vm.expectRevert(WalletPoolWrapperModule.ZeroSequencerAddressNotAllowed.selector);
        walletPoolWrapper.processTransaction(address(0), testData);
    }

    function testProcessTransactionWithEmptyData() public {
        vm.prank(allowedWallet);

        // Process transaction with empty data
        bytes memory emptyData = "";
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, emptyData);

        // Verify empty data was passed to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), emptyData);
    }

    function testProcessTransactionWithLargeData() public {
        vm.prank(allowedWallet);

        // Create large data payload
        bytes memory largeData = new bytes(1000);
        // Fill with some non-zero data
        for (uint256 i = 0; i < 1000; i++) {
            largeData[i] = bytes1(uint8(i % 256));
        }

        // Process transaction with large data
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, largeData);

        // Verify large data was passed to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), largeData);
    }

    function testProcessTransactionWhenSequencerReverts() public {
        // Make the mock sequencer revert
        mockSequencerChain.setRevertFlag(true);

        vm.prank(allowedWallet);

        // Expect the call to revert
        vm.expectRevert("Sequencer error");
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);
    }

    function testProcessTransactionWithDifferentSequencers() public {
        // Create a second mock sequencer
        MockSyndicateSequencerChain mockSequencerChain2 = new MockSyndicateSequencerChain();
        address sequencerAddress2 = address(mockSequencerChain2);

        vm.prank(allowedWallet);

        // Process transaction with first sequencer
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);
        assertEq(mockSequencerChain.lastProcessedData(), testData);

        // Different data for second sequencer
        bytes memory testData2 = abi.encode("second sequencer data");

        vm.prank(allowedWallet);

        // Process transaction with second sequencer
        walletPoolWrapper.processTransaction(sequencerAddress2, testData2);
        assertEq(mockSequencerChain2.lastProcessedData(), testData2);
    }

    function testProcessTransactionRaw() public {
        vm.prank(allowedWallet);

        // Expect the event to be emitted
        vm.expectEmit(true, true, false, false);
        emit WalletPoolWrapperModule.WalletPoolWrapperTransactionSent(allowedWallet, syndicateSequencerChainAddress);

        // Process the raw transaction
        walletPoolWrapper.processTransactionRaw(syndicateSequencerChainAddress, testData);

        // Verify the data was forwarded to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), testData);
    }

    function testProcessTransactionRawWithZeroAddress() public {
        vm.prank(allowedWallet);

        // Expect the call to revert with ZeroSequencerAddressNotAllowed error
        vm.expectRevert(WalletPoolWrapperModule.ZeroSequencerAddressNotAllowed.selector);
        walletPoolWrapper.processTransactionRaw(address(0), testData);
    }

    function testProcessBulkTransactions() public {
        vm.prank(allowedWallet);

        // Expect the event to be emitted
        vm.expectEmit(true, true, false, false);
        emit WalletPoolWrapperModule.WalletPoolWrapperBulkTransactionsSent(
            allowedWallet, syndicateSequencerChainAddress, testBulkData.length
        );

        // Process bulk transactions
        walletPoolWrapper.processBulkTransactions(syndicateSequencerChainAddress, testBulkData);

        // Verify each piece of data was forwarded to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedBulkData(0), testBulkData[0]);
        assertEq(mockSequencerChain.lastProcessedBulkData(1), testBulkData[1]);
        assertEq(mockSequencerChain.lastProcessedBulkData(2), testBulkData[2]);
    }

    function testProcessBulkTransactionsWithZeroAddress() public {
        vm.prank(allowedWallet);

        // Expect the call to revert with ZeroSequencerAddressNotAllowed error
        vm.expectRevert(WalletPoolWrapperModule.ZeroSequencerAddressNotAllowed.selector);
        walletPoolWrapper.processBulkTransactions(address(0), testBulkData);
    }

    function testProcessBulkTransactionsFromNonAllowedWallet() public {
        vm.prank(nonAllowedWallet);

        // Expect the call to revert
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        walletPoolWrapper.processBulkTransactions(syndicateSequencerChainAddress, testBulkData);
    }

    function testAllowlistIntegration() public {
        // Test adding and removing from allowlist
        vm.startPrank(admin);

        // Add nonAllowedWallet to allowlist
        walletPoolWrapper.addToAllowlist(nonAllowedWallet);
        assertTrue(walletPoolWrapper.allowlist(nonAllowedWallet));

        // Now it should be able to process transactions
        vm.stopPrank();
        vm.prank(nonAllowedWallet);
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);

        // Admin removes it from allowlist
        vm.prank(admin);
        walletPoolWrapper.removeFromAllowlist(nonAllowedWallet);
        assertFalse(walletPoolWrapper.allowlist(nonAllowedWallet));

        // Should revert now
        vm.prank(nonAllowedWallet);
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);
    }

    // Add a test to verify that processTransaction behaves correctly after admin changes
    function testProcessTransactionAfterAdminChange() public {
        address newAdmin = address(0x5);

        // Transfer admin
        vm.prank(admin);
        walletPoolWrapper.transferAdmin(newAdmin);

        // New admin adds a new wallet
        address newWallet = address(0x6);
        vm.prank(newAdmin);
        walletPoolWrapper.addToAllowlist(newWallet);

        // New wallet should be able to process transactions
        vm.prank(newWallet);
        walletPoolWrapper.processTransaction(syndicateSequencerChainAddress, testData);

        // Verify the transaction was forwarded
        assertEq(mockSequencerChain.lastProcessedData(), testData);
    }
}
