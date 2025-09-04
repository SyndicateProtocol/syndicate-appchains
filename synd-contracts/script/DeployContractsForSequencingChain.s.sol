// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {RequireAndModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeploySyndicateFactory is Script {
    SyndicateFactory public syndicateFactory;
    RequireAndModuleFactory public requireAndModuleFactory;
    uint256 public appchainId;

    function run() public {
        vm.startBroadcast();

        appchainId = 0; // TODO: Set the App chain ID

        // syndicate admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy implementation and proxy
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        syndicateFactory = SyndicateFactory(address(proxy));
        console.log("Deployed SyndicateFactory", address(syndicateFactory));
        requireAndModuleFactory = new RequireAndModuleFactory(admin);
        console.log("Deployed RequireAndModuleFactory", address(requireAndModuleFactory));

        address module = requireAndModuleFactory.createRequireAndModule(admin, bytes32(appchainId));
        console.log("Deployed RequireAndModule", module);

        // create SyndicateSequencingChain with the permission module
        (address sequencingChain, uint256 chainId) = syndicateFactory.createSyndicateSequencingChain(
            0, // auto-increment
            admin,
            IRequirementModule(module)
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
        sequencingChain = new SyndicateSequencingChain();
        sequencingChain.initialize(admin, address(permissionModule), appchainId);
        console.log("Deployed SyndicateSequencingChain", address(sequencingChain));

        // Deploy and add always allowed module
        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        permissionModule.addPermissionCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to permission checks", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
