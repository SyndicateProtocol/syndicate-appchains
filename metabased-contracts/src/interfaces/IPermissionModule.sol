// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

interface IPermissionModule {
    /**
     * @notice Checks if the caller is allowed.
     * @param proposer The address of proposed to be checked.
     * @param originator The address of tx.origin. Useful to know the sender originator in wrapper contracts
     * @param data The calldata to be checked.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address proposer, address originator, bytes calldata data) external view returns (bool);
}
