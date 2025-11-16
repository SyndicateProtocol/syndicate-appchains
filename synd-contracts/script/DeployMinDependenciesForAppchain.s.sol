// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {RequireAndModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {RequireOrModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {RequireCompositeModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateFactoryWrapper} from "src/factory/SyndicateFactoryWrapper.sol";

import {ArbConfigManagerFactory} from "src/config/ArbConfigManagerFactory.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";

import {console} from "forge-std/console.sol";

contract DeployMinRequiredSequencingChain is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        address admin = vm.envAddress("ADMIN_ADDR");

        RequireAndModuleFactory requireAndModuleFactory = new RequireAndModuleFactory(admin);
        console.log("requireAndModuleFactory", address(requireAndModuleFactory));

        RequireOrModuleFactory requireOrModuleFactory = new RequireOrModuleFactory(admin);
        console.log("requireOrModuleFactory", address(requireOrModuleFactory));

        RequireCompositeModuleFactory requireCompositeModuleFactory = new RequireCompositeModuleFactory(admin);
        console.log("requireCompositeModuleFactory", address(requireCompositeModuleFactory));

        SyndicateFactory syndicateFactory = new SyndicateFactory(admin);
        console.log("syndicateFactory", address(syndicateFactory));

        SyndicateFactoryWrapper syndicateFactoryWrapper = new SyndicateFactoryWrapper(
            admin, address(syndicateFactory), address(requireAndModuleFactory), address(requireOrModuleFactory)
        );
        console.log("syndicateFactoryWrapper", address(syndicateFactoryWrapper));

        vm.stopBroadcast();
    }
}

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
