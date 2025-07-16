// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {TestnetSyndTokenCrosschain} from "src/token/TestnetSyndTokenCrosschain.sol";
import {IERC7802} from "src/token/crosschain/interfaces/IERC7802.sol";
import {IBridgeRateLimiter} from "src/token/crosschain/interfaces/IBridgeRateLimiter.sol";

contract TestnetSyndTokenCrosschainTest is Test {
    TestnetSyndTokenCrosschain public token;

    address public admin = address(0x1234);
    address public minter = address(0x5678);
    address public bridge1;
    address public bridge2;
    address public user = address(0x1111);

    uint256 public constant DAILY_LIMIT = 1_000_000 * 10 ** 18;

    event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
    event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
    event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);

    function setUp() public {
        // Deploy token directly for testing
        token = new TestnetSyndTokenCrosschain(admin, minter);

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
        assertEq(token.name(), "Testnet Syndicate");
        assertEq(token.symbol(), "TestnetSYND");
        assertEq(token.decimals(), 18);
        assertEq(token.totalSupply(), 920_000_000 * 10 ** 18); // Inherits initial supply from base contract
        assertEq(token.balanceOf(admin), 920_000_000 * 10 ** 18); // Admin receives initial supply
    }

    function test_RoleSetup() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(token.hasRole(token.MINTER_ROLE(), minter));
        assertTrue(token.hasRole(token.BRIDGE_MANAGER_ROLE(), admin));
    }

    function test_InterfaceSupport() public view {
        assertTrue(token.supportsInterface(type(IERC7802).interfaceId));
        assertTrue(token.supportsInterface(type(IBridgeRateLimiter).interfaceId));
    }

    function test_TestnetMinting() public {
        uint256 amount = 1000 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.prank(minter);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.totalSupply(), initialSupply + amount);
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

    function test_PreventEOABridgeAssignment() public {
        // Test that bridge must be a contract, not an EOA
        address eoa = address(0x1337);
        vm.prank(admin);
        vm.expectRevert(abi.encodeWithSelector(IBridgeRateLimiter.BridgeMustBeContract.selector));
        token.setBridgeLimits(eoa, DAILY_LIMIT, DAILY_LIMIT);
    }

    function test_EmissionBudgetPreventsUnauthorizedMinting() public {
        // Test that crosschain minting requires emission budget allocation
        uint256 mintAmount = 100_000 * 10 ** 18;

        // Attempt to mint without emission budget should fail
        vm.prank(bridge1);
        vm.expectRevert(abi.encodeWithSelector(IBridgeRateLimiter.InsufficientEmissionBudget.selector));
        token.crosschainMint(user, mintAmount);

        // Allocate emission budget
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, mintAmount);

        // Now minting should work
        vm.prank(bridge1);
        token.crosschainMint(user, mintAmount);

        assertEq(token.balanceOf(user), mintAmount);
        assertEq(token.getEmissionBudget(bridge1), 0); // Budget should be consumed
    }

    /*//////////////////////////////////////////////////////////////
                        CROSSCHAIN MINT/BURN TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CrosschainMint() public {
        uint256 mintAmount = 100_000 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        // Allocate emission budget first
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, mintAmount);

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

        // First mint tokens via minter
        vm.prank(minter);
        token.mint(user, mintAmount);

        // user allow bridge to use tokens
        vm.prank(user);
        token.approve(bridge1, mintAmount);

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

        // First mint tokens to user via minter
        vm.prank(minter);
        token.mint(user, mintAmount);

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

        // Allocate emission budget first (enough to cover the excessive amount)
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, excessiveAmount);

        vm.prank(bridge1);
        vm.expectRevert();
        token.crosschainMint(user, excessiveAmount);
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
        // Allocate emission budget first
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, DAILY_LIMIT);

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
        // Allocate emission budget first
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, DAILY_LIMIT);

        uint256 halfLimit = DAILY_LIMIT / 2;

        // Use half the limit
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), halfLimit, "Limit should be half after minting");

        // Fast forward 1 day + 1 second
        vm.warp(block.timestamp + 1 days + 1);

        // Limit should be reset
        assertEq(token.getAvailableMintLimit(bridge1), DAILY_LIMIT, "Limit should reset after 1 day");

        // Should be able to mint again
        vm.prank(bridge1);
        token.crosschainMint(user, halfLimit);

        assertEq(token.getAvailableMintLimit(bridge1), halfLimit, "Limit should be half after minting again");
    }

    /*//////////////////////////////////////////////////////////////
                        CREATE2 DEPLOYMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CREATE2_DeterministicDeployment() public {
        // Test CREATE2 deployment with deterministic salt
        bytes32 salt = keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, block.chainid));

        // Deploy a new token with the same salt
        TestnetSyndTokenCrosschain newToken = new TestnetSyndTokenCrosschain{salt: salt}(admin, minter);

        assertTrue(address(newToken) != address(0));
        assertEq(newToken.totalSupply(), 920_000_000 * 10 ** 18); // Inherits initial supply
        assertEq(newToken.balanceOf(admin), 920_000_000 * 10 ** 18); // Admin receives initial supply
        assertTrue(newToken.hasRole(newToken.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(newToken.hasRole(newToken.MINTER_ROLE(), minter));
    }

    function test_CREATE2_CrossChainConsistency() public view {
        // Test that same parameters produce predictable salts for cross-chain deployment
        bytes32 sepoliaSalt = keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, uint256(11155111)));
        bytes32 arbitrumSepoliaSalt =
            keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, uint256(421614)));
        bytes32 optimismSepoliaSalt =
            keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, uint256(11155420)));

        // Salts should be different per chain but deterministic
        assertTrue(sepoliaSalt != arbitrumSepoliaSalt);
        assertTrue(arbitrumSepoliaSalt != optimismSepoliaSalt);
        assertTrue(sepoliaSalt != optimismSepoliaSalt);

        // But should be consistent for same inputs
        bytes32 sepoliaSalt2 = keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, uint256(11155111)));
        assertEq(sepoliaSalt, sepoliaSalt2);
    }

    /*//////////////////////////////////////////////////////////////
                        INTEGRATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Integration_CrosschainFlow() public {
        uint256 amount = 500_000 * 10 ** 18;

        // Allocate emission budget first
        vm.prank(admin);
        token.allocateEmissionBudget(bridge1, amount);

        // 1. Bridge mints tokens on L2
        vm.prank(bridge1);
        token.crosschainMint(user, amount);

        // 2. User transfers some tokens
        vm.prank(user);
        token.transfer(minter, amount / 2);

        // 3. User bridges back to L1 (burns tokens) - user needs to approve bridge first
        vm.prank(user);
        token.approve(bridge1, amount / 2);

        vm.prank(bridge1);
        token.crosschainBurn(user, amount / 2);

        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(minter), amount / 2);
    }

    function test_Integration_TestnetTokenFunctionality() public {
        // Test that all existing TestnetSyndToken functionality still works

        // Test regular minting
        vm.prank(minter);
        token.mint(user, 100_000 * 10 ** 18);

        // Check that tokens were minted
        assertEq(token.balanceOf(user), 100_000 * 10 ** 18);

        // Test governance functionality
        vm.prank(user);
        token.delegate(user);

        assertEq(token.getVotingPower(user), 100_000 * 10 ** 18);

        // Test that role management works
        bytes32 minterRole = token.MINTER_ROLE();
        vm.prank(admin);
        token.grantRole(minterRole, admin);

        assertTrue(token.hasRole(token.MINTER_ROLE(), admin));
    }
}

// Mock contract to test bridge validation
contract MockBridge {
// Empty contract that can be used as a bridge for testing
}
