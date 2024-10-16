// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {L3BackfillMapper} from "src/backfill/L3BackfillMapper.sol";
import {L3BackfillStorage} from "src/backfill/L3BackfillStorage.sol";

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
        address sequencerChainAddress = factory.createMetabasedSequencerChain(l3ChainId);
        assertTrue(sequencerChainAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        assertTrue(address(sequencerChain) == sequencerChainAddress);
    }

    function testCreateBackfillMapper() public {
        address backfillMapperAddress = factory.createBackfillMapper(admin, manager, l3ChainId);
        assertTrue(backfillMapperAddress != address(0));

        L3BackfillMapper backfillMapper = L3BackfillMapper(backfillMapperAddress);
        assertTrue(address(backfillMapper) == backfillMapperAddress);
        assertTrue(backfillMapper.hasRole(backfillMapper.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(backfillMapper.hasRole(backfillMapper.MANAGER_ROLE(), manager));
    }

    function testCreateBackfillStorage() public {
        address backfillStorageAddress = factory.createBackfillStorage(admin, manager, l3ChainId);
        assertTrue(backfillStorageAddress != address(0));

        L3BackfillStorage backfillStorage = L3BackfillStorage(backfillStorageAddress);
        assertTrue(address(backfillStorage) == backfillStorageAddress);
        assertTrue(backfillStorage.hasRole(backfillStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(backfillStorage.hasRole(backfillStorage.MANAGER_ROLE(), manager));
    }

    function testCreateAllContracts() public {
        (address sequencerChainAddress, address backfillMapperAddress, address backfillStorageAddress) =
            factory.createAllContracts(admin, manager, l3ChainId);

        assertTrue(sequencerChainAddress != address(0));
        assertTrue(backfillMapperAddress != address(0));
        assertTrue(backfillStorageAddress != address(0));

        MetabasedSequencerChain sequencerChain = MetabasedSequencerChain(sequencerChainAddress);
        L3BackfillMapper backfillMapper = L3BackfillMapper(backfillMapperAddress);
        L3BackfillStorage backfillStorage = L3BackfillStorage(backfillStorageAddress);

        assertTrue(address(sequencerChain) == sequencerChainAddress);
        assertTrue(address(backfillMapper) == backfillMapperAddress);
        assertTrue(address(backfillStorage) == backfillStorageAddress);

        assertTrue(backfillMapper.hasRole(backfillMapper.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(backfillMapper.hasRole(backfillMapper.MANAGER_ROLE(), manager));
        assertTrue(backfillStorage.hasRole(backfillStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(backfillStorage.hasRole(backfillStorage.MANAGER_ROLE(), manager));
    }
}
