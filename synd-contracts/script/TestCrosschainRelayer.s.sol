// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {L1Relayer} from "../src/staking/L1Relayer.sol";
import {L2Relayer} from "../src/staking/L2Relayer.sol";
import {RelayerMocks} from "../src/staking/RelayerMocks.sol";
import {Refunder} from "../src/staking/Refunder.sol";

contract RelayTester is Script {
    // DEPLOYED ON SEPOLIA
    address public opBridge = address(0xfd0Bf71F60660E2f608ed56e1659C450eB113120);
    address public opMessageRelayer = address(0xC34855F4De64F1840e5686e64278da901e261f20);
    address public l1Token = address(0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C);

    // DEPLOYED ON BASE SEPOLIA
    address public arbBridge = address(0xf324b8d22a73Ebc59537c7666F72aD5229B81b0f);
    address public l2Token = address(0x234Faa9cdeE5822767076495A9E258Dd8F21fFD8);

    address public l2Relayer = address(0x56167352a538EEaDe967a5e830D5CF320c10Dbd5);

    // DEPLOYED ON CHEESESTEAK
    address public mocks = address(0x2372Fd8d69dA29b4B328b518C6D7e84F3aa25Dc3);
    address public refunder = address(0x22a3d80299d4F2437611E1CA0B7c8D50F4816c6e);

    function run() public {
        vm.startBroadcast();

        deployL1Relayer();
        // deployL2Relayer();
        // deployL3Stuff();

        vm.stopBroadcast();
    }

    function deployL1Relayer() public {
        assert(block.chainid == 11155111);

        L1Relayer _l1Relayer = new L1Relayer(opBridge, opMessageRelayer, l1Token, l2Token, l2Relayer, msg.sender);

        console2.log("L1Relayer deployed to:", address(_l1Relayer));
    }

    function deployL2Relayer() public {
        assert(block.chainid == 84532);

        L2Relayer _l2Relayer = new L2Relayer(arbBridge, l2Token, refunder, msg.sender);

        console2.log("L2Relayer deployed to:", address(_l2Relayer));
    }

    function deployL3Stuff() public {
        assert(block.chainid == 510002);

        RelayerMocks _relayerMocks = new RelayerMocks();
        Refunder _refunder = new Refunder(address(_relayerMocks), address(_relayerMocks), msg.sender);

        console2.log("RelayerMocks deployed to:", address(_relayerMocks));
        console2.log("Refunder deployed to:", address(_refunder));
    }
}
