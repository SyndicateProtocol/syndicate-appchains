// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule, IRequirementModule} from "src/requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";

contract MetabasedFactoryTest is Test {
    MetabasedFactory public factory;
    address public admin;
    address public manager;
    uint256 public appChainId = 10042001;
    uint256 public namespaceId;

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        factory = new MetabasedFactory();
        namespaceId = factory.NAMESPACE_PREFIX() * factory.NAMESPACE_MULTIPLIER();
    }

    function testCreateSequencerChainWithRequireAll() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.expectEmit(true, false, false, true);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            appChainId,
            address(0), // Only check appChainId and permissionModuleAddress
            permissionModuleAddress
        );
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createMetabasedSequencerChain(appChainId, admin, permissionModule, bytes32(appChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appChainId);

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);

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
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            appChainId,
            address(0), // Only check appChainId and permissionModuleAddress
            permissionModuleAddress
        );
        (address sequencerChainAddress, uint256 actualChainId) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(permissionModule), bytes32(appChainId)
        );

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appChainId);

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), appChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateMetabasedSequencerChainWithRequireAll() public {
        vm.recordLogs();

        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, bytes32(appChainId));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("MetabasedSequencerChainCreated(uint256,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(uint256(entries[i].topics[1]), appChainId);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                found = true;
                break;
            }
        }

        assertTrue(found, "MetabasedSequencerChainCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));
        assertEq(actualChainId, appChainId);

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);
        assertEq(sequencerChainContract.appChainId(), appChainId);

        uint256 newAppChainId = 10042002;
        vm.expectEmit(true, false, false, false);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            newAppChainId,
            factory.computeSequencerChainAddress(bytes32(newAppChainId), newAppChainId),
            address(permissionModuleAddr)
        );
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, newAppChainId, bytes32(newAppChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));
        assertEq(newActualChainId, newAppChainId);

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        RequireAllModule permissionModule = RequireAllModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), newAppChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateMetabasedSequencerChainWithRequireAny() public {
        vm.recordLogs();

        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, appChainId, bytes32(appChainId));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("MetabasedSequencerChainCreated(uint256,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(uint256(entries[i].topics[1]), appChainId);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                found = true;
                break;
            }
        }

        assertTrue(found, "MetabasedSequencerChainCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));
        assertEq(actualChainId, appChainId);

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);

        assertEq(sequencerChainContract.appChainId(), appChainId);
        uint256 newAppChainId = 10042002;
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, newAppChainId, bytes32(newAppChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));
        assertEq(newActualChainId, newAppChainId);

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
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
        (address sequencerChain1, uint256 actualChainId1) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), bytes32(appChainId)
        );
        (address sequencerChain2, uint256 actualChainId2) = factory.createMetabasedSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), bytes32(differentChainId)
        );

        assertEq(MetabasedSequencerChain(sequencerChain1).appChainId(), appChainId);
        assertEq(MetabasedSequencerChain(sequencerChain2).appChainId(), differentChainId);
        assertEq(actualChainId1, appChainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule)), bytes32(appChainId)
        );

        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule2)), bytes32(appChainId)
        );

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(address(0), appChainId, bytes32(appChainId));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAnyModule(address(0), appChainId, bytes32(appChainId));
    }

    function testRevertsOnZeroManager() public {
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(address(0), appChainId, bytes32(appChainId));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAnyModule(address(0), appChainId, bytes32(appChainId));
    }

    // The zero chain ID test is modified since we now allow zero to trigger auto-increment
    function testAutoIncrementOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        uint256 expectedChainId = namespaceId + 1;

        (address sequencerChainAddress, uint256 actualChainId) = factory.createMetabasedSequencerChain(
            0, admin, IRequirementModule(address(permissionModule)), bytes32(expectedChainId)
        );

        assertTrue(sequencerChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(MetabasedSequencerChain(sequencerChainAddress).appChainId(), expectedChainId);

        // Verify next chain ID incremented
        assertEq(factory.getNextAutoChainId(), namespaceId + 2);
    }

    function testCreateSequencerChainAddressIsDeterministic() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(appChainId);
        (address sequencerChainAddress,) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testCreateSequencerChainAddressIsDeterministicWithDifferentSaltThanChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(appChainId + 10);
        (address sequencerChainAddress,) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testGetBytecode() public view {
        bytes memory bytecode = factory.getBytecode(appChainId);
        bytes memory expectedBytecode =
            abi.encodePacked(type(MetabasedSequencerChain).creationCode, abi.encode(appChainId));
        assertEq(bytecode, expectedBytecode);
    }

    // New tests for the 510 namespace functionality
    function testReservedNamespaceRejection() public {
        uint256 reservedId = factory.NAMESPACE_PREFIX() * factory.NAMESPACE_MULTIPLIER() + 5;
        RequireAllModule permissionModule = new RequireAllModule(admin);

        vm.expectRevert(MetabasedFactory.ReservedNamespace.selector);
        factory.createMetabasedSequencerChain(
            reservedId, admin, IRequirementModule(address(permissionModule)), bytes32(reservedId)
        );
    }

    function testAutoIncrementChainIds() public {
        // First auto-incremented chain ID
        (address chain1,, uint256 id1) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, bytes32(uint256(1)));

        assertEq(id1, namespaceId + 1);

        // Second auto-incremented chain ID
        (address chain2,, uint256 id2) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, bytes32(uint256(2)));

        assertEq(id2, namespaceId + 2);

        // Third with RequireAny
        (address chain3,, uint256 id3) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, 0, bytes32(uint256(3)));

        assertEq(id3, namespaceId + 3);

        // Verify all chains have correct IDs
        assertEq(MetabasedSequencerChain(chain1).appChainId(), namespaceId + 1);
        assertEq(MetabasedSequencerChain(chain2).appChainId(), namespaceId + 2);
        assertEq(MetabasedSequencerChain(chain3).appChainId(), namespaceId + 3);
    }

    function testMixedAutoAndManualChainIds() public {
        // Auto chain ID
        (,, uint256 id1) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, bytes32(uint256(1)));

        assertEq(id1, namespaceId + 1);

        // Manual chain ID
        uint256 manualId = 42000;
        (,, uint256 id2) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, manualId, bytes32(uint256(2)));

        assertEq(id2, manualId);

        // Back to auto chain ID
        (,, uint256 id3) = factory.createMetabasedSequencerChainWithRequireAnyModule(admin, 0, bytes32(uint256(3)));

        // Should be namespaceId + 2 since we only used one auto ID so far
        assertEq(id3, namespaceId + 2);
    }

    function testChainIdAlreadyExists() public {
        // Create first chain
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, bytes32(uint256(1)));

        // Try to create another with same chain ID
        vm.expectRevert(MetabasedFactory.ChainIdAlreadyExists.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, bytes32(uint256(2)));
    }

    function testIsChainIdUsed() public {
        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appChainId), 0);

        // Create chain
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, bytes32(uint256(1)));

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appChainId), 1);

        // Auto-incremented ID should also be marked as used
        (,, uint256 id2) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, bytes32(uint256(2)));

        assertEq(factory.isChainIdUsed(id2), 1);
    }
}
