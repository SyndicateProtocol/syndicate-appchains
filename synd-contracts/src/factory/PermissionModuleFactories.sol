// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RequireAndModule} from "../requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "../requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "../requirement-modules/RequireCompositeModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

/// @title RequireAndModuleFactory
/// @notice Factory for deploying RequireAndModule contracts using CREATE2 pattern
contract RequireAndModuleFactory is AccessControl, Pausable {
    /// @notice Emitted when a new RequireAndModule is created
    event RequireAndModuleCreated(address indexed module, address indexed admin);

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);
    }

    /// @notice Deploy a new RequireAndModule using CREATE2
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return module The deployed module address
    function createRequireAndModule(address admin, bytes32 salt) external whenNotPaused returns (address module) {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireAndModule).creationCode, abi.encode(admin));
        module = Create2.deploy(0, salt, bytecode);

        emit RequireAndModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireAndModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}

/// @title RequireOrModuleFactory
/// @notice Factory for deploying RequireOrModule contracts using CREATE2 pattern
contract RequireOrModuleFactory is AccessControl, Pausable {
    /// @notice Emitted when a new RequireOrModule is created
    event RequireOrModuleCreated(address indexed module, address indexed admin);

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);
    }

    /// @notice Deploy a new RequireOrModule using CREATE2
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return module The deployed module address
    function createRequireOrModule(address admin, bytes32 salt) external whenNotPaused returns (address module) {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireOrModule).creationCode, abi.encode(admin));
        module = Create2.deploy(0, salt, bytecode);

        emit RequireOrModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireOrModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}

/// @title RequireCompositeModuleFactory
/// @notice Factory for deploying RequireCompositeModule contracts using CREATE2 pattern
contract RequireCompositeModuleFactory is AccessControl, Pausable {
    /// @notice Emitted when a new RequireCompositeModule is created
    event RequireCompositeModuleCreated(address indexed module, address indexed admin);

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);
    }

    /// @notice Deploy a new RequireCompositeModule using CREATE2
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return module The deployed module address
    function createRequireCompositeModule(address admin, bytes32 salt)
        external
        whenNotPaused
        returns (address module)
    {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireCompositeModule).creationCode, abi.encode(admin));
        module = Create2.deploy(0, salt, bytecode);

        emit RequireCompositeModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address for the module
    /// @param salt The salt for deterministic deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireCompositeModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
