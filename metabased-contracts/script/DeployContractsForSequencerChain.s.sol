// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

import {MetabasedFactory} from "src/MetabasedFactory.sol";

contract DeployMetabasedFactory is Script {
    MetabasedFactory public metabasedFactory;

    uint256 public l3ChainId;

    function run() public {
        vm.startBroadcast();

        l3ChainId = 0; // TODO: Set the L3 chain ID

        // metafiller  admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);
        address manager = vm.envOr("MANAGER_ADDR", msg.sender);

        metabasedFactory = new MetabasedFactory();
        console.log("Deployed MetabasedFactory", address(metabasedFactory));

        // create new MetabasedSequencerChain contracts
        (address sequencerChain, address metafillerStorage) =
            metabasedFactory.createAllContracts(admin, manager, l3ChainId);
        console.log("Deployed MetabasedSequencerChain", sequencerChain);
        console.log("Deployed MetafillerStorage", metafillerStorage);

        vm.stopBroadcast();
    }
}

contract DeployMetabasedSequencerChainPlusSetupWithAlwaysAllowModule is Script {
    MetabasedSequencerChain public metabasedSequencerChainContract;

    uint256 public l3ChainId;

    function run() public {
        vm.startBroadcast();

        l3ChainId = 0; // TODO: Set the L3 chain ID

        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        metabasedSequencerChainContract = new MetabasedSequencerChain(l3ChainId, admin);
        console.log("Deployed MetabasedSequencerChain", address(metabasedSequencerChainContract));

        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        metabasedSequencerChainContract.addRequireAllCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to MetabasedSequencerChain", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
