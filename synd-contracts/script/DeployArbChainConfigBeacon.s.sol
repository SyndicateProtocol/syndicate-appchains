// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {UpgradeableBeacon} from "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";
import {ArbChainConfig} from "src/config/ArbChainConfig.sol";

contract DeployArbChainConfigBeacon is Script {
    function run() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        address beaconOwner = vm.envAddress("BEACON_OWNER");

        console2.log("Deployer address:", deployer);
        console2.log("Beacon owner:", beaconOwner);

        vm.startBroadcast(privateKey);

        // Deploy the ArbChainConfig implementation
        ArbChainConfig implementation = new ArbChainConfig();
        console2.log("ArbChainConfig implementation deployed to:", address(implementation));

        // Deploy the UpgradeableBeacon pointing to the implementation
        // The beacon owner can upgrade the implementation for all proxies
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(implementation), beaconOwner);
        console2.log("UpgradeableBeacon deployed to:", address(beacon));

        vm.stopBroadcast();

        console2.log("");
        console2.log("=== Deployment Summary ===");
        console2.log("Implementation:", address(implementation));
        console2.log("Beacon:", address(beacon));
        console2.log("Beacon Owner:", beaconOwner);
        console2.log("");
    }
}
