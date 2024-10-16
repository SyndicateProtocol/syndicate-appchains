// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {L3BackfillMapper} from "./backfill/L3BackfillMapper.sol";
import {L3BackfillStorage} from "./backfill/L3BackfillStorage.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChain, L3BackfillMapper, and L3BackfillStorage instances
contract MetabasedFactory {
    /// @notice Emitted when a new MetabasedSequencerChain is created
    event MetabasedSequencerChainCreated(uint256 indexed l3ChainId, address indexed metabasedSequencerChainAddress);

    /// @notice Emitted when a new L3BackfillMapper is created
    event BackfillMapperCreated(uint256 indexed l3ChainId, address indexed backfillMapperAddress);

    /// @notice Emitted when a new L3BackfillStorage is created
    event BackfillStorageCreated(uint256 indexed l3ChainId, address indexed backfillStorageAddress);

    /// @notice Emitted when all three contracts are created at once
    event AllContractsCreated(
        uint256 l3ChainId,
        address indexed sequencerChainAddress,
        address indexed backfillMapperAddress,
        address indexed backfillStorageAddress
    );

    /// @notice Creates a new MetabasedSequencerChain contract
    /// @param l3ChainId the l3 chain the contract referes to
    /// @return The address of the newly created MetabasedSequencerChain contract
    function createMetabasedSequencerChain(uint256 l3ChainId) public returns (address) {
        MetabasedSequencerChain newMetabasedSequencerChain = new MetabasedSequencerChain(l3ChainId);

        emit MetabasedSequencerChainCreated(l3ChainId, address(newMetabasedSequencerChain));

        return address(newMetabasedSequencerChain);
    }

    /// @notice Creates a new L3BackfillMapper contract
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId The L3 chain ID
    /// @return The address of the newly created L3BackfillMapper contract
    function createBackfillMapper(address admin, address manager, uint256 l3ChainId) public returns (address) {
        L3BackfillMapper newBackfillMapper = new L3BackfillMapper(admin, manager, l3ChainId);

        emit BackfillMapperCreated(l3ChainId, address(newBackfillMapper));

        return address(newBackfillMapper);
    }

    /// @notice Creates a new L3BackfillStorage contract
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId The L3 chain ID
    /// @return The address of the newly created L3BackfillStorage contract
    function createBackfillStorage(address admin, address manager, uint256 l3ChainId) public returns (address) {
        L3BackfillStorage newBackfillStorage = new L3BackfillStorage(admin, manager, l3ChainId);

        emit BackfillStorageCreated(l3ChainId, address(newBackfillStorage));

        return address(newBackfillStorage);
    }

    /// @notice Creates all three contracts: MetabasedSequencerChain, L3BackfillMapper, and L3BackfillStorage
    /// @param admin The address that will be the default admin role for L3BackfillMapper and L3BackfillStorage
    /// @param manager The address that will be the manager role for L3BackfillMapper and L3BackfillStorage
    /// @param l3ChainId The L3 chain ID
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain contract
    /// @return backfillMapper The address of the newly created L3BackfillMapper contract
    /// @return backfillStorage The address of the newly created L3BackfillStorage contract
    function createAllContracts(address admin, address manager, uint256 l3ChainId)
        public
        returns (address sequencerChain, address backfillMapper, address backfillStorage)
    {
        sequencerChain = createMetabasedSequencerChain(l3ChainId);
        backfillMapper = createBackfillMapper(admin, manager, l3ChainId);
        backfillStorage = createBackfillStorage(admin, manager, l3ChainId);

        emit AllContractsCreated(l3ChainId, sequencerChain, backfillMapper, backfillStorage);

        return (sequencerChain, backfillMapper, backfillStorage);
    }
}
