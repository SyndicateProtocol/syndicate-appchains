// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {ArbConfigManagerFactory} from "src/config/ArbConfigManagerFactory.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";

import {console} from "forge-std/console.sol";

contract DeployMinRequiredSettlementChain is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address admin = vm.envAddress("ADMIN_ADDR");
        vm.startBroadcast(deployerPrivateKey);

        ArbConfigManagerFactory arbConfigManagerFactory = new ArbConfigManagerFactory();
        console.log("ArbConfigManagerFactory", address(arbConfigManagerFactory));

        bytes32 salt = keccak256(abi.encodePacked("067e372eec0a360fe6fed1cc80430d7c680172cf"));
        address arbConfigManager = arbConfigManagerFactory.deployArbConfigManager(admin, salt);
        console.log("arbConfigManager", address(arbConfigManager));

        vm.stopBroadcast();
    }
}
