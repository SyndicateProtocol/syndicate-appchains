// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireAnyModule, Ownable} from "src/requirement-modules/RequireAnyModule.sol";
import {ProposerPermissionModule} from "src/interfaces/ProposerPermissionModule.sol";
import {CalldataPermissionModule} from "src/interfaces/CalldataPermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract MockProposerAllowedTrue is ProposerPermissionModule {
    function isAllowed(address) external pure returns (bool) {
        return true;
    }
}

contract MockProposerAllowedFalse is ProposerPermissionModule {
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

contract RequireAnyModuleTest is Test {
    RequireAnyModule public module;
    address public admin;
    address public nonAdmin;

    // Events to check emission
    event ProposerCheckAdded(address indexed check);
    event ProposerCheckRemoved(address indexed check);
    event CalldataCheckAdded(address indexed check);
    event CalldataCheckRemoved(address indexed check);

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        module = new RequireAnyModule(admin);
    }

    // ----------------------
    // Proposer Check Tests
    // ----------------------

    function testIsAllowedNoChecks() public {
        // Should pass if no checks exist
        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedPassOne() public {
        address checkerTrue = address(new MockProposerAllowedTrue());
        address checkerFalse = address(new MockProposerAllowedFalse());

        vm.startPrank(admin);
        module.addProposerCheck(checkerFalse, true);
        module.addProposerCheck(checkerTrue, true); // Add to front
        vm.stopPrank();

        // Should pass as long as at least one check passes
        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedPassOnlyOne() public {
        address checkerTrue = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checkerTrue, true);
        vm.stopPrank();

        assertTrue(module.isAllowed(address(this)));
    }

    function testIsAllowedFailure() public {
        address checkerFalse1 = address(new MockProposerAllowedFalse());
        address checkerFalse2 = address(new MockProposerAllowedFalse());

        vm.startPrank(admin);
        module.addProposerCheck(checkerFalse1, true);
        module.addProposerCheck(checkerFalse2, false);
        vm.stopPrank();

        // Should fail if all checks fail
        vm.expectRevert(abi.encodeWithSelector(RequireAnyModule.ProposerCheckFailed.selector, address(this)));
        module.isAllowed(address(this));
    }

    function testAddRemoveProposerCheck() public {
        address checker = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);

        // Test adding a checker
        vm.expectEmit(true, true, false, false);
        emit ProposerCheckAdded(checker);
        module.addProposerCheck(checker, true);

        address[] memory checks = module.getAllProposerChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        // Test removing a checker
        vm.expectEmit(true, true, false, false);
        emit ProposerCheckRemoved(checker);
        module.removeProposerCheck(checker);

        checks = module.getAllProposerChecks();
        assertEq(checks.length, 0);
        vm.stopPrank();
    }

    function testAddDuplicateProposerCheck() public {
        address checker = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker, true);

        vm.expectRevert(RequireAnyModule.AddressAlreadyExists.selector);
        module.addProposerCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentProposerCheck() public {
        address checker = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.AddressDoesNotExist.selector);
        module.removeProposerCheck(checker);
        vm.stopPrank();
    }

    function testAddProposerCheckToTail() public {
        address checker1 = address(new MockProposerAllowedTrue());
        address checker2 = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker1, false); // Add to tail
        module.addProposerCheck(checker2, false); // Add to tail
        vm.stopPrank();

        address[] memory checks = module.getAllProposerChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker1);
        assertEq(checks[1], checker2);
    }

    function testAddProposerCheckToHead() public {
        address checker1 = address(new MockProposerAllowedTrue());
        address checker2 = address(new MockProposerAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(checker1, true); // Add to head
        module.addProposerCheck(checker2, true); // Add to head
        vm.stopPrank();

        address[] memory checks = module.getAllProposerChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker2); // checker2 should be first since it was added to head last
        assertEq(checks[1], checker1);
    }

    function testOnlyOwnerModifiersForProposerChecks() public {
        address checker = address(new MockProposerAllowedTrue());

        vm.startPrank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.addProposerCheck(checker, true);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        module.removeProposerCheck(checker);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressAddProposer() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.InvalidAddress.selector);
        module.addProposerCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemoveProposer() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.InvalidAddress.selector);
        module.removeProposerCheck(address(0));
        vm.stopPrank();
    }

    // ----------------------
    // Calldata Check Tests
    // ----------------------

    function testIsCalldataAllowedNoChecks() public {
        // Should pass if no checks exist
        assertTrue(module.isCalldataAllowed(abi.encode("test data")));
    }

    function testIsCalldataAllowedPassOne() public {
        address checkerTrue = address(new MockCalldataAllowedTrue());
        address checkerFalse = address(new MockCalldataAllowedFalse());

        vm.startPrank(admin);
        module.addCalldataCheck(checkerFalse, true);
        module.addCalldataCheck(checkerTrue, true); // Add to front
        vm.stopPrank();

        // Should pass as long as at least one check passes
        assertTrue(module.isCalldataAllowed(abi.encode("test data")));
    }

    function testIsCalldataAllowedPassOnlyOne() public {
        address checkerTrue = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checkerTrue, true);
        vm.stopPrank();

        assertTrue(module.isCalldataAllowed(abi.encode("test data")));
    }

    function testIsCalldataAllowedFailure() public {
        address checkerFalse1 = address(new MockCalldataAllowedFalse());
        address checkerFalse2 = address(new MockCalldataAllowedFalse());

        vm.startPrank(admin);
        module.addCalldataCheck(checkerFalse1, true);
        module.addCalldataCheck(checkerFalse2, false);
        vm.stopPrank();

        // Should fail if all checks fail
        vm.expectRevert(RequireAnyModule.CalldataCheckFailed.selector);
        module.isCalldataAllowed(abi.encode("test data"));
    }

    function testAddRemoveCalldataCheck() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);

        // Test adding a checker
        vm.expectEmit(true, true, false, false);
        emit CalldataCheckAdded(checker);
        module.addCalldataCheck(checker, true);

        address[] memory checks = module.getAllCalldataChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], checker);

        // Test removing a checker
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

        vm.expectRevert(RequireAnyModule.AddressAlreadyExists.selector);
        module.addCalldataCheck(checker, true);
        vm.stopPrank();
    }

    function testRemoveNonExistentCalldataCheck() public {
        address checker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.AddressDoesNotExist.selector);
        module.removeCalldataCheck(checker);
        vm.stopPrank();
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

    function testAddCalldataCheckToHead() public {
        address checker1 = address(new MockCalldataAllowedTrue());
        address checker2 = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addCalldataCheck(checker1, true); // Add to head
        module.addCalldataCheck(checker2, true); // Add to head
        vm.stopPrank();

        address[] memory checks = module.getAllCalldataChecks();
        assertEq(checks.length, 2);
        assertEq(checks[0], checker2); // checker2 should be first since it was added to head last
        assertEq(checks[1], checker1);
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
        vm.expectRevert(RequireAnyModule.InvalidAddress.selector);
        module.addCalldataCheck(address(0), true);
        vm.stopPrank();
    }

    function testRevertsOnZeroAddressRemoveCalldata() public {
        vm.startPrank(admin);
        vm.expectRevert(RequireAnyModule.InvalidAddress.selector);
        module.removeCalldataCheck(address(0));
        vm.stopPrank();
    }

    // -----------------------
    // Integration Tests
    // -----------------------

    function testIndependentProposerAndCalldataLists() public {
        address proposerChecker = address(new MockProposerAllowedTrue());
        address calldataChecker = address(new MockCalldataAllowedTrue());

        vm.startPrank(admin);
        module.addProposerCheck(proposerChecker, true);
        module.addCalldataCheck(calldataChecker, true);
        vm.stopPrank();

        // Verify that both lists contain the correct checkers
        address[] memory proposerChecks = module.getAllProposerChecks();
        address[] memory calldataChecks = module.getAllCalldataChecks();

        assertEq(proposerChecks.length, 1);
        assertEq(calldataChecks.length, 1);

        assertEq(proposerChecks[0], proposerChecker);
        assertEq(calldataChecks[0], calldataChecker);
    }

    function testEmptyGetAllChecks() public {
        // Test that the get all functions work with empty lists
        address[] memory proposerChecks = module.getAllProposerChecks();
        address[] memory calldataChecks = module.getAllCalldataChecks();

        assertEq(proposerChecks.length, 0);
        assertEq(calldataChecks.length, 0);
    }

    function testCheckCombinations() public {
        address proposerTrue = address(new MockProposerAllowedTrue());
        address proposerFalse = address(new MockProposerAllowedFalse());
        address calldataTrue = address(new MockCalldataAllowedTrue());
        address calldataFalse = address(new MockCalldataAllowedFalse());

        vm.startPrank(admin);

        // Test different combinations of checkers

        // 1. Both pass: True + False
        module.addProposerCheck(proposerTrue, true);
        module.addProposerCheck(proposerFalse, false);
        module.addCalldataCheck(calldataTrue, true);
        module.addCalldataCheck(calldataFalse, false);

        assertTrue(module.isAllowed(address(this)));
        assertTrue(module.isCalldataAllowed(abi.encode("test data")));

        // Reset
        module.removeProposerCheck(proposerTrue);
        module.removeProposerCheck(proposerFalse);
        module.removeCalldataCheck(calldataTrue);
        module.removeCalldataCheck(calldataFalse);

        // 2. Both fail: False only
        module.addProposerCheck(proposerFalse, true);
        module.addCalldataCheck(calldataFalse, true);

        vm.expectRevert(abi.encodeWithSelector(RequireAnyModule.ProposerCheckFailed.selector, address(this)));
        module.isAllowed(address(this));

        vm.expectRevert(RequireAnyModule.CalldataCheckFailed.selector);
        module.isCalldataAllowed(abi.encode("test data"));

        vm.stopPrank();
    }
}
