// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IsAllowed} from "./interfaces/IsAllowed.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @title SequencingModuleChecker
/// @notice A simplified contract that delegates permission checks to a master module
abstract contract SequencingModuleChecker is Ownable, IsAllowed {
    /// @notice The master permission module that handles all checks
    IsAllowed public masterPermissionModule;

    event MasterModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();

    /// @dev Constructor function
    /// @param admin The address that will be set as the admin
    /// @param _masterModule The address of the master permission module
    constructor(address admin, address _masterModule) Ownable(admin) {
        if (_masterModule == address(0)) revert InvalidModuleAddress();
        masterPermissionModule = IsAllowed(_masterModule);
    }

    /// @notice Updates the master permission module
    /// @param _newModule The address of the new master module
    function updateMasterModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        masterPermissionModule = IsAllowed(_newModule);
        emit MasterModuleUpdated(_newModule);
    }

    /// @notice Checks if an address is allowed to submit batches
    /// @param batchSubmitter The address to check
    modifier onlyWhenAllowed(address batchSubmitter) {
        isAllowed(batchSubmitter);
        _;
    }

    /// @notice Implementation of the IsAllowed interface
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the address is allowed
    function isAllowed(address proposer) public view virtual override returns (bool) {
        return masterPermissionModule.isAllowed(proposer);
    }
}
