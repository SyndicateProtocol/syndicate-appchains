// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {ERC1967Proxy} from "src/factory/SyndicateFactory.sol";

contract SyndicateFactoryTest is Test {
    SyndicateFactory public factory;
    address public admin;
    address public nonAdmin;
    uint256 public appchainId = 10042001;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;

    // Events
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    event ChainIdManuallyMarked(uint256 indexed chainId);

    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    function setUp() public {
        admin = address(0x1);
        nonAdmin = address(0x3);
        // Deploy implementation and proxy
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        factory = SyndicateFactory(address(proxy));
    }

    function testCreateSequencingChainWithRequireAndModule() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address permissionModuleAddress = address(permissionModule);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

        assertTrue(sequencingChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appchainId);
        assertEq(sequencingChainAddress, expectedAddress);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);

        // Verify sequencer setup
        assertEq(address(sequencingChain), sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);

        // Verify permission module setup
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCreateSequencingChainWithRequireOrModule() public {
        RequireOrModule permissionModule = new RequireOrModule(admin);
        address permissionModuleAddress = address(permissionModule);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(permissionModule));

        assertTrue(sequencingChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appchainId);
        assertEq(sequencingChainAddress, expectedAddress);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);

        // Verify sequencer setup
        assertEq(address(sequencingChain), sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);

        // Verify permission module setup
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCreateSequencingChainWithRequireCompositeModule() public {
        RequireCompositeModule permissionModule = new RequireCompositeModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(permissionModule));

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, appchainId);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCorrectAppChainIdAssignment() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        uint256 differentChainId = 10042002;

        vm.prank(admin);
        (address sequencingChain1, uint256 actualChainId1) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );
        vm.prank(admin);
        (address sequencingChain2, uint256 actualChainId2) = factory.createSyndicateSequencingChainWithCustomId(
            differentChainId, admin, IRequirementModule(address(permissionModule2))
        );

        assertEq(SyndicateSequencingChain(sequencingChain1).appchainId(), appchainId);
        assertEq(SyndicateSequencingChain(sequencingChain2).appchainId(), differentChainId);
        assertEq(actualChainId1, appchainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, address(0), IRequirementModule(address(permissionModule))
        );
    }

    function testRevertsOnZeroPermissionModule() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(address(0)));
    }

    function testCreateSequencingChainWithCustomId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 customChainId = 1001;

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            customChainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, customChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), customChainId);
    }

    function testCreateSequencingChainAddressIsDeterministic() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);
        vm.prank(admin);
        (address sequencingChainAddress,) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );

        assertEq(sequencingChainAddress, expectedAddress);
    }

    function testGetProxyBytecode() public view {
        bytes memory bytecode = factory.getProxyBytecode();
        bytes memory expectedBytecode =
            abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(factory.stubImplementation(), ""));
        assertEq(bytecode, expectedBytecode);
    }

    function testGetImplBytecode() public {
        address impl = address(new SyndicateSequencingChain());
        bytes memory bytecode = factory.getImplBytecode(impl);
        bytes memory expectedBytecode = abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(impl, ""));
        assertEq(bytecode, expectedBytecode);
    }

    function testCreateMultipleSequencingChainsWithCustomIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        uint256 chainId1 = 1001;
        uint256 chainId2 = 1002;
        uint256 chainId3 = 1003;

        // First chain
        vm.prank(admin);
        (, uint256 id1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );
        assertEq(id1, chainId1);

        // Second chain
        vm.prank(admin);
        (, uint256 id2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );
        assertEq(id2, chainId2);

        // Third chain
        vm.prank(admin);
        (, uint256 id3) = factory.createSyndicateSequencingChainWithCustomId(
            chainId3, admin, IRequirementModule(address(permissionModule3))
        );
        assertEq(id3, chainId3);
    }

    function testCreateSequencingChainsWithMixedChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First custom chain ID
        uint256 chainId1 = 2001;
        vm.prank(admin);
        (, uint256 id1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );
        assertEq(id1, chainId1);

        // Different custom chain ID
        uint256 chainId2 = 42000;
        vm.prank(admin);
        (, uint256 id2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );
        assertEq(id2, chainId2);

        // Another custom chain ID
        uint256 chainId3 = 3001;
        vm.prank(admin);
        (, uint256 id3) = factory.createSyndicateSequencingChainWithCustomId(
            chainId3, admin, IRequirementModule(address(permissionModule3))
        );
        assertEq(id3, chainId3);
    }

    function testChainIdAlreadyExists() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule1))
        );

        // Try to create another with same chain ID
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule2))
        );
    }

    function testIsChainIdUsed() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        uint256 chainId2 = 2002;

        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appchainId), false);
        assertEq(factory.isChainIdUsed(chainId2), false);

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule1))
        );

        // Now first chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);
        assertEq(factory.isChainIdUsed(chainId2), false);

        // Create second chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );

        // Now both chain IDs should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);
        assertEq(factory.isChainIdUsed(chainId2), true);
    }

    // Pausability Tests
    function testPauseUnpause() public {
        // Initially not paused
        assertFalse(factory.paused());

        // Admin can pause
        vm.prank(admin);
        factory.pause();
        assertTrue(factory.paused());

        // Admin can unpause
        vm.prank(admin);
        factory.unpause();
        assertFalse(factory.paused());
    }

    function testPauseNonAdminReverts() public {
        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();
    }

    function testUnpauseNonAdminReverts() public {
        // Pause first
        vm.prank(admin);
        factory.pause();

        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl revert
        factory.unpause();
    }

    function testCreateSequencingChainWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create sequencing chain
        vm.expectRevert(); // Pausable will revert with "Pausable: paused"
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );
    }

    function testCreateSequencingChainAfterUnpauseWorks() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause then unpause
        vm.prank(admin);
        factory.pause();
        vm.prank(admin);
        factory.unpause();

        // Should work after unpause
        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, appchainId);
    }

    // Access Control tests
    function testRoleSetup() public view {
        // Admin should have the default admin role
        assertTrue(factory.hasRole(DEFAULT_ADMIN_ROLE, admin));

        assertFalse(factory.hasRole(DEFAULT_ADMIN_ROLE, nonAdmin));
    }

    function testPublicVariables() public view {
        // Test that variables are publicly accessible
        assertEq(factory.appchainContracts(appchainId), address(0));
    }

    function testInitializeWithZeroAddressReverts() public {
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (address(0)));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new ERC1967Proxy(address(implementation), initData);
    }

    function testLargeChainIdNumbers() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test with a large custom chain ID
        uint256 largeChainId = 999999999;

        vm.prank(admin);
        (, uint256 id) = factory.createSyndicateSequencingChainWithCustomId(
            largeChainId, admin, IRequirementModule(address(permissionModule))
        );
        assertEq(id, largeChainId);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        uint256 chainId = 1001; // Use a test chain ID
        address impl = factory.computeSequencingChainAddress(chainId);
        assertTrue(impl != address(0));
    }

    // Edge cases and fuzz tests
    function testFuzzManualChainIds(uint256 chainId) public {
        // Skip invalid chain IDs
        vm.assume(chainId != 0);
        vm.assume(chainId < type(uint256).max / 2); // Avoid overflow
        vm.assume(factory.isChainIdUsed(chainId) == false);

        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            chainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, chainId);
        assertEq(factory.isChainIdUsed(chainId), true);
    }

    function testChainIdNoCollisions() public {
        // Test that different custom chain IDs produce different results
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireAndModule permissionModule2 = new RequireAndModule(admin);

        uint256 chainId1 = 1001;
        uint256 chainId2 = 2001;

        vm.prank(admin);
        (, uint256 actualId1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );

        vm.prank(admin);
        (, uint256 actualId2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );

        // These should be completely different
        assertTrue(actualId1 != actualId2);
        assertEq(actualId1, chainId1);
        assertEq(actualId2, chainId2);
    }





    // ================== DETERMINISTIC CHAIN ID TESTS ==================

    function testGenerateDeterministicChainId() public view {
        address sender = address(0x123);
        uint256 nonce = 0;

        uint256 chainId = factory.generateDeterministicChainId(sender, nonce);

        // Verify chainID is deterministic (same inputs -> same output)
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);
        assertEq(chainId, chainId2);

        // Verify chainID is never 0
        assertTrue(chainId > 0);

        // Verify different sender produces different chainID
        uint256 chainId3 = factory.generateDeterministicChainId(address(0x456), nonce);
        assertTrue(chainId != chainId3);

        // Verify different nonce produces different chainID
        uint256 chainId4 = factory.generateDeterministicChainId(sender, 1);
        assertTrue(chainId != chainId4);
    }

    function testCreateSequencingChainDeterministicWithAutoIncrement() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Check initial nonce is 0
        assertEq(factory.senderNonces(admin), 0);

        // Create sequencing chain with auto-increment nonce (nonce = 0)
        vm.prank(admin);

        (address sequencingChain, uint256 chainId) =
            factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // Verify the chain was deployed
        assertTrue(sequencingChain != address(0));
        assertTrue(chainId > 0);

        // Verify nonce incremented
        assertEq(factory.senderNonces(admin), 1);

        // Verify deterministic generation
        uint256 expectedChainId = factory.generateDeterministicChainId(admin, 0);
        assertEq(chainId, expectedChainId);

        // Verify chain is marked as used
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testCreateSequencingChainDeterministicWithSpecificNonce() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Create sequencing chain deterministically
        vm.prank(admin);

        (address sequencingChain, uint256 chainId) =
            factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // Verify the chain was deployed
        assertTrue(sequencingChain != address(0));
        assertTrue(chainId > 0);

        // Verify deterministic generation works
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testCreateSequencingChainDeterministicIncreasesNonce() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Create first chain deterministically (uses nonce 0)
        vm.prank(admin);
        (, uint256 chainId1) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // Create second chain deterministically (uses nonce 1) - should succeed with different chain ID
        vm.prank(admin);
        (, uint256 chainId2) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // Chain IDs should be different
        assertTrue(chainId1 != chainId2);

        // Both should be marked as used
        assertTrue(factory.isChainIdUsed(chainId1));
        assertTrue(factory.isChainIdUsed(chainId2));

        // Nonce should be incremented to 2
        assertEq(factory.senderNonces(admin), 2);
    }

    function testCreateSequencingChainDeterministicDifferentSendersGetDifferentChainIds() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        address sender1 = address(0x111);
        address sender2 = address(0x222);

        // Both senders use nonce 0
        vm.prank(sender1);
        (, uint256 chainId1) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        vm.prank(sender2);
        (, uint256 chainId2) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // Chain IDs should be different
        assertTrue(chainId1 != chainId2);

        // Both nonces should be 1 now
        assertEq(factory.senderNonces(sender1), 1);
        assertEq(factory.senderNonces(sender2), 1);
    }

    function testCreateSequencingChainDeterministicWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create deterministic sequencing chain
        vm.prank(admin);
        vm.expectRevert(); // Pausable will revert
        factory.createSyndicateSequencingChain(chainAdmin, permissionModule);
    }

    function testCreateSequencingChainDeterministicRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(address(0), permissionModule);
    }

    function testCreateSequencingChainDeterministicRevertsOnZeroPermissionModule() public {
        address chainAdmin = address(0x789);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(chainAdmin, IRequirementModule(address(0)));
    }

    function testCreateSyndicateSequencingChainWithCustomIdAdminOnly() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 999999;

        // Admin can create with custom ID
        vm.prank(admin);
        (address sequencingChain, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);

        assertTrue(sequencingChain != address(0));
        assertEq(actualChainId, customChainId);
        assertTrue(factory.isChainIdUsed(customChainId));

        // Non-admin cannot create with custom ID
        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId + 1, chainAdmin, permissionModule);

        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId + 2, chainAdmin, permissionModule);
    }

    function testCreateCustomIdCannotReuseChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 888888;

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);

        // Try to reuse same chain ID - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);
    }

    function testCreateCustomIdCannotUseZeroChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(0, chainAdmin, permissionModule);
    }

    function testCreateCustomIdWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 777777;

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create custom ID chain
        vm.prank(admin);
        vm.expectRevert(); // Pausable will revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);
    }

    function testCreateCustomIdRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 customChainId = 666666;

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, address(0), permissionModule);
    }

    function testCreateCustomIdRevertsOnZeroPermissionModule() public {
        address chainAdmin = address(0x789);
        uint256 customChainId = 555555;

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, IRequirementModule(address(0)));
    }

    function testAntiSquattingAcrossChains() public {
        // This test demonstrates that the same sender will generate deterministic chainIDs
        // preventing squatting across different deployments

        address sender = address(0x555);

        // Deploy a chain - this will use nonce 0 for the sender
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Generate expected chainID for sender with nonce 0 (first deployment)
        uint256 expectedChainId = factory.generateDeterministicChainId(sender, 0);

        vm.prank(sender);
        (, uint256 actualChainId) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        // ChainID should match expected
        assertEq(actualChainId, expectedChainId);

        // Now simulate trying to deploy again with the same sender
        // This should generate a different chain ID (nonce 1) but should not revert
        uint256 expectedChainId2 = factory.generateDeterministicChainId(sender, 1);
        vm.prank(sender);
        (, uint256 actualChainId2) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        assertEq(actualChainId2, expectedChainId2);
        assertTrue(actualChainId != actualChainId2);
    }

    function testFuzzDeterministicChainIdGeneration(address sender, uint256 nonce) public view {
        // Ensure we have a valid sender (not zero address since our contract checks for this)
        vm.assume(sender != address(0));

        uint256 chainId = factory.generateDeterministicChainId(sender, nonce);

        // Chain ID should never be 0
        assertTrue(chainId > 0);

        // Chain ID should be deterministic
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);
        assertEq(chainId, chainId2);

        // Chain ID should be within reasonable bounds (less than 10^18)
        assertTrue(chainId < 10 ** 18);
    }

    function testMultipleSequentialDeterministicDeployments() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        address sender = address(0x666);

        uint256[] memory chainIds = new uint256[](5);

        // Deploy 5 sequential chains using auto-increment nonces
        for (uint256 i = 0; i < 5; i++) {
            vm.prank(sender);
            (, chainIds[i]) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

            // Verify nonce incremented
            assertEq(factory.senderNonces(sender), i + 1);
        }

        // All chain IDs should be different
        for (uint256 i = 0; i < 5; i++) {
            for (uint256 j = i + 1; j < 5; j++) {
                assertTrue(chainIds[i] != chainIds[j]);
            }
        }

        // All should be marked as used
        for (uint256 i = 0; i < 5; i++) {
            assertTrue(factory.isChainIdUsed(chainIds[i]));
        }
    }

    // ================== STUB IMPLEMENTATION TESTS ==================

    function testStubImplementationConsistency() public view {
        // Test that stub implementation address is deterministic
        address computedStub = factory.computeStubImplementationAddress();
        address actualStub = factory.stubImplementation();
        assertEq(computedStub, actualStub);
    }

    // ================== COMPREHENSIVE INTEGRATION TESTS ==================

    function testFullFactoryLifecycle() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test 1: Regular chain creation
        vm.prank(admin);
        (address regularChain, uint256 regularChainId) =
            factory.createSyndicateSequencingChainWithCustomId(5001, admin, permissionModule);
        assertTrue(regularChain != address(0));
        assertEq(regularChainId, 5001);

        // Test 2: Another custom chain creation
        vm.prank(admin);
        (, uint256 customChainId1) = factory.createSyndicateSequencingChainWithCustomId(6001, admin, permissionModule);
        assertEq(customChainId1, 6001);

        // Test 3: Deterministic chain creation
        (, uint256 detChainId) = factory.createSyndicateSequencingChain(admin, permissionModule);
        assertTrue(detChainId > 0);

        // Test 4: Another custom chain creation
        vm.prank(admin);
        (, uint256 customChainId2) = factory.createSyndicateSequencingChainWithCustomId(9999, admin, permissionModule);
        assertEq(customChainId2, 9999);

        // Test 6: Verify all chain IDs are different
        assertTrue(regularChainId != customChainId1);
        assertTrue(regularChainId != detChainId);
        assertTrue(regularChainId != customChainId2);
        assertTrue(customChainId1 != detChainId);
        assertTrue(customChainId1 != customChainId2);
        assertTrue(detChainId != customChainId2);
    }

    function testMixedCreationMethodsWithCollisions() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create regular chain
        uint256 regularId = 1000;
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(regularId, admin, permissionModule);

        // Try to create deterministic chain that might collide
        vm.prank(admin);
        (, uint256 detId) = factory.createSyndicateSequencingChain(admin, permissionModule);

        // Try to create custom chain with same ID as regular - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(regularId, admin, permissionModule);

        // Try to create regular chain with same ID as deterministic - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(detId, admin, permissionModule);

        // All methods should respect the shared chainId space
        assertTrue(factory.isChainIdUsed(regularId));
        assertTrue(factory.isChainIdUsed(detId));
    }

    // ================== ERROR HANDLING TESTS ==================

    function testZeroAddressValidation() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test all creation methods with zero admin
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1001, address(0), permissionModule);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(address(0), permissionModule);

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1002, address(0), permissionModule);

        // Test all creation methods with zero permission module
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1003, admin, IRequirementModule(address(0)));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(admin, IRequirementModule(address(0)));

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1004, admin, IRequirementModule(address(0)));
    }


    // ================== EDGE CASE TESTS ==================

    function testLargeNonceValues() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Should handle normal nonce values (avoiding overflow issues)
        vm.prank(admin);
        (, uint256 chainId) = factory.createSyndicateSequencingChain(chainAdmin, permissionModule);

        assertTrue(chainId > 0);
        assertEq(factory.senderNonces(admin), 1);

        // Verify deterministic generation still works
        uint256 expectedId = factory.generateDeterministicChainId(admin, 0);
        assertEq(chainId, expectedId);
    }

    function testMaxChainIdValue() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 maxChainId = type(uint256).max - 1;

        // Should handle very large chain ID values
        vm.prank(admin);
        (, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(maxChainId, admin, permissionModule);

        assertEq(actualChainId, maxChainId);
        assertTrue(factory.isChainIdUsed(maxChainId));
    }

    // ================== FUZZ TESTS ==================

    function testFuzzCreateRegularChain(uint256 chainId, address chainAdmin) public {
        vm.assume(chainId != 0);
        vm.assume(chainAdmin != address(0));
        vm.assume(!factory.isChainIdUsed(chainId));

        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.prank(admin);
        (, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(chainId, chainAdmin, permissionModule);
        assertEq(actualChainId, chainId);
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testFuzzDeterministicChainGeneration(address sender, uint256 nonce) public view {
        vm.assume(sender != address(0));

        uint256 chainId1 = factory.generateDeterministicChainId(sender, nonce);
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);

        // Should be deterministic
        assertEq(chainId1, chainId2);
        assertTrue(chainId1 > 0);
        assertTrue(chainId1 < 10 ** 18); // Within reasonable bounds
    }

    // ================== VERSION TRACKING TESTS ==================

    function testInitialVersion() public view {
        assertEq(factory.version(), "1.0.0", "Initial version should be 1.0.0");
    }

    function testUpdateVersion() public {
        vm.prank(admin);
        factory.updateVersion("1.1.0");

        assertEq(factory.version(), "1.1.0", "Version should be updated to 1.1.0");
    }

    function testUpdateVersionOnlyAdmin() public {
        vm.prank(nonAdmin);
        vm.expectRevert();
        factory.updateVersion("1.1.0");
    }

    function testUpdateVersionWithDifferentFormats() public {
        string[5] memory versions = ["1.1.0", "2.0.0-beta", "1.2.3", "3.0.0-alpha.1", "10.15.20"];

        for (uint256 i = 0; i < versions.length; i++) {
            vm.prank(admin);
            factory.updateVersion(versions[i]);
            assertEq(factory.version(), versions[i], "Version should match updated value");
        }
    }

    function testVersionPersistsAfterOperations() public {
        // Update version
        vm.prank(admin);
        factory.updateVersion("1.5.0");

        // Perform other operations
        RequireAndModule permissionModule = new RequireAndModule(admin);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(12345, admin, permissionModule);

        // Version should still be the same
        assertEq(factory.version(), "1.5.0", "Version should persist after operations");
    }
}
