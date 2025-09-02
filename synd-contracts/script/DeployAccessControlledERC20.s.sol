// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {SyndicateStorage} from "../src/backfill/SyndicateStorage.sol";
import {AccessControlledERC20} from "../src/token/AccessControlledERC20.sol";
import {console} from "forge-std/console.sol";

contract DeployAccessControlledERC20 is Script {
    AccessControlledERC20 public accessControlledERC20;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address admin = vm.envAddress("ADMIN_ADDR");

        vm.startBroadcast(deployerPrivateKey);

        string memory name = "TestToken";
        // symbol should be <= 6 characters long
        string memory symbol = "TEST";
        console.log("deploying access controlled ERC20 with name", name, "and symbol", symbol);

        accessControlledERC20 = new AccessControlledERC20(name, symbol, admin);

        address[1] memory addressesToAllowlist = [admin];

        for (uint256 i = 0; i < addressesToAllowlist.length; i++) {
            accessControlledERC20.grantRole(accessControlledERC20.MINTER_ROLE(), addressesToAllowlist[i]);
        }

        vm.stopBroadcast();
    }
}
