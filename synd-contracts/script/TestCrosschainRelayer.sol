// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {L1Relayer} from "../src/staking/L1Relayer.sol";
import {L2Relayer} from "../src/staking/L2Relayer.sol";
import {DummyPool} from "../src/staking/DummyPool.sol";

contract RelayTester is Script {
    // DEPLOYED ON SEPOLIA
    address public opBridge = address(0xfd0Bf71F60660E2f608ed56e1659C450eB113120);
    address public opMessageRelayer = address(0xC34855F4De64F1840e5686e64278da901e261f20);
    address public l1Token = address(0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C);

    // DEPLOYED ON BASE SEPOLIA
    address public arbBridge = address(0xf324b8d22a73Ebc59537c7666F72aD5229B81b0f);
    address public l2Token = address(0x234Faa9cdeE5822767076495A9E258Dd8F21fFD8);

    address public l2Relayer = address(0x362F360FE913E72f08fa948Fe09b7598fA656F50);

    // DEPLOYED ON CHEESESTEAK
    address public l3Pool = address(0x1aCc3a26FCB9751D5E3b698D009b9C944eb98F9e);

    function run() public {
        vm.startBroadcast();

        deployL1Relayer();
        // deployL2Relayer();
        // deployL3Pool();

        vm.stopBroadcast();
    }

    function deployL1Relayer() public {
        L1Relayer l1Relayer = new L1Relayer(opBridge, opMessageRelayer, l1Token, l2Token, l2Relayer, msg.sender);

        console2.log("L1Relayer deployed to:", address(l1Relayer));
    }

    function deployL2Relayer() public {
        L2Relayer l2Relayer = new L2Relayer(arbBridge, l2Token, msg.sender);

        console2.log("L2Relayer deployed to:", address(l2Relayer));
    }

    function deployL3Pool() public {
        DummyPool _l3Pool = new DummyPool();

        console2.log("L3Pool deployed to:", address(_l3Pool));
    }
}
