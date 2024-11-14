// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {MetafillerStorage} from "./backfill/MetafillerStorage.sol";
import {RequirementChainModule} from "./RequirementChainModule.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChain and related contracts
contract MetabasedFactory {
    /// @notice Emitted when a new MetabasedSequencerChain is created
    event MetabasedSequencerChainCreated(
        uint256 indexed l3ChainId,
        address indexed metabasedSequencerChainAddress,
        address indexed permissionModuleAddress
    );

    /// @notice Emitted when a new MetafillerStorage is created
    event MetafillerStorageCreated(uint256 indexed l3ChainId, address indexed metafillerStorageAddress);

    /// @notice Emitted when all contracts are created at once
    event AllContractsCreated(
        uint256 l3ChainId,
        address indexed sequencerChainAddress,
        address indexed metafillerStorageAddress,
        address indexed permissionModuleAddress
    );

    /// @notice Creates a new MetabasedSequencerChain contract with its permission module
    /// @param l3ChainId the l3 chain the contract refers to
    /// @param admin The address that will be the admin
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return permissionModule The address of the newly created RequirementChainModule
    function createMetabasedSequencerChain(uint256 l3ChainId, address admin)
        public
        returns (address sequencerChain, address permissionModule)
    {
        RequirementChainModule newPermissionModule = new RequirementChainModule(admin);
        MetabasedSequencerChain newSequencerChain =
            new MetabasedSequencerChain(l3ChainId, admin, address(newPermissionModule));

        emit MetabasedSequencerChainCreated(l3ChainId, address(newSequencerChain), address(newPermissionModule));

        return (address(newSequencerChain), address(newPermissionModule));
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

    /// @notice Creates all contracts: MetabasedSequencerChain, RequirementChainModule, and MetafillerStorage
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role for MetafillerStorage
    /// @param l3ChainId The L3 chain ID
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return metafillerStorage The address of the newly created MetafillerStorage
    /// @return permissionModule The address of the newly created RequirementChainModule
    function createAllContracts(address admin, address manager, uint256 l3ChainId)
        public
        returns (address sequencerChain, address metafillerStorage, address permissionModule)
    {
        (sequencerChain, permissionModule) = createMetabasedSequencerChain(l3ChainId, admin);
        metafillerStorage = createMetafillerStorage(admin, manager, l3ChainId);

        emit AllContractsCreated(l3ChainId, sequencerChain, metafillerStorage, permissionModule);

        return (sequencerChain, metafillerStorage, permissionModule);
    }
}
