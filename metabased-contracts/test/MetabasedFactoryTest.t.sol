// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule, IRequirementModule} from "src/requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";
import {MetafillerStorage} from "src/backfill/MetafillerStorage.sol";

contract MetabasedFactoryTest is Test {
    MetabasedFactory public factory;
    address public admin;
    address public manager;
    uint256 public l3ChainId = 10042001;

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        factory = new MetabasedFactory();
    }

    function testCreateSequencerChainWithRequireAll() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.expectEmit(true, false, false, true);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            l3ChainId,
            address(0), // Only check l3ChainId and permissionModuleAddress
            permissionModuleAddress
        );
        (address sequencerChainAddress) =
            factory.createMetabasedSequencerChain(l3ChainId, admin, permissionModule, bytes32(l3ChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateSequencerChainWithRequireAny() public {
        RequireAnyModule permissionModule = new RequireAnyModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.expectEmit(true, false, false, true);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            l3ChainId,
            address(0), // Only check l3ChainId and permissionModuleAddress
            permissionModuleAddress
        );
        (address sequencerChainAddress) = factory.createMetabasedSequencerChain(
            l3ChainId, admin, IRequirementModule(permissionModule), bytes32(l3ChainId)
        );

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateMetafillerStorage() public {
        vm.expectEmit(true, false, false, false);
        emit MetabasedFactory.MetafillerStorageCreated(l3ChainId, address(0)); // Only check l3ChainId since address will be dynamic
        address metafillerStorageAddress = factory.createMetafillerStorage(admin, manager, l3ChainId);
        assertTrue(metafillerStorageAddress != address(0));

        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        assertTrue(address(metafillerStorage) == metafillerStorageAddress);
        assertTrue(metafillerStorage.hasRole(metafillerStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(metafillerStorage.hasRole(metafillerStorage.MANAGER_ROLE(), manager));
    }

    function testCreateAllContractsWithRequireAll() public {
        vm.recordLogs();

        (address sequencerChainAddr, address metafillerStorageAddr, IRequirementModule permissionModuleAddr) =
            factory.createAllContractsWithRequireAllModule(admin, manager, l3ChainId, bytes32(l3ChainId));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("AllContractsCreated(uint256,address,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(address(uint160(uint256(entries[i].topics[1]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), metafillerStorageAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                // Check non-indexed parameter (in data)
                (uint256 emittedChainId) = abi.decode(entries[i].data, (uint256));
                assertEq(emittedChainId, l3ChainId);

                found = true;
                break;
            }
        }

        assertTrue(found, "AllContractsCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(metafillerStorageAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);
        assertEq(sequencerChainContract.l3ChainId(), l3ChainId);

        uint256 newL3ChainId = 10042002;
        vm.expectEmit(true, false, false, false);
        emit MetabasedFactory.AllContractsCreated(
            newL3ChainId,
            factory.computeSequencerChainAddress(bytes32(newL3ChainId), newL3ChainId),
            0xD76ffbd1eFF76C510C3a509fE22864688aC3A588,
            0x7FdB3132Ff7D02d8B9e221c61cC895ce9a4bb773
        );
        (address sequencerChainAddress, address metafillerStorageAddress, IRequirementModule permissionModuleAddress) =
            factory.createAllContractsWithRequireAllModule(admin, manager, newL3ChainId, bytes32(newL3ChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(metafillerStorageAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        RequireAllModule permissionModule = RequireAllModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), newL3ChainId);

        // Verify metafiller setup
        assertTrue(address(metafillerStorage) == metafillerStorageAddress);
        assertTrue(metafillerStorage.hasRole(metafillerStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(metafillerStorage.hasRole(metafillerStorage.MANAGER_ROLE(), manager));

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateAllContractsWithRequireAny() public {
        vm.recordLogs();

        (address sequencerChainAddr, address metafillerStorageAddr, IRequirementModule permissionModuleAddr) =
            factory.createAllContractsWithRequireAnyModule(admin, manager, l3ChainId, bytes32(l3ChainId));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;

        for (uint256 i = 0; i < entries.length; i++) {
            // The event signature for AllContractsCreated
            if (entries[i].topics[0] == keccak256("AllContractsCreated(uint256,address,address,address)")) {
                // Check indexed parameters (they are in topics)
                assertEq(address(uint160(uint256(entries[i].topics[1]))), sequencerChainAddr);
                assertEq(address(uint160(uint256(entries[i].topics[2]))), metafillerStorageAddr);
                assertEq(address(uint160(uint256(entries[i].topics[3]))), address(permissionModuleAddr));

                // Check non-indexed parameter (in data)
                (uint256 emittedChainId) = abi.decode(entries[i].data, (uint256));
                assertEq(emittedChainId, l3ChainId);

                found = true;
                break;
            }
        }

        assertTrue(found, "AllContractsCreated event not found");

        assertTrue(sequencerChainAddr != address(0));
        assertTrue(metafillerStorageAddr != address(0));
        assertTrue(address(permissionModuleAddr) != address(0));

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);

        assertEq(sequencerChainContract.l3ChainId(), l3ChainId);
        uint256 newL3ChainId = 10042002;
        (address sequencerChainAddress, address metafillerStorageAddress, IRequirementModule permissionModuleAddress) =
            factory.createAllContractsWithRequireAnyModule(admin, manager, newL3ChainId, bytes32(newL3ChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(metafillerStorageAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        RequireAnyModule permissionModule = RequireAnyModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), newL3ChainId);

        // Verify metafiller setup
        assertTrue(address(metafillerStorage) == metafillerStorageAddress);
        assertTrue(metafillerStorage.hasRole(metafillerStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(metafillerStorage.hasRole(metafillerStorage.MANAGER_ROLE(), manager));

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCorrectL3ChainIdAssignment() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        uint256 differentChainId = 10042002;
        (address sequencerChain1) = factory.createMetabasedSequencerChain(
            l3ChainId, admin, IRequirementModule(address(permissionModule)), bytes32(l3ChainId)
        );
        (address sequencerChain2) = factory.createMetabasedSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), bytes32(differentChainId)
        );

        assertEq(MetabasedSequencerChain(sequencerChain1).l3ChainId(), l3ChainId);
        assertEq(MetabasedSequencerChain(sequencerChain2).l3ChainId(), differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            l3ChainId, address(0), IRequirementModule(address(permissionModule)), bytes32(l3ChainId)
        );

        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            l3ChainId, address(0), IRequirementModule(address(permissionModule2)), bytes32(l3ChainId)
        );

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createAllContractsWithRequireAllModule(address(0), manager, l3ChainId, bytes32(l3ChainId));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createAllContractsWithRequireAnyModule(address(0), manager, l3ChainId, bytes32(l3ChainId));
    }

    function testRevertsOnZeroManager() public {
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetafillerStorage(admin, address(0), l3ChainId);

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createAllContractsWithRequireAllModule(admin, address(0), l3ChainId, bytes32(l3ChainId));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createAllContractsWithRequireAnyModule(admin, address(0), l3ChainId, bytes32(l3ChainId));
    }

    function testRevertsOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createMetabasedSequencerChain(0, admin, IRequirementModule(address(permissionModule)), bytes32(0));

        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createMetafillerStorage(admin, manager, 0);

        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createAllContractsWithRequireAllModule(admin, manager, 0, bytes32(0));

        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createAllContractsWithRequireAnyModule(admin, manager, 0, bytes32(0));
    }

    function testCreateSequencerChainAddressIsDeterministic() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(l3ChainId);
        address sequencerChainAddress =
            factory.createMetabasedSequencerChain(l3ChainId, admin, IRequirementModule(address(permissionModule)), salt);
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, l3ChainId));
    }

    function testCreateSequencerChainAddressIsDeterministicWithDifferentSaltThanChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(l3ChainId + 10);
        address sequencerChainAddress =
            factory.createMetabasedSequencerChain(l3ChainId, admin, IRequirementModule(address(permissionModule)), salt);
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, l3ChainId));
    }

    function testGetBytecode() public {
        bytes memory bytecode = factory.getBytecode(l3ChainId);
        bytes memory expectedBytecode =
            abi.encodePacked(type(MetabasedSequencerChain).creationCode, abi.encode(l3ChainId));
        assertEq(bytecode, expectedBytecode);
    }
}
