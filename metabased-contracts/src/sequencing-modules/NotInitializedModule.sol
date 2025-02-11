// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "../interfaces/PermissionModule.sol";

contract NotInitializedModule is PermissionModule {
    /// @notice Returns false for all addresses
    function isAllowed(address) external pure override returns (bool) {
        return false;
    }
}
