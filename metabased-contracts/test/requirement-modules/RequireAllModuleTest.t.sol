// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireAllModule, Ownable} from "src/requirement-modules/RequireAllModule.sol";
import {PermissionModule} from "src/interfaces/PermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowedTrue is PermissionModule {
    function isAllowed(address) external pure returns (bool) {
        return true;
    }
}

contract MockIsAllowedFalse is PermissionModule {
    function isAllowed(address) external pure returns (bool) {
        return false;
    }
}

contract RequireAllModuleTest is Test {
    RequireAllModule public module;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireAllModule(admin);
    }

    function testIsAllowed() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedFailure() public {
        address checker = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAllModule.CheckFailed.selector, checker, address(this)));
        module.isAllowed(address(this));
    }

    function testAddRemoveCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit RequireAllModule.CheckAdded(checker);
        module.addCheck(checker, true);

        address[] memory checks = module.getAllChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        vm.expectEmit(true, true, false, false);
        emit RequireAllModule.CheckRemoved(checker);
        module.removeCheck(checker);
        checks = module.getAllChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker, true);

        vm.expectRevert(RequireAllModule.AddressAlreadyExists.selector);
        module.addCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.AddressDoesNotExist.selector);
        module.removeCheck(checker);
        vm.stopPrank();
    }

    function testMultipleChecks() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker1, true);
        module.addCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testAddCheckToTail() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker1, false); // Add to tail
        module.addCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testOnlyOwnerModifiers() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removeCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAdd() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.addCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemove() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.removeCheck(address(0));
        vm.stopPrank();
    }
}
