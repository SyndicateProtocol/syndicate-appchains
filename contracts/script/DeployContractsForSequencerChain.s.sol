// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";
import {TokenBalanceSequencingModule} from "src/sequencing-modules/TokenBalanceSequencingModule.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

contract DeployMetabasedSequencerChain is Script {
    MetabasedSequencerChain public metabasedSequencerChainContract;

    function run() public {
        vm.startBroadcast();
        metabasedSequencerChainContract = new MetabasedSequencerChain();
        console.log("Deployed MetabasedSequencerChain", address(metabasedSequencerChainContract));

        // Example on how to add/remove the modules to the requireAllList and/or requireAnyList of MetabasedSequencerChain

        // metabasedSequencerChainContract.addRequireAllCheck(address(sealedBidAuctionSequencingModule), true);
        // metabasedSequencerChainContract.removeRequireAllCheck(address(sealedBidAuctionSequencingModule));

        // metabasedSequencerChainContract.addRequireAnyCheck(address(tokenBalanceSequencingModule), true);
        // metabasedSequencerChainContract.removeRequireAnyCheck(address(allowlistSequencingModule));

        vm.stopBroadcast();
    }
}

contract AddModuleToMetabasedSequencerChain is Script {
    MetabasedSequencerChain public metabasedSequencerChainContract;

    address public module;

    function run() public {
        vm.startBroadcast();
        // MetabasedSequencerChain address on Base Sepolia
        metabasedSequencerChainContract = MetabasedSequencerChain(0x73Ba7D784d13Ec0070a4Ea6F49Ff57dc007Bb48d);
        // AlwaysAllowedModule address on Base Sepolia
        module = 0x0324b4d8F786e11206F82e016DD4480de2332cF3;

        metabasedSequencerChainContract.addRequireAllCheck(module, true);
        console.log("Added module to MetabasedSequencerChain", module);

        vm.stopBroadcast();
    }
}

contract DeploySealedBidAuctionSequencingModule is Script {
    SealedBidAuctionSequencingModule public sealedBidAuctionSequencingModule;

    uint256 public auctionDuration;
    address public treasury;

    function run() public {
        vm.startBroadcast();
        auctionDuration = 1 days; // TODO: Set the auction duration
        treasury = address(0); // TODO: Set the treasury address

        sealedBidAuctionSequencingModule = new SealedBidAuctionSequencingModule(auctionDuration, treasury);
        console.log("Deployed SealedBidAuctionSequencingModule", address(sealedBidAuctionSequencingModule));

        vm.stopBroadcast();
    }
}

contract DeployTokenBalanceSequencingModule is Script {
    TokenBalanceSequencingModule public tokenBalanceSequencingModule;

    address public token;
    uint256 public minimumBalance;

    function run() public {
        vm.startBroadcast();

        token = address(0); // TODO: Set the token address
        minimumBalance = 0; // TODO: Set the minimum balance

        tokenBalanceSequencingModule = new TokenBalanceSequencingModule(token, minimumBalance);
        console.log("Deployed TokenBalanceSequencingModule", address(tokenBalanceSequencingModule));

        vm.stopBroadcast();
    }
}

contract DeployAllowlistSequencingModule is Script {
    AllowlistSequencingModule public allowlistSequencingModule;

    address public admin;

    function run() public {
        vm.startBroadcast();

        admin = 0x45d6450fC59A6F8D9c753126cE1EFF6fa4D7e0fb; // deployer address

        allowlistSequencingModule = new AllowlistSequencingModule(admin);
        console.log("Deployed AllowlistSequencingModule", address(allowlistSequencingModule));

        vm.stopBroadcast();
    }
}

contract DeployAlwaysAllowedModule is Script {
    AlwaysAllowedModule public alwaysAllowedModule;

    function run() public {
        vm.startBroadcast();

        alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}

contract DeployMetabasedSequencerChainForTestnet is Script {
    MetabasedSequencerChain public metabasedSequencerChainContract;

    function run() public {
        vm.startBroadcast();
        metabasedSequencerChainContract = new MetabasedSequencerChain();
        console.log("Deployed MetabasedSequencerChain", address(metabasedSequencerChainContract));

        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        metabasedSequencerChainContract.addRequireAllCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to MetabasedSequencerChain", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
