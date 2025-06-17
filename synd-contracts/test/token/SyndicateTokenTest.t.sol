// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract SyndicateTokenTest is Test {
    SyndicateToken public token;

    address public defaultAdmin = address(0x1234);
    address public syndFoundationAddress = address(0x5678);
    address public emissionMinter = address(0x9ABC); // Emission scheduler contract
    address public airdropManager = address(0xDEF0); // Airdrop manager
    address public user = address(0x1111);
    address public user2 = address(0x2222);


    function setUp() public {
        vm.startPrank(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, syndFoundationAddress);

        // Grant emission minter role to simulate emission scheduler
        token.grantRole(token.EMISSION_MINTER_ROLE(), emissionMinter);

        // Grant airdrop manager role
        token.grantRole(token.AIRDROP_MANAGER_ROLE(), airdropManager);

        vm.stopPrank();
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(token.name(), "Syndicate");
        assertEq(token.symbol(), "SYND");
        assertEq(token.decimals(), 18);
        assertEq(token.TOTAL_SUPPLY(), 1_000_000_000 * 10 ** 18);
        assertEq(token.INITIAL_MINT_SUPPLY(), 900_000_000 * 10 ** 18);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(token.hasRole(token.EMISSION_MINTER_ROLE(), emissionMinter));
    }

    function test_Constructor_InitialMint() public view {
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY());
        assertEq(token.totalSupply(), token.INITIAL_MINT_SUPPLY());
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(SyndicateToken.ZeroAddress.selector);
        new SyndicateToken(address(0), syndFoundationAddress);
    }

    function test_RevertWhen_Constructor_ZeroFoundation() public {
        vm.expectRevert(SyndicateToken.ZeroAddress.selector);
        new SyndicateToken(defaultAdmin, address(0));
    }


    // ============ EMISSION MINTING TESTS ============

    function test_Mint_Success() public {
        uint256 amount = 1_000_000 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.prank(emissionMinter);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.totalSupply(), initialSupply + amount);
    }

    function test_Mint_MultipleTransactions() public {
        uint256 amount1 = 500_000 * 10 ** 18;
        uint256 amount2 = 300_000 * 10 ** 18;

        vm.startPrank(emissionMinter);
        token.mint(user, amount1);
        token.mint(syndFoundationAddress, amount2);
        vm.stopPrank();

        assertEq(token.balanceOf(user), amount1);
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY() + amount2);
    }

    function test_Mint_ToTotalSupply() public {
        uint256 remainingSupply = token.getRemainingEmissions();

        vm.prank(emissionMinter);
        token.mint(user, remainingSupply);

        assertEq(token.balanceOf(user), remainingSupply);
        assertEq(token.totalSupply(), token.TOTAL_SUPPLY());
        assertEq(token.getRemainingEmissions(), 0);
    }

    function test_RevertWhen_Mint_NotMinter() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.EMISSION_MINTER_ROLE()
            )
        );
        token.mint(user, 1000 * 10 ** 18);
        vm.stopPrank();
    }

    function test_RevertWhen_Mint_ZeroAddress() public {
        vm.prank(emissionMinter);
        vm.expectRevert(SyndicateToken.ZeroAddress.selector);
        token.mint(address(0), 1000 * 10 ** 18);
    }

    function test_RevertWhen_Mint_ZeroAmount() public {
        vm.prank(emissionMinter);
        vm.expectRevert(SyndicateToken.ZeroAmount.selector);
        token.mint(user, 0);
    }

    function test_RevertWhen_Mint_ExceedsTotalSupply() public {
        uint256 exceedingAmount = token.getRemainingEmissions() + 1;

        vm.prank(emissionMinter);
        vm.expectRevert(SyndicateToken.ExceedsTotalSupply.selector);
        token.mint(user, exceedingAmount);
    }


    // ============ VIEW FUNCTION TESTS ============

    function test_GetRemainingEmissions_Initial() public view {
        assertEq(token.getRemainingEmissions(), token.TOTAL_SUPPLY() - token.INITIAL_MINT_SUPPLY());
    }

    function test_GetRemainingEmissions_AfterMinting() public {
        uint256 mintedAmount = 10_000_000 * 10 ** 18;
        uint256 initialRemaining = token.getRemainingEmissions();

        vm.prank(emissionMinter);
        token.mint(user, mintedAmount);

        assertEq(token.getRemainingEmissions(), initialRemaining - mintedAmount);
    }

    // ============ GOVERNANCE FUNCTIONALITY TESTS ============

    function test_GetVotingPower_WithTokens() public {
        vm.prank(syndFoundationAddress);
        token.delegate(syndFoundationAddress);

        assertEq(token.getVotingPower(syndFoundationAddress), token.INITIAL_MINT_SUPPLY());
    }

    function test_GetVotingPower_WithoutTokens() public view {
        assertEq(token.getVotingPower(user), 0);
    }

    function test_Delegate_Success() public {
        vm.prank(syndFoundationAddress);
        token.delegate(user);

        assertEq(token.getVotes(user), token.INITIAL_MINT_SUPPLY());
        assertEq(token.delegates(syndFoundationAddress), user);
    }

    function test_GetPastVotingPower() public {
        vm.prank(syndFoundationAddress);
        token.delegate(user);

        vm.roll(block.number + 1);

        assertEq(token.getPastVotingPower(user, block.number - 1), token.INITIAL_MINT_SUPPLY());
    }

    function test_GetCurrentTotalSupply() public view {
        assertEq(token.getCurrentTotalSupply(), token.INITIAL_MINT_SUPPLY());
    }

    function test_Delegate_WithMintedTokens() public {
        uint256 mintedAmount = 1_000_000 * 10 ** 18;

        // Mint some tokens to user
        vm.prank(emissionMinter);
        token.mint(user, mintedAmount);

        // User delegates to themselves
        vm.prank(user);
        token.delegate(user);

        assertEq(token.getVotingPower(user), mintedAmount);
    }


    // ============ ERC20 FUNCTIONALITY TESTS ============

    function test_Transfer_Success() public {
        uint256 transferAmount = 1000 * 10 ** 18;

        vm.prank(syndFoundationAddress);
        token.transfer(user, transferAmount);

        assertEq(token.balanceOf(user), transferAmount);
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY() - transferAmount);
    }

    function test_Approve_Success() public {
        uint256 amount = 500 * 10 ** 18;

        vm.prank(syndFoundationAddress);
        token.approve(user, amount);

        assertEq(token.allowance(syndFoundationAddress, user), amount);
    }

    function test_TransferFrom_Success() public {
        uint256 amount = 300 * 10 ** 18;

        vm.prank(syndFoundationAddress);
        token.approve(user, amount);

        vm.prank(user);
        token.transferFrom(syndFoundationAddress, user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.allowance(syndFoundationAddress, user), 0);
    }

    // ============ ROLE MANAGEMENT TESTS ============

    function test_GrantEmissionMinterRole_Success() public {
        address newMinter = address(0x7777);

        vm.startPrank(defaultAdmin);
        token.grantRole(token.EMISSION_MINTER_ROLE(), newMinter);
        vm.stopPrank();

        assertTrue(token.hasRole(token.EMISSION_MINTER_ROLE(), newMinter));

        // Test that new minter can mint
        vm.prank(newMinter);
        token.mint(user, 1000 * 10 ** 18);

        assertEq(token.balanceOf(user), 1000 * 10 ** 18);
    }

    function test_RevokeEmissionMinterRole_Success() public {
        vm.startPrank(defaultAdmin);
        token.revokeRole(token.EMISSION_MINTER_ROLE(), emissionMinter);
        vm.stopPrank();

        assertFalse(token.hasRole(token.EMISSION_MINTER_ROLE(), emissionMinter));

        // Test that revoked minter cannot mint
        vm.prank(emissionMinter);
        vm.expectRevert();
        token.mint(user, 1000 * 10 ** 18);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_Mint_ValidAmounts(uint256 amount) public {
        amount = bound(amount, 1, token.getRemainingEmissions());

        vm.prank(emissionMinter);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
    }

    function testFuzz_Transfer_ValidAmounts(uint256 amount) public {
        amount = bound(amount, 1, token.INITIAL_MINT_SUPPLY());

        vm.prank(syndFoundationAddress);
        token.transfer(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY() - amount);
    }

    // ============ INVARIANT TESTS ============

    function test_Invariant_TotalSupplyConsistency() public {
        uint256 mintedAmount = 10_000_000 * 10 ** 18;

        vm.prank(emissionMinter);
        token.mint(user, mintedAmount);

        uint256 expectedTotalSupply = token.INITIAL_MINT_SUPPLY() + mintedAmount;
        assertEq(token.totalSupply(), expectedTotalSupply);
        assertEq(token.totalSupply(), token.balanceOf(syndFoundationAddress) + token.balanceOf(user));
    }

    function test_Invariant_RemainingEmissionsConsistency() public {
        uint256 mint1 = 5_000_000 * 10 ** 18;
        uint256 mint2 = 3_000_000 * 10 ** 18;
        uint256 initialRemaining = token.getRemainingEmissions();

        vm.startPrank(emissionMinter);
        token.mint(user, mint1);
        token.mint(syndFoundationAddress, mint2);
        vm.stopPrank();

        assertEq(token.getRemainingEmissions(), initialRemaining - (mint1 + mint2));
    }

    function test_Invariant_CannotExceedSupplyLimits() public {
        uint256 maxMint = token.getRemainingEmissions();

        vm.prank(emissionMinter);
        token.mint(user, maxMint);

        // Should not be able to mint any more
        vm.prank(emissionMinter);
        vm.expectRevert(SyndicateToken.ExceedsTotalSupply.selector);
        token.mint(user, 1);
    }

    // ============ AIRDROP LOCK FUNCTIONALITY TESTS ============

    function test_Constructor_NoInitialLock() public view {
        assertEq(token.unlockTimestamp(), 0);
        assertFalse(token.transfersLocked());
        assertEq(token.getRemainingLockTime(), 0);
    }

    function test_SetUnlockTimestamp_Success() public {
        uint256 futureTimestamp = block.timestamp + 30 days;

        vm.expectEmit(true, false, false, true);
        emit UnlockTimestampUpdated(0, futureTimestamp, defaultAdmin);

        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        assertEq(token.unlockTimestamp(), futureTimestamp);
        assertTrue(token.transfersLocked());
        assertEq(token.getRemainingLockTime(), 30 days);
    }

    function test_RevertWhen_SetUnlockTimestamp_AlreadySet() public {
        // First set a lock
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Try to set it again
        vm.prank(defaultAdmin);
        vm.expectRevert(SyndicateToken.UnlockTimestampAlreadySet.selector);
        token.setUnlockTimestamp(block.timestamp + 60 days);
    }

    function test_RevertWhen_SetUnlockTimestamp_NotAdmin() public {
        uint256 futureTimestamp = block.timestamp + 30 days;

        vm.prank(user);
        vm.expectRevert(); // Simplified expectRevert
        token.setUnlockTimestamp(futureTimestamp);
    }

    function test_RevertWhen_SetUnlockTimestamp_InPast() public {
        uint256 pastTimestamp = block.timestamp;

        vm.prank(defaultAdmin);
        vm.expectRevert(SyndicateToken.UnlockTimestampInPast.selector);
        token.setUnlockTimestamp(pastTimestamp);
    }

    function test_RevertWhen_SetUnlockTimestamp_Zero() public {
        vm.prank(defaultAdmin);
        vm.expectRevert(SyndicateToken.UnlockTimestampInPast.selector);
        token.setUnlockTimestamp(0);
    }

    function test_RevertWhen_SetUnlockTimestamp_TooLate() public {
        uint256 tooLateTimestamp = block.timestamp + token.MAX_LOCK_DURATION() + 1;

        vm.prank(defaultAdmin);
        vm.expectRevert(SyndicateToken.UnlockTimestampTooLate.selector);
        token.setUnlockTimestamp(tooLateTimestamp);
    }

    function test_TransferLocked_RegularUser() public {
        // Set lock
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Mint some tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        // Regular transfer should fail
        vm.prank(user);
        vm.expectRevert(SyndicateToken.TransfersLocked.selector);
        token.transfer(user2, 100 * 10 ** 18);
    }

    function test_TransferAllowed_AirdropManager() public {
        // Set lock
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Mint some tokens to airdrop manager
        vm.prank(emissionMinter);
        token.mint(airdropManager, 1000 * 10 ** 18);

        // Airdrop manager can transfer despite lock
        vm.prank(airdropManager);
        token.transfer(user, 100 * 10 ** 18);

        assertEq(token.balanceOf(user), 100 * 10 ** 18);
        assertEq(token.balanceOf(airdropManager), 900 * 10 ** 18);
    }

    function test_TransferAfterUnlock() public {
        // Set lock for 1 second
        uint256 shortLockTimestamp = block.timestamp + 1;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(shortLockTimestamp);

        // Mint tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        // Fast forward past unlock time
        vm.warp(block.timestamp + 2);

        // Now transfer should work
        vm.prank(user);
        token.transfer(user2, 100 * 10 ** 18);

        assertEq(token.balanceOf(user), 900 * 10 ** 18);
        assertEq(token.balanceOf(user2), 100 * 10 ** 18);
    }

    function test_BurnFrom_Success() public {
        // Set lock first
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Mint tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        uint256 burnAmount = 100 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(true, false, false, true);
        emit TokensBurnedByManager(user, burnAmount, airdropManager);

        vm.prank(airdropManager);
        token.burnFrom(user, burnAmount);

        assertEq(token.balanceOf(user), 900 * 10 ** 18);
        assertEq(token.totalSupply(), initialSupply - burnAmount);
    }

    function test_RevertWhen_BurnFrom_NotDuringLock() public {
        // Mint tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        // No lock is set, so burnFrom should fail
        vm.prank(airdropManager);
        vm.expectRevert(SyndicateToken.BurnOnlyDuringLockPeriod.selector);
        token.burnFrom(user, 100 * 10 ** 18);
    }

    function test_BurnFrom_OnlyDuringLock() public {
        // Set lock first
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Mint tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        // Now burnFrom should work
        vm.prank(airdropManager);
        token.burnFrom(user, 100 * 10 ** 18);

        assertEq(token.balanceOf(user), 900 * 10 ** 18);
    }

    function test_RevertWhen_BurnFrom_AfterUnlock() public {
        // Set lock for 1 second
        uint256 shortLockTimestamp = block.timestamp + 1;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(shortLockTimestamp);

        // Mint tokens to user
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        // Fast forward past unlock time
        vm.warp(block.timestamp + 2);

        // burnFrom should fail after unlock
        vm.prank(airdropManager);
        vm.expectRevert(SyndicateToken.BurnOnlyDuringLockPeriod.selector);
        token.burnFrom(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_BurnFrom_NotManager() public {
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        vm.prank(user);
        vm.expectRevert(); // Simplified expectRevert
        token.burnFrom(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_BurnFrom_ZeroAddress() public {
        vm.prank(airdropManager);
        vm.expectRevert(SyndicateToken.ZeroAddress.selector);
        token.burnFrom(address(0), 100 * 10 ** 18);
    }

    function test_RevertWhen_BurnFrom_ZeroAmount() public {
        vm.prank(airdropManager);
        vm.expectRevert(SyndicateToken.ZeroAmount.selector);
        token.burnFrom(user, 0);
    }


    function test_MintingAllowedDuringLock() public {
        // Set lock
        uint256 futureTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(futureTimestamp);

        // Minting should still work during lock
        vm.prank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);

        assertEq(token.balanceOf(user), 1000 * 10 ** 18);
    }


    function test_Integration_AirdropWorkflow() public {
        // Step 1: Set lock for airdrop
        uint256 lockTimestamp = block.timestamp + 30 days;
        vm.prank(defaultAdmin);
        token.setUnlockTimestamp(lockTimestamp);

        // Step 2: Mint tokens for airdrop recipients
        vm.startPrank(emissionMinter);
        token.mint(user, 1000 * 10 ** 18);
        token.mint(user2, 1000 * 10 ** 18);
        vm.stopPrank();

        // Step 3: Verify transfers are locked
        vm.prank(user);
        vm.expectRevert(SyndicateToken.TransfersLocked.selector);
        token.transfer(user2, 100 * 10 ** 18);

        // Step 4: Manager can fix mistakes (burn from wrong recipient)
        vm.prank(airdropManager);
        token.burnFrom(user2, 500 * 10 ** 18);

        assertEq(token.balanceOf(user), 1000 * 10 ** 18);
        assertEq(token.balanceOf(user2), 500 * 10 ** 18);

        // Step 5: After unlock, normal transfers work
        vm.warp(lockTimestamp + 1);
        vm.prank(user);
        token.transfer(user2, 100 * 10 ** 18);

        assertEq(token.balanceOf(user), 900 * 10 ** 18);
        assertEq(token.balanceOf(user2), 600 * 10 ** 18);
    }

    // Events that need to be declared for testing
    event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);
    event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
}
