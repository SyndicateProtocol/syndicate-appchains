// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireAllModule, Ownable} from "src/requirement-modules/RequireAllModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

import {Test} from "forge-std/Test.sol";

contract MockIsAllowedTrue is IPermissionModule {
    function isAllowed(address, address, bytes calldata) external pure returns (bool) {
        return true;
    }
}

contract MockIsAllowedFalse is IPermissionModule {
    function isAllowed(address, address, bytes calldata) external pure returns (bool) {
        return false;
    }
}

contract RequireAllModuleTest is Test {
    RequireAllModule public module;
    address public admin;
    address public nonAdmin;

    // create bytes memory zero
    bytes public emptyData = new bytes(0);

    // Events to check emission
    event PermissionCheckAdded(address indexed check);
    event PermissionCheckRemoved(address indexed check);
    event CheckAdded(address indexed check); // Legacy event
    event CheckRemoved(address indexed check); // Legacy event

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireAllModule(admin);
    }

    // ----------------------
    // Permission Check Tests
    // ----------------------

    function testIsAllowed() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this), address(0), emptyData));
    }

    function testIsAllowedFailure() public {
        address checker = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addPermissionCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAllModule.CheckFailed.selector, checker, address(this)));
        module.isAllowed(address(this), address(0), emptyData);
    }

    function testAddRemovePermissionCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit PermissionCheckAdded(checker);
        module.addPermissionCheck(checker, true);

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        vm.expectEmit(true, true, false, false);
        emit PermissionCheckRemoved(checker);
        module.removePermissionCheck(checker);
        checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicatePermissionCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker, true);

        vm.expectRevert(RequireAllModule.AddressAlreadyExists.selector);
        module.addPermissionCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentPermissionCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.AddressDoesNotExist.selector);
        module.removePermissionCheck(checker);
        vm.stopPrank();
    }

    function testMultiplePermissionChecks() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker1, true);
        module.addPermissionCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this), address(0), emptyData));
    }

    function testAddPermissionCheckToTail() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker1, false); // Add to tail
        module.addPermissionCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testOnlyOwnerModifiersForPermissionChecks() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addPermissionCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removePermissionCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAddPermission() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.addPermissionCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemovePermission() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.removePermissionCheck(address(0));
        vm.stopPrank();
    }
}
