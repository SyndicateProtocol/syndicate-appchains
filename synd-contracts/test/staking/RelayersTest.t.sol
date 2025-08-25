// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;


import {L1Relayer} from "src/staking/L1Relayer.sol";
import {L2Relayer} from "src/staking/L2Relayer.sol";

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract DummyToken is ERC20 {
    constructor() ERC20("DummyToken", "DT") {}
}

contract RelayersTest is Test {
    L1Relayer public l1Relayer;
    L2Relayer public l2Relayer;

    DummyToken public dummyToken;

    address public admin;
    address public opBridge;
    address public opMessageRelayer;
    address public arbBridge;

    function setUp() public {
        admin = makeAddr("admin");
        opBridge = makeAddr("opBridge");
        opMessageRelayer = makeAddr("opMessageRelayer");
        arbBridge = makeAddr("arbBridge");

        dummyToken = new DummyToken();

        l2Relayer = new L2Relayer(arbBridge, address(dummyToken), admin);
        l1Relayer = new L1Relayer(opBridge, opMessageRelayer, address(dummyToken), address(dummyToken), address(l2Relayer), admin);
    }

    function test_admin_L2Relayer() public {
        // Try as non-admin
        address nonAdmin = makeAddr("nonAdmin");
        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl: account ... is missing role ...
        l1Relayer.setMinGasLimit(0);

        // As admin
        vm.prank(admin);
        l1Relayer.setMinGasLimit(20000);
        assertEq(l1Relayer.minGasLimit(), 20000);
    }

    function test_admin_L1Relayer() public {
        // Try as non-admin
        address nonAdmin = makeAddr("nonAdmin");
        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl: account ... is missing role ...
        l2Relayer.setGasSettings(0, 0);

        // As admin
        vm.prank(admin);
        l2Relayer.setGasSettings(600_000, 3 gwei);
        assertEq(l2Relayer.gasLimit(), 600_000);
        assertEq(l2Relayer.maxFeePerGas(), 3 gwei);
    }
}
