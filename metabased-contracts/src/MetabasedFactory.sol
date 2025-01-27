// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {MetafillerStorage} from "./backfill/MetafillerStorage.sol";
import {RequireAllModule} from "./requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

/// @title MetabasedFactory // Sample change TODO: remove 
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

    error ZeroAddress();
    error ZeroValue();

    modifier zeroValuesNotAllowed(uint256 l3ChainId, address firstAddrCheck, address secondAddrCheck) {
        if (l3ChainId == 0) {
            revert ZeroValue();
        }
        if (firstAddrCheck == address(0) || secondAddrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    /// @notice Creates a new MetabasedSequencerChain contract with a permission module
    /// @param l3ChainId the l3 chain the contract refers to
    /// @param admin The address that will be the admin
    /// @param permissionModule The address of the permission module
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    function createMetabasedSequencerChain(uint256 l3ChainId, address admin, IRequirementModule permissionModule)
        public
        zeroValuesNotAllowed(l3ChainId, admin, address(permissionModule))
        returns (address sequencerChain)
    {
        MetabasedSequencerChain newSequencerChain =
            new MetabasedSequencerChain(l3ChainId, admin, address(permissionModule));

        emit MetabasedSequencerChainCreated(l3ChainId, address(newSequencerChain), address(permissionModule));

        return address(newSequencerChain);
    }

    /// @notice Creates a new MetafillerStorage contract
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId The L3 chain ID
    /// @return The address of the newly created MetafillerStorage contract
    function createMetafillerStorage(address admin, address manager, uint256 l3ChainId)
        public
        zeroValuesNotAllowed(l3ChainId, admin, manager)
        returns (address)
    {
        MetafillerStorage newMetafillerStorage = new MetafillerStorage(admin, manager, l3ChainId);
        emit MetafillerStorageCreated(l3ChainId, address(newMetafillerStorage));
        return address(newMetafillerStorage);
    }

    /// @notice Creates all contracts: MetabasedSequencerChain, RequireAllModule, and MetafillerStorage
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role for MetafillerStorage
    /// @param l3ChainId The L3 chain ID
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return metafillerStorage The address of the newly created MetafillerStorage
    /// @return permissionModule The address of the newly created RequireAllModule
    function createAllContractsWithRequireAllModule(address admin, address manager, uint256 l3ChainId)
        public
        zeroValuesNotAllowed(l3ChainId, admin, manager)
        returns (address sequencerChain, address metafillerStorage, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAllModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(l3ChainId, admin, permissionModule);
        metafillerStorage = createMetafillerStorage(admin, manager, l3ChainId);

        emit AllContractsCreated(l3ChainId, sequencerChain, metafillerStorage, address(permissionModule));

        return (sequencerChain, metafillerStorage, permissionModule);
    }

    /// @notice Creates all contracts: MetabasedSequencerChain, RequireAnyModule, and MetafillerStorage
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role for MetafillerStorage
    /// @param l3ChainId The L3 chain ID
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return metafillerStorage The address of the newly created MetafillerStorage
    /// @return permissionModule The address of the newly created RequireAnyModule
    function createAllContractsWithRequireAnyModule(address admin, address manager, uint256 l3ChainId)
        public
        zeroValuesNotAllowed(l3ChainId, admin, manager)
        returns (address sequencerChain, address metafillerStorage, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAnyModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(l3ChainId, admin, permissionModule);
        metafillerStorage = createMetafillerStorage(admin, manager, l3ChainId);

        emit AllContractsCreated(l3ChainId, sequencerChain, metafillerStorage, address(permissionModule));

        return (sequencerChain, metafillerStorage, permissionModule);
    }
}
