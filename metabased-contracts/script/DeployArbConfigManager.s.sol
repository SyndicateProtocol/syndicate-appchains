// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";

/**
 * @title DeployArbConfigManager
 * @notice Forge script to deploy ArbConfigManager deterministically using CREATE2
 * @dev Ensures the same address across different chains when deployed with the same private key
 */
contract DeployArbConfigManager is Script {
    function run() public {
        // The fixed salt for CREATE2 deployment
        bytes32 arbConfigManagerSalt = keccak256("ARB_CONFIG_MANAGER_v1");

        // Get deployer information from environment
        // MUST use the same private key (resulting in the same deployer address) across all chains
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        address owner = vm.envAddress("OWNER_ADDRESS");

        console2.log("Deployer address:", deployer);
        console2.log("Owner address:", owner);

        // Get the bytecode for ArbConfigManager with constructor arguments
        bytes memory constructorArgs = abi.encode(owner);
        bytes memory bytecode = abi.encodePacked(type(ArbConfigManager).creationCode, constructorArgs);
        bytes32 bytecodeHash = keccak256(bytecode);

        // Calculate the expected address
        address expectedAddress = Create2.computeAddress(arbConfigManagerSalt, bytecodeHash, deployer);
        console2.log("Expected ArbConfigManager address:", expectedAddress);

        // Start broadcasting transactions
        vm.startBroadcast(privateKey);

        // Deploy using CREATE2
        address arbConfigManagerAddress;
        assembly {
            arbConfigManagerAddress :=
                create2(
                    0, // value: 0 ETH
                    add(bytecode, 0x20), // pointer to bytecode
                    mload(bytecode), // length of bytecode
                    arbConfigManagerSalt // salt
                )

            if iszero(arbConfigManagerAddress) {
                // Revert if deployment failed
                mstore(0x00, "Deployment failed")
                revert(0x00, 0x20)
            }
        }

        console2.log("ArbConfigManager deployed to:", arbConfigManagerAddress);
        console2.log("Deployment successful:", arbConfigManagerAddress == expectedAddress ? "Yes" : "No");

        // Optional: Test creating a configuration for a chain
        // Uncomment if you want to test right after deployment
        /*
        ArbConfigManager manager = ArbConfigManager(arbConfigManagerAddress);
        uint256 testChainId = 12345;
        address configAddress = manager.getArbChainConfigAddress(testChainId);
        console2.log("Predicted config address for chain", testChainId, ":", configAddress);
        */

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
