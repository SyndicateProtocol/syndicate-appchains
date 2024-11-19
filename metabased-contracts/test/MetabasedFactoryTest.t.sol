// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
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

    function testCreateSequencerChain() public {
        (address sequencerChainAddress, address permissionModuleAddress) =
            factory.createMetabasedSequencerChain(l3ChainId, admin);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        RequireAllModule permissionModule = RequireAllModule(permissionModuleAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }

    function testCreateMetafillerStorage() public {
        address metafillerStorageAddress = factory.createMetafillerStorage(admin, manager, l3ChainId);
        assertTrue(metafillerStorageAddress != address(0));

        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        assertTrue(address(metafillerStorage) == metafillerStorageAddress);
        assertTrue(metafillerStorage.hasRole(metafillerStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(metafillerStorage.hasRole(metafillerStorage.MANAGER_ROLE(), manager));
    }

    function testCreateAllContracts() public {
        (address sequencerChainAddress, address metafillerStorageAddress, address permissionModuleAddress) =
            factory.createAllContracts(admin, manager, l3ChainId);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(metafillerStorageAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        MetafillerStorage metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        RequireAllModule permissionModule = RequireAllModule(permissionModuleAddress);

        // Verify sequencer setup
        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertEq(sequencerChain.l3ChainId(), l3ChainId);

        // Verify metafiller setup
        assertTrue(address(metafillerStorage) == metafillerStorageAddress);
        assertTrue(metafillerStorage.hasRole(metafillerStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(metafillerStorage.hasRole(metafillerStorage.MANAGER_ROLE(), manager));

        // Verify permission module setup
        assertTrue(address(sequencerChain.requirementModule()) == permissionModuleAddress);
        assertTrue(permissionModule.owner() == admin);
    }
}
