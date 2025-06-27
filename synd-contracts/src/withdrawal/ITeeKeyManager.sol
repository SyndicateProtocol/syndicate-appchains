// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface ITeeKeyManager {
    /**
     * @notice Checks if a public key is considered a valid TEE key.
     * @param publicKey The public key to check.
     * @return True if the key is valid, false otherwise.
     */
    function isKeyValid(address publicKey) external view returns (bool);
}
