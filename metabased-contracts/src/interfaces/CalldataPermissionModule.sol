// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

interface CalldataPermissionModule {
    /**
     * @notice Checks if the calldata is allowed.
     * @param data The calldata to be checked.
     * @return bool indicating if the calldata is allowed.
     */
    function isCalldataAllowed(bytes calldata data) external view returns (bool);
}
