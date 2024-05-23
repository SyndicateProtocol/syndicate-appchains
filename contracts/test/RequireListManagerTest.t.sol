// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager, IsAllowed, Ownable} from "src/RequireListManager.sol";
import {Test} from "forge-std/Test.sol";

contract RequireListManagerMock is RequireListManager {
    function exposed_requireAllAllowed(address batchSubmitter) public view {
        requireAllAllowed(batchSubmitter);
    }

    function exposed_requireAnyAllowed(address batchSubmitter) public view {
        requireAnyAllowed(batchSubmitter);
    }
}

contract MockIsAllowedTrue is IsAllowed {
    function isAllowed() external pure returns (bool) {
        return true;
    }
}

contract MockIsAllowedFalse is IsAllowed {
    function isAllowed() external pure returns (bool) {
        return false;
    }
}

contract RequireListManagerTest is Test {
    RequireListManager public requireListManager;
    address public admin;
    address public nonAdmin;

    address public mockModule = address(0x1);

    function setUp() public {
        requireListManager = new RequireListManagerMock();
        admin = requireListManager.owner();
        nonAdmin = address(0x456);
    }

    function testRequireAllAllowed() public {
        // Add a mock contract that always returns true
        MockIsAllowedTrue mockIsAllowedTrue = new MockIsAllowedTrue();
        vm.prank(admin);
        requireListManager.addRequireAllCheck(address(mockIsAllowedTrue), true);

        // Should not revert
        requireListManager.requireAllAllowed(address(this));
    }

    function testRequireAllAllowedFailure() public {
        // Add a mock contract that always returns false
        MockIsAllowedFalse mockIsAllowedFalse = new MockIsAllowedFalse();
        vm.prank(admin);
        requireListManager.addRequireAllCheck(address(mockIsAllowedFalse), true);

        // Should revert with RequireAllCheckFailed
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireListManager.RequireAllCheckFailed.selector, address(mockIsAllowedFalse), address(this)
            )
        );
        requireListManager.requireAllAllowed(address(this));
    }

    function testRequireAnyAllowed() public {
        // Add a mock contract that always returns true
        MockIsAllowedTrue mockIsAllowedTrue = new MockIsAllowedTrue();
        vm.prank(admin);
        requireListManager.addRequireAnyCheck(address(mockIsAllowedTrue), true);

        // Should not revert
        requireListManager.requireAnyAllowed(address(this));
    }

    function testRequireAnyAllowedFailure() public {
        // Add a mock contract that always returns false
        MockIsAllowedFalse mockIsAllowedFalse = new MockIsAllowedFalse();
        vm.prank(admin);
        requireListManager.addRequireAnyCheck(address(mockIsAllowedFalse), true);

        // Should revert with RequireAnyCheckFailed
        vm.expectRevert(abi.encodeWithSelector(RequireListManager.RequireAnyCheckFailed.selector, address(this)));
        requireListManager.requireAnyAllowed(address(this));
    }

    function testGetAllRequirements() public {
        // Add mock contracts to requireAllList
        MockIsAllowedTrue mockIsAllowedTrue1 = new MockIsAllowedTrue();
        MockIsAllowedTrue mockIsAllowedTrue2 = new MockIsAllowedTrue();
        vm.startPrank(admin);
        requireListManager.addRequireAllCheck(address(mockIsAllowedTrue1), true);
        requireListManager.addRequireAllCheck(address(mockIsAllowedTrue2), false);

        // Add mock contracts to requireAnyList
        MockIsAllowedFalse mockIsAllowedFalse1 = new MockIsAllowedFalse();
        MockIsAllowedFalse mockIsAllowedFalse2 = new MockIsAllowedFalse();
        requireListManager.addRequireAnyCheck(address(mockIsAllowedFalse1), true);
        requireListManager.addRequireAnyCheck(address(mockIsAllowedFalse2), false);
        vm.stopPrank();

        address[] memory requireAllChecks = requireListManager.getAllRequirements(true);
        address[] memory requireAnyChecks = requireListManager.getAllRequirements(false);

        assertEq(requireAllChecks.length, 2);
        assertEq(requireAllChecks[0], address(mockIsAllowedTrue1));
        assertEq(requireAllChecks[1], address(mockIsAllowedTrue2));

        assertEq(requireAnyChecks.length, 2);
        assertEq(requireAnyChecks[0], address(mockIsAllowedFalse1));
        assertEq(requireAnyChecks[1], address(mockIsAllowedFalse2));
    }

    function testAddRequireAllCheck() public {
        vm.prank(admin);
        requireListManager.addRequireAllCheck(mockModule, true);
        address[] memory requireAllChecks = requireListManager.getAllRequirements(true);
        assertEq(requireAllChecks.length, 1);
        assertEq(requireAllChecks[0], mockModule);
    }

    function testAddRequireAllCheckNonAdmin() public {
        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        requireListManager.addRequireAllCheck(mockModule, true);
    }

    function testAddRequireAllCheckDuplicate() public {
        vm.startPrank(admin);
        requireListManager.addRequireAllCheck(mockModule, true);
        vm.expectRevert(RequireListManager.AddressAlreadyExistsInRequireAllList.selector);
        requireListManager.addRequireAllCheck(mockModule, true);
        vm.stopPrank();
    }

    function testRemoveRequireAllCheck() public {
        vm.startPrank(admin);
        requireListManager.addRequireAllCheck(mockModule, true);
        requireListManager.removeRequireAllCheck(mockModule);
        address[] memory requireAllChecks = requireListManager.getAllRequirements(true);
        assertEq(requireAllChecks.length, 0);
        vm.stopPrank();
    }

    function testRemoveRequireAllCheckNonAdmin() public {
        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        requireListManager.removeRequireAllCheck(mockModule);
    }

    function testRemoveRequireAllCheckNonExistent() public {
        vm.prank(admin);
        vm.expectRevert(abi.encodeWithSelector(RequireListManager.AddressDoesNotExistInRequireAllList.selector));
        requireListManager.removeRequireAllCheck(mockModule);
    }

    function testAddRequireAnyCheck() public {
        vm.prank(admin);
        requireListManager.addRequireAnyCheck(mockModule, true);
        address[] memory requireAnyChecks = requireListManager.getAllRequirements(false);
        assertEq(requireAnyChecks.length, 1);
        assertEq(requireAnyChecks[0], mockModule);
    }

    function testAddRequireAnyCheckNonAdmin() public {
        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        requireListManager.addRequireAnyCheck(mockModule, true);
    }

    function testAddRequireAnyCheckDuplicate() public {
        vm.startPrank(admin);
        requireListManager.addRequireAnyCheck(mockModule, true);
        vm.expectRevert(abi.encodeWithSelector(RequireListManager.AddressAlreadyExistsInRequireAnyList.selector));
        requireListManager.addRequireAnyCheck(mockModule, true);
        vm.stopPrank();
    }

    function testRemoveRequireAnyCheck() public {
        vm.startPrank(admin);
        requireListManager.addRequireAnyCheck(mockModule, true);
        requireListManager.removeRequireAnyCheck(mockModule);
        address[] memory requireAnyChecks = requireListManager.getAllRequirements(false);
        assertEq(requireAnyChecks.length, 0);
        vm.stopPrank();
    }

    function testRemoveRequireAnyCheckNonAdmin() public {
        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        requireListManager.removeRequireAnyCheck(mockModule);
    }

    function testRemoveRequireAnyCheckNonExistent() public {
        vm.prank(admin);
        vm.expectRevert(abi.encodeWithSelector(RequireListManager.AddressDoesNotExistInRequireAnyList.selector));
        requireListManager.removeRequireAnyCheck(mockModule);
    }
}
