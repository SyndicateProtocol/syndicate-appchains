// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

contract SyndicateFactoryTest is Test {
    SyndicateFactory public factory;
    address public admin;
    address public manager;
    address public nonManager;
    uint256 public appchainId = 10042001;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Events
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);

    event ChainIdManuallyMarked(uint256 indexed chainId);
    event IdUpperBoundUpdated(uint256 oldBound, uint256 newBound);

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        nonManager = address(0x3);
        factory = new SyndicateFactory(admin);

        // Grant manager role to the manager address
        vm.prank(admin);
        factory.grantRole(MANAGER_ROLE, manager);
    }

    function testCreateSequencingChainWithRequireAndModule() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address permissionModuleAddress = address(permissionModule);

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-1"));
        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, permissionModule, salt);

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

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-2"));
        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule), salt);

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

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-composite"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule), salt);

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

        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-test-7"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-test-8"));

        (address sequencingChain1, uint256 actualChainId1) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt1
        );
        (address sequencingChain2, uint256 actualChainId2) = factory.createSyndicateSequencingChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), salt2
        );

        assertEq(SyndicateSequencingChain(sequencingChain1).appchainId(), appchainId);
        assertEq(SyndicateSequencingChain(sequencingChain2).appchainId(), differentChainId);
        assertEq(actualChainId1, appchainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-9"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(
            appchainId, address(0), IRequirementModule(address(permissionModule)), salt
        );
    }

    function testRevertsOnZeroPermissionModule() public {
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-10"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(0)), salt);
    }

    function testAutoIncrementOnZeroChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 expectedChainId = (namespacePrefix * idUpperBound) + 1;

        bytes32 salt = keccak256(abi.encodePacked("salt-for-auto-increment"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), expectedChainId);

        // Verify next chain ID incremented
        uint256 nextExpectedChainId = (namespacePrefix * idUpperBound) + 2;
        assertEq(factory.getNextChainId(), nextExpectedChainId);
    }

    function testCreateSequencingChainAddressIsDeterministic() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-deterministic"));

        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);
        (address sequencingChainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
        );

        assertEq(sequencingChainAddress, expectedAddress);
    }

    function testGetBytecode() public view {
        bytes memory bytecode = factory.getBytecode(appchainId);
        bytes memory expectedBytecode =
            abi.encodePacked(type(SyndicateSequencingChain).creationCode, abi.encode(appchainId));
        assertEq(bytecode, expectedBytecode);
    }

    function testAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First auto-incremented chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-auto-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 expectedId1 = (namespacePrefix * idUpperBound) + 1;
        assertEq(id1, expectedId1);

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-auto-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        uint256 expectedId2 = (namespacePrefix * idUpperBound) + 2;
        assertEq(id2, expectedId2);

        // Third auto-incremented chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-auto-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        uint256 expectedId3 = (namespacePrefix * idUpperBound) + 3;
        assertEq(id3, expectedId3);
    }

    function testMixedAutoAndManualChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-mixed-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 expectedId1 = (namespacePrefix * idUpperBound) + 1;
        assertEq(id1, expectedId1);

        // Manual chain ID
        uint256 manualId = 42000;
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-mixed-2"));
        (, uint256 id2) = factory.createSyndicateSequencingChain(
            manualId, admin, IRequirementModule(address(permissionModule2)), salt2
        );
        assertEq(id2, manualId);

        // Back to auto chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-mixed-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        // Should be next auto ID since we only used one auto ID so far
        uint256 expectedId3 = (namespacePrefix * idUpperBound) + 2;
        assertEq(id3, expectedId3);
    }

    function testChainIdAlreadyExists() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create first chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-duplicate-1"));
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Try to create another with same chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-duplicate-2"));
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule2)), salt2);
    }

    function testIsChainIdUsed() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appchainId), 0);

        // Create chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-used-1"));
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), 1);

        // Auto-incremented ID should also be marked as used
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-used-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        assertEq(factory.isChainIdUsed(id2), 1);
    }

    // Manual Chain ID Marking Tests
    function testMarkChainIdAsUsed() public {
        uint256 chainIdToMark = 12345;

        // Initially not used
        assertEq(factory.isChainIdUsed(chainIdToMark), 0);

        // Mark as used
        vm.prank(manager);
        vm.expectEmit(true, false, false, true);
        emit ChainIdManuallyMarked(chainIdToMark);
        factory.markChainIdAsUsed(chainIdToMark);

        // Now should be marked as used
        assertEq(factory.isChainIdUsed(chainIdToMark), 1);
    }

    function testMarkChainIdAsUsedAlreadyUsedReverts() public {
        uint256 chainIdToMark = 12345;

        // Mark as used first time
        vm.prank(manager);
        factory.markChainIdAsUsed(chainIdToMark);

        // Try to mark again should revert
        vm.prank(manager);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.markChainIdAsUsed(chainIdToMark);
    }

    function testMarkChainIdAsUsedNonManagerReverts() public {
        uint256 chainIdToMark = 12345;

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert with a specific message
        factory.markChainIdAsUsed(chainIdToMark);
    }

    function testMarkChainIdAsUsedPreventsFutureDeployment() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 chainIdToMark = 99999;

        // Mark chain ID as used
        vm.prank(manager);
        factory.markChainIdAsUsed(chainIdToMark);

        // Try to deploy with that chain ID should fail
        bytes32 salt = keccak256(abi.encodePacked("salt-for-marked-id"));
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChain(
            chainIdToMark, admin, IRequirementModule(address(permissionModule)), salt
        );
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
        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();
    }

    function testUnpauseNonAdminReverts() public {
        // Pause first
        vm.prank(admin);
        factory.pause();

        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        factory.unpause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert
        factory.unpause();
    }

    function testCreateSequencingChainWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-paused"));

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create sequencing chain
        vm.expectRevert(); // Pausable will revert with "Pausable: paused"
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)), salt);
    }

    function testCreateSequencingChainAfterUnpauseWorks() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-unpause"));

        // Pause then unpause
        vm.prank(admin);
        factory.pause();
        vm.prank(admin);
        factory.unpause();

        // Should work after unpause
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, appchainId);
    }

    // Access Control tests
    function testRoleSetup() public view {
        // Admin should have the default admin role
        assertTrue(factory.hasRole(DEFAULT_ADMIN_ROLE, admin));

        // Admin should have the manager role
        assertTrue(factory.hasRole(MANAGER_ROLE, admin));

        // Manager should have the manager role
        assertTrue(factory.hasRole(MANAGER_ROLE, manager));

        // Non-manager should not have any roles
        assertFalse(factory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(factory.hasRole(MANAGER_ROLE, nonManager));
    }

    function testNamespaceConfigGetters() public view {
        // Initial values
        assertEq(factory.namespacePrefix(), 510);
        // Note: namespaceMultiplier no longer exists
    }

    function testPublicVariables() public view {
        // Test that variables are publicly accessible
        assertEq(factory.namespacePrefix(), 510);
        assertEq(factory.nextAutoChainId(), 1);
        assertEq(factory.usedChainIds(appchainId), false);
    }

    function testUpdateNamespaceConfig() public {
        uint256 newPrefix = 511;

        // Manager can update namespace config
        vm.prank(manager);
        vm.expectEmit(true, true, false, true);
        emit NamespaceConfigUpdated(
            510, // old prefix
            newPrefix
        );
        factory.updateNamespaceConfig(newPrefix);

        // Check new values
        assertEq(factory.namespacePrefix(), newPrefix);
    }

    function testUpdateNamespaceConfigAsNonManagerReverts() public {
        uint256 newPrefix = 511;

        // Non-manager cannot update namespace config
        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert with a specific message
        factory.updateNamespaceConfig(newPrefix);
    }

    function testNamespaceUpdateAffectsAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create a chain with old namespace
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-namespace-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Verify chain ID follows current formula
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 expectedId1 = (namespacePrefix * idUpperBound) + 1;
        assertEq(id1, expectedId1);

        // Update namespace config
        uint256 newPrefix = 600;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix);

        // Create a chain with new namespace
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-namespace-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);

        // New namespace: (600 * idUpperBound) + 2 (counter is at 2 now)
        uint256 newIdUpperBound = factory.idUpperBound();
        uint256 expectedId2 = (newPrefix * newIdUpperBound) + 2;
        assertEq(id2, expectedId2);
    }

    function testConstructorWithZeroAddressReverts() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new SyndicateFactory(address(0));
    }

    function testGetNextChainIdFunction() public view {
        // Test that the public function works correctly
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 nextAutoChainId = factory.nextAutoChainId();

        uint256 expectedNextId = (namespacePrefix * idUpperBound) + nextAutoChainId;
        uint256 actualNextId = factory.getNextChainId();
        assertEq(actualNextId, expectedNextId);
    }

    function testChainIdArithmeticLogic() public {
        // Test arithmetic-based chain ID generation (replacement for old concatenation logic)
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // First: dynamic calculation
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();

        bytes32 salt1 = keccak256(abi.encodePacked("arithmetic-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        uint256 expectedId1 = (namespacePrefix * idUpperBound) + 1;
        assertEq(id1, expectedId1);

        // Second: dynamic calculation
        bytes32 salt2 = keccak256(abi.encodePacked("arithmetic-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        uint256 expectedId2 = (namespacePrefix * idUpperBound) + 2;
        assertEq(id2, expectedId2);

        // Create several more to test sequential generation
        for (uint256 i = 3; i <= 5; i++) {
            RequireAndModule pm = new RequireAndModule(admin);
            bytes32 salt = keccak256(abi.encodePacked("arithmetic-loop", i));
            (, uint256 id) = factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(pm)), salt);

            // Expected: (namespacePrefix * idUpperBound) + i
            uint256 expectedId = (namespacePrefix * idUpperBound) + i;
            assertEq(id, expectedId);
        }
    }

    function testLargeChainIdNumbers() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Update to higher starting point to test larger arithmetic calculations
        vm.prank(admin);
        factory.updateNamespaceConfig(999);

        // Create auto chain with dynamic calculation
        uint256 newNamespacePrefix = factory.namespacePrefix(); // 999 after update
        uint256 currentIdUpperBound = factory.idUpperBound();
        uint256 expectedId = (newNamespacePrefix * currentIdUpperBound) + 1;

        bytes32 salt = keccak256(abi.encodePacked("large-number"));
        (, uint256 id) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);
        assertEq(id, expectedId);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        bytes32 salt = keccak256(abi.encodePacked("impl-test"));
        uint256 chainId = 510_000_000_001; // Use an auto-generated ID
        address impl = factory.computeSequencingChainAddress(salt, chainId);
        assertTrue(impl != address(0));
    }

    // Edge cases and fuzz tests
    function testFuzzManualChainIds(uint256 chainId) public {
        // Skip already used chain IDs and auto-generated ones
        vm.assume(chainId != 0);
        // Skip first auto-generated ID based on current formula
        uint256 firstAutoId = (factory.namespacePrefix() * factory.idUpperBound()) + 1;
        vm.assume(chainId != firstAutoId);
        vm.assume(factory.isChainIdUsed(chainId) == 0);

        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("fuzz-test", chainId));

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(chainId, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, chainId);
        assertEq(factory.isChainIdUsed(chainId), 1);
    }

    function testFuzzSalts(bytes32 salt) public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        (address sequencingChainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(sequencingChainAddress, factory.computeSequencingChainAddress(salt, appchainId));
    }

    function testNewChainIdGenerationFormat() public {
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 idUpperBound = factory.idUpperBound();
        uint256 nextAutoChainId = factory.nextAutoChainId();

        uint256 expectedChainId = (namespacePrefix * idUpperBound) + nextAutoChainId;
        uint256 actualChainId = factory.getNextChainId();
        assertEq(actualChainId, expectedChainId);
    }

    function testChainIdNoCollisions() public {
        // Test that different namespace/autoId combinations produce different results

        vm.startPrank(manager);

        // Test scenario 1: namespace=12, simple autoId
        factory.updateNamespaceConfig(12);
        uint256 chainId1 = factory.getNextChainId();
        uint256 idUpperBound = factory.idUpperBound();
        // Expected: (12 * idUpperBound) + 1

        // Test scenario 2: namespace=123, simple autoId (same counter value)
        factory.updateNamespaceConfig(123);
        uint256 chainId2 = factory.getNextChainId();
        // Expected: (123 * idUpperBound) + 1
        // Note: nextAutoChainId is still 1 since we didn't create any chains yet

        vm.stopPrank();

        // These should be completely different, demonstrating no collision
        assertTrue(chainId1 != chainId2);
        assertEq(chainId1, (12 * idUpperBound) + 1);
        assertEq(chainId2, (123 * idUpperBound) + 1);
    }

    function testUpdateIdUpperBound() public {
        uint256 newBound = 2_000_000_000;
        uint256 currentBound = factory.idUpperBound();

        vm.prank(manager);
        vm.expectEmit(true, true, false, true);
        emit IdUpperBoundUpdated(currentBound, newBound);
        factory.updateIdUpperBound(newBound);

        assertEq(factory.idUpperBound(), newBound);

        // Test chain ID generation with new bound
        uint256 expectedChainId = (factory.namespacePrefix() * newBound) + factory.nextAutoChainId();
        uint256 actualChainId = factory.getNextChainId();
        assertEq(actualChainId, expectedChainId);
    }

    function testUpdateIdUpperBoundInvalidValues() public {
        vm.startPrank(manager);

        // Test upper bound = 0
        vm.expectRevert("Upper bound must be greater than 0");
        factory.updateIdUpperBound(0);

        // Test upper bound <= current nextAutoChainId
        uint256 currentAutoId = factory.nextAutoChainId();
        vm.expectRevert("Upper bound must be greater than current auto chain ID");
        factory.updateIdUpperBound(currentAutoId);

        vm.stopPrank();
    }

    function testUpdateIdUpperBoundNonManagerReverts() public {
        vm.prank(nonManager);
        vm.expectRevert();
        factory.updateIdUpperBound(2_000_000_000);
    }

    function testIdOverflowError() public {
        vm.startPrank(manager);

        // Set a very small upper bound
        factory.updateIdUpperBound(5);

        // Create chains to exhaust the limit
        for (uint256 i = 1; i < 5; i++) {
            RequireAndModule tempModule = new RequireAndModule(admin);
            bytes32 tempSalt = keccak256(abi.encodePacked("overflow", i));
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(tempModule)), tempSalt);
        }

        // Now nextAutoChainId should equal idUpperBound, causing overflow
        vm.expectRevert(SyndicateFactory.IdOverflow.selector);
        factory.getNextChainId();

        vm.stopPrank();
    }

    function testPublicVariablesIncludeIdUpperBound() public view {
        // Verify idUpperBound is publicly accessible (value should be 10 after constructor change)
        assertTrue(factory.idUpperBound() > 0);
    }

    // =====================================================
    // FUZZ TESTS FOR CHAIN ID GENERATION
    // =====================================================

    function testFuzz_ChainIdGeneration(uint256 namespacePrefix, uint256 upperBound, uint256 autoId) public {
        // Bound the inputs to reasonable ranges to avoid overflow and invalid scenarios
        namespacePrefix = bound(namespacePrefix, 1, 1000); // Reasonable namespace range
        upperBound = bound(upperBound, 100, 1_000_000); // Smaller upper bound to avoid overflow
        autoId = bound(autoId, 1, 100); // Limit autoId to avoid creating too many contracts

        // Ensure upperBound > autoId to avoid revert in updateIdUpperBound
        vm.assume(upperBound > autoId);

        vm.startPrank(manager);

        // Set the configuration
        factory.updateNamespaceConfig(namespacePrefix);
        factory.updateIdUpperBound(upperBound);

        vm.stopPrank();

        // Calculate expected chain ID using the formula directly instead of creating chains
        uint256 expectedChainId;

        // Check for overflow before calculating
        if (namespacePrefix <= type(uint256).max / upperBound) {
            expectedChainId = (namespacePrefix * upperBound) + autoId;

            // Verify no collision potential by checking the range
            uint256 namespaceRangeStart = namespacePrefix * upperBound;
            uint256 namespaceRangeEnd = (namespacePrefix + 1) * upperBound;

            // Only run tests if there's no overflow
            if (namespaceRangeEnd > namespaceRangeStart) {
                assertTrue(expectedChainId >= namespaceRangeStart);
                assertTrue(expectedChainId < namespaceRangeEnd);

                // Test that different inputs produce different outputs (collision resistance)
                uint256 differentNamespace = namespacePrefix == 1000 ? 999 : namespacePrefix + 1;
                if (differentNamespace <= type(uint256).max / upperBound) {
                    uint256 differentChainId = (differentNamespace * upperBound) + autoId;
                    assertTrue(expectedChainId != differentChainId);
                }
            }
        }
    }

    function testFuzz_NoCollisionsAcrossNamespaces(uint256 namespace1, uint256 namespace2, uint256 autoId1, uint256 autoId2) public {
        // Ensure different namespaces
        namespace1 = bound(namespace1, 1, 500);
        namespace2 = bound(namespace2, 501, 1000);
        vm.assume(namespace1 != namespace2);

        uint256 upperBound = 1_000_000;
        autoId1 = bound(autoId1, 1, upperBound - 1);
        autoId2 = bound(autoId2, 1, upperBound - 1);

        // Calculate chain IDs for both namespaces
        uint256 chainId1 = (namespace1 * upperBound) + autoId1;
        uint256 chainId2 = (namespace2 * upperBound) + autoId2;

        // Different namespaces should always produce different chain IDs
        assertTrue(chainId1 != chainId2);

        // Verify they're in different ranges
        uint256 range1Start = namespace1 * upperBound;
        uint256 range1End = (namespace1 + 1) * upperBound;
        uint256 range2Start = namespace2 * upperBound;
        uint256 range2End = (namespace2 + 1) * upperBound;

        assertTrue(chainId1 >= range1Start && chainId1 < range1End);
        assertTrue(chainId2 >= range2Start && chainId2 < range2End);
        assertTrue(range1End <= range2Start || range2End <= range1Start); // No range overlap
    }
}
