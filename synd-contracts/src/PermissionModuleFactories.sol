// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RequireAndModule} from "./requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "./requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "./requirement-modules/RequireCompositeModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

/// @title RequireAndModuleFactory
/// @notice Factory for deploying RequireAndModule contracts
contract RequireAndModuleFactory {
    /// @notice Emitted when a new RequireAndModule is created
    event RequireAndModuleCreated(address indexed module, address indexed admin);

    error ZeroAddress();

    /// @notice Deploy a new RequireAndModule
    /// @param admin The admin address for the module
    /// @param salt The salt for CREATE2 deployment
    /// @return module The deployed module address
    function createRequireAndModule(address admin, bytes32 salt) external returns (address module) {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireAndModule).creationCode, abi.encode(admin));

        module = Create2.deploy(0, salt, bytecode);

        emit RequireAndModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address
    /// @param salt The salt for CREATE2 deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireAndModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }
}

/// @title RequireOrModuleFactory
/// @notice Factory for deploying RequireOrModule contracts
contract RequireOrModuleFactory {
    /// @notice Emitted when a new RequireOrModule is created
    event RequireOrModuleCreated(address indexed module, address indexed admin);

    error ZeroAddress();

    /// @notice Deploy a new RequireOrModule
    /// @param admin The admin address for the module
    /// @param salt The salt for CREATE2 deployment
    /// @return module The deployed module address
    function createRequireOrModule(address admin, bytes32 salt) external returns (address module) {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireOrModule).creationCode, abi.encode(admin));

        module = Create2.deploy(0, salt, bytecode);

        emit RequireOrModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address
    /// @param salt The salt for CREATE2 deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireOrModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }
}

/// @title RequireCompositeModuleFactory
/// @notice Factory for deploying RequireCompositeModule contracts
contract RequireCompositeModuleFactory {
    /// @notice Emitted when a new RequireCompositeModule is created
    event RequireCompositeModuleCreated(address indexed module, address indexed admin);

    error ZeroAddress();

    /// @notice Deploy a new RequireCompositeModule
    /// @param admin The admin address for the module
    /// @param salt The salt for CREATE2 deployment
    /// @return module The deployed module address
    function createRequireCompositeModule(address admin, bytes32 salt) external returns (address module) {
        if (admin == address(0)) revert ZeroAddress();

        bytes memory bytecode = abi.encodePacked(type(RequireCompositeModule).creationCode, abi.encode(admin));

        module = Create2.deploy(0, salt, bytecode);

        emit RequireCompositeModuleCreated(module, admin);
        return module;
    }

    /// @notice Compute the address where a module will be deployed
    /// @param admin The admin address
    /// @param salt The salt for CREATE2 deployment
    /// @return The computed address
    function computeModuleAddress(address admin, bytes32 salt) external view returns (address) {
        bytes memory bytecode = abi.encodePacked(type(RequireCompositeModule).creationCode, abi.encode(admin));
        return Create2.computeAddress(salt, keccak256(bytecode));
    }
}
