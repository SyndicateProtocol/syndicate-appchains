// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {AddressAliasHelper} from "@arbitrum/nitro-contracts/src/libraries/AddressAliasHelper.sol";

/// @title SyndicateFactoryAddressAliasingTest
/// @notice Tests for address aliasing functionality in SyndicateFactory
/// @dev Address aliasing is used to distinguish L1 contract addresses when they interact with L2
///      This is important for cross-chain admin control where an L1 contract needs to manage L2 contracts
contract SyndicateFactoryAddressAliasingTest is Test {
    SyndicateFactory public factory;
    address public l1Admin;
    address public l2AliasedAdmin;
    address public nonAdmin;
    uint256 public appchainId = 10042001;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;

    // Events
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    function setUp() public {
        vm.warp(1754089200 + 1 days); // after epoch start

        // Simulate an L1 admin contract address
        l1Admin = address(0xC0FFEE);

        // Apply L1 to L2 aliasing to get the L2 representation
        l2AliasedAdmin = AddressAliasHelper.applyL1ToL2Alias(l1Admin);

        nonAdmin = address(0x3);

        // Deploy factory implementation and proxy with ALIASED admin
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (l2AliasedAdmin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        factory = SyndicateFactory(address(proxy));
    }

    /// @notice Test that the aliased admin has correct permissions
    function testAliasedAdminHasAdminRole() public view {
        assertTrue(factory.hasRole(DEFAULT_ADMIN_ROLE, l2AliasedAdmin), "Aliased admin should have DEFAULT_ADMIN_ROLE");
        assertFalse(factory.hasRole(DEFAULT_ADMIN_ROLE, l1Admin), "Original L1 admin should NOT have admin role");
    }

    /// @notice Test that the original L1 address cannot perform admin actions
    function testL1AdminCannotPerformAdminActions() public {
        RequireAndModule permissionModule = new RequireAndModule(l1Admin);

        vm.prank(l1Admin);
        vm.expectRevert(); // AccessControl will revert
        factory.createSyndicateSequencingChainWithCustomId(appchainId, l1Admin, permissionModule);
    }

    /// @notice Test that the aliased L2 address can perform admin actions
    function testL2AliasedAdminCanPerformAdminActions() public {
        RequireAndModule permissionModule = new RequireAndModule(l2AliasedAdmin);

        vm.prank(l2AliasedAdmin);
        (address sequencingChain, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, l2AliasedAdmin, permissionModule);

        assertTrue(sequencingChain != address(0), "Sequencing chain should be deployed");
        assertEq(actualChainId, appchainId, "Chain ID should match");
    }

    /// @notice Test pause/unpause with aliased admin
    function testAliasedAdminCanPauseUnpause() public {
        assertFalse(factory.paused(), "Factory should not be paused initially");

        vm.prank(l2AliasedAdmin);
        factory.pause();
        assertTrue(factory.paused(), "Factory should be paused");

        vm.prank(l2AliasedAdmin);
        factory.unpause();
        assertFalse(factory.paused(), "Factory should be unpaused");
    }

    /// @notice Test that L1 admin cannot pause (only aliased address can)
    function testL1AdminCannotPause() public {
        vm.prank(l1Admin);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();
    }

    /// @notice Test version is constant
    function testVersionIsConstant() public view {
        assertEq(factory.VERSION(), 1_000_000, "Version should be 1.0.0");
    }

    /// @notice Test undoing alias to get back original L1 address
    function testUndoL2ToL1Alias() public view {
        address recoveredL1Address = AddressAliasHelper.undoL1ToL2Alias(l2AliasedAdmin);
        assertEq(recoveredL1Address, l1Admin, "Undoing alias should recover original L1 address");
    }

    /// @notice Test aliasing is deterministic
    function testAliasingIsDeterministic() public view {
        address alias1 = AddressAliasHelper.applyL1ToL2Alias(l1Admin);
        address alias2 = AddressAliasHelper.applyL1ToL2Alias(l1Admin);
        assertEq(alias1, alias2, "Aliasing should be deterministic");
        assertEq(alias1, l2AliasedAdmin, "Should match the aliased admin");
    }

    /// @notice Test that aliasing changes the address
    function testAliasingChangesAddress() public view {
        assertTrue(l2AliasedAdmin != l1Admin, "Aliased address should differ from original");

        // The offset is 0x1111000000000000000000000000000000001111
        uint160 expectedDiff = uint160(0x1111000000000000000000000000000000001111);
        uint160 actualDiff = uint160(l2AliasedAdmin) - uint160(l1Admin);
        assertEq(actualDiff, expectedDiff, "Alias offset should match expected value");
    }

    /// @notice Test setting implementation with aliased admin
    function testAliasedAdminCanSetImplementation() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        vm.prank(l2AliasedAdmin);
        factory.setSyndicateSequencingChainImplementation(address(newImpl));

        assertEq(factory.syndicateChainImpl(), address(newImpl), "Implementation should be updated");
    }

    /// @notice Test that L1 admin cannot set implementation
    function testL1AdminCannotSetImplementation() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        vm.prank(l1Admin);
        vm.expectRevert(); // AccessControl will revert
        factory.setSyndicateSequencingChainImplementation(address(newImpl));
    }

    /// @notice Test multiple different L1 addresses and their aliases
    function testMultipleL1AddressAliases() public {
        address l1Addr1 = address(0x1111);
        address l1Addr2 = address(0x2222);
        address l1Addr3 = address(0x3333);

        address l2Alias1 = AddressAliasHelper.applyL1ToL2Alias(l1Addr1);
        address l2Alias2 = AddressAliasHelper.applyL1ToL2Alias(l1Addr2);
        address l2Alias3 = AddressAliasHelper.applyL1ToL2Alias(l1Addr3);

        // All aliases should be different
        assertTrue(l2Alias1 != l2Alias2, "Alias 1 and 2 should differ");
        assertTrue(l2Alias1 != l2Alias3, "Alias 1 and 3 should differ");
        assertTrue(l2Alias2 != l2Alias3, "Alias 2 and 3 should differ");

        // All should be different from originals
        assertTrue(l2Alias1 != l1Addr1, "Alias 1 should differ from original");
        assertTrue(l2Alias2 != l1Addr2, "Alias 2 should differ from original");
        assertTrue(l2Alias3 != l1Addr3, "Alias 3 should differ from original");

        // Should be able to recover originals
        assertEq(AddressAliasHelper.undoL1ToL2Alias(l2Alias1), l1Addr1);
        assertEq(AddressAliasHelper.undoL1ToL2Alias(l2Alias2), l1Addr2);
        assertEq(AddressAliasHelper.undoL1ToL2Alias(l2Alias3), l1Addr3);
    }

    /// @notice Test that non-admin cannot grant roles to aliased addresses
    function testNonAdminCannotGrantRoles() public {
        bytes32 someRole = keccak256("SOME_ROLE");

        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl will revert
        factory.grantRole(someRole, l2AliasedAdmin);
    }

    /// @notice Test that aliased admin can grant roles
    function testAliasedAdminCanGrantRoles() public {
        bytes32 someRole = keccak256("SOME_ROLE");
        address recipient = address(0x999);

        vm.prank(l2AliasedAdmin);
        factory.grantRole(someRole, recipient);

        assertTrue(factory.hasRole(someRole, recipient), "Role should be granted");
    }

    /// @notice Fuzz test: various L1 addresses should produce valid aliases
    function testFuzzL1ToL2Aliasing(address l1Address) public view {
        // Skip zero address
        vm.assume(l1Address != address(0));

        address l2Alias = AddressAliasHelper.applyL1ToL2Alias(l1Address);

        // Alias should be different from original
        assertTrue(l2Alias != l1Address, "Alias must differ from original");

        // Should be able to recover original
        address recovered = AddressAliasHelper.undoL1ToL2Alias(l2Alias);
        assertEq(recovered, l1Address, "Should recover original address");
    }

    /// @notice Test that applying alias twice doesn't break things
    function testDoubleAliasing() public view {
        address doubleAliased = AddressAliasHelper.applyL1ToL2Alias(l2AliasedAdmin);

        // Double aliasing should produce a different address
        assertTrue(doubleAliased != l2AliasedAdmin, "Double alias should differ");
        assertTrue(doubleAliased != l1Admin, "Double alias should differ from L1 original");

        // Undoing once should give us the first alias
        address undoOnce = AddressAliasHelper.undoL1ToL2Alias(doubleAliased);
        assertEq(undoOnce, l2AliasedAdmin, "Undo once should give first alias");

        // Undoing twice should give us the original
        address undoTwice = AddressAliasHelper.undoL1ToL2Alias(undoOnce);
        assertEq(undoTwice, l1Admin, "Undo twice should give original");
    }

    /// @notice Test factory deployment with different aliasing scenarios
    function testDeployFactoryWithDifferentAdmins() public {
        // Test 1: Regular EOA admin (no aliasing needed)
        address eoaAdmin = address(0xDEADBEEF);
        SyndicateFactory impl1 = new SyndicateFactory();
        bytes memory initData1 = abi.encodeCall(SyndicateFactory.initialize, (eoaAdmin));
        ERC1967Proxy proxy1 = new ERC1967Proxy(address(impl1), initData1);
        SyndicateFactory factory1 = SyndicateFactory(address(proxy1));

        assertTrue(factory1.hasRole(DEFAULT_ADMIN_ROLE, eoaAdmin), "EOA admin should have role");

        // Test 2: Aliased contract admin (for L1 contract control)
        address l1ContractAdmin = address(0xCAFEBABE);
        address l2AliasedContractAdmin = AddressAliasHelper.applyL1ToL2Alias(l1ContractAdmin);

        SyndicateFactory impl2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (l2AliasedContractAdmin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(impl2), initData2);
        SyndicateFactory factory2 = SyndicateFactory(address(proxy2));

        assertTrue(factory2.hasRole(DEFAULT_ADMIN_ROLE, l2AliasedContractAdmin), "Aliased admin should have role");
        assertFalse(factory2.hasRole(DEFAULT_ADMIN_ROLE, l1ContractAdmin), "Original L1 address should not have role");
    }

    /// @notice Test creating sequencing chains with aliased admins for the chains themselves
    function testCreateSequencingChainWithAliasedChainAdmin() public {
        // L1 address that will manage the sequencing chain
        address l1ChainManager = address(0xBEEF);
        address l2AliasedChainManager = AddressAliasHelper.applyL1ToL2Alias(l1ChainManager);

        RequireAndModule permissionModule = new RequireAndModule(l2AliasedChainManager);

        vm.prank(l2AliasedAdmin);
        (address sequencingChain, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, l2AliasedChainManager, permissionModule
        );

        assertTrue(sequencingChain != address(0), "Sequencing chain should be deployed");
        assertEq(actualChainId, appchainId, "Chain ID should match");

        SyndicateSequencingChain chain = SyndicateSequencingChain(sequencingChain);

        // The chain should have the aliased manager as owner, not the L1 address
        assertEq(chain.owner(), l2AliasedChainManager, "Chain should recognize aliased admin as owner");
        assertTrue(chain.owner() != l1ChainManager, "Chain should not recognize L1 address as owner");
    }

    /// @notice Test edge case: zero address aliasing
    function testZeroAddressAliasing() public view {
        address zeroAlias = AddressAliasHelper.applyL1ToL2Alias(address(0));

        // Zero address + offset = offset value
        assertEq(zeroAlias, address(uint160(0x1111000000000000000000000000000000001111)), "Zero alias should equal offset");

        // Undoing should give back zero
        address recovered = AddressAliasHelper.undoL1ToL2Alias(zeroAlias);
        assertEq(recovered, address(0), "Should recover zero address");
    }

    /// @notice Test max address aliasing (edge case)
    function testMaxAddressAliasing() public view {
        // This will overflow and wrap around due to unchecked in AddressAliasHelper
        address maxAddress = address(type(uint160).max);
        address aliased = AddressAliasHelper.applyL1ToL2Alias(maxAddress);

        // Due to overflow, the aliased address will wrap around
        assertTrue(aliased != maxAddress, "Should produce different address");

        // Undoing should still recover original due to symmetric underflow
        address recovered = AddressAliasHelper.undoL1ToL2Alias(aliased);
        assertEq(recovered, maxAddress, "Should recover max address");
    }
}
