// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ConsolidatedPermissionModule} from "./interfaces/ConsolidatedPermissionModule.sol";
import {NotInitializedModule} from "./sequencing-modules/NotInitializedModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title OptionOneSequencingModuleChecker
/// @notice A contract that delegates permission checks to a single module for both proposer and calldata validation
abstract contract OptionOneSequencingModuleChecker is Ownable {
    /// @notice The module that validates both proposers and calldata
    ConsolidatedPermissionModule public requirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error NotAllowed(address batchSubmitter);
    error CalldataNotAllowed();
    error AlreadyInitialized();

    bool internal hasBeenInitialized = false;

    /// @dev Constructor function
    constructor() Ownable(msg.sender) {}

    /// @notice Initializes the contract with a new admin and requirement module
    /// @param admin The address of the new admin
    /// @param _requirementModule The address of the requirement module
    function initialize(address admin, address _requirementModule) external onlyOwner {
        if (hasBeenInitialized) revert AlreadyInitialized();
        if (_requirementModule == address(0)) revert InvalidModuleAddress();

        hasBeenInitialized = true;
        requirementModule = ConsolidatedPermissionModule(_requirementModule);
        transferOwnership(admin);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        requirementModule = ConsolidatedPermissionModule(_newModule);
        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if a proposer is allowed to submit transactions
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the proposer is allowed
    function isAllowed(address proposer) public view returns (bool) {
        return requirementModule.isAllowed(proposer, "");
    }

    /// @notice Checks if the calldata is allowed
    /// @param data The calldata to check
    /// @return bool indicating if the calldata is allowed
    function isCalldataAllowed(bytes calldata data) public view returns (bool) {
        return requirementModule.isAllowed(address(0), data);
    }

    /// @notice Checks if both the proposer and calldata are allowed
    /// @param proposer The address to check
    /// @param data The calldata to check
    /// @return bool indicating if both the proposer and calldata are allowed
    function isAllowedWithCalldata(address proposer, bytes calldata data) public view returns (bool) {
        return requirementModule.isAllowed(proposer, data);
    }

    /// @notice Modifier to revert if the proposer is not allowed
    /// @param proposer The address to check
    modifier onlyWhenAllowed(address proposer) {
        if (!isAllowed(proposer)) revert NotAllowed(proposer);
        _;
    }

    /// @notice Modifier to revert if the calldata is not allowed
    /// @param data The calldata to check
    modifier revertForUnallowedCalldata(bytes calldata data) {
        _revertForUnallowedCalldata(data);
        _;
    }

    /// @notice Revert if the calldata is not allowed
    /// @param data The calldata to check
    function _revertForUnallowedCalldata(bytes calldata data) internal view {
        if (!isCalldataAllowed(data)) revert CalldataNotAllowed();
    }
}
