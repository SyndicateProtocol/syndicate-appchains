// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {SyndicateStorage} from "../src/backfill/SyndicateStorage.sol";
import {AccessRoledERC20} from "../src/token/AccessRoledERC20.sol";

contract DeployAccessRoledERC20 is Script {
    AccessRoledERC20 public accessRoledERC20;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address admin = vm.envAddress("ADMIN");

        vm.startBroadcast(deployerPrivateKey);

        accessRoledERC20 = new AccessRoledERC20("TestMON", "TESTMON", admin);

        address[3] memory addressesToAllowlist = [
            0x5E2F16dF1550e49aA44aF99Ed3cdF1be72913118,
            0xec0e25aBc32e5dcee851133c59a0bE9Fe6BA452A,
            0x8AB5496a45c92c36eC293d2681F1d3706eaff85D
        ];

        for (uint256 i = 0; i < addressesToAllowlist.length; i++) {
            accessRoledERC20.grantRole(accessRoledERC20.MINTER_ROLE(), addressesToAllowlist[i]);
        }

        vm.stopBroadcast();
    }
}
