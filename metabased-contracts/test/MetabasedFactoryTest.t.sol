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
    uint256 public appChainId = 10042001;

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
            appChainId,
            address(0), // Only check appChainId and permissionModuleAddress
            permissionModuleAddress
        );
        (address sequencerChainAddress) =
            factory.createMetabasedSequencerChain(appChainId, admin, permissionModule, bytes32(appChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

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
        (address sequencerChainAddress) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(permissionModule), bytes32(appChainId)
        );

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

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

        (address sequencerChainAddr, IRequirementModule permissionModuleAddr) =
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

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);
        assertEq(sequencerChainContract.appChainId(), appChainId);

        uint256 newL3ChainId = 10042002;
        vm.expectEmit(true, false, false, false);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            newL3ChainId,
            factory.computeSequencerChainAddress(bytes32(newL3ChainId), newL3ChainId),
            address(permissionModuleAddr)
        );
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, newL3ChainId, bytes32(newL3ChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        RequireAllModule permissionModule = RequireAllModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), newL3ChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateMetabasedSequencerChainWithRequireAny() public {
        vm.recordLogs();

        (address sequencerChainAddr, IRequirementModule permissionModuleAddr) =
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

        MetabasedSequencerChain sequencerChainContract = MetabasedSequencerChain(sequencerChainAddr);

        assertEq(sequencerChainContract.appChainId(), appChainId);
        uint256 newL3ChainId = 10042002;
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, newL3ChainId, bytes32(newL3ChainId));

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(address(permissionModuleAddress) != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        RequireAnyModule permissionModule = RequireAnyModule(address(permissionModuleAddress));

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.appChainId(), newL3ChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.permissionRequirementModule()) == address(permissionModuleAddress));
        assertTrue(permissionModule.owner() == admin);
    }

    function testCorrectL3ChainIdAssignment() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        uint256 differentChainId = 10042002;
        (address sequencerChain1) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), bytes32(appChainId)
        );
        (address sequencerChain2) = factory.createMetabasedSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), bytes32(differentChainId)
        );

        assertEq(MetabasedSequencerChain(sequencerChain1).appChainId(), appChainId);
        assertEq(MetabasedSequencerChain(sequencerChain2).appChainId(), differentChainId);
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

    function testRevertsOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createMetabasedSequencerChain(0, admin, IRequirementModule(address(permissionModule)), bytes32(0));

        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, bytes32(0));

        vm.expectRevert(MetabasedFactory.ZeroValue.selector);
        factory.createMetabasedSequencerChainWithRequireAnyModule(admin, 0, bytes32(0));
    }

    function testCreateSequencerChainAddressIsDeterministic() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(appChainId);
        address sequencerChainAddress = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testCreateSequencerChainAddressIsDeterministicWithDifferentSaltThanChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = bytes32(appChainId + 10);
        address sequencerChainAddress = factory.createMetabasedSequencerChain(
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
}
