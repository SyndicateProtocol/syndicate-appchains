// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {DeployerParent} from "src/deployment/DeployerParent.sol";

contract MockContract {
    function foo() external {}
}

contract MockForwarder {
    function call(address dest, bytes calldata data) external payable {}

    function deploy(bytes32 salt, address impl, bytes calldata init) external payable returns (address) {
        return address(0);
    }
}

contract ChainRegistry is UUPSUpgradeable {
    function _authorizeUpgrade(address newImplementation) internal override {}

    function chainRegistry() external returns (bool) {
        return true;
    }
}

contract DeployerParentTest is Test {
    address public admin;

    DeployerParent public deployerParent;
    MockForwarder public mockForwarder;
    MockContract public mockContract;

    function setUp() public {
        admin = makeAddr("admin");
        mockForwarder = new MockForwarder();
        mockContract = new MockContract();
        address deployerParentImpl = address(new DeployerParent(admin, address(mockForwarder)));

        deployerParent = DeployerParent(payable(new ERC1967Proxy(address(deployerParentImpl), "")));
    }

    function testInitialDeployment() external {
        assertEq(deployerParent.owner(), admin);
        assertEq(address(deployerParent.forwarder()), address(mockForwarder));
    }

    function testNonOwnerCannotCall() external {
        vm.prank(address(0x1));
        vm.expectRevert();
        deployerParent.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));

        vm.prank(address(0x1));
        vm.expectRevert();
        deployerParent.deploy(bytes32(0), address(mockContract), "");

        vm.prank(address(0x1));
        vm.expectRevert();
        deployerParent.upgradeToAndCall(address(mockContract), "");
    }

    function testCall() external {
        vm.prank(admin);
        deployerParent.call(address(mockContract), abi.encodeWithSelector(MockContract.foo.selector));

        vm.prank(admin);
        deployerParent.deploy(bytes32(0), address(mockContract), "");
    }

    function testUpgrade() external {
        address chainRegistryImpl = address(new ChainRegistry());

        vm.prank(admin);
        deployerParent.upgradeToAndCall(chainRegistryImpl, "");

        assertTrue(ChainRegistry(address(deployerParent)).chainRegistry());
    }
}
