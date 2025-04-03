// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {WalletPoolWrapperModule, AllowlistSequencingModule} from "src/sequencing-modules/WalletPoolWrapperModule.sol";
import {IMetabasedSequencerChain} from "src/interfaces/IMetabasedSequencerChain.sol";

// Mock MetabasedSequencerChain for testing
contract MockMetabasedSequencerChain is IMetabasedSequencerChain {
    bytes public lastProcessedData;
    bool public shouldRevert;

    function processTransaction(bytes calldata data) external {
        if (shouldRevert) {
            revert("Sequencer error");
        }
        lastProcessedData = data;
    }

    function setRevertFlag(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }
}

contract WalletPoolWrapperModuleTest is Test {
    WalletPoolWrapperModule walletPoolWrapper;
    MockMetabasedSequencerChain mockSequencerChain;

    address admin = address(0x1);
    address allowedWallet = address(0x2);
    address nonAllowedWallet = address(0x3);

    bytes testData = abi.encode("test transaction data");

    function setUp() public {
        // Deploy the mock sequencer chain
        mockSequencerChain = new MockMetabasedSequencerChain();

        // Deploy the WalletPoolWrapperModule with admin and mock sequencer
        vm.prank(admin);
        walletPoolWrapper = new WalletPoolWrapperModule(admin, address(mockSequencerChain));

        // Add the allowed wallet to the allowlist
        vm.prank(admin);
        walletPoolWrapper.addToAllowlist(allowedWallet);
    }

    function testConstructor() public {
        // Verify the metabasedSequencerChain was set correctly
        assertEq(address(walletPoolWrapper.metabasedSequencerChain()), address(mockSequencerChain));
    }

    function testConstructorRejectsZeroAddress() public {
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        new WalletPoolWrapperModule(admin, address(0));
    }

    function testProcessTransactionFromAllowedWallet() public {
        vm.prank(allowedWallet);

        // Expect the event to be emitted
        vm.expectEmit(true, false, false, false);
        emit WalletPoolWrapperModule.WalletPoolWrapperTransactionSent(allowedWallet);

        // Process the transaction
        walletPoolWrapper.processTransaction(testData);

        // Verify the data was forwarded to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), testData);
    }

    function testProcessTransactionFromNonAllowedWallet() public {
        vm.prank(nonAllowedWallet);

        // Expect the call to revert
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        walletPoolWrapper.processTransaction(testData);
    }

    function testProcessTransactionWithEmptyData() public {
        vm.prank(allowedWallet);

        // Process transaction with empty data
        bytes memory emptyData = "";
        walletPoolWrapper.processTransaction(emptyData);

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
        walletPoolWrapper.processTransaction(largeData);

        // Verify large data was passed to the sequencer chain
        assertEq(mockSequencerChain.lastProcessedData(), largeData);
    }

    function testProcessTransactionWhenSequencerReverts() public {
        // Make the mock sequencer revert
        mockSequencerChain.setRevertFlag(true);

        vm.prank(allowedWallet);

        // Expect the call to revert
        vm.expectRevert("Sequencer error");
        walletPoolWrapper.processTransaction(testData);
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
        walletPoolWrapper.processTransaction(testData);

        // Admin removes it from allowlist
        vm.prank(admin);
        walletPoolWrapper.removeFromAllowlist(nonAllowedWallet);
        assertFalse(walletPoolWrapper.allowlist(nonAllowedWallet));

        // Should revert now
        vm.prank(nonAllowedWallet);
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        walletPoolWrapper.processTransaction(testData);
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
        walletPoolWrapper.processTransaction(testData);

        // Verify the transaction was forwarded
        assertEq(mockSequencerChain.lastProcessedData(), testData);
    }
}
