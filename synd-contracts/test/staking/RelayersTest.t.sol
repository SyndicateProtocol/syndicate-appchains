// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {L1Relayer} from "src/staking/L1Relayer.sol";
import {L2Relayer} from "src/staking/L2Relayer.sol";
import {Refunder} from "src/staking/Refunder.sol";
import {RelayerMocks} from "src/staking/RelayerMocks.sol";

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract DummyToken is ERC20 {
    constructor() ERC20("DummyToken", "DT") {}
}

contract DummyBridge {
    function messenger() external view returns (address) {
        return address(0);
    }
}

contract RelayersTest is Test {
    L1Relayer public l1Relayer;
    L2Relayer public l2Relayer;
    Refunder public refunder;
    RelayerMocks public relayerMocks;

    DummyToken public dummyToken;

    address public admin;
    address public opBridge;
    address public opMessageRelayer;
    address public arbBridge;

    function setUp() public {
        admin = makeAddr("admin");
        opBridge = address(new DummyBridge());
        arbBridge = makeAddr("arbBridge");

        dummyToken = new DummyToken();

        relayerMocks = new RelayerMocks();
        vm.prank(admin);
        refunder = new Refunder(address(relayerMocks));
        vm.prank(admin);
        l2Relayer = new L2Relayer(arbBridge, address(dummyToken), address(refunder));
        l1Relayer = new L1Relayer(opBridge, address(dummyToken), address(dummyToken), address(l2Relayer), 200_000);
    }

    function test_admin_L2Relayer() public {
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

    function test_admin_refunder() public {
        // Try as non-admin
        address nonAdmin = makeAddr("nonAdmin");
        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl: account ... is missing role ...
        refunder.setRecoverPool(address(relayerMocks));
    }

    function test_refunder() public {
        vm.deal(address(refunder), 100 ether);

        address anyone = makeAddr("anyone");
        vm.prank(anyone);
        vm.warp(refunder.getEpochStart(1));
        refunder.recover();

        assertEq(address(relayerMocks).balance, 100 ether);
        assertEq(address(refunder).balance, 0);
    }
}
