// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule_ReleaseCandidate} from "./interfaces/PermissionModule_ReleaseCandidate.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @title SequencingModuleChecker
/// @notice A simplified contract that delegates permission checks to modules
abstract contract SequencingModuleChecker_ReleaseCandidate is Ownable, PermissionModule_ReleaseCandidate {
    /// @notice The requirement module that handles checks
    PermissionModule_ReleaseCandidate public requirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();

    /// @dev Constructor function
    /// @param admin The address that will be set as the admin
    /// @param _requirementModule The address of the requirement module
    constructor(address admin, address _requirementModule) Ownable(admin) {
        if (_requirementModule == address(0)) revert InvalidModuleAddress();
        requirementModule = PermissionModule_ReleaseCandidate(_requirementModule);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        requirementModule = PermissionModule_ReleaseCandidate(_newModule);
        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if an address is allowed to submit batches
    /// @param batchSubmitter The address to check
    modifier onlyWhenAllowed(address batchSubmitter) {
        isAllowed(batchSubmitter);
        _;
    }

    /// @notice Implementation of the PermissionModule interface
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the address is allowed
    function isAllowed(address proposer) public view virtual override returns (bool) {
        return requirementModule.isAllowed(proposer);
    }
}
