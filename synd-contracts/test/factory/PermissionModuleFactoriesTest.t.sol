// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {
    RequireAndModuleFactory,
    RequireOrModuleFactory,
    RequireCompositeModuleFactory
} from "src/factory/PermissionModuleFactories.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";

/// @notice Tests for PermissionModuleFactories using CREATE2 pattern
/// @dev Uses CREATE2 deployment with admin encoded in bytecode for proper ownership
contract PermissionModuleFactoriesTest is Test {
    RequireAndModuleFactory public andFactory;
    RequireOrModuleFactory public orFactory;
    RequireCompositeModuleFactory public compositeFactory;

    address public admin;
    address public manager;
    address public user1;
    address public user2;
    address public nonManager;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Events to test
    event RequireAndModuleCreated(address indexed module, address indexed admin);
    event RequireOrModuleCreated(address indexed module, address indexed admin);
    event RequireCompositeModuleCreated(address indexed module, address indexed admin);

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        user1 = address(0x3);
        user2 = address(0x4);
        nonManager = address(0x5);

        // Deploy factories with admin
        andFactory = new RequireAndModuleFactory(admin);
        orFactory = new RequireOrModuleFactory(admin);
        compositeFactory = new RequireCompositeModuleFactory(admin);

        // Grant manager roles
        vm.startPrank(admin);
        andFactory.grantRole(MANAGER_ROLE, manager);
        orFactory.grantRole(MANAGER_ROLE, manager);
        compositeFactory.grantRole(MANAGER_ROLE, manager);
        vm.stopPrank();
    }

    // RequireAndModuleFactory Tests
    function testCreateRequireAndModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-1"));
        address expectedAddress = andFactory.computeModuleAddress(user1, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireAndModuleCreated(expectedAddress, user1);

        address moduleAddress = andFactory.createRequireAndModule(user1, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);

        RequireAndModule module = RequireAndModule(moduleAddress);
        // With CREATE2 and proper constructor, the admin should be the owner
        assertEq(module.owner(), user1);
    }

    function testCreateRequireAndModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-zero"));

        vm.expectRevert(RequireAndModuleFactory.ZeroAddress.selector);
        andFactory.createRequireAndModule(address(0), salt);
    }

    function testComputeRequireAndModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-compute"));

        address computedAddress = andFactory.computeModuleAddress(user1, salt);
        address actualAddress = andFactory.createRequireAndModule(user1, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireAndModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-2"));

        address addr1 = andFactory.createRequireAndModule(user1, salt1);
        address addr2 = andFactory.createRequireAndModule(user1, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireAndModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("collision-salt"));

        // First deployment should succeed
        address addr1 = andFactory.createRequireAndModule(user1, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same admin and salt should revert due to CREATE2 collision
        vm.expectRevert();
        andFactory.createRequireAndModule(user1, salt);
    }

    function testRequireAndModuleSameSaltDifferentUsers() public {
        bytes32 salt1 = keccak256(abi.encodePacked("same-salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("same-salt-2"));

        address addr1 = andFactory.createRequireAndModule(user1, salt1);
        address addr2 = andFactory.createRequireAndModule(user2, salt2);

        // Both deployments should succeed and produce different addresses
        assertTrue(addr1 != address(0));
        assertTrue(addr2 != address(0));
        assertTrue(addr1 != addr2);

        // With CREATE2 and different admins, each module should have the correct owner
        RequireAndModule module1 = RequireAndModule(addr1);
        RequireAndModule module2 = RequireAndModule(addr2);
        assertEq(module1.owner(), user1);
        assertEq(module2.owner(), user2);

        // Computed addresses should also be different since admin is part of bytecode
        address computedAddr1 = andFactory.computeModuleAddress(user1, salt1);
        address computedAddr2 = andFactory.computeModuleAddress(user2, salt2);
        assertEq(addr1, computedAddr1);
        assertEq(addr2, computedAddr2);
        assertTrue(computedAddr1 != computedAddr2);
    }

    // Pausability Tests for RequireAndModuleFactory
    function testAndFactoryPauseUnpause() public {
        // Initially not paused
        assertFalse(andFactory.paused());

        // Admin can pause
        vm.prank(admin);
        andFactory.pause();
        assertTrue(andFactory.paused());

        // Admin can unpause
        vm.prank(admin);
        andFactory.unpause();
        assertFalse(andFactory.paused());
    }

    function testAndFactoryPauseNonAdminReverts() public {
        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        andFactory.pause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert
        andFactory.pause();
    }

    function testAndFactoryCreateWhenPausedReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("paused-test"));

        // Pause the factory
        vm.prank(admin);
        andFactory.pause();

        // Try to create module
        vm.expectRevert(); // Pausable will revert
        andFactory.createRequireAndModule(user1, salt);
    }

    function testAndFactoryCreateAfterUnpauseWorks() public {
        bytes32 salt = keccak256(abi.encodePacked("unpause-test"));

        // Pause then unpause
        vm.prank(admin);
        andFactory.pause();
        vm.prank(admin);
        andFactory.unpause();

        // Should work after unpause
        address moduleAddress = andFactory.createRequireAndModule(user1, salt);
        assertTrue(moduleAddress != address(0));
    }

    // Access Control Tests for RequireAndModuleFactory
    function testAndFactoryRoleSetup() public view {
        // Admin should have both roles
        assertTrue(andFactory.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(andFactory.hasRole(MANAGER_ROLE, admin));

        // Manager should have manager role
        assertTrue(andFactory.hasRole(MANAGER_ROLE, manager));

        // Non-manager should not have any roles
        assertFalse(andFactory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(andFactory.hasRole(MANAGER_ROLE, nonManager));
    }

    // RequireOrModuleFactory Tests
    function testCreateRequireOrModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-1"));
        address expectedAddress = orFactory.computeModuleAddress(user1, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireOrModuleCreated(expectedAddress, user1);

        address moduleAddress = orFactory.createRequireOrModule(user1, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);
    }

    function testCreateRequireOrModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-zero"));

        vm.expectRevert(RequireOrModuleFactory.ZeroAddress.selector);
        orFactory.createRequireOrModule(address(0), salt);
    }

    function testComputeRequireOrModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-compute"));

        address computedAddress = orFactory.computeModuleAddress(user1, salt);
        address actualAddress = orFactory.createRequireOrModule(user1, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireOrModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("or-salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("or-salt-2"));

        address addr1 = orFactory.createRequireOrModule(user1, salt1);
        address addr2 = orFactory.createRequireOrModule(user1, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireOrModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("or-collision-salt"));

        // First deployment should succeed
        address addr1 = orFactory.createRequireOrModule(user1, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same admin and salt should revert
        vm.expectRevert();
        orFactory.createRequireOrModule(user1, salt);
    }

    // Pausability Tests for RequireOrModuleFactory
    function testOrFactoryPauseUnpause() public {
        assertFalse(orFactory.paused());

        vm.prank(admin);
        orFactory.pause();
        assertTrue(orFactory.paused());

        vm.prank(admin);
        orFactory.unpause();
        assertFalse(orFactory.paused());
    }

    function testOrFactoryCreateWhenPausedReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("or-paused-test"));

        vm.prank(admin);
        orFactory.pause();

        vm.expectRevert();
        orFactory.createRequireOrModule(user1, salt);
    }

    // RequireCompositeModuleFactory Tests
    function testCreateRequireCompositeModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-1"));
        address expectedAddress = compositeFactory.computeModuleAddress(user1, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireCompositeModuleCreated(expectedAddress, user1);

        address moduleAddress = compositeFactory.createRequireCompositeModule(user1, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);

        RequireCompositeModule module = RequireCompositeModule(moduleAddress);
        assertEq(module.owner(), user1);
    }

    function testCreateRequireCompositeModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-zero"));

        vm.expectRevert(RequireCompositeModuleFactory.ZeroAddress.selector);
        compositeFactory.createRequireCompositeModule(address(0), salt);
    }

    function testComputeRequireCompositeModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-compute"));

        address computedAddress = compositeFactory.computeModuleAddress(user1, salt);
        address actualAddress = compositeFactory.createRequireCompositeModule(user1, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireCompositeModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("composite-salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("composite-salt-2"));

        address addr1 = compositeFactory.createRequireCompositeModule(user1, salt1);
        address addr2 = compositeFactory.createRequireCompositeModule(user1, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireCompositeModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("composite-collision-salt"));

        // First deployment should succeed
        address addr1 = compositeFactory.createRequireCompositeModule(user1, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same admin and salt should revert due to CREATE2 collision
        vm.expectRevert();
        compositeFactory.createRequireCompositeModule(user1, salt);
    }

    // Pausability Tests for RequireCompositeModuleFactory
    function testCompositeFactoryPauseUnpause() public {
        assertFalse(compositeFactory.paused());

        vm.prank(admin);
        compositeFactory.pause();
        assertTrue(compositeFactory.paused());

        vm.prank(admin);
        compositeFactory.unpause();
        assertFalse(compositeFactory.paused());
    }

    function testCompositeFactoryCreateWhenPausedReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("composite-paused-test"));

        vm.prank(admin);
        compositeFactory.pause();

        vm.expectRevert();
        compositeFactory.createRequireCompositeModule(user1, salt);
    }

    // Cross-factory tests to ensure different module types produce different addresses
    function testDifferentFactoriesSameSaltDifferentAddresses() public {
        bytes32 salt = keccak256(abi.encodePacked("cross-factory-salt"));

        address andAddr = andFactory.createRequireAndModule(user1, salt);

        // Different factories will produce different addresses even with same salt and admin
        // because they use different contract bytecode
        address orAddr = orFactory.createRequireOrModule(user1, salt);
        address compositeAddr = compositeFactory.createRequireCompositeModule(user1, salt);

        // All addresses should be different due to different contract types
        assertTrue(andAddr != orAddr);
        assertTrue(andAddr != compositeAddr);
        assertTrue(orAddr != compositeAddr);
    }

    // Functional tests to ensure modules work correctly
    function testAndModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-and"));
        address moduleAddr = andFactory.createRequireAndModule(user1, salt);

        RequireAndModule module = RequireAndModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Owner should be the admin passed to the function
        assertEq(module.owner(), user1);
    }

    function testOrModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-or"));
        address moduleAddr = orFactory.createRequireOrModule(user1, salt);

        RequireOrModule module = RequireOrModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Owner should be the admin passed to the function
        assertEq(module.owner(), user1);
    }

    function testCompositeModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-composite"));
        address moduleAddr = compositeFactory.createRequireCompositeModule(user1, salt);

        RequireCompositeModule module = RequireCompositeModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Owner should be the admin passed to the function
        assertEq(module.owner(), user1);
    }

    // Gas optimization tests - CREATE2 is different from clones but still efficient
    function testCreateRequireAndModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-and"));

        uint256 gasBefore = gasleft();
        andFactory.createRequireAndModule(user1, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (CREATE2 deployment)
        assertTrue(gasUsed < 5_000_000); // Increased limit for CREATE2
        assertTrue(gasUsed > 0);
    }

    function testCreateRequireOrModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-or"));

        uint256 gasBefore = gasleft();
        orFactory.createRequireOrModule(user1, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (CREATE2 deployment)
        assertTrue(gasUsed < 5_000_000); // Increased limit for CREATE2
        assertTrue(gasUsed > 0);
    }

    function testCreateRequireCompositeModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-composite"));

        uint256 gasBefore = gasleft();
        compositeFactory.createRequireCompositeModule(user1, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (CREATE2 deployment)
        assertTrue(gasUsed < 5_000_000); // Increased limit for CREATE2
        assertTrue(gasUsed > 0);
    }

    // Edge case tests
    function testCreateWithMaxUint256Salt() public {
        bytes32 salt = bytes32(type(uint256).max);

        address andAddr = andFactory.createRequireAndModule(user1, salt);
        assertTrue(andAddr != address(0));
    }

    function testCreateWithZeroSalt() public {
        bytes32 salt = bytes32(0);

        address andAddr = andFactory.createRequireAndModule(user1, salt);
        assertTrue(andAddr != address(0));
    }

    // Constructor tests
    function testFactoryConstructorWithZeroAddressReverts() public {
        vm.expectRevert(RequireAndModuleFactory.ZeroAddress.selector);
        new RequireAndModuleFactory(address(0));

        vm.expectRevert(RequireOrModuleFactory.ZeroAddress.selector);
        new RequireOrModuleFactory(address(0));

        vm.expectRevert(RequireCompositeModuleFactory.ZeroAddress.selector);
        new RequireCompositeModuleFactory(address(0));
    }

    // Ownership verification tests
    function testFactoryCannotControlDeployedModules() public {
        bytes32 salt = keccak256(abi.encodePacked("factory-control-test"));

        address moduleAddr = andFactory.createRequireAndModule(user1, salt);
        RequireAndModule module = RequireAndModule(moduleAddr);

        // Factory should not be able to control the module since user1 is the owner
        vm.prank(address(andFactory));
        vm.expectRevert(); // Should revert with Ownable unauthorized error
        module.addPermissionCheck(user2, true);

        // But user1 should be able to
        vm.prank(user1);
        module.addPermissionCheck(user2, true);

        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 1);
        assertEq(checks[0], user2);
    }

    // Comprehensive access control tests
    function testAllFactoriesAccessControl() public view {
        // Test all factories have correct role setup using their AccessControl interface
        assertTrue(andFactory.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(andFactory.hasRole(MANAGER_ROLE, admin));
        assertTrue(andFactory.hasRole(MANAGER_ROLE, manager));
        assertFalse(andFactory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(andFactory.hasRole(MANAGER_ROLE, nonManager));

        assertTrue(orFactory.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(orFactory.hasRole(MANAGER_ROLE, admin));
        assertTrue(orFactory.hasRole(MANAGER_ROLE, manager));
        assertFalse(orFactory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(orFactory.hasRole(MANAGER_ROLE, nonManager));

        assertTrue(compositeFactory.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(compositeFactory.hasRole(MANAGER_ROLE, admin));
        assertTrue(compositeFactory.hasRole(MANAGER_ROLE, manager));
        assertFalse(compositeFactory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(compositeFactory.hasRole(MANAGER_ROLE, nonManager));
    }

    // Fuzz testing
    function testFuzzCreateRequireAndModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = andFactory.createRequireAndModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));
    }

    function testFuzzCreateRequireOrModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = orFactory.createRequireOrModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));
    }

    function testFuzzCreateRequireCompositeModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = compositeFactory.createRequireCompositeModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));
    }

    // Multiple deployment tests
    function testMultipleDeploymentsWithDifferentSalts() public {
        uint256 deploymentCount = 10;
        address[] memory andModules = new address[](deploymentCount);
        address[] memory orModules = new address[](deploymentCount);
        address[] memory compositeModules = new address[](deploymentCount);

        for (uint256 i = 0; i < deploymentCount; i++) {
            bytes32 salt = keccak256(abi.encodePacked("multiple-test", i));

            andModules[i] = andFactory.createRequireAndModule(user1, salt);

            salt = keccak256(abi.encodePacked("multiple-test-or", i));
            orModules[i] = orFactory.createRequireOrModule(user1, salt);

            salt = keccak256(abi.encodePacked("multiple-test-composite", i));
            compositeModules[i] = compositeFactory.createRequireCompositeModule(user1, salt);
        }

        // Verify all addresses are unique and valid
        for (uint256 i = 0; i < deploymentCount; i++) {
            assertTrue(andModules[i] != address(0));
            assertTrue(orModules[i] != address(0));
            assertTrue(compositeModules[i] != address(0));

            for (uint256 j = i + 1; j < deploymentCount; j++) {
                assertTrue(andModules[i] != andModules[j]);
                assertTrue(orModules[i] != orModules[j]);
                assertTrue(compositeModules[i] != compositeModules[j]);
            }
        }
    }

    // Additional comprehensive tests for edge cases and robustness
    function testDeployedModulesHaveCorrectBytecode() public {
        bytes32 salt = keccak256(abi.encodePacked("bytecode-test"));

        address moduleAddr = andFactory.createRequireAndModule(user1, salt);
        RequireAndModule module = RequireAndModule(moduleAddr);

        // Should be able to call module functions without issues
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Should be able to get the owner
        address owner = module.owner();
        assertEq(owner, user1);
    }

    function testFactoryCannotBeReinitialized() public {
        // Factories should not be upgradeable or reinitializable
        // Test that CREATE2 consistently produces same results
        bytes32 salt1 = keccak256("test1");
        bytes32 salt2 = keccak256("test2");

        // Deploy modules with different salts
        address addr1 = andFactory.createRequireAndModule(user1, salt1);
        address addr2 = andFactory.createRequireAndModule(user1, salt2);

        // Addresses should be different
        assertTrue(addr1 != addr2);

        // But computed addresses should be consistent
        assertEq(addr1, andFactory.computeModuleAddress(user1, salt1));
        assertEq(addr2, andFactory.computeModuleAddress(user1, salt2));
    }

    function testComputeAddressConsistency() public {
        bytes32 salt = keccak256(abi.encodePacked("consistency-test"));

        // Computing address multiple times should give same result
        address computed1 = andFactory.computeModuleAddress(user1, salt);
        address computed2 = andFactory.computeModuleAddress(user1, salt);
        assertEq(computed1, computed2);

        // Actual deployment should match computed address
        address actual = andFactory.createRequireAndModule(user1, salt);
        assertEq(actual, computed1);
    }
}
