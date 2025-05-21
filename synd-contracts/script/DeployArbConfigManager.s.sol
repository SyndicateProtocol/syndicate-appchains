// SPDX-License-Identifier: MIT
pragma solidity 0.8.29;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {ArbConfigManagerFactory} from "src/config/ArbConfigManagerFactory.sol";

contract DeployArbConfigManager is Script {
    function run() public {
        // Get deployer information from environment
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        address owner = vm.envAddress("OWNER_ADDRESS");

        // Use a fixed salt to ensure consistency across chains
        bytes32 salt = keccak256("ARB_CONFIG_MANAGER_v1");

        console2.log("Deployer address:", deployer);
        console2.log("Owner address:", owner);

        vm.startBroadcast(privateKey);

        // Step 1: Deploy the factory if it doesn't exist already
        // You could get this address from your environment if already deployed
        ArbConfigManagerFactory factory = new ArbConfigManagerFactory();
        console2.log("Factory deployed to:", address(factory));

        // Step 2: Get the predicted address
        address predictedAddress = factory.predictDeploymentAddress(owner, salt);
        console2.log("Predicted ArbConfigManager address:", predictedAddress);

        // Step 3: Deploy the ArbConfigManager through the factory
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        console2.log("ArbConfigManager deployed to:", deployedAddress);
        console2.log("Deployment successful:", deployedAddress == predictedAddress ? "Yes" : "No");

        require(deployedAddress == predictedAddress, "Deployed address does not match the predicted address");

        vm.stopBroadcast();
    }
}
