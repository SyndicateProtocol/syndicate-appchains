// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "../interfaces/IPermissionModule.sol";

/// @title NotInitializedModule
/// @notice A module that returns false for all addresses
contract NotInitializedModule is IPermissionModule {
    /// @notice Returns false for all addresses
    function isAllowed(address, address, bytes calldata) external pure override returns (bool) {
        return false;
    }
}
