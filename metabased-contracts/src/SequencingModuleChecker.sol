// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ProposerPermissionModule} from "./interfaces/ProposerPermissionModule.sol";

import {CalldataPermissionModule} from "./interfaces/CalldataPermissionModule.sol";

import {NotInitializedModule} from "./sequencing-modules/NotInitializedModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title SequencingModuleChecker
/// @notice A simplified contract that delegates permission checks to modules

abstract contract SequencingModuleChecker is Ownable, ProposerPermissionModule {
    /// @notice The requirement module that handles checks for proposer
    ProposerPermissionModule public proposerRequirementModule;

    /// @notice The requirement module that handles checks for calldata;
    CalldataPermissionModule public calldataRequirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error NotAllowed(address batchSubmitter);
    error CalldataNotAllowed();
    error AlreadyInitialized();

    bool internal hasBeenInitialized = false;

    /// @dev Constructor function
    // [Olympix Warning: no parameter validation in constructor] Admin validation handled by OpenZeppelin's Ownable
    constructor() Ownable(msg.sender) {
        proposerRequirementModule = new NotInitializedModule();
    }

    /// @notice Initializes the contract with a new admin and a requirement module
    /// @param admin The address of the new admin
    /// @param _requirementModule The address of the RequireAll or RequireAny module
    function initialize(address admin, address _requirementModule) external onlyOwner {
        if (hasBeenInitialized) revert AlreadyInitialized();
        if (_requirementModule == address(0)) revert InvalidModuleAddress();
        hasBeenInitialized = true;
        proposerRequirementModule = ProposerPermissionModule(_requirementModule);
        calldataRequirementModule = CalldataPermissionModule(_requirementModule);
        transferOwnership(admin);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        proposerRequirementModule = ProposerPermissionModule(_newModule);
        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if an address is allowed to submit batches
    /// @param batchSubmitter The address to check
    modifier onlyWhenAllowed(address batchSubmitter) {
        if (!isAllowed(batchSubmitter)) revert NotAllowed(batchSubmitter);
        _;
    }

    /// @notice Implementation of the ProposerPermissionModule interface
    /// @param proposer The address to check permissions for
    /// @return bool indicating if the address is allowed
    function isAllowed(address proposer) public view override returns (bool) {
        return proposerRequirementModule.isAllowed(proposer);
    }

    /// @notice Checks if the calldata is allowed
    /// @param data The calldata to check
    /// @return bool indicating if the calldata is allowed
    function isCalldataAllowed(bytes calldata data) public view returns (bool) {
        return calldataRequirementModule.isCalldataAllowed(data);
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
