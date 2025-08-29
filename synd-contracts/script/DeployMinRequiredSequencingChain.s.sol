// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {RequireAndModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {RequireOrModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {RequireCompositeModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateFactoryWrapper} from "src/factory/SyndicateFactoryWrapper.sol";

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
