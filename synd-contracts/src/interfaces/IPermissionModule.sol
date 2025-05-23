// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface IPermissionModule {
    /**
     * @notice Checks if a transaction sender is allowed.
     * @param msgSender The address that called the function (msg.sender).
     * @param txOrigin The address that initiated the transaction (tx.origin).
     * @param data The calldata to be checked.
     * @return bool indicating if the transaction is allowed.
     */
    function isAllowed(address msgSender, address txOrigin, bytes calldata data) external view returns (bool);
}
