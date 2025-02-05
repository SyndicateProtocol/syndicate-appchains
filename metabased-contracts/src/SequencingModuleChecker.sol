// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "./interfaces/PermissionModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
/// @title SequencingModuleChecker
/// @notice A simplified contract that delegates permission checks to modules

abstract contract SequencingModuleChecker is Ownable, PermissionModule, Initializable {
    /// @notice The requirement module that handles checks
    PermissionModule public requirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error NotAllowed(address batchSubmitter);

    /// @dev Constructor function
    // [Olympix Warning: no parameter validation in constructor] Admin validation handled by OpenZeppelin's Ownable
    constructor() Ownable(msg.sender) {}

    /// @notice Initializes the contract by a requirement module and new admin
    /// @param admin The address of the admin
    /// @param _requirementModule The address of the requirement module
    function initialize(address admin, address _requirementModule) external initializer onlyOwner {
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
        // Contract must be initialized before allowed
        require(_getInitializedVersion() > 0, "SequencingModuleChecker is initializing");
        return requirementModule.isAllowed(proposer);
    }
}
