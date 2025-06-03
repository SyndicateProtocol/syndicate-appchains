// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {TestnetSyndToken, AbstractXERC20} from "src/token/TestnetSyndToken.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract TestnetSyndTokenTest is Test {
    TestnetSyndToken public token;

    address public defaultAdmin;
    address public minter;
    address public user;
    address public spender;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    function setUp() public {
        defaultAdmin = makeAddr("defaultAdmin");
        minter = makeAddr("minter");
        user = makeAddr("user");
        spender = makeAddr("spender");

        token = new TestnetSyndToken(defaultAdmin, minter);
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(token.name(), "Testnet Syndicate");
        assertEq(token.symbol(), "TestnetSYND");
        assertEq(token.decimals(), 18);
        assertEq(token.totalSupply(), 0);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(token.hasRole(token.MINTER_ROLE(), minter));
        assertTrue(token.hasRole(token.BRIDGE_MANAGER_ROLE(), defaultAdmin));
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        new TestnetSyndToken(address(0), minter);
    }

    function test_RevertWhen_Constructor_ZeroMinter() public {
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        new TestnetSyndToken(defaultAdmin, address(0));
    }

    // ============ ADMIN MINT TESTS ============

    function test_AdminMint_Success() public {
        uint256 amount = 1000 * 10 ** 18;

        vm.expectEmit(true, true, false, true);
        emit Transfer(address(0), user, amount);

        vm.prank(minter);
        token.adminMint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.totalSupply(), amount);
    }

    function test_AdminMint_MultipleMints() public {
        uint256 amount1 = 500 * 10 ** 18;
        uint256 amount2 = 300 * 10 ** 18;

        vm.startPrank(minter);
        token.adminMint(user, amount1);
        token.adminMint(spender, amount2);
        vm.stopPrank();

        assertEq(token.balanceOf(user), amount1);
        assertEq(token.balanceOf(spender), amount2);
        assertEq(token.totalSupply(), amount1 + amount2);
    }

    function test_RevertWhen_AdminMint_NotMinter() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.MINTER_ROLE())
        );
        token.adminMint(user, 1000 * 10 ** 18);
        vm.stopPrank();
    }

    function test_RevertWhen_AdminMint_ZeroAmount() public {
        vm.prank(minter);
        vm.expectRevert(AbstractXERC20.ZeroAmount.selector);
        token.adminMint(user, 0);
    }

    // ============ XERC20 FUNCTIONALITY TESTS ============

    function test_SetLimits_AuthorizesBridge() public {
        address bridge = makeAddr("bridge");
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, mintingLimit, burningLimit);

        assertTrue(token.isBridge(bridge));
        assertEq(token.mintingMaxLimitOf(bridge), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge), burningLimit);
        assertEq(token.mintingCurrentLimitOf(bridge), mintingLimit);
        assertEq(token.burningCurrentLimitOf(bridge), burningLimit);
    }

    function test_SetLimits_DeauthorizesBridge() public {
        address bridge = makeAddr("bridge");

        // First authorize
        vm.startPrank(defaultAdmin);
        token.setLimits(bridge, 1000 * 10 ** 18, 500 * 10 ** 18);
        assertTrue(token.isBridge(bridge));

        // Then deauthorize by setting limits to zero
        token.setLimits(bridge, 0, 0);
        assertFalse(token.isBridge(bridge));
        vm.stopPrank();
    }

    function test_XercMint_Success() public {
        address bridge = makeAddr("bridge");
        uint256 amount = 100 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.mintingCurrentLimitOf(bridge), 1000 * 10 ** 18 - amount);
    }

    function test_XercBurn_Success() public {
        address bridge = makeAddr("bridge");
        uint256 mintAmount = 200 * 10 ** 18;
        uint256 burnAmount = 50 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, 1000 * 10 ** 18, 500 * 10 ** 18);

        // First mint tokens
        vm.prank(bridge);
        token.mint(user, mintAmount);

        // Then burn some
        vm.prank(bridge);
        token.burn(user, burnAmount);

        assertEq(token.balanceOf(user), mintAmount - burnAmount);
        assertEq(token.burningCurrentLimitOf(bridge), 500 * 10 ** 18 - burnAmount);
    }

    function test_RevertWhen_XercMint_NotAuthorizedBridge() public {
        vm.prank(user);
        vm.expectRevert(AbstractXERC20.BridgeNotAuthorized.selector);
        token.mint(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_XercMint_ExceedsLimit() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 100 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        vm.prank(bridge);
        vm.expectRevert(AbstractXERC20.InsufficientLimit.selector);
        token.mint(user, limit + 1);
    }

    function test_RevertWhen_XercBurn_NotAuthorizedBridge() public {
        vm.prank(user);
        vm.expectRevert(AbstractXERC20.BridgeNotAuthorized.selector);
        token.burn(user, 100 * 10 ** 18);
    }

    function test_RevertWhen_XercBurn_ExceedsLimit() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 100 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        // Mint tokens first
        vm.prank(bridge);
        token.mint(user, limit);

        vm.prank(bridge);
        vm.expectRevert(AbstractXERC20.InsufficientLimit.selector);
        token.burn(user, limit + 1);
    }

    // ============ ROLE MANAGEMENT TESTS ============

    function test_GrantMinterRole_Success() public {
        // Verify that defaultAdmin has the admin role
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), defaultAdmin));

        // Grant the role using defaultAdmin
        vm.startPrank(defaultAdmin);
        token.grantRole(token.MINTER_ROLE(), user);
        vm.stopPrank();

        // Verify the role was granted
        assertTrue(token.hasRole(token.MINTER_ROLE(), user));
    }

    function test_RevokeMinterRole_Success() public {
        vm.expectEmit(true, true, true, true);
        emit RoleRevoked(token.MINTER_ROLE(), minter, defaultAdmin);

        vm.startPrank(defaultAdmin);
        token.revokeRole(token.MINTER_ROLE(), minter);
        vm.stopPrank();

        assertFalse(token.hasRole(token.MINTER_ROLE(), minter));
    }

    function test_RevertWhen_GrantRole_NotAdmin() public {
        address wrongAddress = makeAddr("wrongAddress");

        bytes32 adminRole = token.DEFAULT_ADMIN_ROLE();
        bytes32 minterRole = token.MINTER_ROLE();

        vm.startPrank(wrongAddress);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, wrongAddress, adminRole)
        );
        token.grantRole(minterRole, spender);
        vm.stopPrank();
    }

    // ============ ERC20 FUNCTIONALITY TESTS ============

    function test_Transfer_Success() public {
        uint256 amount = 1000 * 10 ** 18;
        uint256 transferAmount = 300 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, amount);

        vm.expectEmit(true, true, false, true);
        emit Transfer(user, spender, transferAmount);

        vm.prank(user);
        token.transfer(spender, transferAmount);

        assertEq(token.balanceOf(user), amount - transferAmount);
        assertEq(token.balanceOf(spender), transferAmount);
    }

    function test_Approve_Success() public {
        uint256 amount = 500 * 10 ** 18;

        vm.expectEmit(true, true, false, true);
        emit Approval(user, spender, amount);

        vm.prank(user);
        token.approve(spender, amount);

        assertEq(token.allowance(user, spender), amount);
    }

    function test_TransferFrom_Success() public {
        uint256 mintAmount = 1000 * 10 ** 18;
        uint256 allowanceAmount = 500 * 10 ** 18;
        uint256 transferAmount = 300 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, mintAmount);

        vm.prank(user);
        token.approve(spender, allowanceAmount);

        vm.prank(spender);
        token.transferFrom(user, spender, transferAmount);

        assertEq(token.balanceOf(user), mintAmount - transferAmount);
        assertEq(token.balanceOf(spender), transferAmount);
        assertEq(token.allowance(user, spender), allowanceAmount - transferAmount);
    }

    // ============ ERC20PERMIT TESTS ============

    function test_Permit_Success() public {
        uint256 privateKey = 0xA11CE;
        address owner = vm.addr(privateKey);
        uint256 value = 1000 * 10 ** 18;
        uint256 deadline = block.timestamp + 1 hours;

        uint256 nonce = token.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );

        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);

        token.permit(owner, spender, value, deadline, v, r, s);

        assertEq(token.allowance(owner, spender), value);
        assertEq(token.nonces(owner), nonce + 1);
    }

    function test_RevertWhen_Permit_ExpiredDeadline() public {
        uint256 privateKey = 0xA11CE;
        address owner = vm.addr(privateKey);
        uint256 value = 1000 * 10 ** 18;
        uint256 deadline = block.timestamp - 1; // Expired

        uint256 nonce = token.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );

        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);

        vm.expectRevert(abi.encodeWithSignature("ERC2612ExpiredSignature(uint256)", deadline));
        token.permit(owner, spender, value, deadline, v, r, s);
    }

    // ============ ERC20VOTES TESTS ============

    function test_Delegate_Success() public {
        uint256 amount = 1000 * 10 ** 18;

        vm.prank(minter);
        token.adminMint(user, amount);

        vm.prank(user);
        token.delegate(spender);

        assertEq(token.getVotes(spender), amount);
        assertEq(token.delegates(user), spender);
    }

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

    // ============ BRIDGE INFO TESTS ============

    function test_GetBridgeInfo_AuthorizedBridge() public {
        address bridge = makeAddr("bridge");
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, mintingLimit, burningLimit);

        (bool authorized, uint256 mintingMax, uint256 mintingCurrent, uint256 burningMax, uint256 burningCurrent) =
            token.getBridgeInfo(bridge);

        assertTrue(authorized);
        assertEq(mintingMax, mintingLimit);
        assertEq(mintingCurrent, mintingLimit);
        assertEq(burningMax, burningLimit);
        assertEq(burningCurrent, burningLimit);
    }

    function test_GetBridgeInfo_UnauthorizedBridge() public {
        address bridge = makeAddr("bridge");

        (bool authorized, uint256 mintingMax, uint256 mintingCurrent, uint256 burningMax, uint256 burningCurrent) =
            token.getBridgeInfo(bridge);

        assertFalse(authorized);
        assertEq(mintingMax, 0);
        assertEq(mintingCurrent, 0);
        assertEq(burningMax, 0);
        assertEq(burningCurrent, 0);
    }

    function test_GetBridgeUtilization_NoUsage() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 1000 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        (uint256 mintingUtilization, uint256 burningUtilization) = token.getBridgeUtilization(bridge);

        assertEq(mintingUtilization, 0); // 0% used
        assertEq(burningUtilization, 0); // 0% used
    }

    function test_GetBridgeUtilization_PartialUsage() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 1000 * 10 ** 18;
        uint256 usedAmount = 300 * 10 ** 18; // 30% usage

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        vm.prank(bridge);
        token.mint(user, usedAmount);

        (uint256 mintingUtilization, uint256 burningUtilization) = token.getBridgeUtilization(bridge);

        assertEq(mintingUtilization, 3000); // 30% in basis points
        assertEq(burningUtilization, 0); // 0% used for burning
    }

    // ============ LIMIT REPLENISHMENT TESTS ============

    function test_LimitReplenishment_FullDuration() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 1000 * 10 ** 18;
        uint256 usedAmount = 300 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        // Use some of the limit
        vm.prank(bridge);
        token.mint(user, usedAmount);

        assertEq(token.mintingCurrentLimitOf(bridge), limit - usedAmount);

        // Fast forward 1 day
        vm.warp(block.timestamp + 1 days);

        // Limits should be fully replenished
        assertEq(token.mintingCurrentLimitOf(bridge), limit);
    }

    function test_LimitReplenishment_PartialDuration() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 1000 * 10 ** 18;
        uint256 usedAmount = 600 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        // Use some of the limit
        vm.prank(bridge);
        token.mint(user, usedAmount);

        // Fast forward 12 hours (50% of day)
        vm.warp(block.timestamp + 12 hours);

        uint256 expectedReplenishment = (limit * 12 hours) / 1 days; // 50% of limit
        uint256 expectedCurrentLimit = (limit - usedAmount) + expectedReplenishment;

        assertEq(token.mintingCurrentLimitOf(bridge), expectedCurrentLimit);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_AdminMint_ValidAmounts(address to, uint256 amount) public {
        vm.assume(to != address(0));
        amount = bound(amount, 1, type(uint128).max);

        vm.prank(minter);
        token.adminMint(to, amount);

        assertEq(token.balanceOf(to), amount);
        assertEq(token.totalSupply(), amount);
    }

    function testFuzz_SetLimits_ValidParameters(uint256 mintingLimit, uint256 burningLimit) public {
        mintingLimit = bound(mintingLimit, 0, type(uint128).max);
        burningLimit = bound(burningLimit, 0, type(uint128).max);

        address bridge = makeAddr("bridge");

        vm.prank(defaultAdmin);
        token.setLimits(bridge, mintingLimit, burningLimit);

        assertEq(token.mintingMaxLimitOf(bridge), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge), burningLimit);

        if (mintingLimit > 0 || burningLimit > 0) {
            assertTrue(token.isBridge(bridge));
        } else {
            assertFalse(token.isBridge(bridge));
        }
    }

    function testFuzz_Transfer_ValidAmounts(uint256 mintAmount, uint256 transferAmount) public {
        mintAmount = bound(mintAmount, 1, type(uint128).max);
        transferAmount = bound(transferAmount, 1, mintAmount);

        vm.prank(minter);
        token.adminMint(user, mintAmount);

        vm.prank(user);
        token.transfer(spender, transferAmount);

        assertEq(token.balanceOf(user), mintAmount - transferAmount);
        assertEq(token.balanceOf(spender), transferAmount);
    }

    // ============ INVARIANT TESTS ============

    function test_Invariant_TotalSupplyMatchesBalances() public {
        uint256 amount1 = 1000 * 10 ** 18;
        uint256 amount2 = 2000 * 10 ** 18;
        uint256 amount3 = 3000 * 10 ** 18;

        vm.startPrank(minter);
        token.adminMint(user, amount1);
        token.adminMint(spender, amount2);
        token.adminMint(makeAddr("user3"), amount3);
        vm.stopPrank();

        uint256 totalSupply = token.totalSupply();
        uint256 sumOfBalances = token.balanceOf(user) + token.balanceOf(spender) + token.balanceOf(makeAddr("user3"));

        assertEq(totalSupply, sumOfBalances);
    }

    function test_Invariant_BridgeLimitsNeverNegative() public {
        address bridge = makeAddr("bridge");
        uint256 limit = 1000 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, limit, limit);

        // Use the maximum allowed
        vm.prank(bridge);
        token.mint(user, limit);

        // Limits should be exactly zero, not negative
        assertEq(token.mintingCurrentLimitOf(bridge), 0);
        assertTrue(token.mintingCurrentLimitOf(bridge) >= 0);
    }
}
