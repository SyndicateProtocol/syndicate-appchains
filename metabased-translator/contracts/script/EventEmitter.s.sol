// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {EventEmitter} from "../src/EventEmitter.sol";

contract EventEmitterScript is Script {
    EventEmitter public eventEmitter;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        eventEmitter = new EventEmitter();

        vm.stopBroadcast();
    }
}
