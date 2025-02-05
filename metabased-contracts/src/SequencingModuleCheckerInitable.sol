// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "./interfaces/PermissionModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title SequencingModuleCheckerInitable
/// @notice A simplified contract that delegates permission checks to modules
abstract contract SequencingModuleCheckerInitable is Ownable, PermissionModule {
    /// @notice The requirement module that handles checks
    PermissionModule public requirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error NotAllowed(address batchSubmitter);

    constructor() Ownable(msg.sender) {}

    function init(address admin, address _requirementModule) public virtual onlyOwner {
        if (_requirementModule == address(0)) revert InvalidModuleAddress();
        requirementModule = PermissionModule(_requirementModule);

        transferOwnership(admin);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        requirementModule = PermissionModule(_newModule);
        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if an address is allowed to submit batches
    /// @param batchSubmitter The address to check
    modifier onlyWhenAllowed(address batchSubmitter) {
        if (!isAllowed(batchSubmitter)) revert NotAllowed(batchSubmitter);
        _;
    }

    /// @notice Implementation of the PermissionModule interface
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the address is allowed
    function isAllowed(address proposer) public view virtual override returns (bool) {
        return requirementModule.isAllowed(proposer);
    }
}
