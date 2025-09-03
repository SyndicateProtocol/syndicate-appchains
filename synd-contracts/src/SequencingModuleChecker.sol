// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "./interfaces/IPermissionModule.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {DataTooLarge} from "@arbitrum/nitro-contracts/src/libraries/Error.sol";

/// @custom:storage-location erc7201:syndicate.storage.SequencingModule
struct SequencingModuleStorage {
    /// @notice The requirement module that handles checks
    IPermissionModule permissionRequirementModule;
}

/// @title SequencingModuleChecker
/// @notice A contract that delegates permission checks to modular permission systems
/// @dev This separation of concerns allows for flexible permission systems:
/// 1. The SequencingModuleChecker manages the core permission interface
/// 2. The permissionRequirementModule (typically RequireAndModule or RequireOrModule) handles the actual permission logic
/// 3. This design allows for complex permission structures (AND/OR logic) that can be upgraded over time
/// 4. The initialization pattern allows for proper setup of the permission system after deployment
abstract contract SequencingModuleChecker is Initializable, OwnableUpgradeable, IPermissionModule {
    event RequirementModuleUpdated(address indexed newModule);

    // Just in case, limit the amount of tx data sent to the isAllowed function.
    uint256 public constant maxDataSize = 200000;

    // cast index-erc7201 syndicate.storage.SequencingModule
    bytes32 public constant SEQUENCING_MODULE_STORAGE_LOCATION =
        0x5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500;

    function _getSequencingModuleStorage() private pure returns (SequencingModuleStorage storage $) {
        assembly {
            $.slot := SEQUENCING_MODULE_STORAGE_LOCATION
        }
    }

    function permissionRequirementModule() public view returns (IPermissionModule) {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        return $.permissionRequirementModule;
    }

    function __SequencingModuleChecker_init(address admin, address _permissionRequirementModule)
        internal
        onlyInitializing
    {
        __Ownable_init(admin);
        _getSequencingModuleStorage().permissionRequirementModule = IPermissionModule(_permissionRequirementModule);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    /// Note that the zero address is allowed and corresponds to a forbid all module which reverts.
    /// All addresses without code cause all transaction types to be forbidden by reverting.
    /// Address one corresponds to the always allowed module.
    function updateRequirementModule(address _newModule) external onlyOwner {
        _getSequencingModuleStorage().permissionRequirementModule = IPermissionModule(_newModule);

        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if both the proposer and calldata are allowed
    /// @param proposer The address to check
    /// @param originator The address of tx.origin.
    /// @param data The calldata to check
    /// @return bool indicating if both the proposer and calldata are allowed
    function isAllowed(address proposer, address originator, bytes memory data) public view returns (bool) {
        require(data.length <= maxDataSize, DataTooLarge(data.length, maxDataSize));
        return address(permissionRequirementModule()) == address(1)
            || permissionRequirementModule().isAllowed(proposer, originator, data); //#olympix-ignore-calls-in-loop
    }
}
