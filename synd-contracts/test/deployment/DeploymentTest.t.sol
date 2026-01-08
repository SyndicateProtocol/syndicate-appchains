// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {SyndForwarder} from "src/deployment/SyndForwarder.sol";

contract MockContract {
    function foo() external {}
}

contract DeploymentTest is Test {
    address public admin;

    SyndForwarder public syndForwarder;
    SyndForwarder public syndForwarderL2;

    MockContract public mockContract;

    function setUp() public {
        admin = address(0x1);

        syndForwarder = new SyndForwarder(admin, block.chainid);
        syndForwarderL2 = new SyndForwarder(admin, block.chainid + 1);

        mockContract = new MockContract();
    }

    function getAlias(SyndForwarder addr) internal pure returns (address) {
        return address(uint160(address(addr)) + uint160(0x1111000000000000000000000000000000001111));
    }

    function testInitialDeployment() external {
        assertEq(syndForwarder.allowedSender(), admin);

        assertEq(syndForwarderL2.allowedSender(), getAlias(syndForwarderL2));
    }

    function testAllowedSender() external {
        vm.prank(admin);
        syndForwarder.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));

        vm.prank(getAlias(syndForwarder));
        vm.expectRevert();
        syndForwarder.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));

        vm.prank(getAlias(syndForwarderL2));
        syndForwarderL2.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));

        vm.prank(admin);
        vm.expectRevert();
        syndForwarderL2.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));
    }
}
