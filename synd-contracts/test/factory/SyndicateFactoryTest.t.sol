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
        uint256 expectedChainId = 5101; // 510 + 1

        bytes32 salt = keccak256(abi.encodePacked("salt-for-auto-increment"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), expectedChainId);

        // Verify next chain ID incremented
        assertEq(factory.getNextChainId(), 5102); // 510 + 2
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
        assertEq(id1, 5101); // 510 + 1

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-auto-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        assertEq(id2, 5102); // 510 + 2

        // Third auto-incremented chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-auto-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        assertEq(id3, 5103); // 510 + 3
    }

    function testMixedAutoAndManualChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-mixed-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        assertEq(id1, 5101); // 510 + 1

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
        // Should be 510 + 2 since we only used one auto ID so far
        assertEq(id3, 5102); // 510 + 2
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

        // Old namespace: "510" + "1" = 5101
        assertEq(id1, 5101);

        // Update namespace config
        uint256 newPrefix = 600;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix);

        // Create a chain with new namespace
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-namespace-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);

        // New namespace: "600" + "2" = 6002 (counter is at 2 now)
        assertEq(id2, 6002);
    }

    function testConstructorWithZeroAddressReverts() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new SyndicateFactory(address(0));
    }

    function testGetNextChainIdFunction() public view {
        // Test that the public function works correctly
        uint256 nextId = factory.getNextChainId();
        assertEq(nextId, 5101); // "510" + "1"
    }

    function testChainIdConcatenationLogic() public {
        // Test various chain ID concatenations
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First: 510 + 1 = 5101
        bytes32 salt1 = keccak256(abi.encodePacked("concat-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        assertEq(id1, 5101);

        // Second: 510 + 2 = 5102
        bytes32 salt2 = keccak256(abi.encodePacked("concat-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        assertEq(id2, 5102);

        // Create many to test larger numbers
        for (uint256 i = 3; i <= 10; i++) {
            RequireAndModule pm = new RequireAndModule(admin);
            bytes32 salt = keccak256(abi.encodePacked("concat-loop", i));
            (, uint256 id) = factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(pm)), salt);

            // Expected: "510" + string(i)
            if (i < 10) {
                assertEq(id, 5100 + i); // 5103, 5104, ..., 5109
            } else {
                assertEq(id, 51010); // "510" + "10" = 51010
            }
        }
    }

    function testLargeChainIdNumbers() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Update to higher starting point to test larger concatenations
        vm.prank(admin);
        factory.updateNamespaceConfig(999);

        // Create auto chain - should be "999" + "1" = 9991
        bytes32 salt = keccak256(abi.encodePacked("large-number"));
        (, uint256 id) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);
        assertEq(id, 9991);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        bytes32 salt = keccak256(abi.encodePacked("impl-test"));
        uint256 chainId = 5101; // Use an auto-generated ID
        address impl = factory.computeSequencingChainAddress(salt, chainId);
        assertTrue(impl != address(0));
    }

    // Edge cases and fuzz tests
    function testFuzzManualChainIds(uint256 chainId) public {
        // Skip already used chain IDs and auto-generated ones
        vm.assume(chainId != 0);
        vm.assume(chainId != 5101); // Skip first auto-generated
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
}
