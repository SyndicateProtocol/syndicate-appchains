// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {BlockHashRelayer, IArbInbox} from "src/staking/BlockHashRelayer.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract DeployBlockHashRelayer is Script {
    function run() public {
        vm.startBroadcast();

        // Read configuration from environment variables
        address arbInbox = vm.envAddress("ARB_INBOX_ADDRESS");
        address syndToken = vm.envAddress("SYND_TOKEN_ADDRESS");

        console2.log("Deploying BlockHashRelayer...");
        console2.log("Arbitrum Inbox:", arbInbox);
        console2.log("SYND Token:", syndToken);

        // Deploy BlockHashRelayer
        BlockHashRelayer messenger = new BlockHashRelayer(IArbInbox(arbInbox), IERC20(syndToken), msg.sender);

        console2.log("BlockHashMessenger deployed to:", address(messenger));

        console2.log("=== Deployment Summary ===");
        console2.log("BlockHashRelayer:", address(messenger));
        console2.log("Arbitrum Inbox:", address(messenger.arbInbox()));
        console2.log("SYND Token:", address(messenger.syndToken()));

        vm.stopBroadcast();
    }
}
