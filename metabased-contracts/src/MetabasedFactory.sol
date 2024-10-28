// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {MetafillerStorage} from "./backfill/MetafillerStorage.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChain and MetafillerStorage instances
contract MetabasedFactory {
    /// @notice Emitted when a new MetabasedSequencerChain is created
    event MetabasedSequencerChainCreated(uint256 indexed l3ChainId, address indexed metabasedSequencerChainAddress);

    /// @notice Emitted when a new MetafillerStorage is created
    event MetafillerStorageCreated(uint256 indexed l3ChainId, address indexed metafillerStorageAddress);

    /// @notice Emitted when all three contracts are created at once
    event AllContractsCreated(
        uint256 l3ChainId, address indexed sequencerChainAddress, address indexed metafillerStorageAddress
    );

    /// @notice Creates a new MetabasedSequencerChain contract
    /// @param l3ChainId the l3 chain the contract referes to
    /// @param admin The address that will be the admin
    /// @return The address of the newly created MetabasedSequencerChain contract
    function createMetabasedSequencerChain(uint256 l3ChainId, address admin) public returns (address) {
        MetabasedSequencerChain newMetabasedSequencerChain = new MetabasedSequencerChain(l3ChainId, admin);

        emit MetabasedSequencerChainCreated(l3ChainId, address(newMetabasedSequencerChain));

        return address(newMetabasedSequencerChain);
    }

    /// @notice Creates a new MetafillerStorage contract
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId The L3 chain ID
    /// @return The address of the newly created MetafillerStorage contract
    function createMetafillerStorage(address admin, address manager, uint256 l3ChainId) public returns (address) {
        MetafillerStorage newMetafillerStorage = new MetafillerStorage(admin, manager, l3ChainId);

        emit MetafillerStorageCreated(l3ChainId, address(newMetafillerStorage));

        return address(newMetafillerStorage);
    }

    /// @notice Creates all contracts: MetabasedSequencerChain and MetafillerStorage
    /// @param admin The address that will be the default admin role for MetafillerStorage
    /// @param manager The address that will be the manager role for MetafillerStorage
    /// @param l3ChainId The L3 chain ID
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain contract
    /// @return metafillerStorage The address of the newly created MetafillerStorage contract
    function createAllContracts(address admin, address manager, uint256 l3ChainId)
        public
        returns (address sequencerChain, address metafillerStorage)
    {
        sequencerChain = createMetabasedSequencerChain(l3ChainId, admin);
        metafillerStorage = createMetafillerStorage(admin, manager, l3ChainId);

        emit AllContractsCreated(l3ChainId, sequencerChain, metafillerStorage);

        return (sequencerChain, metafillerStorage);
    }
}
