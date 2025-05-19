// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {ArbConfigManager} from "./ArbConfigManager.sol";

/**
 * @title ArbConfigManagerFactory
 * @dev Factory contract to deploy ArbConfigManager deterministically across chains
 */
contract ArbConfigManagerFactory {
    event ArbConfigManagerDeployed(address deployedAddress, address owner); //#olympix-ignore-missing-events-assertion

    /**
     * @dev Deploy the ArbConfigManager with deterministic address
     * @param owner The owner of the ArbConfigManager
     * @param salt A unique salt value to deterministically generate the address
     * @return The address of the deployed ArbConfigManager
     */
    function deployArbConfigManager(address owner, bytes32 salt) external returns (address) {
        // Step 1: Compute the expected address (for logging/verification)
        //#olympix-ignore-external-call-potential-out-of-gas
        bytes memory bytecode = getBytecode(owner);
        address computedAddress = getAddress(bytecode, salt);

        // Step 2: Deploy using CREATE2
        address deployedAddress;
        assembly {
            deployedAddress :=
                create2(
                    0, // No ETH sent with deployment
                    add(bytecode, 0x20), // Actual bytecode starts after length prefix (32 bytes)
                    mload(bytecode), // Length of bytecode
                    salt // Salt for deterministic address
                )

            if iszero(deployedAddress) { revert(0, 0) } // Revert if deployment failed
        }

        // Verify the deployment matched the computed address
        require(deployedAddress == computedAddress, "Address mismatch");

        emit ArbConfigManagerDeployed(deployedAddress, owner);

        return deployedAddress;
    }

    /**
     * @dev Get the bytecode for ArbConfigManager construction
     * @param owner The owner address
     * @return The complete contract bytecode
     */
    function getBytecode(address owner) public pure returns (bytes memory) {
        bytes memory bytecode = type(ArbConfigManager).creationCode;
        return abi.encodePacked(bytecode, abi.encode(owner));
    }

    /**
     * @dev Calculate the expected deployment address
     * @param bytecode The contract bytecode with constructor arguments
     * @param salt The unique salt for address generation
     * @return The deterministic address where the contract will be deployed
     */
    function getAddress(bytes memory bytecode, bytes32 salt) public view returns (address) {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0xff), // Standard CREATE2 prefix //#olympix-ignore-unsafe-downcast
                address(this), // This factory address
                salt, // User-provided salt
                keccak256(bytecode) // Hash of the contract bytecode
            )
        );
        //#olympix-ignore-unsafe-downcast
        return address(uint160(uint256(hash)));
    }

    /**
     * @dev Predict the deployment address before actual deployment
     * @param owner The owner address
     * @param salt The unique salt for address generation
     * @return The address where the contract will be deployed
     */
    function predictDeploymentAddress(address owner, bytes32 salt) external view returns (address) {
        return getAddress(getBytecode(owner), salt); //#olympix-ignore-external-call-potential-out-of-gas
    }
}
