// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {AbstractXERC20} from "src/token/AbstractXERC20.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Concrete implementation of AbstractXERC20 for testing
contract TestXERC20 is AbstractXERC20 {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor(string memory name_, string memory symbol_, address admin, address bridgeManager, address minter)
        AbstractXERC20(name_, symbol_)
    {
        if (admin == address(0)) revert ZeroAddress();
        if (bridgeManager == address(0)) revert ZeroAddress();
        if (minter == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_MANAGER_ROLE, bridgeManager);
        _grantRole(MINTER_ROLE, minter);
    }

    function adminMint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        if (amount == 0) revert ZeroAmount();
        _mint(to, amount);
    }
}

contract AbstractXERC20Test is Test {
    TestXERC20 public token;

    address public admin;
    address public bridgeManager;
    address public minter;
    address public user;
    address public bridge1;
    address public bridge2;

    // Events
    event BridgeLimitsSet(address indexed bridge, uint256 mintingLimit, uint256 burningLimit);
    event BridgeAuthorized(address indexed bridge);
    event BridgeDeauthorized(address indexed bridge);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function setUp() public {
        admin = makeAddr("admin");
        bridgeManager = makeAddr("bridgeManager");
        minter = makeAddr("minter");
        user = makeAddr("user");
        bridge1 = makeAddr("bridge1");
        bridge2 = makeAddr("bridge2");

        token = new TestXERC20("Test XERC20", "TXERC", admin, bridgeManager, minter);
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(token.name(), "Test XERC20");
        assertEq(token.symbol(), "TXERC");
        assertEq(token.decimals(), 18);
        assertEq(token.totalSupply(), 0);
        assertEq(token.BRIDGE_LIMIT_DURATION(), 1 days);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(token.hasRole(token.BRIDGE_MANAGER_ROLE(), bridgeManager));
        assertTrue(token.hasRole(token.MINTER_ROLE(), minter));
    }

    // ============ SET LIMITS TESTS ============

    function test_SetLimits_AuthorizeNewBridge() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.expectEmit(true, false, false, false);
        emit BridgeAuthorized(bridge1);

        vm.expectEmit(true, false, false, true);
        emit BridgeLimitsSet(bridge1, mintingLimit, burningLimit);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);

        assertTrue(token.isBridge(bridge1));
        assertEq(token.mintingMaxLimitOf(bridge1), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge1), burningLimit);
        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit);
        assertEq(token.burningCurrentLimitOf(bridge1), burningLimit);
    }

    function test_SetLimits_UpdateExistingBridge() public {
        uint256 initialMinting = 1000 * 10 ** 18;
        uint256 initialBurning = 500 * 10 ** 18;
        uint256 newMinting = 2000 * 10 ** 18;
        uint256 newBurning = 1000 * 10 ** 18;

        // Set initial limits
        vm.prank(bridgeManager);
        token.setLimits(bridge1, initialMinting, initialBurning);

        // Update limits
        vm.prank(bridgeManager);
        token.setLimits(bridge1, newMinting, newBurning);

        assertEq(token.mintingMaxLimitOf(bridge1), newMinting);
        assertEq(token.burningMaxLimitOf(bridge1), newBurning);
        assertEq(token.mintingCurrentLimitOf(bridge1), newMinting);
        assertEq(token.burningCurrentLimitOf(bridge1), newBurning);
    }

    function test_SetLimits_DeauthorizeBridge() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        // First authorize bridge
        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);
        assertTrue(token.isBridge(bridge1));

        // Then deauthorize by setting limits to zero
        vm.expectEmit(true, false, false, false);
        emit BridgeDeauthorized(bridge1);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 0, 0);

        assertFalse(token.isBridge(bridge1));
        assertEq(token.mintingMaxLimitOf(bridge1), 0);
        assertEq(token.burningMaxLimitOf(bridge1), 0);
    }

    function test_SetLimits_OnlyMintingLimit() public {
        uint256 mintingLimit = 1000 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 0);

        assertTrue(token.isBridge(bridge1));
        assertEq(token.mintingMaxLimitOf(bridge1), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge1), 0);
    }

    function test_SetLimits_OnlyBurningLimit() public {
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 0, burningLimit);

        assertTrue(token.isBridge(bridge1));
        assertEq(token.mintingMaxLimitOf(bridge1), 0);
        assertEq(token.burningMaxLimitOf(bridge1), burningLimit);
    }

    function test_RevertWhen_SetLimits_NotBridgeManager() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.BRIDGE_MANAGER_ROLE()
            )
        );
        token.setLimits(bridge1, 1000 * 10 ** 18, 500 * 10 ** 18);
        vm.stopPrank();
    }

    function test_RevertWhen_SetLimits_ZeroAddress() public {
        vm.prank(bridgeManager);
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        token.setLimits(address(0), 1000 * 10 ** 18, 500 * 10 ** 18);
    }

    // ============ BRIDGE MINT TESTS ============

    function test_BridgeMint_Success() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 mintAmount = 300 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 500 * 10 ** 18);

        vm.expectEmit(true, true, false, true);
        emit Transfer(address(0), user, mintAmount);

        vm.prank(bridge1);
        token.mint(user, mintAmount);

        assertEq(token.balanceOf(user), mintAmount);
        assertEq(token.totalSupply(), mintAmount);
        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit - mintAmount);
    }

    function test_BridgeMint_MultipleTransactions() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 mintAmount1 = 300 * 10 ** 18;
        uint256 mintAmount2 = 200 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 500 * 10 ** 18);

        vm.prank(bridge1);
        token.mint(user, mintAmount1);

        vm.prank(bridge1);
        token.mint(user, mintAmount2);

        assertEq(token.balanceOf(user), mintAmount1 + mintAmount2);
        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit - mintAmount1 - mintAmount2);
    }

    function test_RevertWhen_BridgeMint_NotAuthorizedBridge() public {
        vm.prank(user);
        vm.expectRevert(AbstractXERC20.BridgeNotAuthorized.selector);
        token.mint(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_BridgeMint_ZeroUser() public {
        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        token.mint(address(0), 100 * 10 ** 18);
    }

    function test_RevertWhen_BridgeMint_ZeroAmount() public {
        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.ZeroAmount.selector);
        token.mint(user, 0);
    }

    function test_RevertWhen_BridgeMint_ExceedsLimit() public {
        uint256 mintingLimit = 100 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 500 * 10 ** 18);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.InsufficientLimit.selector);
        token.mint(user, mintingLimit + 1);
    }

    // ============ BRIDGE BURN TESTS ============

    function test_BridgeBurn_Success() public {
        uint256 mintAmount = 1000 * 10 ** 18;
        uint256 burnAmount = 300 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        // First mint some tokens to user
        vm.prank(minter);
        token.adminMint(user, mintAmount);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, burningLimit);

        vm.expectEmit(true, true, false, true);
        emit Transfer(user, address(0), burnAmount);

        vm.prank(bridge1);
        token.burn(user, burnAmount);

        assertEq(token.balanceOf(user), mintAmount - burnAmount);
        assertEq(token.totalSupply(), mintAmount - burnAmount);
        assertEq(token.burningCurrentLimitOf(bridge1), burningLimit - burnAmount);
    }

    function test_BridgeBurn_MultipleTransactions() public {
        uint256 mintAmount = 1000 * 10 ** 18;
        uint256 burnAmount1 = 200 * 10 ** 18;
        uint256 burnAmount2 = 100 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, mintAmount);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, burningLimit);

        vm.prank(bridge1);
        token.burn(user, burnAmount1);

        vm.prank(bridge1);
        token.burn(user, burnAmount2);

        assertEq(token.balanceOf(user), mintAmount - burnAmount1 - burnAmount2);
        assertEq(token.burningCurrentLimitOf(bridge1), burningLimit - burnAmount1 - burnAmount2);
    }

    function test_RevertWhen_BridgeBurn_NotAuthorizedBridge() public {
        vm.prank(minter);
        token.adminMint(user, 1000 * 10 ** 18);

        vm.prank(user);
        vm.expectRevert(AbstractXERC20.BridgeNotAuthorized.selector);
        token.burn(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_BridgeBurn_ZeroUser() public {
        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        token.burn(address(0), 100 * 10 ** 18);
    }

    function test_RevertWhen_BridgeBurn_ZeroAmount() public {
        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.ZeroAmount.selector);
        token.burn(user, 0);
    }

    function test_RevertWhen_BridgeBurn_ExceedsLimit() public {
        uint256 burningLimit = 100 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, 1000 * 10 ** 18);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 1000 * 10 ** 18, burningLimit);

        vm.prank(bridge1);
        vm.expectRevert(AbstractXERC20.InsufficientLimit.selector);
        token.burn(user, burningLimit + 1);
    }

    // ============ LIMIT REPLENISHMENT TESTS ============

    function test_LimitReplenishment_FullDuration() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;
        uint256 usedAmount = 300 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);

        // Use some limits
        vm.prank(bridge1);
        token.mint(user, usedAmount);

        vm.prank(minter);
        token.adminMint(user, usedAmount);
        vm.prank(bridge1);
        token.burn(user, usedAmount);

        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit - usedAmount);
        assertEq(token.burningCurrentLimitOf(bridge1), burningLimit - usedAmount);

        // Fast forward full duration
        vm.warp(block.timestamp + 1 days);

        // Limits should be fully replenished
        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit);
        assertEq(token.burningCurrentLimitOf(bridge1), burningLimit);
    }

    function test_LimitReplenishment_PartialDuration() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 usedAmount = 600 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 500 * 10 ** 18);

        vm.prank(bridge1);
        token.mint(user, usedAmount);

        // Fast forward half duration
        vm.warp(block.timestamp + 12 hours);

        uint256 expectedReplenishment = (mintingLimit * 12 hours) / 1 days;
        uint256 expectedCurrentLimit = (mintingLimit - usedAmount) + expectedReplenishment;

        assertEq(token.mintingCurrentLimitOf(bridge1), expectedCurrentLimit);
    }

    function test_LimitReplenishment_CannotExceedMax() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 usedAmount = 100 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, 500 * 10 ** 18);

        vm.prank(bridge1);
        token.mint(user, usedAmount);

        // Fast forward 2 days (more than needed for full replenishment)
        vm.warp(block.timestamp + 2 days);

        // Should not exceed max limit
        assertEq(token.mintingCurrentLimitOf(bridge1), mintingLimit);
    }

    // ============ BRIDGE INFO TESTS ============

    function test_GetBridgeInfo_AuthorizedBridge() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);

        (bool authorized, uint256 mintingMax, uint256 mintingCurrent, uint256 burningMax, uint256 burningCurrent) =
            token.getBridgeInfo(bridge1);

        assertTrue(authorized);
        assertEq(mintingMax, mintingLimit);
        assertEq(mintingCurrent, mintingLimit);
        assertEq(burningMax, burningLimit);
        assertEq(burningCurrent, burningLimit);
    }

    function test_GetBridgeInfo_UnauthorizedBridge() public view {
        (bool authorized, uint256 mintingMax, uint256 mintingCurrent, uint256 burningMax, uint256 burningCurrent) =
            token.getBridgeInfo(bridge1);

        assertFalse(authorized);
        assertEq(mintingMax, 0);
        assertEq(mintingCurrent, 0);
        assertEq(burningMax, 0);
        assertEq(burningCurrent, 0);
    }

    function test_GetBridgeUtilization_NoUsage() public {
        uint256 limit = 1000 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, limit, limit);

        (uint256 mintingUtilization, uint256 burningUtilization) = token.getBridgeUtilization(bridge1);

        assertEq(mintingUtilization, 0);
        assertEq(burningUtilization, 0);
    }

    function test_GetBridgeUtilization_PartialUsage() public {
        uint256 limit = 1000 * 10 ** 18;
        uint256 usedAmount = 300 * 10 ** 18; // 30%

        vm.prank(bridgeManager);
        token.setLimits(bridge1, limit, limit);

        vm.prank(bridge1);
        token.mint(user, usedAmount);

        vm.prank(minter);
        token.adminMint(user, usedAmount);
        vm.prank(bridge1);
        token.burn(user, usedAmount);

        (uint256 mintingUtilization, uint256 burningUtilization) = token.getBridgeUtilization(bridge1);

        assertEq(mintingUtilization, 3000); // 30% in basis points
        assertEq(burningUtilization, 3000); // 30% in basis points
    }

    function test_GetBridgeUtilization_FullUsage() public {
        uint256 limit = 1000 * 10 ** 18;

        vm.prank(bridgeManager);
        token.setLimits(bridge1, limit, limit);

        vm.prank(bridge1);
        token.mint(user, limit);

        vm.prank(minter);
        token.adminMint(user, limit);
        vm.prank(bridge1);
        token.burn(user, limit);

        (uint256 mintingUtilization, uint256 burningUtilization) = token.getBridgeUtilization(bridge1);

        assertEq(mintingUtilization, 10000); // 100% in basis points
        assertEq(burningUtilization, 10000); // 100% in basis points
    }

    // ============ ERC20 VOTES TESTS ============

    function test_GetVotingPower_WithTokens() public {
        uint256 amount = 1000 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, amount);

        vm.prank(user);
        token.delegate(user);

        assertEq(token.getVotingPower(user), amount);
    }

    function test_GetVotingPower_WithoutTokens() public view {
        assertEq(token.getVotingPower(user), 0);
    }

    function test_GetCurrentTotalSupply() public {
        uint256 amount1 = 500 * 10 ** 18;
        uint256 amount2 = 300 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, amount1);

        vm.prank(minter);
        token.adminMint(bridge1, amount2);

        assertEq(token.getCurrentTotalSupply(), amount1 + amount2);
    }

    // ============ MULTIPLE BRIDGES TESTS ============

    function test_MultipleBridges_IndependentLimits() public {
        uint256 limit1 = 1000 * 10 ** 18;
        uint256 limit2 = 500 * 10 ** 18;

        vm.startPrank(bridgeManager);
        token.setLimits(bridge1, limit1, limit1);
        token.setLimits(bridge2, limit2, limit2);
        vm.stopPrank();

        // Use bridge1
        vm.prank(bridge1);
        token.mint(user, 300 * 10 ** 18);

        // Bridge2 should still have full limits
        assertEq(token.mintingCurrentLimitOf(bridge2), limit2);

        // Use bridge2
        vm.prank(bridge2);
        token.mint(user, 200 * 10 ** 18);

        // Verify independent limits
        assertEq(token.mintingCurrentLimitOf(bridge1), limit1 - 300 * 10 ** 18);
        assertEq(token.mintingCurrentLimitOf(bridge2), limit2 - 200 * 10 ** 18);
    }

    function test_MultipleBridges_DifferentReplenishmentRates() public {
        uint256 limit1 = 1000 * 10 ** 18;
        uint256 limit2 = 2000 * 10 ** 18;
        uint256 usedAmount = 600 * 10 ** 18;

        vm.startPrank(bridgeManager);
        token.setLimits(bridge1, limit1, limit1);
        token.setLimits(bridge2, limit2, limit2);
        vm.stopPrank();

        // Use same amount on both bridges
        vm.prank(bridge1);
        token.mint(user, usedAmount);

        vm.prank(bridge2);
        token.mint(user, usedAmount);

        // Fast forward 12 hours
        vm.warp(block.timestamp + 12 hours);

        // Different replenishment amounts due to different max limits
        uint256 replenishment1 = (limit1 * 12 hours) / 1 days;
        uint256 replenishment2 = (limit2 * 12 hours) / 1 days;

        uint256 expected1 = (limit1 - usedAmount) + replenishment1;
        uint256 expected2 = (limit2 - usedAmount) + replenishment2;

        // Cap at maximum limits
        if (expected1 > limit1) expected1 = limit1;
        if (expected2 > limit2) expected2 = limit2;

        assertEq(token.mintingCurrentLimitOf(bridge1), expected1);
        assertEq(token.mintingCurrentLimitOf(bridge2), expected2);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_BridgeLifecycle() public {
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        // 1. Authorize bridge
        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);

        // 2. Use bridge for minting
        vm.prank(bridge1);
        token.mint(user, 300 * 10 ** 18);

        // 3. Use bridge for burning
        vm.prank(bridge1);
        token.burn(user, 100 * 10 ** 18);

        // 4. Update limits
        vm.prank(bridgeManager);
        token.setLimits(bridge1, 2000 * 10 ** 18, 1000 * 10 ** 18);

        // 5. Verify final state
        assertEq(token.balanceOf(user), 200 * 10 ** 18);
        assertTrue(token.isBridge(bridge1));
        assertEq(token.mintingMaxLimitOf(bridge1), 2000 * 10 ** 18);
        assertEq(token.burningMaxLimitOf(bridge1), 1000 * 10 ** 18);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_SetLimits_ValidParameters(uint256 mintingLimit, uint256 burningLimit) public {
        mintingLimit = bound(mintingLimit, 0, type(uint128).max);
        burningLimit = bound(burningLimit, 0, type(uint128).max);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, mintingLimit, burningLimit);

        assertEq(token.mintingMaxLimitOf(bridge1), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge1), burningLimit);

        if (mintingLimit > 0 || burningLimit > 0) {
            assertTrue(token.isBridge(bridge1));
        } else {
            assertFalse(token.isBridge(bridge1));
        }
    }

    function testFuzz_BridgeMint_ValidAmounts(uint256 limit, uint256 amount) public {
        limit = bound(limit, 1, type(uint128).max);
        amount = bound(amount, 1, limit);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, limit, 0);

        vm.prank(bridge1);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.mintingCurrentLimitOf(bridge1), limit - amount);
    }

    function testFuzz_BridgeBurn_ValidAmounts(uint256 limit, uint256 mintAmount, uint256 burnAmount) public {
        limit = bound(limit, 1, type(uint128).max);
        mintAmount = bound(mintAmount, 1, type(uint128).max);
        burnAmount = bound(burnAmount, 1, min(mintAmount, limit));

        // Mint tokens first
        vm.prank(minter);
        token.adminMint(user, mintAmount);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, 0, limit);

        vm.prank(bridge1);
        token.burn(user, burnAmount);

        assertEq(token.balanceOf(user), mintAmount - burnAmount);
        assertEq(token.burningCurrentLimitOf(bridge1), limit - burnAmount);
    }

    function testFuzz_LimitReplenishment_PartialTime(uint256 timeElapsed, uint256 limit, uint256 usedAmount) public {
        timeElapsed = bound(timeElapsed, 1, 1 days - 1);
        limit = bound(limit, 1000, type(uint128).max);
        usedAmount = bound(usedAmount, 1, limit);

        vm.prank(bridgeManager);
        token.setLimits(bridge1, limit, 0);

        vm.prank(bridge1);
        token.mint(user, usedAmount);

        vm.warp(block.timestamp + timeElapsed);

        uint256 expectedReplenishment = (limit * timeElapsed) / 1 days;
        uint256 expectedCurrentLimit = min((limit - usedAmount) + expectedReplenishment, limit);

        assertEq(token.mintingCurrentLimitOf(bridge1), expectedCurrentLimit);
    }

    // ============ HELPER FUNCTIONS ============

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}
