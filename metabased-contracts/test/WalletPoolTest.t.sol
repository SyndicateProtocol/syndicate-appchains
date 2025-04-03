// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {WalletPool} from "src/WalletPool.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";

contract WalletPoolTest is Test {
    WalletPool walletPool;
    AllowlistSequencingModule allowlistModule1;
    AllowlistSequencingModule allowlistModule2;

    address admin = address(0x1);
    address manager = address(0x2);
    address wallet1 = address(0x3);
    address wallet2 = address(0x4);

    // Role constants
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    function setUp() public {
        // Deploy contracts
        vm.startPrank(admin);
        walletPool = new WalletPool(admin);
        allowlistModule1 = new AllowlistSequencingModule(address(walletPool));
        allowlistModule2 = new AllowlistSequencingModule(address(walletPool));

        // Grant manager role to manager address
        walletPool.grantRole(MANAGER_ROLE, manager);
        vm.stopPrank();
    }

    function testConstructorSetsRoles() public view {
        assertTrue(walletPool.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(walletPool.hasRole(MANAGER_ROLE, admin));
    }

    function testConstructorRejectsZeroAddress() public {
        vm.expectRevert(WalletPool.AddressNotAllowed.selector);
        new WalletPool(address(0));
    }

    function testManagerCanAddToWalletPool() public {
        address[] memory modules = new address[](0);

        vm.startPrank(manager);
        vm.expectEmit(true, false, false, false);
        emit WalletPool.WalletAddedToPool(wallet1);
        walletPool.addToWalletPool(wallet1, modules);
        assertTrue(walletPool.walletPool(wallet1));
        vm.stopPrank();
    }

    function testManagerCanRemoveFromWalletPool() public {
        address[] memory modules = new address[](0);

        vm.startPrank(manager);
        walletPool.addToWalletPool(wallet1, modules);
        vm.expectEmit(true, false, false, false);
        emit WalletPool.WalletRemovedFromPool(wallet1);
        walletPool.removeFromWalletPool(wallet1, modules);
        assertFalse(walletPool.walletPool(wallet1));
        vm.stopPrank();
    }

    function testNonManagerCannotAddToWalletPool() public {
        address[] memory modules = new address[](0);

        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", address(this), MANAGER_ROLE);
        vm.expectRevert(errorMsg);
        walletPool.addToWalletPool(wallet1, modules);
    }

    function testNonManagerCannotRemoveFromWalletPool() public {
        address[] memory modules = new address[](0);

        vm.startPrank(manager);
        walletPool.addToWalletPool(wallet1, modules);
        vm.stopPrank();

        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", address(this), MANAGER_ROLE);
        vm.expectRevert(errorMsg);
        walletPool.removeFromWalletPool(wallet1, modules);
    }

    function testIsInWalletPool() public {
        address[] memory modules = new address[](0);

        vm.startPrank(manager);
        walletPool.addToWalletPool(wallet1, modules);
        vm.stopPrank();

        assertTrue(walletPool.isInWalletPool(wallet1));
        assertFalse(walletPool.isInWalletPool(wallet2));
    }

    function testUpdateSingleAllowlistModule() public {
        vm.startPrank(manager);

        // Update allowlist module
        vm.expectEmit(true, true, false, false);
        emit WalletPool.AllowlistModuleUpdated(address(allowlistModule1), wallet1, true);
        walletPool.updateAllowlistModule(address(allowlistModule1), wallet1, true);

        // Verify wallet is allowed in the module
        assertTrue(allowlistModule1.allowlist(wallet1));

        // Remove from allowlist module
        walletPool.updateAllowlistModule(address(allowlistModule1), wallet1, false);

        // Verify wallet is not allowed in the module
        assertFalse(allowlistModule1.allowlist(wallet1));

        vm.stopPrank();
    }

    function testUpdateMultipleAllowlistModules() public {
        address[] memory modules = new address[](2);
        modules[0] = address(allowlistModule1);
        modules[1] = address(allowlistModule2);

        vm.startPrank(manager);

        // Update multiple allowlist modules
        walletPool.updateAllowlistModules(wallet1, true, modules);

        // Verify wallet is allowed in both modules
        assertTrue(allowlistModule1.allowlist(wallet1));
        assertTrue(allowlistModule2.allowlist(wallet1));

        // Remove from all allowlist modules
        walletPool.updateAllowlistModules(wallet1, false, modules);

        // Verify wallet is not allowed in both modules
        assertFalse(allowlistModule1.allowlist(wallet1));
        assertFalse(allowlistModule2.allowlist(wallet1));

        vm.stopPrank();
    }

    function testAddToWalletPoolWithModules() public {
        address[] memory modules = new address[](2);
        modules[0] = address(allowlistModule1);
        modules[1] = address(allowlistModule2);

        vm.startPrank(manager);

        // Add to wallet pool and update modules
        walletPool.addToWalletPool(wallet1, modules);

        // Verify wallet is in pool and allowed in both modules
        assertTrue(walletPool.walletPool(wallet1));
        assertTrue(allowlistModule1.allowlist(wallet1));
        assertTrue(allowlistModule2.allowlist(wallet1));

        vm.stopPrank();
    }

    function testRemoveFromWalletPoolWithModules() public {
        address[] memory modules = new address[](2);
        modules[0] = address(allowlistModule1);
        modules[1] = address(allowlistModule2);

        vm.startPrank(manager);

        // Add to wallet pool and update modules
        walletPool.addToWalletPool(wallet1, modules);

        // Remove from wallet pool and update modules
        walletPool.removeFromWalletPool(wallet1, modules);

        // Verify wallet is not in pool and not allowed in both modules
        assertFalse(walletPool.walletPool(wallet1));
        assertFalse(allowlistModule1.allowlist(wallet1));
        assertFalse(allowlistModule2.allowlist(wallet1));

        vm.stopPrank();
    }

    function testUpdateInvalidAllowlistModuleFails() public {
        address invalidModule = address(0x9999);

        vm.startPrank(manager);

        // This should revert with AllowlistUpdateFailed
        vm.expectRevert(WalletPool.AllowlistUpdateFailed.selector);
        walletPool.updateAllowlistModule(invalidModule, wallet1, true);

        vm.stopPrank();
    }

    function testRoleManagement() public {
        address newManager = makeAddr("newManager");
        address[] memory modules = new address[](0);

        vm.startPrank(admin);
        walletPool.grantRole(MANAGER_ROLE, newManager);
        vm.stopPrank();

        // New manager should be able to add to wallet pool
        vm.startPrank(newManager);
        walletPool.addToWalletPool(wallet1, modules);
        assertTrue(walletPool.walletPool(wallet1));
        vm.stopPrank();

        // Admin revokes role
        vm.startPrank(admin);
        walletPool.revokeRole(MANAGER_ROLE, newManager);
        vm.stopPrank();

        // New manager should no longer be able to add to wallet pool
        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", newManager, MANAGER_ROLE);
        vm.prank(newManager);
        vm.expectRevert(errorMsg);
        walletPool.addToWalletPool(wallet2, modules);
    }
}
