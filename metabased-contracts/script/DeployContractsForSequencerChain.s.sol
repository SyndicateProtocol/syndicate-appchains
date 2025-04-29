// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script, console} from "forge-std/Script.sol";

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

contract DeployMetabasedFactory is Script {
    MetabasedFactory public metabasedFactory;
    uint256 public l3ChainId;

    function run() public {
        vm.startBroadcast();

        l3ChainId = 0; // TODO: Set the L3 chain ID

        // metafiller admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);
        address manager = vm.envOr("MANAGER_ADDR", msg.sender);

        metabasedFactory = new MetabasedFactory();
        console.log("Deployed MetabasedFactory", address(metabasedFactory));

        // create new contracts
        (address sequencerChain, address metafillerStorage, IRequirementModule permissionModule) =
            metabasedFactory.createAllContractsWithRequireAllModule(admin, manager, l3ChainId, bytes32(l3ChainId));

        console.log("Deployed MetabasedSequencerChain", sequencerChain);
        console.log("Deployed MetafillerStorage", metafillerStorage);
        console.log("Deployed RequireAllModule", address(permissionModule));

        vm.stopBroadcast();
    }
}

contract DeployMetabasedSequencerChainPlusSetupWithAlwaysAllowModule is Script {
    MetabasedSequencerChain public sequencerChain;
    RequireAllModule public permissionModule;
    uint256 public l3ChainId;

    function run() public {
        vm.startBroadcast();

        l3ChainId = 0; // TODO: Set the L3 chain ID
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy permission module first
        permissionModule = new RequireAllModule(admin);
        console.log("Deployed RequireAllModule", address(permissionModule));

        // Deploy sequencer with permission module
        sequencerChain = new MetabasedSequencerChain(l3ChainId);
        sequencerChain.initialize(admin, address(permissionModule));
        console.log("Deployed MetabasedSequencerChain", address(sequencerChain));

        // Deploy and add always allowed module
        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        permissionModule.addPermissionCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to permission checks", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
