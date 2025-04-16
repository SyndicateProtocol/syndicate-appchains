// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ProposerPermissionModule} from "../interfaces/ProposerPermissionModule.sol";

/// @title NotInitializedModule
/// @notice A module that returns false for all addresses
contract NotInitializedModule is ProposerPermissionModule {
    /// @notice Returns false for all addresses
    function isAllowed(address) external pure override returns (bool) {
        return false;
    }
}
