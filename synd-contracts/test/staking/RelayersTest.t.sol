// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {L1Relayer} from "src/staking/L1Relayer.sol";
import {L2Relayer} from "src/staking/L2Relayer.sol";
import {Refunder} from "src/staking/Refunder.sol";
import {RelayerMocks} from "src/staking/RelayerMocks.sol";
import {RelayHelper} from "src/staking/RelayHelper.sol";

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract DummyToken is ERC20 {
    constructor() ERC20("DummyToken", "DT") {}

    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}

contract ContractMock {
    fallback() external {}
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

    RelayHelper public relayHelper;

    function setUp() public {
        admin = makeAddr("admin");
        opBridge = address(new ContractMock());
        opMessageRelayer = address(new ContractMock());
        arbBridge = address(new ContractMock());

        dummyToken = new DummyToken();

        relayerMocks = new RelayerMocks();
        refunder = new Refunder(address(relayerMocks), address(relayerMocks), admin);
        l2Relayer = new L2Relayer(arbBridge, address(dummyToken), address(refunder), admin);
        l1Relayer = new L1Relayer(
            opBridge, opMessageRelayer, address(dummyToken), address(dummyToken), address(l2Relayer), admin
        );
        relayHelper = new RelayHelper(admin, address(l1Relayer), address(dummyToken));
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
        refunder.recover();

        assertEq(address(relayerMocks).balance, 100 ether);
        assertEq(address(refunder).balance, 0);
    }

    function test_relayHelper() public {
        vm.prank(admin);
        vm.expectRevert(); // InsufficientBalance
        relayHelper.relay(address(l2Relayer), 1);

        dummyToken.mint(address(relayHelper), 1 ether);

        vm.prank(makeAddr("anyone"));
        vm.expectRevert(); // AccessControl: account ... is missing role ...
        relayHelper.relay(address(l2Relayer), 1);

        vm.prank(admin);
        relayHelper.relay(address(l2Relayer), 1);
    }

    function test_relayHelperAmount() public {
        address anyone = makeAddr("anyone");
        dummyToken.mint(anyone, 1 ether);

        vm.prank(anyone);
        vm.expectRevert();
        relayHelper.relayAmount(1 ether, address(l2Relayer), 1);

        vm.prank(anyone);
        dummyToken.approve(address(relayHelper), 1 ether);

        vm.prank(anyone);
        relayHelper.relayAmount(1 ether, address(l2Relayer), 1);
    }

    function test_relayHelperWithdraw() public {
        dummyToken.mint(address(relayHelper), 1 ether);

        vm.prank(makeAddr("anyone"));
        vm.expectRevert();
        relayHelper.withdraw(1 ether, admin);

        vm.prank(admin);
        relayHelper.withdraw(1 ether, admin);

        assertEq(dummyToken.balanceOf(address(relayHelper)), 0);
        assertEq(dummyToken.balanceOf(admin), 1 ether);
    }
}
