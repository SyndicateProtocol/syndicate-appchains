// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IsAllowed} from "../interfaces/IsAllowed.sol";

/**
 * @title AlwaysAllowedModule
 * @dev Module used for testing and experimentations. It allows any proposer to send batch txs.
 */
contract AlwaysAllowedModule is IsAllowed {
    /**
     * @notice caller is allowed.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address) external pure override returns (bool) {
        return true;
    }
}
