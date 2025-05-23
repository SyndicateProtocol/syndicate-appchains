// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RequireOrModule, BaseRequirementModule} from "src/requirement-modules/RequireOrModule.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

import {Test} from "forge-std/Test.sol";

contract MockPermissionAllowedTrue is IPermissionModule {
    function isAllowed(address, address, bytes calldata) external pure returns (bool) {
        return true;
    }
}

contract MockPermissionAllowedFalse is IPermissionModule {
    function isAllowed(address, address, bytes calldata) external pure returns (bool) {
        return false;
    }
}

contract RequireOrModuleTest is Test {
    RequireOrModule public module;
    address public admin;
    address public nonAdmin;

    // create bytes memory zero
    bytes public emptyData = new bytes(0);

    // Events to check emission
    event PermissionCheckAdded(address indexed check);
    event PermissionCheckRemoved(address indexed check);

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireOrModule(admin);
    }

    // ----------------------
    // Permission Check Tests
    // ----------------------

    function testIsAllowedNoChecks() public view {
        // Should pass if no checks exist
        assertTrue(module.isAllowed(address(this), address(0), emptyData));
    }

    function testIsAllowedPassOne() public {
        address checkerTrue = address(new MockPermissionAllowedTrue());
        address checkerFalse = address(new MockPermissionAllowedFalse());

        vm.startPrank(admin);
        module.addPermissionCheck(checkerFalse, true);
        module.addPermissionCheck(checkerTrue, true); // Add to front
        vm.stopPrank();

        // Should pass as long as at least one check passes
        assertTrue(module.isAllowed(address(this), address(0), emptyData));
    }

    function testIsAllowedPassOnlyOne() public {
        address checkerTrue = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checkerTrue, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this), address(0), emptyData));
    }

    function testIsAllowedFailure() public {
        address checkerFalse1 = address(new MockPermissionAllowedFalse());
        address checkerFalse2 = address(new MockPermissionAllowedFalse());

        vm.startPrank(admin);
        module.addPermissionCheck(checkerFalse1, true);
        module.addPermissionCheck(checkerFalse2, false);
        vm.stopPrank();

        // Should fail if all checks fail
        vm.expectRevert(abi.encodeWithSelector(RequireOrModule.CheckFailed.selector, address(this)));
        module.isAllowed(address(this), address(0), emptyData);
    }

    function testAddRemovePermissionCheck() public {
        address checker = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);

        // Test adding a checker
        vm.expectEmit(true, true, false, false);
        emit PermissionCheckAdded(checker);
        module.addPermissionCheck(checker, true);

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        // Test removing a checker
        vm.expectEmit(true, true, false, false);
        emit PermissionCheckRemoved(checker);
        module.removePermissionCheck(checker);

        checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicatePermissionCheck() public {
        address checker = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker, true);

        vm.expectRevert(BaseRequirementModule.AddressAlreadyExists.selector);
        module.addPermissionCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentPermissionCheck() public {
        address checker = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(BaseRequirementModule.AddressDoesNotExist.selector);
        module.removePermissionCheck(checker);
        vm.stopPrank();
    }

    function testAddPermissionCheckToTail() public {
        address checker1 = address(new MockPermissionAllowedTrue());
        address checker2 = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker1, false); // Add to tail
        module.addPermissionCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testAddPermissionCheckToHead() public {
        address checker1 = address(new MockPermissionAllowedTrue());
        address checker2 = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(checker1, true); // Add to head
        module.addPermissionCheck(checker2, true); // Add to head
        vm.stopPrank();

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker2); // checker2 should be first since it was added to head last
        assertEq(checks[1], checker1);
    }

    function testOnlyOwnerModifiersForPermissionChecks() public {
        address checker = address(new MockPermissionAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addPermissionCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removePermissionCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAddPermission() public {
        vm.startPrank(admin);
        vm.expectRevert(BaseRequirementModule.InvalidAddress.selector);
        module.addPermissionCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemovePermission() public {
        vm.startPrank(admin);
        vm.expectRevert(BaseRequirementModule.InvalidAddress.selector);
        module.removePermissionCheck(address(0));
        vm.stopPrank();
    }

    // -----------------------
    // Integration Tests
    // -----------------------

    function testIndependentPermissionAndCalldataLists() public {
        address permissionChecker = address(new MockPermissionAllowedTrue());

        vm.startPrank(admin);
        module.addPermissionCheck(permissionChecker, true);

        vm.stopPrank();

        // Verify that both lists contain the correct checkers
        address[] memory permissionChecks = module.getAllPermissionChecks();

        assertEq(permissionChecks.length, 1);

        assertEq(permissionChecks[0], permissionChecker);
    }

    function testEmptyGetAllChecks() public view {
        // Test that the get all functions work with empty lists
        address[] memory permissionChecks = module.getAllPermissionChecks();

        assertEq(permissionChecks.length, 0);
    }

    function testCheckCombinations() public {
        address permissionTrue = address(new MockPermissionAllowedTrue());
        address permissionFalse = address(new MockPermissionAllowedFalse());

        vm.startPrank(admin);

        // Test different combinations of checkers

        // 1. Both pass: True + False
        module.addPermissionCheck(permissionTrue, true);
        module.addPermissionCheck(permissionFalse, false);

        assertTrue(module.isAllowed(address(this), address(0), emptyData));

        // Reset
        module.removePermissionCheck(permissionTrue);
        module.removePermissionCheck(permissionFalse);

        // 2. Both fail: False only
        module.addPermissionCheck(permissionFalse, true);

        vm.expectRevert(abi.encodeWithSelector(RequireOrModule.CheckFailed.selector, address(this)));
        module.isAllowed(address(this), address(0), emptyData);

        vm.stopPrank();
    }
}
