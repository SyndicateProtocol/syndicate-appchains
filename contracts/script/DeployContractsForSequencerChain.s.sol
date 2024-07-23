// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

import {BasedSequencerChain} from "src/BasedSequencerChain.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";
import {TokenBalanceSequencingModule} from "src/sequencing-modules/TokenBalanceSequencingModule.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

contract DeployBasedSequencerChain is Script {
    BasedSequencerChain public basedSequencerChain;

    function run() public {
        vm.startBroadcast();
        basedSequencerChain = new BasedSequencerChain();
        console.log("Deployed BasedSequencerChain", address(basedSequencerChain));

        // Example on how to add/remove the modules to the requireAllList and/or requireAnyList of BasedSequencerChain

        // basedSequencerChain.addRequireAllCheck(address(sealedBidAuctionSequencingModule), true);
        // basedSequencerChain.removeRequireAllCheck(address(sealedBidAuctionSequencingModule));

        // basedSequencerChain.addRequireAnyCheck(address(tokenBalanceSequencingModule), true);
        // basedSequencerChain.removeRequireAnyCheck(address(allowlistSequencingModule));

        vm.stopBroadcast();
    }
}

contract AddModuleToBasedSequencerChain is Script {
    BasedSequencerChain public basedSequencerChain;

    address public module;

    function run() public {
        vm.startBroadcast();
        // BaseSequencerChain address on Base Sepolia
        basedSequencerChain = BasedSequencerChain(0xdf4F1Be1fc0222Bb45c3a6B85F977AC89f49a9C5);
        // AlwaysAllowedModule address on Base Sepolia
        module = 0x0324b4d8F786e11206F82e016DD4480de2332cF3;

        basedSequencerChain.addRequireAllCheck(module, true);
        console.log("Added module to BasedSequencerChain", module);

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
