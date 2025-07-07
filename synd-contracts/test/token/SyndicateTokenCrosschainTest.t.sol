// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateTokenCrosschain, SyndicateToken} from "src/token/SyndicateTokenCrosschain.sol";
import {IERC7802} from "src/token/crosschain/interfaces/IERC7802.sol";
import {IBridgeRateLimiter} from "src/token/crosschain/interfaces/IBridgeRateLimiter.sol";

contract SyndicateTokenCrosschainTest is Test {
    SyndicateTokenCrosschain public token;

    address public admin = address(0x1234);
    address public treasury = address(0x5678);
    address public bridge1;
    address public bridge2;
    address public user = address(0x1111);

    uint256 public constant INITIAL_MINT_SUPPLY = 900_000_000 * 10 ** 18;
    uint256 public constant DAILY_LIMIT = 1_000_000 * 10 ** 18;

    event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
    event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
    event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);

    function setUp() public {
        // Deploy token directly for testing
        token = new SyndicateTokenCrosschain(admin, treasury);

        // Deploy mock bridge contracts for testing
        bridge1 = address(new MockBridge());
        bridge2 = address(new MockBridge());

        // Setup bridges
        vm.prank(admin);
        token.setBridgeLimits(bridge1, DAILY_LIMIT, DAILY_LIMIT);
    }

    /*//////////////////////////////////////////////////////////////
                            BASIC FUNCTIONALITY
    //////////////////////////////////////////////////////////////*/

    function test_BasicTokenProperties() public view {
        assertEq(token.name(), "Syndicate");
        assertEq(token.symbol(), "SYND");
        assertEq(token.decimals(), 18);
        assertEq(token.totalSupply(), INITIAL_MINT_SUPPLY);
        assertEq(token.balanceOf(treasury), INITIAL_MINT_SUPPLY);
    }

    function test_RoleSetup() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(token.hasRole(token.BRIDGE_MANAGER_ROLE(), admin));
    }

    function test_InterfaceSupport() public view {
        assertTrue(token.supportsInterface(type(IERC7802).interfaceId));
        assertTrue(token.supportsInterface(type(IBridgeRateLimiter).interfaceId));
    }

    /*//////////////////////////////////////////////////////////////
                        BRIDGE MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetBridgeLimits() public {
        vm.expectEmit(true, false, false, true);
        emit BridgeLimitsSet(bridge2, DAILY_LIMIT, DAILY_LIMIT);

        vm.prank(admin);
        token.setBridgeLimits(bridge2, DAILY_LIMIT, DAILY_LIMIT);

        IBridgeRateLimiter.BridgeConfig memory config = token.getBridgeConfig(bridge2);
        assertEq(config.dailyMintLimit, DAILY_LIMIT);
        assertEq(config.dailyBurnLimit, DAILY_LIMIT);
        assertTrue(config.isActive);
    }

    function test_RevertWhen_SetBridgeLimits_NotAuthorized() public {
        vm.prank(user);
        vm.expectRevert();
        token.setBridgeLimits(bridge2, DAILY_LIMIT, DAILY_LIMIT);
    }

    function test_SetBridgeActive() public {
        vm.prank(admin);
        token.setBridgeActive(bridge1, false);

        IBridgeRateLimiter.BridgeConfig memory config = token.getBridgeConfig(bridge1);
        assertFalse(config.isActive);
    }

    function test_GetBridgeInfo() public view {
        assertTrue(token.isBridgeAuthorized(bridge1));
        assertEq(token.getAvailableMintLimit(bridge1), DAILY_LIMIT);
        assertEq(token.getAvailableBurnLimit(bridge1), DAILY_LIMIT);
        assertEq(token.getBridgeCount(), 1);
        assertEq(token.getBridgeAtIndex(0), bridge1);
    }

    function test_PreventBridgeManagerSelfAssignment() public {
        // Test that bridge manager cannot add themselves as a bridge
        vm.prank(admin);
        vm.expectRevert(abi.encodeWithSelector(IBridgeRateLimiter.CannotAddSelfAsBridge.selector));
        token.setBridgeLimits(admin, DAILY_LIMIT, DAILY_LIMIT);
    }

    function test_PreventEOABridgeAssignment() public {
        // Test that bridge must be a contract, not an EOA
        address eoa = address(0x1337);
        vm.prank(admin);
        vm.expectRevert(abi.encodeWithSelector(IBridgeRateLimiter.BridgeMustBeContract.selector));
        token.setBridgeLimits(eoa, DAILY_LIMIT, DAILY_LIMIT);
    }

    function test_AllowValidContractBridge() public {
        // Deploy a mock contract to act as a bridge
        address mockBridge = address(new MockBridge());
        
        vm.prank(admin);
        token.setBridgeLimits(mockBridge, DAILY_LIMIT, DAILY_LIMIT);
        
        assertTrue(token.isBridgeAuthorized(mockBridge));
        assertEq(token.getAvailableMintLimit(mockBridge), DAILY_LIMIT);
    }

    /*//////////////////////////////////////////////////////////////
                        CROSSCHAIN MINT/BURN TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CrosschainMint() public {
        uint256 mintAmount = 100_000 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(true, false, true, true);
        emit CrosschainMint(user, mintAmount, bridge1);

        vm.prank(bridge1);
        token.crosschainMint(user, mintAmount);

        assertEq(token.balanceOf(user), mintAmount);
        assertEq(token.totalSupply(), initialSupply + mintAmount);
        assertEq(token.getAvailableMintLimit(bridge1), DAILY_LIMIT - mintAmount);
    }

    function test_CrosschainBurn() public {
        uint256 mintAmount = 100_000 * 10 ** 18;
        uint256 burnAmount = 50_000 * 10 ** 18;

        // First mint tokens
        vm.prank(bridge1);
        token.crosschainMint(user, mintAmount);

        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(true, false, true, true);
        emit CrosschainBurn(user, burnAmount, bridge1);

        vm.prank(bridge1);
        token.crosschainBurn(user, burnAmount);

        assertEq(token.balanceOf(user), mintAmount - burnAmount);
        assertEq(token.totalSupply(), initialSupply - burnAmount);
        assertEq(token.getAvailableBurnLimit(bridge1), DAILY_LIMIT - burnAmount);
    }

    function test_CrosschainBurnWithApproval() public {
        uint256 mintAmount = 100_000 * 10 ** 18;
        uint256 burnAmount = 50_000 * 10 ** 18;

        // First mint tokens to user
        vm.prank(bridge1);
        token.crosschainMint(user, mintAmount);

        // User approves bridge to burn tokens
        vm.prank(user);
        token.approve(bridge1, burnAmount);

        // Bridge burns tokens on behalf of user
        vm.prank(bridge1);
        token.crosschainBurn(user, burnAmount);

        assertEq(token.balanceOf(user), mintAmount - burnAmount);
        assertEq(token.allowance(user, bridge1), 0);
    }

    function test_RevertWhen_CrosschainMint_UnauthorizedBridge() public {
        vm.prank(bridge2); // Not configured
        vm.expectRevert(abi.encodeWithSelector(IBridgeRateLimiter.UnauthorizedBridge.selector, bridge2));
        token.crosschainMint(user, 1000);
    }

    function test_RevertWhen_CrosschainMint_ExceedsLimit() public {
        uint256 excessiveAmount = DAILY_LIMIT + 1;

        vm.prank(bridge1);
        vm.expectRevert();
        token.crosschainMint(user, excessiveAmount);
    }

    function test_CrosschainMintRespectsTotalSupplyCap() public {
        // Test that crosschain minting respects the total supply cap
        vm.prank(admin);
        token.setBridgeLimits(bridge2, type(uint256).max, type(uint256).max);

        // Get initial state
        uint256 initialSupply = token.totalSupply();
        uint256 totalSupplyCap = token.TOTAL_SUPPLY();
        uint256 remainingSupply = totalSupplyCap - initialSupply;

        // Test 1: Verify minting up to total supply cap works
        vm.prank(bridge2);
        token.crosschainMint(user, remainingSupply);

        assertEq(token.totalSupply(), totalSupplyCap, "Should reach total supply cap");
        assertEq(token.balanceOf(user), remainingSupply, "User should receive remaining supply");

        // Test 2: Verify that attempting to mint beyond total supply fails
        vm.prank(bridge2);
        vm.expectRevert(abi.encodeWithSelector(SyndicateToken.ExceedsTotalSupply.selector));
        token.crosschainMint(user, 1);

        // Test 3: Verify large amounts also fail when exceeding cap
        vm.prank(bridge2);
        vm.expectRevert(abi.encodeWithSelector(SyndicateToken.ExceedsTotalSupply.selector));
        token.crosschainMint(user, 1_000_000_000 * 10 ** 18);

        // Verify total supply remains at cap
        assertEq(token.totalSupply(), totalSupplyCap, "Total supply should remain at cap");
    }

    function test_RevertWhen_CrosschainBurn_InsufficientBalance() public {
        vm.prank(bridge1);
        vm.expectRevert();
        token.crosschainBurn(user, 1000); // User has no tokens
    }

    /*//////////////////////////////////////////////////////////////
                        RATE LIMITING TESTS
    //////////////////////////////////////////////////////////////*/

    function test_RateLimitingMint() public {
        uint256 halfLimit = DAILY_LIMIT / 2;

        // Use half the limit
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), halfLimit);

        // Use remaining limit
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), 0);

        // Try to exceed limit
        vm.prank(bridge1);
        vm.expectRevert();
        token.crosschainMint(user, 1);
    }

    function test_RateLimitingReset() public {
        uint256 halfLimit = DAILY_LIMIT / 2;

        // Use half the limit
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), halfLimit);

        // Fast forward 1 day + 1 second
        vm.warp(block.timestamp + 1 days + 1);

        // Limit should be reset
        assertEq(token.getAvailableMintLimit(bridge1), DAILY_LIMIT);

        // Should be able to mint again
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), halfLimit);
    }

    /*//////////////////////////////////////////////////////////////
                        CREATE2 DEPLOYMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CREATE2_DeterministicDeployment() public {
        // Test CREATE2 deployment with deterministic salt
        bytes32 salt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, block.chainid));

        // Deploy a new token with the same salt
        SyndicateTokenCrosschain newToken = new SyndicateTokenCrosschain{salt: salt}(admin, treasury);

        assertTrue(address(newToken) != address(0));
        assertEq(newToken.balanceOf(treasury), INITIAL_MINT_SUPPLY);
        assertTrue(newToken.hasRole(newToken.DEFAULT_ADMIN_ROLE(), admin));
    }

    function test_CREATE2_CrossChainConsistency() public view {
        // Test that same parameters produce predictable salts for cross-chain deployment
        bytes32 mainnetSalt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, uint256(1)));
        bytes32 arbitrumSalt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, uint256(42161)));
        bytes32 optimismSalt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, uint256(10)));

        // Salts should be different per chain but deterministic
        assertTrue(mainnetSalt != arbitrumSalt);
        assertTrue(arbitrumSalt != optimismSalt);
        assertTrue(mainnetSalt != optimismSalt);

        // But should be consistent for same inputs
        bytes32 mainnetSalt2 = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, uint256(1)));
        assertEq(mainnetSalt, mainnetSalt2);
    }

    /*//////////////////////////////////////////////////////////////
                        INTEGRATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Integration_CrosschainFlow() public {
        uint256 amount = 500_000 * 10 ** 18;

        // 1. Bridge mints tokens on L2
        vm.prank(bridge1);
        token.crosschainMint(user, amount);

        // 2. User transfers some tokens
        vm.prank(user);
        token.transfer(treasury, amount / 2);

        // 3. User bridges back to L1 (burns tokens) - user needs to approve bridge first
        vm.prank(user);
        token.approve(bridge1, amount / 2);

        vm.prank(bridge1);
        token.crosschainBurn(user, amount / 2);

        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(treasury), INITIAL_MINT_SUPPLY + amount / 2);
    }

    function test_Integration_ExistingTokenFunctionality() public {
        // Test that all existing SyndicateToken functionality still works

        // Debug: Check who has admin role
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), admin));

        // Test emission minting - use startPrank for consistent context
        vm.startPrank(admin);
        token.grantRole(token.EMISSION_MINTER_ROLE(), admin);

        address emissionUser = address(0x2222);
        token.mint(emissionUser, 100_000 * 10 ** 18);

        // Check that tokens were minted
        assertEq(token.balanceOf(emissionUser), 100_000 * 10 ** 18);

        // Test airdrop management
        token.grantRole(token.AIRDROP_MANAGER_ROLE(), admin);

        token.setUnlockTimestamp(block.timestamp + 1 days);

        vm.stopPrank();

        assertTrue(token.transfersLocked(), "Transfers should be locked after setting unlock timestamp");
    }
}

// Mock contract to test bridge validation
contract MockBridge {
    // Empty contract that can be used as a bridge for testing
}
