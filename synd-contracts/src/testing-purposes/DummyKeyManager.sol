// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ITeeKeyManager} from "src/withdrawal/ITeeKeyManager.sol";

contract DummyKeyManager is ITeeKeyManager {
    using EnumerableSet for EnumerableSet.AddressSet;

    event KeyAdded(address indexed key);
    event KeysRevoked();

    EnumerableSet.AddressSet internal validKeys;

    /**
     * @notice Checks if a public key is considered a valid TEE key.
     * @param publicKey The public key to check.
     * @return True if the key is valid (i.e., marked as valid in `keyValidity`), false otherwise.
     */
    function isKeyValid(address publicKey) external view override returns (bool) {
        return validKeys.contains(publicKey);
    }

    function addKey(address publicKey) external {
        bool added = validKeys.add(publicKey);
        require(added, "TeeKeyManager: Key already exists or failed to add");
        emit KeyAdded(publicKey);
    }

    function revokeAllKeys() public {
        address[] memory keys = validKeys.values();
        for (uint256 i = 0; i < keys.length; i++) {
            require(validKeys.remove(keys[i]), "TeeKeyManager: Failed to remove key");
        }
        emit KeysRevoked();
    }
}
