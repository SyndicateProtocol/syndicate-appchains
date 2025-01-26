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
        (address sequencerChainAddress) = factory.createMetabasedSequencerChain(l3ChainId, admin, permissionModule);

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
        (address sequencerChainAddress) =
            factory.createMetabasedSequencerChain(l3ChainId, admin, IRequirementModule(permissionModule));

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
            factory.createAllContractsWithRequireAllModule(admin, manager, l3ChainId);

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;
        
        for (uint i = 0; i < entries.length; i++) {
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
        MetafillerStorage metafillerStorageContract = MetafillerStorage(metafillerStorageAddr);
        RequireAllModule permissionModuleContract = RequireAllModule(address(permissionModuleAddr));

        assertEq(sequencerChainContract.l3ChainId(), l3ChainId);
        (address sequencerChainAddress, address metafillerStorageAddress, IRequirementModule permissionModuleAddress) =
            factory.createAllContractsWithRequireAllModule(admin, manager, l3ChainId);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(metafillerStorageAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        RequireAllModule permissionModule = RequireAllModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

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
            factory.createAllContractsWithRequireAnyModule(admin, manager, l3ChainId);

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bool found = false;
        
        for (uint i = 0; i < entries.length; i++) {
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
        MetafillerStorage metafillerStorageContract = MetafillerStorage(metafillerStorageAddr);
        RequireAnyModule permissionModuleContract = RequireAnyModule(address(permissionModuleAddr));

        assertEq(sequencerChainContract.l3ChainId(), l3ChainId);
        (address sequencerChainAddress, address metafillerStorageAddress, IRequirementModule permissionModuleAddress) =
            factory.createAllContractsWithRequireAnyModule(admin, manager, l3ChainId);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(metafillerStorageAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        RequireAnyModule permissionModule = RequireAnyModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

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
        (address sequencerChain1) =
            factory.createMetabasedSequencerChain(l3ChainId, admin, IRequirementModule(address(permissionModule)));
        (address sequencerChain2) = factory.createMetabasedSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2))
        );

        assertEq(MetabasedSequencerChain(sequencerChain1).l3ChainId(), l3ChainId);
        assertEq(MetabasedSequencerChain(sequencerChain2).l3ChainId(), differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert("Admin address cannot be zero");
        factory.createMetabasedSequencerChain(l3ChainId, address(0), IRequirementModule(address(permissionModule)));

        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        vm.expectRevert("Admin address cannot be zero");
        factory.createMetabasedSequencerChain(l3ChainId, address(0), IRequirementModule(address(permissionModule2)));

        vm.expectRevert("Admin address cannot be zero");
        factory.createAllContractsWithRequireAllModule(address(0), manager, l3ChainId);

        vm.expectRevert("Admin address cannot be zero");
        factory.createAllContractsWithRequireAnyModule(address(0), manager, l3ChainId);
    }

    function testRevertsOnZeroManager() public {
        vm.expectRevert("Manager address cannot be zero");
        factory.createMetafillerStorage(admin, address(0), l3ChainId);

        vm.expectRevert("Manager address cannot be zero");
        factory.createAllContractsWithRequireAllModule(admin, address(0), l3ChainId);

        vm.expectRevert("Manager address cannot be zero");
        factory.createAllContractsWithRequireAnyModule(admin, address(0), l3ChainId);
    }

    function testRevertsOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert("L3 chain ID cannot be zero");
        factory.createMetabasedSequencerChain(0, admin, IRequirementModule(address(permissionModule)));

        vm.expectRevert("L3 chain ID cannot be zero");
        factory.createMetafillerStorage(admin, manager, 0);

        vm.expectRevert("L3 chain ID cannot be zero");
        factory.createAllContractsWithRequireAllModule(admin, manager, 0);

        vm.expectRevert("L3 chain ID cannot be zero");
        factory.createAllContractsWithRequireAnyModule(admin, manager, 0);
    }
}
