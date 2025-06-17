// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Airdrop} from "src/airdrop/Airdrop.sol";
import {TestnetSyndToken} from "src/token/TestnetSyndToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract AirdropTest is Test {
    Airdrop public airdrop;
    TestnetSyndToken public token;

    address public admin = address(0x1234);
    address public minter = address(0x5678);
    address public airdropper = address(0x9ABC);
    
    address public recipient1 = address(0x1111);
    address public recipient2 = address(0x2222);
    address public recipient3 = address(0x3333);
    address public recipient4 = address(0x4444);

    uint256 public constant INITIAL_BALANCE = 1_000_000 * 10 ** 18;

    function setUp() public {
        // Deploy contracts
        airdrop = new Airdrop();
        token = new TestnetSyndToken(admin, minter);

        // Mint tokens to airdropper
        vm.prank(minter);
        token.mint(airdropper, INITIAL_BALANCE);

        // Approve airdrop contract to spend tokens
        vm.prank(airdropper);
        token.approve(address(airdrop), INITIAL_BALANCE);
    }

    // ============ SUCCESS TESTS ============

    function test_AirdropERC20_SingleRecipient() public {
        address[] memory addresses = new address[](1);
        uint256[] memory amounts = new uint256[](1);
        
        addresses[0] = recipient1;
        amounts[0] = 1000 * 10 ** 18;
        
        uint256 totalAmount = 1000 * 10 ** 18;

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);
        uint256 recipient1BalanceBefore = token.balanceOf(recipient1);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        assertEq(token.balanceOf(recipient1), recipient1BalanceBefore + amounts[0]);
    }

    function test_AirdropERC20_MultipleRecipients() public {
        address[] memory addresses = new address[](4);
        uint256[] memory amounts = new uint256[](4);
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        addresses[2] = recipient3;
        addresses[3] = recipient4;
        
        amounts[0] = 1000 * 10 ** 18;
        amounts[1] = 2000 * 10 ** 18;
        amounts[2] = 3000 * 10 ** 18;
        amounts[3] = 4000 * 10 ** 18;
        
        uint256 totalAmount = 10000 * 10 ** 18;

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);
        uint256[] memory recipientBalancesBefore = new uint256[](4);
        recipientBalancesBefore[0] = token.balanceOf(recipient1);
        recipientBalancesBefore[1] = token.balanceOf(recipient2);
        recipientBalancesBefore[2] = token.balanceOf(recipient3);
        recipientBalancesBefore[3] = token.balanceOf(recipient4);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        assertEq(token.balanceOf(recipient1), recipientBalancesBefore[0] + amounts[0]);
        assertEq(token.balanceOf(recipient2), recipientBalancesBefore[1] + amounts[1]);
        assertEq(token.balanceOf(recipient3), recipientBalancesBefore[2] + amounts[2]);
        assertEq(token.balanceOf(recipient4), recipientBalancesBefore[3] + amounts[3]);
    }

    function test_AirdropERC20_EqualAmounts() public {
        address[] memory addresses = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        addresses[2] = recipient3;
        
        uint256 amountPerRecipient = 5000 * 10 ** 18;
        amounts[0] = amountPerRecipient;
        amounts[1] = amountPerRecipient;
        amounts[2] = amountPerRecipient;
        
        uint256 totalAmount = amountPerRecipient * 3;

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        assertEq(token.balanceOf(recipient1), amountPerRecipient);
        assertEq(token.balanceOf(recipient2), amountPerRecipient);
        assertEq(token.balanceOf(recipient3), amountPerRecipient);
    }

    function test_AirdropERC20_ZeroAmounts() public {
        address[] memory addresses = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        
        amounts[0] = 0;
        amounts[1] = 1000 * 10 ** 18;
        
        uint256 totalAmount = 1000 * 10 ** 18;

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);
        uint256 recipient1BalanceBefore = token.balanceOf(recipient1);
        uint256 recipient2BalanceBefore = token.balanceOf(recipient2);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        assertEq(token.balanceOf(recipient1), recipient1BalanceBefore); // No change for zero amount
        assertEq(token.balanceOf(recipient2), recipient2BalanceBefore + amounts[1]);
    }

    function test_AirdropERC20_DuplicateRecipients() public {
        address[] memory addresses = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        
        addresses[0] = recipient1;
        addresses[1] = recipient1; // Duplicate
        addresses[2] = recipient2;
        
        amounts[0] = 1000 * 10 ** 18;
        amounts[1] = 2000 * 10 ** 18;
        amounts[2] = 3000 * 10 ** 18;
        
        uint256 totalAmount = 6000 * 10 ** 18;

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        // recipient1 should receive both amounts (1000 + 2000 = 3000)
        assertEq(token.balanceOf(recipient1), amounts[0] + amounts[1]);
        assertEq(token.balanceOf(recipient2), amounts[2]);
    }

    // ============ FAILURE TESTS ============

    function test_RevertWhen_AirdropERC20_ArrayLengthMismatch() public {
        address[] memory addresses = new address[](2);
        uint256[] memory amounts = new uint256[](3); // Different length
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        
        amounts[0] = 1000 * 10 ** 18;
        amounts[1] = 2000 * 10 ** 18;
        amounts[2] = 3000 * 10 ** 18;
        
        uint256 totalAmount = 6000 * 10 ** 18;

        vm.prank(airdropper);
        vm.expectRevert();
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
    }

    function test_RevertWhen_AirdropERC20_InsufficientAllowance() public {
        address[] memory addresses = new address[](1);
        uint256[] memory amounts = new uint256[](1);
        
        addresses[0] = recipient1;
        amounts[0] = 1000 * 10 ** 18;
        
        uint256 totalAmount = 1000 * 10 ** 18;

        // Reset allowance to 0
        vm.prank(airdropper);
        token.approve(address(airdrop), 0);

        vm.prank(airdropper);
        vm.expectRevert();
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
    }

    function test_RevertWhen_AirdropERC20_InsufficientBalance() public {
        address[] memory addresses = new address[](1);
        uint256[] memory amounts = new uint256[](1);
        
        addresses[0] = recipient1;
        amounts[0] = 1000 * 10 ** 18;
        
        uint256 totalAmount = INITIAL_BALANCE + 1; // More than balance

        vm.prank(airdropper);
        vm.expectRevert();
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
    }

    function test_AirdropERC20_TotalAmountMismatch_StillWorks() public {
        address[] memory addresses = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        
        amounts[0] = 1000 * 10 ** 18;
        amounts[1] = 2000 * 10 ** 18;
        
        uint256 totalAmount = 4000 * 10 ** 18; // More than sum of amounts

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        // This should work - the contract transfers totalAmount but distributes based on amounts array
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        // Verify tokens were transferred and distributed correctly
        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        assertEq(token.balanceOf(recipient1), amounts[0]);
        assertEq(token.balanceOf(recipient2), amounts[1]);
        // Remaining tokens (1000 * 10**18) stay in the airdrop contract
        assertEq(token.balanceOf(address(airdrop)), totalAmount - amounts[0] - amounts[1]);
    }

    // ============ EDGE CASES ============

    function test_RevertWhen_AirdropERC20_EmptyArrays() public {
        address[] memory addresses = new address[](0);
        uint256[] memory amounts = new uint256[](0);
        uint256 totalAmount = 0;

        vm.prank(airdropper);
        vm.expectRevert(); // Empty arrays may cause revert in assembly
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
    }

    function test_AirdropERC20_MaxRecipients() public {
        uint256 numRecipients = 100;
        address[] memory addresses = new address[](numRecipients);
        uint256[] memory amounts = new uint256[](numRecipients);
        
        uint256 amountPerRecipient = 100 * 10 ** 18;
        uint256 totalAmount = amountPerRecipient * numRecipients;
        
        // Generate recipients
        for (uint256 i = 0; i < numRecipients; i++) {
            addresses[i] = address(uint160(i + 1000));
            amounts[i] = amountPerRecipient;
        }

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        
        // Check a few random recipients
        assertEq(token.balanceOf(addresses[0]), amountPerRecipient);
        assertEq(token.balanceOf(addresses[50]), amountPerRecipient);
        assertEq(token.balanceOf(addresses[99]), amountPerRecipient);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_AirdropERC20_ValidInputs(uint8 numRecipients, uint128 baseAmount) public {
        vm.assume(numRecipients > 0 && numRecipients <= 20); // Reasonable bounds
        vm.assume(baseAmount > 0 && baseAmount <= 10000 * 10 ** 18); // Reasonable bounds

        address[] memory addresses = new address[](numRecipients);
        uint256[] memory amounts = new uint256[](numRecipients);
        uint256 totalAmount = 0;
        
        // Generate recipients and amounts
        for (uint256 i = 0; i < numRecipients; i++) {
            addresses[i] = address(uint160(i + 5000));
            amounts[i] = uint256(baseAmount) + i * 1000; // Varying amounts
            totalAmount += amounts[i];
        }

        // Ensure airdropper has enough balance
        vm.assume(totalAmount <= INITIAL_BALANCE);

        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        assertEq(token.balanceOf(airdropper), airdropperBalanceBefore - totalAmount);
        
        // Check first and last recipients
        assertEq(token.balanceOf(addresses[0]), amounts[0]);
        if (numRecipients > 1) {
            assertEq(token.balanceOf(addresses[numRecipients - 1]), amounts[numRecipients - 1]);
        }
    }

    // ============ GAS OPTIMIZATION TESTS ============

    function test_AirdropERC20_GasEfficiency_SmallBatch() public {
        address[] memory addresses = new address[](5);
        uint256[] memory amounts = new uint256[](5);
        
        for (uint256 i = 0; i < 5; i++) {
            addresses[i] = address(uint160(i + 6000));
            amounts[i] = 1000 * 10 ** 18;
        }
        
        uint256 totalAmount = 5000 * 10 ** 18;

        vm.prank(airdropper);
        uint256 gasBefore = gasleft();
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
        uint256 gasUsed = gasBefore - gasleft();

        // Gas usage should be reasonable for 5 transfers (more realistic expectation)
        assertTrue(gasUsed < 300000, "Gas usage too high for small batch");
    }

    function test_AirdropERC20_GasEfficiency_LargeBatch() public {
        uint256 batchSize = 50;
        address[] memory addresses = new address[](batchSize);
        uint256[] memory amounts = new uint256[](batchSize);
        
        uint256 amountPerRecipient = 1000 * 10 ** 18;
        for (uint256 i = 0; i < batchSize; i++) {
            addresses[i] = address(uint160(i + 7000));
            amounts[i] = amountPerRecipient;
        }
        
        uint256 totalAmount = amountPerRecipient * batchSize;

        vm.prank(airdropper);
        uint256 gasBefore = gasleft();
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);
        uint256 gasUsed = gasBefore - gasleft();

        // Gas usage should be reasonable for 50 transfers (more realistic expectation)
        assertTrue(gasUsed < 2500000, "Gas usage too high for large batch");
        
        // Average gas per transfer should be efficient
        uint256 avgGasPerTransfer = gasUsed / batchSize;
        assertTrue(avgGasPerTransfer < 50000, "Average gas per transfer too high");
    }

    // ============ INTEGRATION TESTS ============

    function test_AirdropERC20_Integration_MultipleAirdrops() public {
        // First airdrop
        address[] memory addresses1 = new address[](2);
        uint256[] memory amounts1 = new uint256[](2);
        addresses1[0] = recipient1;
        addresses1[1] = recipient2;
        amounts1[0] = 1000 * 10 ** 18;
        amounts1[1] = 2000 * 10 ** 18;
        uint256 totalAmount1 = 3000 * 10 ** 18;

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses1, amounts1, totalAmount1);

        // Second airdrop
        address[] memory addresses2 = new address[](2);
        uint256[] memory amounts2 = new uint256[](2);
        addresses2[0] = recipient3;
        addresses2[1] = recipient4;
        amounts2[0] = 3000 * 10 ** 18;
        amounts2[1] = 4000 * 10 ** 18;
        uint256 totalAmount2 = 7000 * 10 ** 18;

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses2, amounts2, totalAmount2);

        // Verify all balances
        assertEq(token.balanceOf(recipient1), amounts1[0]);
        assertEq(token.balanceOf(recipient2), amounts1[1]);
        assertEq(token.balanceOf(recipient3), amounts2[0]);
        assertEq(token.balanceOf(recipient4), amounts2[1]);
        assertEq(token.balanceOf(airdropper), INITIAL_BALANCE - totalAmount1 - totalAmount2);
    }

    // ============ INVARIANT TESTS ============

    function test_Invariant_TokenBalanceConservation() public {
        address[] memory addresses = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        
        addresses[0] = recipient1;
        addresses[1] = recipient2;
        addresses[2] = recipient3;
        
        amounts[0] = 1000 * 10 ** 18;
        amounts[1] = 2000 * 10 ** 18;
        amounts[2] = 3000 * 10 ** 18;
        
        uint256 totalAmount = 6000 * 10 ** 18;

        uint256 totalSupplyBefore = token.totalSupply();
        uint256 airdropperBalanceBefore = token.balanceOf(airdropper);

        vm.prank(airdropper);
        airdrop.airdropERC20(address(token), addresses, amounts, totalAmount);

        uint256 totalSupplyAfter = token.totalSupply();
        uint256 airdropperBalanceAfter = token.balanceOf(airdropper);
        uint256 recipientBalancesSum = token.balanceOf(recipient1) + token.balanceOf(recipient2) + token.balanceOf(recipient3);

        // Total supply should remain constant
        assertEq(totalSupplyAfter, totalSupplyBefore);
        
        // Tokens should be conserved
        assertEq(airdropperBalanceBefore - airdropperBalanceAfter, recipientBalancesSum);
        assertEq(airdropperBalanceAfter + recipientBalancesSum, airdropperBalanceBefore);
    }
}