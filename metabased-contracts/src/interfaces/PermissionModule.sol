// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

interface PermissionModule {
    // TODO: Evaluate whether this should be a view function or not. You could
    // imagine isAllowed() checks that also modify some state

    /**
     * @notice Checks if the caller is allowed.
     * @param proposer The address of proposed to be checked.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address proposer) external view returns (bool);
}
