// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script, console} from "forge-std/Script.sol";

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {SyndicateFactory} from "src/SyndicateFactory.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

contract DeploySyndicateFactory is Script {
    SyndicateFactory public syndicateFactory;
    uint256 public appChainId;

    function run() public {
        vm.startBroadcast();

        appChainId = 0; // TODO: Set the App chain ID

        // syndicate admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        syndicateFactory = new SyndicateFactory(admin);
        console.log("Deployed SyndicateFactory", address(syndicateFactory));

        // create new contracts
        (address sequencingChain, IRequirementModule permissionModule,) =
            syndicateFactory.createSyndicateSequencingChainWithRequireAllModule(admin, appChainId, bytes32(appChainId));

        console.log("Deployed SyndicateSequencingChain", sequencingChain);
        console.log("Deployed RequireAllModule", address(permissionModule));

        vm.stopBroadcast();
    }
}

contract DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule is Script {
    SyndicateSequencingChain public sequencingChain;
    RequireAllModule public permissionModule;
    uint256 public appChainId;

    function run() public {
        vm.startBroadcast();

        appChainId = 0; // TODO: Set the App chain ID
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy permission module first
        permissionModule = new RequireAllModule(admin);
        console.log("Deployed RequireAllModule", address(permissionModule));

        // Deploy sequencer with permission module
        sequencingChain = new SyndicateSequencingChain(appChainId);
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
