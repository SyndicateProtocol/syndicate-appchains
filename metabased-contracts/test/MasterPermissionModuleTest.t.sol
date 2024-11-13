// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MasterPermissionModule} from "src/MasterPermissionModule.sol";
import {IsAllowed} from "src/interfaces/IsAllowed.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowedTrue is IsAllowed {
    function isAllowed(address) external pure returns (bool) {
        return true;
    }
}

contract MockIsAllowedFalse is IsAllowed {
    function isAllowed(address) external pure returns (bool) {
        return false;
    }
}

contract MasterPermissionModuleTest is Test {
    MasterPermissionModule public module;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new MasterPermissionModule(admin);
    }

    function testIsAllowedWithRequireAll() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedWithRequireAllFailure() public {
        address checker = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(MasterPermissionModule.RequireAllCheckFailed.selector, checker, address(this))
        );
        module.isAllowed(address(this));
    }

    function testIsAllowedWithRequireAny() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAnyCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedWithRequireAnyFailure() public {
        address checker = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addRequireAnyCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(MasterPermissionModule.RequireAnyCheckFailed.selector, address(this)));
        module.isAllowed(address(this));
    }

    function testAddRemoveRequireAllCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker, true);

        address[] memory checks = module.getAllRequirements(true);
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        module.removeRequireAllCheck(checker);
        checks = module.getAllRequirements(true);
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddRemoveRequireAnyCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAnyCheck(checker, true);

        address[] memory checks = module.getAllRequirements(false);
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        module.removeRequireAnyCheck(checker);
        checks = module.getAllRequirements(false);
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateRequireAllCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker, true);

        vm.expectRevert(MasterPermissionModule.AddressAlreadyExistsInRequireAllList.selector);
        module.addRequireAllCheck(checker, true);
        vm.stopPrank();
    }

    function testAddDuplicateRequireAnyCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAnyCheck(checker, true);

        vm.expectRevert(MasterPermissionModule.AddressAlreadyExistsInRequireAnyList.selector);
        module.addRequireAnyCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentRequireAllCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(MasterPermissionModule.AddressDoesNotExistInRequireAllList.selector);
        module.removeRequireAllCheck(checker);
        vm.stopPrank();
    }

    function testRemoveNonExistentRequireAnyCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(MasterPermissionModule.AddressDoesNotExistInRequireAnyList.selector);
        module.removeRequireAnyCheck(checker);
        vm.stopPrank();
    }

    function testMultipleRequireAllChecks() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker1, true);
        module.addRequireAllCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testMultipleRequireAnyChecks() public {
        address checker1 = address(new MockIsAllowedFalse());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAnyCheck(checker1, true);
        module.addRequireAnyCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testGetAllRequirements() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addRequireAllCheck(checker1, true);
        module.addRequireAllCheck(checker2, false);

        address[] memory requireAllChecks = module.getAllRequirements(true);
        assertEq(requireAllChecks.length, 2);
        assertEq(requireAllChecks[0], checker1);
        assertEq(requireAllChecks[1], checker2);
        vm.stopPrank();
    }
}
