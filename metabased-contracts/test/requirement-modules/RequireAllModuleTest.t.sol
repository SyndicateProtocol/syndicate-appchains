// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireAllModule, Ownable} from "src/requirement-modules/RequireAllModule.sol";
import {ProposerPermissionModule} from "src/interfaces/ProposerPermissionModule.sol";
import {CalldataPermissionModule} from "src/interfaces/CalldataPermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowedTrue is ProposerPermissionModule {
    function isAllowed(address) external pure returns (bool) {
        return true;
    }
}

contract MockIsAllowedFalse is ProposerPermissionModule {
    function isAllowed(address) external pure returns (bool) {
        return false;
    }
}

contract MockCalldataAllowedTrue is CalldataPermissionModule {
    function isCalldataAllowed(bytes calldata) external pure returns (bool) {
        return true;
    }
}

contract MockCalldataAllowedFalse is CalldataPermissionModule {
    function isCalldataAllowed(bytes calldata) external pure returns (bool) {
        return false;
    }
}

contract RequireAllModuleTest is Test {
    RequireAllModule public module;
    address public admin;
    address public nonAdmin;

    // Events to check emission
    event ProposerCheckAdded(address indexed check);
    event ProposerCheckRemoved(address indexed check);
    event CalldataCheckAdded(address indexed check);
    event CalldataCheckRemoved(address indexed check);
    event CheckAdded(address indexed check); // Legacy event
    event CheckRemoved(address indexed check); // Legacy event

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireAllModule(admin);
    }

    // ----------------------
    // Proposer Check Tests
    // ----------------------

    function testIsAllowed() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedFailure() public {
        address checker = address(new MockIsAllowedFalse());

        vm.startPrank(admin);
        module.addProposerCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAllModule.CheckFailed.selector, checker, address(this)));
        module.isAllowed(address(this));
    }

    function testAddRemoveProposerCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit ProposerCheckAdded(checker);
        module.addProposerCheck(checker, true);

        address[] memory checks = module.getAllProposerChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        vm.expectEmit(true, true, false, false);
        emit ProposerCheckRemoved(checker);
        module.removeProposerCheck(checker);
        checks = module.getAllProposerChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateProposerCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker, true);

        vm.expectRevert(RequireAllModule.AddressAlreadyExists.selector);
        module.addProposerCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentProposerCheck() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.AddressDoesNotExist.selector);
        module.removeProposerCheck(checker);
        vm.stopPrank();
    }

    function testMultipleProposerChecks() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker1, true);
        module.addProposerCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testAddProposerCheckToTail() public {
        address checker1 = address(new MockIsAllowedTrue());
        address checker2 = address(new MockIsAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker1, false); // Add to tail
        module.addProposerCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllProposerChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testOnlyOwnerModifiersForProposerChecks() public {
        address checker = address(new MockIsAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addProposerCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removeProposerCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAddProposer() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.addProposerCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemoveProposer() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.removeProposerCheck(address(0));
        vm.stopPrank();
    }

    // ----------------------
    // Calldata Check Tests
    // ----------------------

    function testIsCalldataAllowed() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checker, true);
        vm.stopPrank();

        assertTrue(module.isCalldataAllowed(abi.encode("test data")));
    }

    function testIsCalldataAllowedFailure() public {
        address checker = address(new MockCalldataAllowedFalse());

        vm.startPrank(admin);
        module.addCalldataCheck(checker, true);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAllModule.CalldataCheckFailed.selector, checker));
        module.isCalldataAllowed(abi.encode("test data"));
    }

    function testAddRemoveCalldataCheck() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit CalldataCheckAdded(checker);
        module.addCalldataCheck(checker, true);

        address[] memory checks = module.getAllCalldataChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        vm.expectEmit(true, true, false, false);
        emit CalldataCheckRemoved(checker);
        module.removeCalldataCheck(checker);
        checks = module.getAllCalldataChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateCalldataCheck() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checker, true);

        vm.expectRevert(RequireAllModule.AddressAlreadyExists.selector);
        module.addCalldataCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentCalldataCheck() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.AddressDoesNotExist.selector);
        module.removeCalldataCheck(checker);
        vm.stopPrank();
    }

    function testMultipleCalldataChecks() public {
        address checker1 = address(new MockCalldataAllowedTrue());
        address checker2 = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checker1, true);
        module.addCalldataCheck(checker2, false);
        vm.stopPrank();

        assertTrue(module.isCalldataAllowed(abi.encode("test data")));
    }

    function testAddCalldataCheckToTail() public {
        address checker1 = address(new MockCalldataAllowedTrue());
        address checker2 = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checker1, false); // Add to tail
        module.addCalldataCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllCalldataChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testOnlyOwnerModifiersForCalldataChecks() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addCalldataCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removeCalldataCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAddCalldata() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.addCalldataCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemoveCalldata() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAllModule.InvalidAddress.selector);
        module.removeCalldataCheck(address(0));
        vm.stopPrank();
    }
}
