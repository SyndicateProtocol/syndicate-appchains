// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "./interfaces/IPermissionModule.sol";
import {NotInitializedModule} from "./sequencing-modules/NotInitializedModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title SequencingModuleChecker
/// @notice A contract that delegates permission checks to modular permission systems
/// @dev This separation of concerns allows for flexible permission systems:
/// 1. The SequencingModuleChecker manages the core permission interface
/// 2. The permissionRequirementModule (typically RequireAndModule or RequireOrModule) handles the actual permission logic
/// 3. This design allows for complex permission structures (AND/OR logic) that can be upgraded over time
/// 4. The initialization pattern allows for proper setup of the permission system after deployment
abstract contract SequencingModuleChecker is Ownable, IPermissionModule {
    /// @notice The requirement module that handles checks
    IPermissionModule public permissionRequirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error TransactionOrSenderNotAllowed();
    error AlreadyInitialized();

    bool internal hasBeenInitialized = false;

    /// @dev Constructor function
    // [Olympix Warning: no parameter validation in constructor] Admin validation handled by OpenZeppelin's Ownable
    constructor() Ownable(msg.sender) {
        permissionRequirementModule = new NotInitializedModule();
    }

    /// @notice Initializes the contract with a new admin and a requirement module
    /// @dev This two-step initialization process allows for proper setup of the contract:
    /// 1. First deployed with a temporary admin (deployer)
    /// 2. Then initialized with the permanent admin and requirement module
    /// 3. Once initialized, it cannot be re-initialized (immutable pattern)
    /// @param admin The address of the new admin
    /// @param _permissionRequirementModule The address of the RequireAll or RequireAny module
    function initialize(address admin, address _permissionRequirementModule) external onlyOwner {
        if (hasBeenInitialized) revert AlreadyInitialized();
        if (_permissionRequirementModule == address(0)) revert InvalidModuleAddress();

        hasBeenInitialized = true;

        permissionRequirementModule = IPermissionModule(_permissionRequirementModule);

        transferOwnership(admin);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        permissionRequirementModule = IPermissionModule(_newModule);

        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Modifier to check if an address is allowed to submit txs based on the sender, origin and data
    /// @param msgSender The address calling the function (msg.sender)
    /// @param txOrigin The address that initiated the transaction (tx.origin)
    /// @param data The calldata to check
    modifier onlyWhenAllowed(address msgSender, address txOrigin, bytes calldata data) {
        if (!isAllowed(msgSender, txOrigin, data)) revert TransactionOrSenderNotAllowed();
        _;
    }

    /// @notice Checks if both the proposer and calldata are allowed
    /// @param proposer The address to check
    /// @param originator The address of tx.origin.
    /// @param data The calldata to check
    /// @return bool indicating if both the proposer and calldata are allowed
    function isAllowed(address proposer, address originator, bytes calldata data) public view returns (bool) {
        return permissionRequirementModule.isAllowed(proposer, originator, data); //#olympix-ignore-calls-in-loop
    }
}
