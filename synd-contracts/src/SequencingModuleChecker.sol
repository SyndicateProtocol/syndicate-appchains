// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "./interfaces/IPermissionModule.sol";
import {NotInitializedModule} from "./sequencing-modules/NotInitializedModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {DataTooLarge} from "@arbitrum/nitro-contracts/src/libraries/Error.sol";

/// @title SequencingModuleChecker
/// @notice A contract that delegates permission checks to modular permission systems
/// @dev This separation of concerns allows for flexible permission systems:
/// 1. The SequencingModuleChecker manages the core permission interface
/// 2. The permissionRequirementModule (typically RequireAndModule or RequireOrModule) handles the actual permission logic
/// 3. This design allows for complex permission structures (AND/OR logic) that can be upgraded over time
/// 4. Proper setup of the permission system after deployment involves setting a sequencing module and transferring ownership
///    via the Ownable transferOwnership() function
abstract contract SequencingModuleChecker is Ownable, IPermissionModule {
    // Just in case, limit the amount of tx data sent to the isAllowed function.
    uint256 public constant maxDataSize = 200000;

    /// @notice The requirement module that handles checks
    IPermissionModule public permissionRequirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    /// @dev Constructor function
    // [Olympix Warning: no parameter validation in constructor] Admin validation handled by OpenZeppelin's Ownable
    constructor() Ownable(msg.sender) {}

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    /// Note that the zero address is allowed and corresponds to a forbid all module which reverts.
    /// All addresses without code cause all transaction types to be forbidden by reverting.
    /// Address one corresponds to the always allowed module.
    function updateRequirementModule(address _newModule) external onlyOwner {
        permissionRequirementModule = IPermissionModule(_newModule);

        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if both the proposer and calldata are allowed
    /// @param proposer The address to check
    /// @param originator The address of tx.origin.
    /// @param data The calldata to check
    /// @return bool indicating if both the proposer and calldata are allowed
    function isAllowed(address proposer, address originator, bytes memory data) public view returns (bool) {
        require(data.length <= maxDataSize, DataTooLarge(data.length, maxDataSize));
        return address(permissionRequirementModule) == address(1)
            || permissionRequirementModule.isAllowed(proposer, originator, data); //#olympix-ignore-calls-in-loop
    }
}
