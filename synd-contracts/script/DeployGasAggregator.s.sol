// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {GasAggregator} from "../src/staking/GasAggregator.sol";

contract DeployGasAggregator is Script {
    function run() public {
        vm.startBroadcast();

        // Read configuration from environment variables
        address factory = vm.envAddress("APPCHAIN_FACTORY_ADDRESS");
        address allowedImplementation = vm.envAddress("ALLOWED_IMPLEMENTATION_ADDRESS");
        address admin = vm.envAddress("GAS_AGGREGATOR_ADMIN");
        uint256 epochStart = vm.envUint("EPOCH_START");

        console2.log("Deploying GasAggregator with UUPS proxy pattern...");
        console2.log("Factory address:", factory);
        console2.log("Allowed implementation address:", allowedImplementation);
        console2.log("Admin address:", admin);
        console2.log("Epoch start:", epochStart);

        // 1. Deploy GasAggregator implementation
        GasAggregator implementation = new GasAggregator();
        console2.log("GasAggregator implementation deployed to:", address(implementation));

        // 2. Prepare initialization data
        bytes memory initData =
            abi.encodeWithSelector(GasAggregator.initialize.selector, admin, factory, allowedImplementation, epochStart);

        // 3. Deploy ERC1967Proxy (UUPS)
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        console2.log("GasAggregator proxy deployed to:", address(proxy));

        console2.log("=== Deployment Summary ===");
        console2.log("Implementation:", address(implementation));
        console2.log("GasAggregator (Proxy):", address(proxy));
        console2.log("Admin:", admin);
        console2.log("Default challengeWindow: 24 hours");
        console2.log("Default addChainFee: 5 ether");
        console2.log("Default maxAppchainsToQuery: 100");

        vm.stopBroadcast();
    }
}
