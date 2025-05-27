// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/SyndicateFactory.sol";
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
    uint256 public namespaceId;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Events
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    event NamespaceConfigUpdated(
        uint256 oldNamespacePrefix,
        uint256 oldNamespaceMultiplier,
        uint256 newNamespacePrefix,
        uint256 newNamespaceMultiplier
    );

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        nonManager = address(0x3);
        factory = new SyndicateFactory(admin);

        // Grant manager role to the manager address
        vm.prank(admin);
        factory.grantRole(MANAGER_ROLE, manager);

        // Calculate namespace ID using the getters
        namespaceId = factory.namespacePrefix() * factory.namespaceMultiplier();
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
        uint256 expectedChainId = namespaceId + 1;

        bytes32 salt = keccak256(abi.encodePacked("salt-for-auto-increment"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), expectedChainId);

        // Verify next chain ID incremented
        assertEq(factory.getNextAutoChainId(), namespaceId + 2);
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

    // Namespace functionality tests
    function testReservedNamespaceRejection() public {
        uint256 reservedId = factory.namespacePrefix() * factory.namespaceMultiplier() + 5;
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-reserved-namespace"));

        vm.expectRevert(SyndicateFactory.ReservedNamespace.selector);
        factory.createSyndicateSequencingChain(reservedId, admin, IRequirementModule(address(permissionModule)), salt);
    }

    function testAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First auto-incremented chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-auto-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        assertEq(id1, namespaceId + 1);

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-auto-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        assertEq(id2, namespaceId + 2);

        // Third auto-incremented chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-auto-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        assertEq(id3, namespaceId + 3);
    }

    function testMixedAutoAndManualChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-mixed-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        assertEq(id1, namespaceId + 1);

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
        // Should be namespaceId + 2 since we only used one auto ID so far
        assertEq(id3, namespaceId + 2);
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
        assertEq(factory.namespaceMultiplier(), 1000);
    }

    function testUpdateNamespaceConfig() public {
        uint256 newPrefix = 511;
        uint256 newMultiplier = 2000;

        // Manager can update namespace config
        vm.prank(manager);
        vm.expectEmit(true, true, true, true);
        emit NamespaceConfigUpdated(
            510, // old prefix
            1000, // old multiplier
            newPrefix,
            newMultiplier
        );
        factory.updateNamespaceConfig(newPrefix, newMultiplier);

        // Check new values
        assertEq(factory.namespacePrefix(), newPrefix);
        assertEq(factory.namespaceMultiplier(), newMultiplier);
    }

    function testUpdateNamespaceConfigAsNonManagerReverts() public {
        uint256 newPrefix = 511;
        uint256 newMultiplier = 2000;

        // Non-manager cannot update namespace config
        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert with a specific message
        factory.updateNamespaceConfig(newPrefix, newMultiplier);
    }

    function testNamespaceUpdateAffectsAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create a chain with old namespace
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-namespace-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Old namespace: 510 * 1000 + 1 = 510001
        assertEq(id1, 510 * 1000 + 1);

        // Update namespace config
        uint256 newPrefix = 600;
        uint256 newMultiplier = 1000;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix, newMultiplier);

        // Create a chain with new namespace
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-namespace-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);

        // New namespace: 600 * 1000 + 2 = 600002 (counter is at 2 now)
        assertEq(id2, 600 * 1000 + 2);
    }

    function testReservedNamespaceAfterUpdate() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Update namespace config
        uint256 newPrefix = 600;
        uint256 newMultiplier = 1000;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix, newMultiplier);

        // Try to create a chain in the new reserved namespace
        uint256 reservedId = newPrefix * newMultiplier + 5;
        bytes32 salt = keccak256(abi.encodePacked("salt-for-new-reserved"));

        vm.expectRevert(SyndicateFactory.ReservedNamespace.selector);
        factory.createSyndicateSequencingChain(reservedId, admin, IRequirementModule(address(permissionModule1)), salt);

        // Old namespace should now be allowed
        uint256 oldReservedId = 510 * 1000 + 5;
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-old-reserved"));

        // This should now work (not revert)
        factory.createSyndicateSequencingChain(
            oldReservedId, admin, IRequirementModule(address(permissionModule2)), salt2
        );
    }

    function testConstructorWithZeroAddressReverts() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new SyndicateFactory(address(0));
    }
}
