// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {CalldataPermissionModule} from "../../interfaces/CalldataPermissionModule.sol";

/**
 * @title CalldataAlwaysAllowedModule
 * @dev It allows any calldata to be submitted.
 */
contract CalldataAlwaysAllowedModule is CalldataPermissionModule {
    /**
     * @notice calldata is allowed.
     * @return bool indicating whether calldata is allowed.
     */
    function isCalldataAllowed(bytes calldata) external pure override returns (bool) {
        return true;
    }
}
