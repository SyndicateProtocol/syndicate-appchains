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
        factory = new MetabasedFactory(admin);
        namespaceId = 510 * 1000;
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

        // Use keccak256 to generate a valid bytes32 value
        bytes32 salt = keccak256(abi.encodePacked("salt-for-appchain-", appChainId));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createMetabasedSequencerChain(appChainId, admin, permissionModule, salt);

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

        bytes32 salt = keccak256(abi.encodePacked("salt-for-any-module-", appChainId));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createMetabasedSequencerChain(appChainId, admin, IRequirementModule(permissionModule), salt);

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

        bytes32 salt = keccak256(abi.encodePacked("create-with-all-test-", appChainId));
        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, salt);

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
        bytes32 newSalt = keccak256(abi.encodePacked("new-salt-", newAppChainId));

        vm.expectEmit(true, false, false, false);
        emit MetabasedFactory.MetabasedSequencerChainCreated(
            newAppChainId, factory.computeSequencerChainAddress(newSalt, newAppChainId), address(permissionModuleAddr)
        );
        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createMetabasedSequencerChainWithRequireAllModule(admin, newAppChainId, newSalt);

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

        bytes32 salt = keccak256(abi.encodePacked("create-with-any-test-", appChainId));
        (address sequencerChainAddr, IRequirementModule permissionModuleAddr, uint256 actualChainId) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, appChainId, salt);

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
        bytes32 newSalt = keccak256(abi.encodePacked("new-any-salt-", newAppChainId));

        (address sequencerChainAddress, IRequirementModule permissionModuleAddress, uint256 newActualChainId) =
            factory.createMetabasedSequencerChainWithRequireAnyModule(admin, newAppChainId, newSalt);

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

        bytes32 salt1 = keccak256(abi.encodePacked("salt-chain1-", appChainId));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-chain2-", differentChainId));

        (address sequencerChain1, uint256 actualChainId1) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt1
        );
        (address sequencerChain2, uint256 actualChainId2) = factory.createMetabasedSequencerChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), salt2
        );

        assertEq(MetabasedSequencerChain(sequencerChain1).appChainId(), appChainId);
        assertEq(MetabasedSequencerChain(sequencerChain2).appChainId(), differentChainId);
        assertEq(actualChainId1, appChainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("test-zero-admin"));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule)), salt
        );

        RequireAnyModule permissionModule2 = new RequireAnyModule(admin);
        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChain(
            appChainId, address(0), IRequirementModule(address(permissionModule2)), salt
        );

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(address(0), appChainId, salt);

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAnyModule(address(0), appChainId, salt);
    }

    function testRevertsOnZeroManager() public {
        bytes32 salt = keccak256(abi.encodePacked("test-zero-manager"));

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(address(0), appChainId, salt);

        vm.expectRevert(MetabasedFactory.ZeroAddress.selector);
        factory.createMetabasedSequencerChainWithRequireAnyModule(address(0), appChainId, salt);
    }

    // The zero chain ID test is modified since we now allow zero to trigger auto-increment
    function testAutoIncrementOnZeroChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        uint256 expectedChainId = namespaceId + 1;

        bytes32 salt = keccak256(abi.encodePacked("test-auto-increment"));
        (address sequencerChainAddress, uint256 actualChainId) =
            factory.createMetabasedSequencerChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencerChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(MetabasedSequencerChain(sequencerChainAddress).appChainId(), expectedChainId);

        // Verify next chain ID incremented
        assertEq(factory.getNextAutoChainId(), namespaceId + 2);
    }

    function testCreateSequencerChainAddressIsDeterministic() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("deterministic-test-", appChainId));

        (address sequencerChainAddress,) = factory.createMetabasedSequencerChain(
            appChainId, admin, IRequirementModule(address(permissionModule)), salt
        );
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, appChainId));
    }

    function testCreateSequencerChainAddressIsDeterministicWithDifferentSaltThanChainId() public {
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("diff-salt-test-", appChainId + 10));

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
        uint256 reservedId = namespaceId + 5;
        RequireAllModule permissionModule = new RequireAllModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("reserved-namespace-test"));

        vm.expectRevert(MetabasedFactory.ReservedNamespace.selector);
        factory.createMetabasedSequencerChain(reservedId, admin, IRequirementModule(address(permissionModule)), salt);
    }

    function testAutoIncrementChainIds() public {
        // First auto-incremented chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("auto-increment-1"));
        (address chain1,, uint256 id1) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, salt1);

        assertEq(id1, namespaceId + 1);

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("auto-increment-2"));
        (address chain2,, uint256 id2) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, salt2);

        assertEq(id2, namespaceId + 2);

        // Third with RequireAny
        bytes32 salt3 = keccak256(abi.encodePacked("auto-increment-3"));
        (address chain3,, uint256 id3) = factory.createMetabasedSequencerChainWithRequireAnyModule(admin, 0, salt3);

        assertEq(id3, namespaceId + 3);

        // Verify all chains have correct IDs
        assertEq(MetabasedSequencerChain(chain1).appChainId(), namespaceId + 1);
        assertEq(MetabasedSequencerChain(chain2).appChainId(), namespaceId + 2);
        assertEq(MetabasedSequencerChain(chain3).appChainId(), namespaceId + 3);
    }

    function testMixedAutoAndManualChainIds() public {
        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("mixed-auto-1"));
        (,, uint256 id1) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, salt1);

        assertEq(id1, namespaceId + 1);

        // Manual chain ID
        uint256 manualId = 42000;
        bytes32 salt2 = keccak256(abi.encodePacked("mixed-manual-", manualId));
        (,, uint256 id2) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, manualId, salt2);

        assertEq(id2, manualId);

        // Back to auto chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("mixed-auto-2"));
        (,, uint256 id3) = factory.createMetabasedSequencerChainWithRequireAnyModule(admin, 0, salt3);

        // Should be namespaceId + 2 since we only used one auto ID so far
        assertEq(id3, namespaceId + 2);
    }

    function testChainIdAlreadyExists() public {
        // Create first chain
        bytes32 salt1 = keccak256(abi.encodePacked("duplicate-test-1"));
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, salt1);

        // Try to create another with same chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("duplicate-test-2"));
        vm.expectRevert(MetabasedFactory.ChainIdAlreadyExists.selector);
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, salt2);
    }

    function testIsChainIdUsed() public {
        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appChainId), 0);

        // Create chain
        bytes32 salt1 = keccak256(abi.encodePacked("chain-used-test-1"));
        factory.createMetabasedSequencerChainWithRequireAllModule(admin, appChainId, salt1);

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appChainId), 1);

        // Auto-incremented ID should also be marked as used
        bytes32 salt2 = keccak256(abi.encodePacked("chain-used-test-2"));
        (,, uint256 id2) = factory.createMetabasedSequencerChainWithRequireAllModule(admin, 0, salt2);

        assertEq(factory.isChainIdUsed(id2), 1);
    }
}
