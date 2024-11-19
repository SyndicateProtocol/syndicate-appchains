// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireAnyModule, Ownable} from "src/requirement-modules/RequireAnyModule.sol";
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

contract RequireAnyModuleTest is Test {
    RequireAnyModule public module;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireAnyModule(admin);
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

        vm.expectRevert(abi.encodeWithSelector(RequireAnyModule.CheckFailed.selector, address(this)));
        module.isAllowed(address(this));
    }

    function testAddRemoveCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker, true);

        address[] memory checks = module.getAllChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        module.removeCheck(checker);
        checks = module.getAllChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker, true);

        vm.expectRevert(RequireAnyModule.AddressAlreadyExists.selector);
        module.addCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.AddressDoesNotExist.selector);
        module.removeCheck(checker);
        vm.stopPrank();
    }

    function testMultipleChecks() public {
        address checker1 = address(new MockIsAllowedFalse());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addCheck(checker1, true);
        module.addCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testAllChecksFail() public {
        address checker1 = address(new MockIsAllowedFalse());
        address checker2 = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addCheck(checker1, true);
        module.addCheck(checker2, false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAnyModule.CheckFailed.selector, address(this)));
        module.isAllowed(address(this));
    }

    function testNoChecksAllow() public {
        // When no checks are added, isAllowed should return true
        assertTrue(module.isAllowed(address(this)));
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
}
