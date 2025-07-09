// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {RequireCompositeModule, BaseRequirementModule} from "src/requirement-modules/RequireCompositeModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

contract MockPermissionModule is IPermissionModule {
    bool private _shouldAllow;

    constructor(bool shouldAllow) {
        _shouldAllow = shouldAllow;
    }

    function setShouldAllow(bool shouldAllow) external {
        _shouldAllow = shouldAllow;
    }

    function isAllowed(address, address, bytes calldata) external view override returns (bool) {
        return _shouldAllow;
    }
}

contract RequireCompositeModuleTest is Test {
    RequireCompositeModule public compositeModule;
    address public admin;
    address public user1;
    address public user2;

    // Mock modules
    MockPermissionModule public alwaysAllowModule;
    MockPermissionModule public alwaysDenyModule;
    MockPermissionModule public configModule1;
    MockPermissionModule public configModule2;

    event PermissionCheckAdded(address indexed check);
    event PermissionCheckAddedWithType(address indexed check, RequireCompositeModule.CheckType indexed checkType);
    event PermissionCheckRemoved(address indexed check);
    event CheckTypeUpdated(
        address indexed check,
        RequireCompositeModule.CheckType indexed oldType,
        RequireCompositeModule.CheckType indexed newType
    );

    function setUp() public {
        admin = address(0x1);
        user1 = address(0x2);
        user2 = address(0x3);

        vm.startPrank(admin);
        compositeModule = new RequireCompositeModule(admin);

        // Initialize mock modules
        alwaysAllowModule = new MockPermissionModule(true);
        alwaysDenyModule = new MockPermissionModule(false);
        configModule1 = new MockPermissionModule(true);
        configModule2 = new MockPermissionModule(true);
        vm.stopPrank();
    }

    // Constructor Tests
    function testConstructor() public view {
        assertEq(compositeModule.owner(), admin);
    }

    // Permission Check Addition Tests
    function testAddPermissionCheck() public {
        vm.startPrank(admin);

        // Test default addition (AND type)
        vm.expectEmit(true, true, false, true);
        emit PermissionCheckAdded(address(alwaysAllowModule));
        vm.expectEmit(true, true, false, true);
        emit PermissionCheckAddedWithType(address(alwaysAllowModule), RequireCompositeModule.CheckType.AND);
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Verify it was added
        address[] memory checks = compositeModule.getAllPermissionChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], address(alwaysAllowModule));

        // Verify the type is AND
        assertEq(
            uint256(compositeModule.checkTypes(address(alwaysAllowModule))),
            uint256(RequireCompositeModule.CheckType.AND)
        );

        vm.stopPrank();
    }

    function testAddPermissionCheckWithType() public {
        vm.startPrank(admin);

        // Add as OR type
        vm.expectEmit(true, true, false, true);
        emit PermissionCheckAdded(address(alwaysAllowModule));
        vm.expectEmit(true, true, false, true);
        emit PermissionCheckAddedWithType(address(alwaysAllowModule), RequireCompositeModule.CheckType.OR);
        compositeModule.addPermissionCheckWithType(
            address(alwaysAllowModule), RequireCompositeModule.CheckType.OR, true
        );

        // Verify it was added
        address[] memory checks = compositeModule.getAllPermissionChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], address(alwaysAllowModule));

        // Verify the type is OR
        assertEq(
            uint256(compositeModule.checkTypes(address(alwaysAllowModule))),
            uint256(RequireCompositeModule.CheckType.OR)
        );

        vm.stopPrank();
    }

    function testCannotAddSameCheckTwice() public {
        vm.startPrank(admin);

        // Add check once
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Try to add again - should revert
        vm.expectRevert(BaseRequirementModule.AddressAlreadyExists.selector);
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        vm.stopPrank();
    }

    function testCannotAddZeroAddress() public {
        vm.startPrank(admin);

        vm.expectRevert(BaseRequirementModule.InvalidAddress.selector);
        compositeModule.addPermissionCheck(address(0), true);

        vm.stopPrank();
    }

    function testOnlyOwnerCanAddChecks() public {
        vm.startPrank(user1);

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, user1));
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        vm.stopPrank();
    }

    // Permission Check Removal Tests
    function testRemovePermissionCheck() public {
        vm.startPrank(admin);

        // Add check
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Remove check
        vm.expectEmit(true, false, false, true);
        emit PermissionCheckRemoved(address(alwaysAllowModule));
        compositeModule.removePermissionCheck(address(alwaysAllowModule));

        // Verify it was removed
        address[] memory checks = compositeModule.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Check type should be cleared
        assertEq(uint256(compositeModule.checkTypes(address(alwaysAllowModule))), 0);

        vm.stopPrank();
    }

    function testCannotRemoveNonExistentCheck() public {
        vm.startPrank(admin);

        vm.expectRevert(BaseRequirementModule.AddressDoesNotExist.selector);
        compositeModule.removePermissionCheck(address(alwaysAllowModule));

        vm.stopPrank();
    }

    // Update Check Type Tests
    function testUpdateCheckType() public {
        vm.startPrank(admin);

        // Add check with default AND type
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Update to OR type
        vm.expectEmit(true, true, true, true);
        emit CheckTypeUpdated(
            address(alwaysAllowModule), RequireCompositeModule.CheckType.AND, RequireCompositeModule.CheckType.OR
        );
        compositeModule.updateCheckType(address(alwaysAllowModule), RequireCompositeModule.CheckType.OR);

        // Verify type was updated
        assertEq(
            uint256(compositeModule.checkTypes(address(alwaysAllowModule))),
            uint256(RequireCompositeModule.CheckType.OR)
        );

        vm.stopPrank();
    }

    function testNoEventWhenTypeNotChanged() public {
        vm.startPrank(admin);

        // Add check with OR type
        compositeModule.addPermissionCheckWithType(
            address(alwaysAllowModule), RequireCompositeModule.CheckType.OR, true
        );

        // Try to update to same OR type
        compositeModule.updateCheckType(address(alwaysAllowModule), RequireCompositeModule.CheckType.OR);
        // No event should be emitted

        vm.stopPrank();
    }

    function testCannotUpdateTypeForNonExistentCheck() public {
        vm.startPrank(admin);

        vm.expectRevert(BaseRequirementModule.AddressDoesNotExist.selector);
        compositeModule.updateCheckType(address(alwaysAllowModule), RequireCompositeModule.CheckType.OR);

        vm.stopPrank();
    }

    // Get All Checks With Types Tests
    function testGetAllPermissionChecksWithTypes() public {
        vm.startPrank(admin);

        // Add checks with different types
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true); // AND type
        compositeModule.addPermissionCheckWithType(address(alwaysDenyModule), RequireCompositeModule.CheckType.OR, true); // OR type

        // Get checks with types
        (address[] memory addresses, RequireCompositeModule.CheckType[] memory types) =
            compositeModule.getAllPermissionChecksWithTypes();

        // Verify results
        assertEq(addresses.length, 2);
        assertEq(types.length, 2);

        // Since we added with addToHead=true, the order is reversed
        assertEq(addresses[0], address(alwaysDenyModule));
        assertEq(addresses[1], address(alwaysAllowModule));
        assertEq(uint256(types[0]), uint256(RequireCompositeModule.CheckType.OR));
        assertEq(uint256(types[1]), uint256(RequireCompositeModule.CheckType.AND));

        vm.stopPrank();
    }

    // isAllowed Tests
    function testIsAllowedNoChecks() public view {
        // With no checks, isAllowed should return true
        bool result = compositeModule.isAllowed(user1, user2, bytes(""));
        assertTrue(result);
    }

    function testIsAllowedOnlyAndChecksPass() public {
        vm.startPrank(admin);

        // Add two AND checks, both allow
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);
        compositeModule.addPermissionCheck(address(configModule1), true);

        vm.stopPrank();

        // Should pass as both AND checks pass
        bool result = compositeModule.isAllowed(user1, user2, bytes(""));
        assertTrue(result);
    }

    function testIsAllowedOnlyAndChecksOneFails() public {
        vm.startPrank(admin);

        // Add one allowing AND check and one denying AND check
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);
        compositeModule.addPermissionCheck(address(alwaysDenyModule), true);

        vm.stopPrank();

        // Should fail as one AND check fails
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireCompositeModule.CompositeAndPermissionCheckFailed.selector,
                address(alwaysDenyModule),
                user1,
                bytes("")
            )
        );
        compositeModule.isAllowed(user1, user2, bytes(""));
    }

    function testIsAllowedOnlyOrChecksOnePass() public {
        vm.startPrank(admin);

        // Add one allowing OR check and one denying OR check
        compositeModule.addPermissionCheckWithType(
            address(alwaysAllowModule), RequireCompositeModule.CheckType.OR, true
        );
        compositeModule.addPermissionCheckWithType(address(alwaysDenyModule), RequireCompositeModule.CheckType.OR, true);

        vm.stopPrank();

        // Should pass as at least one OR check passes
        bool result = compositeModule.isAllowed(user1, user2, bytes(""));
        assertTrue(result);
    }

    function testIsAllowedOnlyOrChecksAllFail() public {
        vm.startPrank(admin);

        // Add two denying OR checks
        compositeModule.addPermissionCheckWithType(address(alwaysDenyModule), RequireCompositeModule.CheckType.OR, true);
        configModule1.setShouldAllow(false);
        compositeModule.addPermissionCheckWithType(address(configModule1), RequireCompositeModule.CheckType.OR, true);

        vm.stopPrank();

        // Should fail as all OR checks fail
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireCompositeModule.CompositeAllOrPermissionChecksFailed.selector, user1, bytes("")
            )
        );
        compositeModule.isAllowed(user1, user2, bytes(""));
    }

    function testIsAllowedMixedChecksAndFailOrPass() public {
        vm.startPrank(admin);

        // Add one denying AND check
        compositeModule.addPermissionCheck(address(alwaysDenyModule), true);

        // Add one allowing OR check
        compositeModule.addPermissionCheckWithType(
            address(alwaysAllowModule), RequireCompositeModule.CheckType.OR, true
        );

        vm.stopPrank();

        // Should fail as the AND check fails
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireCompositeModule.CompositeAndPermissionCheckFailed.selector,
                address(alwaysDenyModule),
                user1,
                bytes("")
            )
        );
        compositeModule.isAllowed(user1, user2, bytes(""));
    }

    function testIsAllowedMixedChecksAndPassOrFail() public {
        vm.startPrank(admin);

        // Add one allowing AND check
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Add one denying OR check
        compositeModule.addPermissionCheckWithType(address(alwaysDenyModule), RequireCompositeModule.CheckType.OR, true);

        vm.stopPrank();

        // Should fail as the OR check fails and there are no passing OR checks
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireCompositeModule.CompositeAllOrPermissionChecksFailed.selector, user1, bytes("")
            )
        );
        compositeModule.isAllowed(user1, user2, bytes(""));
    }

    function testIsAllowedMixedChecksAllPass() public {
        vm.startPrank(admin);

        // Add one allowing AND check
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true);

        // Add one allowing OR check
        compositeModule.addPermissionCheckWithType(address(configModule1), RequireCompositeModule.CheckType.OR, true);

        vm.stopPrank();

        // Should pass as all checks pass
        bool result = compositeModule.isAllowed(user1, user2, bytes(""));
        assertTrue(result);
    }

    function testModifyChecksDuringExecution() public {
        vm.startPrank(admin);

        // Add modules in a configuration that would pass
        compositeModule.addPermissionCheck(address(alwaysAllowModule), true); // AND
        compositeModule.addPermissionCheckWithType(address(configModule1), RequireCompositeModule.CheckType.OR, true); // OR

        // Configure to fail during execution
        configModule1.setShouldAllow(false);

        // Add another OR check that passes
        compositeModule.addPermissionCheckWithType(address(configModule2), RequireCompositeModule.CheckType.OR, true); // OR

        vm.stopPrank();

        // Should still pass as one OR check passes
        bool result = compositeModule.isAllowed(user1, user2, bytes(""));
        assertTrue(result);

        // Now make all OR checks fail
        vm.startPrank(admin);
        configModule2.setShouldAllow(false);
        vm.stopPrank();

        // Should now fail
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireCompositeModule.CompositeAllOrPermissionChecksFailed.selector, user1, bytes("")
            )
        );
        compositeModule.isAllowed(user1, user2, bytes(""));
    }
}
