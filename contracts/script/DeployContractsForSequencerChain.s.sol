// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

import {BasedSequencerChain} from "src/BasedSequencerChain.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";
import {TokenBalanceSequencingModule} from "src/sequencing-modules/TokenBalanceSequencingModule.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";

contract DeployBasedSequencerChain is Script {
    BasedSequencerChain public basedSequencerChain;

    function run() public {
        vm.startBroadcast();
        basedSequencerChain = new BasedSequencerChain();
        console.log("Deployed BasedSequencerChain", address(basedSequencerChain));

        // // Example on how to add the modules to the requireAllList and/or requireAnyList of BasedSequencerChain
        // basedSequencerChain.addRequireAllCheck(address(sealedBidAuctionSequencingModule), true);
        // basedSequencerChain.addRequireAnyCheck(address(tokenBalanceSequencingModule), true);
        // basedSequencerChain.addRequireAnyCheck(address(allowlistSequencingModule), true);

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

        admin = address(0); // TODO: Set the allowlist

        allowlistSequencingModule = new AllowlistSequencingModule(admin);
        console.log("Deployed AllowlistSequencingModule", address(allowlistSequencingModule));

        vm.stopBroadcast();
    }
}
