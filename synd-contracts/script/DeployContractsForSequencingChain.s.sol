// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {RequireAndModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

contract DeploySyndicateFactory is Script {
    SyndicateFactory public syndicateFactory;
    RequireAndModuleFactory public requireAndModuleFactory;
    uint256 public appchainId;

    function run() public {
        vm.startBroadcast();

        appchainId = 0; // TODO: Set the App chain ID

        // syndicate admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        syndicateFactory = new SyndicateFactory(admin);
        console.log("Deployed SyndicateFactory", address(syndicateFactory));
        requireAndModuleFactory = new RequireAndModuleFactory(admin);
        console.log("Deployed RequireAndModuleFactory", address(requireAndModuleFactory));

        bytes32 salt = bytes32(appchainId);

        address module = requireAndModuleFactory.createRequireAndModule(admin, salt);
        console.log("Deployed RequireAndModule", module);

        // create SyndicateSequencingChain with the permission module
        (address sequencingChain, uint256 chainId) = syndicateFactory.createSyndicateSequencingChain(
            0, // auto-increment
            admin,
            IRequirementModule(module),
            salt
        );

        console.log("Deployed SyndicateSequencingChain", sequencingChain);
        console.log("Deployed RequireAndModule", address(module));
        console.log("Sequencing Chain ID", chainId);

        vm.stopBroadcast();
    }
}

contract DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule is Script {
    SyndicateSequencingChain public sequencingChain;
    RequireAndModule public permissionModule;
    uint256 public appchainId;

    function run() public {
        vm.startBroadcast();

        appchainId = 0; // TODO: Set the App chain ID
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy permission module first
        permissionModule = new RequireAndModule(admin);
        console.log("Deployed RequireAndModule", address(permissionModule));

        // Deploy sequencer with permission module
        sequencingChain = new SyndicateSequencingChain(appchainId);
        sequencingChain.initialize(admin, address(permissionModule));
        console.log("Deployed SyndicateSequencingChain", address(sequencingChain));

        // Deploy and add always allowed module
        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        permissionModule.addPermissionCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to permission checks", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
