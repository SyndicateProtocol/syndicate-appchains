// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateStorage} from "src/backfill/SyndicateStorage.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";

contract SyndicateStorageTest is Test {
    SyndicateStorage public syndicateStorage;
    address public admin;
    address public manager;
    address public nonAuthorized;
    uint256 public appChainId;

    // Event to test against
    event EpochRangeProcessed(uint256 indexed startEpochNumber, uint256 indexed endEpochNumber);

    function setUp() public {
        admin = address(1);
        manager = address(2);
        nonAuthorized = address(3);
        appChainId = 123;

        // Deploy the contract
        syndicateStorage = new SyndicateStorage(admin, manager, appChainId);
    }

    // ======== Constructor Tests ========

    function testConstructor() public view {
        // Check roles are assigned correctly
        assertTrue(syndicateStorage.hasRole(syndicateStorage.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(syndicateStorage.hasRole(syndicateStorage.MANAGER_ROLE(), manager));

        // Check appChainId is set correctly
        assertEq(syndicateStorage.appChainId(), appChainId);

        // Check initial index is zero
        assertEq(syndicateStorage.indexFromBlock(), 0);
    }

    function testConstructorZeroAdmin() public {
        vm.expectRevert("Admin address cannot be 0");
        new SyndicateStorage(address(0), manager, appChainId);
    }

    function testConstructorZeroManager() public {
        vm.expectRevert("Manager address cannot be 0");
        new SyndicateStorage(admin, address(0), appChainId);
    }

    function testConstructorZeroAppChainId() public {
        vm.expectRevert("App chain ID cannot be 0");
        new SyndicateStorage(admin, manager, 0);
    }

    // ======== setIndexFromBlock Tests ========

    function testSetIndexFromBlock() public {
        uint256 blockNumber = 100;

        // Call function as manager
        vm.prank(manager);
        syndicateStorage.setIndexFromBlock(blockNumber);

        // Check that index is set correctly
        assertEq(syndicateStorage.indexFromBlock(), blockNumber);
    }

    function testSetIndexFromBlockNonManager() public {
        uint256 blockNumber = 100;

        // Try to call as non-manager
        vm.startPrank(nonAuthorized);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, nonAuthorized, syndicateStorage.MANAGER_ROLE()
            )
        );
        syndicateStorage.setIndexFromBlock(blockNumber);
        vm.stopPrank();
    }

    // ======== save Tests ========

    function testSave() public {
        uint256 epochNumber = 10;
        bytes32 epochHash = keccak256("test");
        bytes memory batch = abi.encode("test batch");

        // Expect event to be emitted
        vm.expectEmit(true, true, true, true);
        emit EpochRangeProcessed(epochNumber, epochNumber);

        // Call function as manager
        vm.prank(manager);
        syndicateStorage.save(epochNumber, epochHash, batch);
    }

    function testSaveNonManager() public {
        uint256 epochNumber = 10;
        bytes32 epochHash = keccak256("test");
        bytes memory batch = abi.encode("test batch");

        // Try to call as non-manager
        vm.startPrank(nonAuthorized);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, nonAuthorized, syndicateStorage.MANAGER_ROLE()
            )
        );
        syndicateStorage.save(epochNumber, epochHash, batch);
        vm.stopPrank();
    }

    // ======== saveForMany Tests ========

    function testSaveForMany() public {
        uint256[] memory epochNumbers = new uint256[](3);
        epochNumbers[0] = 10;
        epochNumbers[1] = 11;
        epochNumbers[2] = 12;

        bytes32[] memory epochHashes = new bytes32[](3);
        epochHashes[0] = keccak256("test1");
        epochHashes[1] = keccak256("test2");
        epochHashes[2] = keccak256("test3");

        bytes[] memory batches = new bytes[](3);
        batches[0] = abi.encode("test batch 1");
        batches[1] = abi.encode("test batch 2");
        batches[2] = abi.encode("test batch 3");

        // Expect event to be emitted with first and last epoch numbers
        vm.expectEmit(true, true, true, true);
        emit EpochRangeProcessed(epochNumbers[0], epochNumbers[2]);

        // Call function as manager
        vm.prank(manager);
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyWithSingleEpoch() public {
        uint256[] memory epochNumbers = new uint256[](1);
        epochNumbers[0] = 10;

        bytes32[] memory epochHashes = new bytes32[](1);
        epochHashes[0] = keccak256("test");

        bytes[] memory batches = new bytes[](1);
        batches[0] = abi.encode("test batch");

        // Expect event to be emitted with same start and end epoch
        vm.expectEmit(true, true, true, true);
        emit EpochRangeProcessed(epochNumbers[0], epochNumbers[0]);

        // Call function as manager
        vm.prank(manager);
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyArrayLengthMismatch1() public {
        uint256[] memory epochNumbers = new uint256[](3);
        epochNumbers[0] = 10;
        epochNumbers[1] = 11;
        epochNumbers[2] = 12;

        bytes32[] memory epochHashes = new bytes32[](2); // Different length
        epochHashes[0] = keccak256("test1");
        epochHashes[1] = keccak256("test2");

        bytes[] memory batches = new bytes[](3);
        batches[0] = abi.encode("test batch 1");
        batches[1] = abi.encode("test batch 2");
        batches[2] = abi.encode("test batch 3");

        // Expect revert due to array length mismatch
        vm.prank(manager);
        vm.expectRevert("Array lengths must be equal");
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyArrayLengthMismatch2() public {
        uint256[] memory epochNumbers = new uint256[](3);
        epochNumbers[0] = 10;
        epochNumbers[1] = 11;
        epochNumbers[2] = 12;

        bytes32[] memory epochHashes = new bytes32[](3);
        epochHashes[0] = keccak256("test1");
        epochHashes[1] = keccak256("test2");
        epochHashes[2] = keccak256("test3");

        bytes[] memory batches = new bytes[](2); // Different length
        batches[0] = abi.encode("test batch 1");
        batches[1] = abi.encode("test batch 2");

        // Expect revert due to array length mismatch
        vm.prank(manager);
        vm.expectRevert("Array lengths must be equal");
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyEmptyArrays() public {
        uint256[] memory epochNumbers = new uint256[](0);
        bytes32[] memory epochHashes = new bytes32[](0);
        bytes[] memory batches = new bytes[](0);

        // Calling with empty arrays should revert because we can't access
        // epochNumbers[0] and epochNumbers[length-1] to emit the event
        vm.prank(manager);
        vm.expectRevert();
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyNonManager() public {
        uint256[] memory epochNumbers = new uint256[](1);
        epochNumbers[0] = 10;

        bytes32[] memory epochHashes = new bytes32[](1);
        epochHashes[0] = keccak256("test");

        bytes[] memory batches = new bytes[](1);
        batches[0] = abi.encode("test batch");

        // Try to call as non-manager
        vm.startPrank(nonAuthorized);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, nonAuthorized, syndicateStorage.MANAGER_ROLE()
            )
        );
        syndicateStorage.saveForMany(epochNumbers, epochHashes, batches);
        vm.stopPrank();
    }

    // ======== Role Management Tests ========

    function testRoleManagement() public {
        address newManager = address(4);

        // Admin can grant manager role
        vm.startPrank(admin);
        syndicateStorage.grantRole(syndicateStorage.MANAGER_ROLE(), newManager);
        assertTrue(syndicateStorage.hasRole(syndicateStorage.MANAGER_ROLE(), newManager));

        // Admin can revoke manager role
        syndicateStorage.revokeRole(syndicateStorage.MANAGER_ROLE(), newManager);
        assertFalse(syndicateStorage.hasRole(syndicateStorage.MANAGER_ROLE(), newManager));
        vm.stopPrank();
    }
}
