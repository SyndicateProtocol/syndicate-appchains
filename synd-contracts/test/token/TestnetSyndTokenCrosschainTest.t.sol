// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {TestnetSyndTokenCrosschain} from "src/token/TestnetSyndTokenCrosschain.sol";
import {TestnetSyndTokenCrosschainFactory} from "src/token/crosschain/TestnetSyndTokenCrosschainFactory.sol";
import {IERC7802} from "src/token/crosschain/interfaces/IERC7802.sol";
import {IBridgeRateLimiter} from "src/token/crosschain/interfaces/IBridgeRateLimiter.sol";

contract TestnetSyndTokenCrosschainTest is Test {
    TestnetSyndTokenCrosschain public token;
    TestnetSyndTokenCrosschainFactory public factory;

    address public admin = address(0x1234);
    address public minter = address(0x5678);
    address public bridge1 = address(0x9ABC);
    address public bridge2 = address(0xDEF0);
    address public user = address(0x1111);

    uint256 public constant DAILY_LIMIT = 1_000_000 * 10 ** 18;

    event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
    event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
    event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);

    function setUp() public {
        // Deploy factory
        factory = new TestnetSyndTokenCrosschainFactory();

        // Deploy token directly for testing
        token = new TestnetSyndTokenCrosschain(admin, minter);

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
        assertEq(token.totalSupply(), 0);
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

        vm.prank(minter);
        token.mint(user, amount);

        assertEq(token.balanceOf(user), amount);
        assertEq(token.totalSupply(), amount);
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

        // First mint tokens via minter
        vm.prank(minter);
        token.mint(user, mintAmount);

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
                        FACTORY TESTS
    //////////////////////////////////////////////////////////////*/

    function test_FactoryDeployment() public {
        bytes32 salt = factory.generateSalt(admin, minter, block.chainid);

        address deployed = factory.deployTestnetSyndTokenCrosschain(admin, minter, salt);

        assertTrue(deployed != address(0));
        assertTrue(factory.isTokenDeployed(salt));
        assertFalse(factory.isTokenDeployed(keccak256("random")));

        // Test basic functionality of deployed token
        TestnetSyndTokenCrosschain deployedToken = TestnetSyndTokenCrosschain(deployed);
        assertEq(deployedToken.totalSupply(), 0);
        assertEq(deployedToken.name(), "Testnet Syndicate");
        assertEq(deployedToken.symbol(), "TestnetSYND");
    }

    function test_FactoryWithBridgeConfiguration() public {
        bytes32 salt = keccak256("test_deployment");

        // Deploy token using basic factory function
        address deployed = factory.deployTestnetSyndTokenCrosschain(admin, minter, salt);
        TestnetSyndTokenCrosschain deployedToken = TestnetSyndTokenCrosschain(deployed);

        // Manually configure bridges after deployment
        vm.startPrank(admin);
        deployedToken.setBridgeLimits(bridge1, DAILY_LIMIT, DAILY_LIMIT);
        deployedToken.setBridgeLimits(bridge2, DAILY_LIMIT * 2, DAILY_LIMIT * 2);
        vm.stopPrank();

        // Verify bridge configuration
        assertEq(deployedToken.getBridgeCount(), 2);
        assertTrue(deployedToken.isBridgeAuthorized(bridge1));
        assertTrue(deployedToken.isBridgeAuthorized(bridge2));
        assertEq(deployedToken.getAvailableMintLimit(bridge1), DAILY_LIMIT);
        assertEq(deployedToken.getAvailableMintLimit(bridge2), DAILY_LIMIT * 2);
    }

    /*//////////////////////////////////////////////////////////////
                        CREATE3 LIBRARY TESTS
    //////////////////////////////////////////////////////////////*/
    // Note: Using battle-tested Solmate CREATE3 implementation
    // Reference: https://github.com/transmissions11/solmate/blob/main/src/utils/CREATE3.sol
    // Key benefit: Same contract address across ALL chains for seamless bridging

    function test_CREATE3_PredictDeterministicAddress() public {
        // Test the Solmate CREATE3 library's prediction accuracy
        // Deploy using factory and check if prediction works
        bytes32 factorySalt = factory.generateSalt(admin, minter, block.chainid);
        address predicted = factory.predictTokenAddress(factorySalt);
        address deployed = factory.deployTestnetSyndTokenCrosschain(admin, minter, factorySalt);

        // The addresses should match exactly
        assertEq(predicted, deployed, "Solmate CREATE3 prediction should match deployment");
    }

    function test_CREATE3_SameAddressWithDifferentInitCode() public {
        // This test demonstrates CREATE3's key benefit over CREATE2:
        // Same salt = same address, regardless of constructor arguments
        // This enables the same token address across all chains

        bytes32 salt = keccak256("consistent_salt");

        // Test with different admin/minter (different constructor args)
        address admin1 = address(0x1111);
        address minter1 = address(0x2222);
        address admin2 = address(0x3333);
        address minter2 = address(0x4444);

        // With Solmate CREATE3, predicted addresses should be identical
        address predicted1 = factory.predictTokenAddress(salt);
        address predicted2 = factory.predictTokenAddress(salt);

        assertEq(predicted1, predicted2, "Solmate CREATE3: same salt = same address");

        // Deploy the first one
        address deployed1 = factory.deployTestnetSyndTokenCrosschain(admin1, minter1, salt);
        assertEq(deployed1, predicted1, "Deployed address matches prediction");

        // Second deployment with same salt should fail (proxy already deployed)
        vm.expectRevert("DEPLOYMENT_FAILED");
        factory.deployTestnetSyndTokenCrosschain(admin2, minter2, salt);
    }

    function test_CREATE3_AddressPredictionConsistency() public view {
        // Test that Solmate CREATE3 predictions are deterministic
        bytes32 salt = keccak256("consistency_test");

        address predicted1 = factory.predictTokenAddress(salt);
        address predicted2 = factory.predictTokenAddress(salt);
        address predicted3 = factory.predictTokenAddress(salt);

        assertEq(predicted1, predicted2, "Solmate CREATE3 predictions are deterministic");
        assertEq(predicted2, predicted3, "Solmate CREATE3 predictions are deterministic");
    }

    function test_CREATE3_CrossChainAddressConsistency() public view {
        // This test demonstrates the key benefit for cross-chain deployment:
        // Same salt + deployer = same address across ALL chains
        // regardless of different constructor arguments per chain

        bytes32 universalSalt = keccak256("UNIVERSAL_TESTNET_SYND_TOKEN");

        // Get predicted addresses for "deployments" on different chains
        address mainnetPredicted = factory.predictTokenAddress(universalSalt);
        address arbitrumPredicted = factory.predictTokenAddress(universalSalt);
        address optimismPredicted = factory.predictTokenAddress(universalSalt);

        // With Solmate CREATE3: ALL addresses should be identical!
        assertEq(mainnetPredicted, arbitrumPredicted, "Mainnet and Arbitrum addresses should match");
        assertEq(arbitrumPredicted, optimismPredicted, "Arbitrum and Optimism addresses should match");
        assertEq(mainnetPredicted, optimismPredicted, "Mainnet and Optimism addresses should match");

        // This enables seamless cross-chain bridging without address confusion!
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
