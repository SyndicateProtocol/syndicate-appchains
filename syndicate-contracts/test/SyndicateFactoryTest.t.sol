// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {SyndicateFactory} from "src/SyndicateFactory.sol";
import {SyndicateSequencerChain} from "src/SyndicateSequencerChain.sol";
import {RequireAllModule, IRequirementModule} from "src/requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";

contract SyndicateFactoryTest is Test {
    SyndicateFactory public factory;
    address public admin;
    address public manager;
    address public nonManager;
    uint256 public appChainId = 10042001;
    uint256 public namespaceId;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        nonManager = address(0x3);
        factory = new SyndicateFactory(admin);

        // Grant manager role to the manager address
        vm.prank(admin);
        factory.grantRole(MANAGER_ROLE, manager);

        // Calculate namespace ID using the getters instead of constants
        namespaceId = factory.namespacePrefix() * factory.namespaceMultiplier();
    }

    function testCreateSequencerChainWithRequireAll() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.expectEmit(true, false, false, true);
        emit SyndicateFactory.SyndicateSequencerChainCreated(
            appChainId,
            address(0), // Only check appChainId and permissionModuleAddress
            permissionModuleAddress
        );

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-1"));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencerChain(appChainId, admin, permissionModule, salt);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appChainId);

        SyndicateSequencerChain sequencerChain = SyndicateSequencerChain(sequencerChainAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), appChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateSequencerChainWithRequireAny() public {
        RequireAnyModule permissionModule = new RequireAnyModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.expectEmit(true, false, false, true);
        emit SyndicateFactory.SyndicateSequencerChainCreated(
            appChainId,
            address(0), // Only check appChainId and permissionModuleAddress
            permissionModuleAddress
        );

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-2"));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencerChain(appChainId, admin, IRequirementModule(permissionModule), salt);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appChainId);

        SyndicateSequencerChain sequencerChain = SyndicateSequencerChain(sequencerChainAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), appChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateSyndicateSequencerChainWithRequireAll() public {
        vm.recordLogs();

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-3"));
        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createSyndicateSequencerChainWithRequireAllModule(admin, appChainId, salt);

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("SyndicateSequencerChainCreated(uint256,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(uint256(entries[i].topics[1]), appChainId);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                found = true;
                break;
            }
        }

        assertTrue(found, "SyndicateSequencerChainCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));
        assertEq(actualChainId, appChainId);

        SyndicateSequencerChain sequencerChainContract = SyndicateSequencerChain(sequencerChainAddr);
        assertEq(sequencerChainContract.appChainId(), appChainId);

        uint256 newAppChainId = 10042002;
        bytes32 newSalt = keccak256(abi.encodePacked("salt-for-test-4"));

        vm.expectEmit(true, false, false, false);
        emit SyndicateFactory.SyndicateSequencerChainCreated(
            newAppChainId, factory.computeSequencerChainAddress(newSalt, newAppChainId), address(permissionModuleAddr)
        );
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createSyndicateSequencerChainWithRequireAllModule(admin, newAppChainId, newSalt);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));
        assertEq(newActualChainId, newAppChainId);

        SyndicateSequencerChain sequencerChain = SyndicateSequencerChain(sequencerChainAddress);
        RequireAllModule permissionModule = RequireAllModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), newAppChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateSyndicateSequencerChainWithRequireAny() public {
        vm.recordLogs();

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-5"));
        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createSyndicateSequencerChainWithRequireAnyModule(admin, appChainId, salt);

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("SyndicateSequencerChainCreated(uint256,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(uint256(entries[i].topics[1]), appChainId);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                found = true;
                break;
            }
        }

        assertTrue(found, "SyndicateSequencerChainCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));
        assertEq(actualChainId, appChainId);

        SyndicateSequencerChain sequencerChainContract = SyndicateSequencerChain(sequencerChainAddr);

        assertEq(sequencerChainContract.appChainId(), appChainId);
        uint256 newAppChainId = 10042002;
        bytes32 newSalt = keccak256(abi.encodePacked("salt-for-test-6"));

        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createSyndicateSequencerChainWithRequireAnyModule(admin, newAppChainId, newSalt);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));
        assertEq(newActualChainId, newAppChainId);

        SyndicateSequencerChain sequencerChain = SyndicateSequencerChain(sequencerChainAddress);
        RequireAnyModule permissionModule = RequireAnyModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), newAppChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCorrectAppChainIdAssignment() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        uint256 differentChainId = 10042002;

        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-test-7"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-test-8"));

        (address sequencerChain1, uint256 actualChainId1) = factory.createSyndicateSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt1
        );
        (address sequencerChain2, uint256 actualChainId2) = factory.createSyndicateSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), salt2
        );

        assertEq(SyndicateSequencerChain(sequencerChain1).appChainId(), appChainId);
        assertEq(SyndicateSequencerChain(sequencerChain2).appChainId(), differentChainId);
        assertEq(actualChainId1, appChainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-9"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule)), salt
        );

        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule2)), salt
        );

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChainWithRequireAllModule(address(0), appChainId, salt);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChainWithRequireAnyModule(address(0), appChainId, salt);
    }

    function testRevertsOnZeroManager() public {
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-10"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChainWithRequireAllModule(address(0), appChainId, salt);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencerChainWithRequireAnyModule(address(0), appChainId, salt);
    }

    // The zero chain ID test is modified since we now allow zero to trigger auto-increment
    function testAutoIncrementOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        uint256 expectedChainId = namespaceId + 1;

        bytes32 salt = keccak256(abi.encodePacked("salt-for-auto-increment"));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencerChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencerChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencerChain(sequencerChainAddress).appChainId(), expectedChainId);

        // Verify next chain ID incremented
        assertEq(factory.getNextAutoChainId(), namespaceId + 2);
    }

    function testCreateSequencerChainAddressIsDeterministic() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-deterministic"));

        (address sequencerChainAddress,) = factory.createSyndicateSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testCreateSequencerChainAddressIsDeterministicWithDifferentSaltThanChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-different-deterministic"));

        (address sequencerChainAddress,) = factory.createSyndicateSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testGetBytecode() public view {
        bytes memory bytecode = factory.getBytecode(appChainId);
        bytes memory expectedBytecode =
            abi.encodePacked(type(SyndicateSequencerChain).creationCode, abi.encode(appChainId));
        assertEq(bytecode, expectedBytecode);
    }

    // New tests for the 510 namespace functionality
    function testReservedNamespaceRejection() public {
        uint256 reservedId = factory.namespacePrefix() * factory.namespaceMultiplier() + 5;
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-reserved-namespace"));

        vm.expectRevert(SyndicateFactory.ReservedNamespace.selector);
        factory.createSyndicateSequencerChain(reservedId, admin, IRequirementModule(address(permissionModule)), salt);
    }

    function testAutoIncrementChainIds() public {
        // First auto-incremented chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-auto-1"));
        (address chain1,, uint256 id1) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt1);

        assertEq(id1, namespaceId + 1);

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-auto-2"));
        (address chain2,, uint256 id2) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt2);

        assertEq(id2, namespaceId + 2);

        // Third with RequireAny
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-auto-3"));
        (address chain3,, uint256 id3) = factory.createSyndicateSequencerChainWithRequireAnyModule(admin, 0, salt3);

        assertEq(id3, namespaceId + 3);

        // Verify all chains have correct IDs
        assertEq(SyndicateSequencerChain(chain1).appChainId(), namespaceId + 1);
        assertEq(SyndicateSequencerChain(chain2).appChainId(), namespaceId + 2);
        assertEq(SyndicateSequencerChain(chain3).appChainId(), namespaceId + 3);
    }

    function testMixedAutoAndManualChainIds() public {
        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-mixed-1"));
        (address chain1,, uint256 id1) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt1);

        assertEq(id1, namespaceId + 1);

        // Manual chain ID
        uint256 manualId = 42000;
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-mixed-2"));
        (address chain2,, uint256 id2) =
            factory.createSyndicateSequencerChainWithRequireAllModule(admin, manualId, salt2);

        assertEq(id2, manualId);

        // Back to auto chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-mixed-3"));
        (address chain3,, uint256 id3) = factory.createSyndicateSequencerChainWithRequireAnyModule(admin, 0, salt3);

        // Should be namespaceId + 2 since we only used one auto ID so far
        assertEq(id3, namespaceId + 2);
    }

    function testChainIdAlreadyExists() public {
        // Create first chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-duplicate-1"));
        factory.createSyndicateSequencerChainWithRequireAllModule(admin, appChainId, salt1);

        // Try to create another with same chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-duplicate-2"));
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencerChainWithRequireAllModule(admin, appChainId, salt2);
    }

    function testIsChainIdUsed() public {
        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appChainId), 0);

        // Create chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-used-1"));
        factory.createSyndicateSequencerChainWithRequireAllModule(admin, appChainId, salt1);

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appChainId), 1);

        // Auto-incremented ID should also be marked as used
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-used-2"));
        (,, uint256 id2) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt2);

        assertEq(factory.isChainIdUsed(id2), 1);
    }

    // New tests for AccessControl functionality

    function testRoleSetup() public {
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

    function testNamespaceConfigGetters() public {
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
        emit SyndicateFactory.NamespaceConfigUpdated(
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
        // Create a chain with old namespace
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-namespace-1"));
        (,, uint256 id1) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt1);

        // Old namespace: 510 * 1000 + 1 = 510001
        assertEq(id1, 510 * 1000 + 1);

        // Update namespace config
        uint256 newPrefix = 600;
        uint256 newMultiplier = 1000;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix, newMultiplier);

        // Create a chain with new namespace
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-namespace-2"));
        (,, uint256 id2) = factory.createSyndicateSequencerChainWithRequireAllModule(admin, 0, salt2);

        // New namespace: 600 * 1000 + 2 = 600002 (counter is at 2 now)
        assertEq(id2, 600 * 1000 + 2);
    }

    function testReservedNamespaceAfterUpdate() public {
        // Update namespace config
        uint256 newPrefix = 600;
        uint256 newMultiplier = 1000;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix, newMultiplier);

        // Try to create a chain in the new reserved namespace
        uint256 reservedId = newPrefix * newMultiplier + 5;
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-new-reserved"));

        vm.expectRevert(SyndicateFactory.ReservedNamespace.selector);
        factory.createSyndicateSequencerChain(reservedId, admin, IRequirementModule(address(permissionModule)), salt);

        // Old namespace should now be allowed
        uint256 oldReservedId = 510 * 1000 + 5;
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-old-reserved"));

        // This should now work (not revert)
        factory.createSyndicateSequencerChain(
            oldReservedId, admin, IRequirementModule(address(permissionModule)), salt2
        );
    }
}
