// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "../interfaces/IPermissionModule.sol";

/**
 * @title AlwaysAllowedModule
 * @dev Module used for testing and experimentations. It allows any proposer to send batch data.
 */
contract AlwaysAllowedModule is IPermissionModule {
    /**
     * @notice caller is allowed.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address, address, bytes calldata) external pure override returns (bool) {
        return true;
    }
}
