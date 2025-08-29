// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {SyndicateStorage} from "../src/backfill/SyndicateStorage.sol";
import {AccessControlledERC20} from "../src/token/AccessControlledERC20.sol";

contract DeployAccessControlledERC20 is Script {
    AccessControlledERC20 public accessControlledERC20;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address admin = vm.envAddress("ADMIN");

        vm.startBroadcast(deployerPrivateKey);

        accessControlledERC20 = new AccessControlledERC20("TestMON", "TESTMON", admin);

        address[3] memory addressesToAllowlist = [
            0x5E2F16dF1550e49aA44aF99Ed3cdF1be72913118,
            0xec0e25aBc32e5dcee851133c59a0bE9Fe6BA452A,
            0x8AB5496a45c92c36eC293d2681F1d3706eaff85D
        ];

        for (uint256 i = 0; i < addressesToAllowlist.length; i++) {
            accessControlledERC20.grantRole(accessControlledERC20.MINTER_ROLE(), addressesToAllowlist[i]);
        }

        vm.stopBroadcast();
    }
}
