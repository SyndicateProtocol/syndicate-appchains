// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {
    RequireAndModuleFactory,
    RequireOrModuleFactory,
    RequireCompositeModuleFactory
} from "src/PermissionModuleFactories.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";

contract PermissionModuleFactoriesTest is Test {
    RequireAndModuleFactory public andFactory;
    RequireOrModuleFactory public orFactory;
    RequireCompositeModuleFactory public compositeFactory;

    address public admin;
    address public user1;
    address public user2;

    // Events to test
    event RequireAndModuleCreated(address indexed module, address indexed admin);
    event RequireOrModuleCreated(address indexed module, address indexed admin);
    event RequireCompositeModuleCreated(address indexed module, address indexed admin);

    function setUp() public {
        admin = address(0x1);
        user1 = address(0x2);
        user2 = address(0x3);

        andFactory = new RequireAndModuleFactory();
        orFactory = new RequireOrModuleFactory();
        compositeFactory = new RequireCompositeModuleFactory();
    }

    // RequireAndModuleFactory Tests
    function testCreateRequireAndModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-1"));
        address expectedAddress = andFactory.computeModuleAddress(admin, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireAndModuleCreated(expectedAddress, admin);

        address moduleAddress = andFactory.createRequireAndModule(admin, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);

        RequireAndModule module = RequireAndModule(moduleAddress);
        assertEq(module.owner(), admin);
    }

    function testCreateRequireAndModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-zero"));

        vm.expectRevert(RequireAndModuleFactory.ZeroAddress.selector);
        andFactory.createRequireAndModule(address(0), salt);
    }

    function testComputeRequireAndModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-compute"));

        address computedAddress = andFactory.computeModuleAddress(admin, salt);
        address actualAddress = andFactory.createRequireAndModule(admin, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireAndModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-2"));

        address addr1 = andFactory.createRequireAndModule(admin, salt1);
        address addr2 = andFactory.createRequireAndModule(admin, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireAndModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("collision-salt"));

        // First deployment should succeed
        address addr1 = andFactory.createRequireAndModule(admin, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same parameters should revert due to CREATE2 collision
        vm.expectRevert();
        andFactory.createRequireAndModule(admin, salt);
    }

    function testRequireAndModuleDifferentAdminsSameSaltDifferentAddresses() public {
        bytes32 salt = keccak256(abi.encodePacked("same-salt"));

        address addr1 = andFactory.createRequireAndModule(user1, salt);

        // Different admin should produce different address even with same salt
        // (because admin is part of constructor params, which affects bytecode)
        address addr2 = andFactory.createRequireAndModule(user2, salt);

        // Both deployments should succeed and produce different addresses
        assertTrue(addr1 != address(0));
        assertTrue(addr2 != address(0));
        assertTrue(addr1 != addr2);

        // Verify owners are set correctly
        RequireAndModule module1 = RequireAndModule(addr1);
        RequireAndModule module2 = RequireAndModule(addr2);
        assertEq(module1.owner(), user1);
        assertEq(module2.owner(), user2);

        // Computed addresses should also be different
        address computedAddr1 = andFactory.computeModuleAddress(user1, salt);
        address computedAddr2 = andFactory.computeModuleAddress(user2, salt);
        assertEq(addr1, computedAddr1);
        assertEq(addr2, computedAddr2);
        assertTrue(computedAddr1 != computedAddr2);
    }

    // RequireOrModuleFactory Tests
    function testCreateRequireOrModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-1"));
        address expectedAddress = orFactory.computeModuleAddress(admin, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireOrModuleCreated(expectedAddress, admin);

        address moduleAddress = orFactory.createRequireOrModule(admin, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);

        RequireOrModule module = RequireOrModule(moduleAddress);
        assertEq(module.owner(), admin);
    }

    function testCreateRequireOrModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-zero"));

        vm.expectRevert(RequireOrModuleFactory.ZeroAddress.selector);
        orFactory.createRequireOrModule(address(0), salt);
    }

    function testComputeRequireOrModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-or-compute"));

        address computedAddress = orFactory.computeModuleAddress(admin, salt);
        address actualAddress = orFactory.createRequireOrModule(admin, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireOrModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("or-salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("or-salt-2"));

        address addr1 = orFactory.createRequireOrModule(admin, salt1);
        address addr2 = orFactory.createRequireOrModule(admin, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireOrModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("or-collision-salt"));

        // First deployment should succeed
        address addr1 = orFactory.createRequireOrModule(admin, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same parameters should revert due to CREATE2 collision
        vm.expectRevert();
        orFactory.createRequireOrModule(admin, salt);
    }

    // RequireCompositeModuleFactory Tests
    function testCreateRequireCompositeModule() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-1"));
        address expectedAddress = compositeFactory.computeModuleAddress(admin, salt);

        vm.expectEmit(true, true, false, true);
        emit RequireCompositeModuleCreated(expectedAddress, admin);

        address moduleAddress = compositeFactory.createRequireCompositeModule(admin, salt);

        assertTrue(moduleAddress != address(0));
        assertEq(moduleAddress, expectedAddress);

        RequireCompositeModule module = RequireCompositeModule(moduleAddress);
        assertEq(module.owner(), admin);
    }

    function testCreateRequireCompositeModuleWithZeroAddressReverts() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-zero"));

        vm.expectRevert(RequireCompositeModuleFactory.ZeroAddress.selector);
        compositeFactory.createRequireCompositeModule(address(0), salt);
    }

    function testComputeRequireCompositeModuleAddress() public {
        bytes32 salt = keccak256(abi.encodePacked("test-salt-composite-compute"));

        address computedAddress = compositeFactory.computeModuleAddress(admin, salt);
        address actualAddress = compositeFactory.createRequireCompositeModule(admin, salt);

        assertEq(computedAddress, actualAddress);
    }

    function testRequireCompositeModuleDifferentSaltsDifferentAddresses() public {
        bytes32 salt1 = keccak256(abi.encodePacked("composite-salt-1"));
        bytes32 salt2 = keccak256(abi.encodePacked("composite-salt-2"));

        address addr1 = compositeFactory.createRequireCompositeModule(admin, salt1);
        address addr2 = compositeFactory.createRequireCompositeModule(admin, salt2);

        assertTrue(addr1 != addr2);
    }

    function testRequireCompositeModuleSameParametersRevert() public {
        bytes32 salt = keccak256(abi.encodePacked("composite-collision-salt"));

        // First deployment should succeed
        address addr1 = compositeFactory.createRequireCompositeModule(admin, salt);
        assertTrue(addr1 != address(0));

        // Second deployment with same parameters should revert due to CREATE2 collision
        vm.expectRevert();
        compositeFactory.createRequireCompositeModule(admin, salt);
    }

    // Cross-factory tests to ensure different module types produce different addresses
    function testDifferentFactoriesSameSaltDifferentAddresses() public {
        bytes32 salt = keccak256(abi.encodePacked("cross-factory-salt"));

        address andAddr = andFactory.createRequireAndModule(admin, salt);
        address orAddr = orFactory.createRequireOrModule(admin, salt);
        address compositeAddr = compositeFactory.createRequireCompositeModule(admin, salt);

        // All addresses should be different
        assertTrue(andAddr != orAddr);
        assertTrue(andAddr != compositeAddr);
        assertTrue(orAddr != compositeAddr);
    }

    // Functional tests to ensure modules work correctly
    function testAndModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-and"));
        address moduleAddr = andFactory.createRequireAndModule(admin, salt);

        RequireAndModule module = RequireAndModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Test owner functionality
        assertEq(module.owner(), admin);
    }

    function testOrModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-or"));
        address moduleAddr = orFactory.createRequireOrModule(admin, salt);

        RequireOrModule module = RequireOrModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Test owner functionality
        assertEq(module.owner(), admin);
    }

    function testCompositeModuleFunctionality() public {
        bytes32 salt = keccak256(abi.encodePacked("functional-composite"));
        address moduleAddr = compositeFactory.createRequireCompositeModule(admin, salt);

        RequireCompositeModule module = RequireCompositeModule(moduleAddr);

        // Test that we can get the permission checks (should be empty initially)
        address[] memory checks = module.getAllPermissionChecks();
        assertEq(checks.length, 0);

        // Test owner functionality
        assertEq(module.owner(), admin);
    }

    // Gas optimization tests
    function testCreateRequireAndModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-and"));

        uint256 gasBefore = gasleft();
        andFactory.createRequireAndModule(admin, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (less than 2M for deployment)
        assertTrue(gasUsed < 2_000_000);
        assertTrue(gasUsed > 0);
    }

    function testCreateRequireOrModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-or"));

        uint256 gasBefore = gasleft();
        orFactory.createRequireOrModule(admin, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (less than 2M for deployment)
        assertTrue(gasUsed < 2_000_000);
        assertTrue(gasUsed > 0);
    }

    function testCreateRequireCompositeModuleGasUsage() public {
        bytes32 salt = keccak256(abi.encodePacked("gas-test-composite"));

        uint256 gasBefore = gasleft();
        compositeFactory.createRequireCompositeModule(admin, salt);
        uint256 gasAfter = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (less than 2M for deployment)
        assertTrue(gasUsed < 2_000_000);
        assertTrue(gasUsed > 0);
    }

    // Edge case tests
    function testCreateWithMaxUint256Salt() public {
        bytes32 salt = bytes32(type(uint256).max);

        address andAddr = andFactory.createRequireAndModule(admin, salt);
        address orAddr = orFactory.createRequireOrModule(admin, salt);
        address compositeAddr = compositeFactory.createRequireCompositeModule(admin, salt);

        assertTrue(andAddr != address(0));
        assertTrue(orAddr != address(0));
        assertTrue(compositeAddr != address(0));
    }

    function testCreateWithZeroSalt() public {
        bytes32 salt = bytes32(0);

        address andAddr = andFactory.createRequireAndModule(admin, salt);
        address orAddr = orFactory.createRequireOrModule(admin, salt);
        address compositeAddr = compositeFactory.createRequireCompositeModule(admin, salt);

        assertTrue(andAddr != address(0));
        assertTrue(orAddr != address(0));
        assertTrue(compositeAddr != address(0));
    }

    // Fuzz testing
    function testFuzzCreateRequireAndModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = andFactory.createRequireAndModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));

        RequireAndModule module = RequireAndModule(moduleAddr);
        assertEq(module.owner(), _admin);
    }

    function testFuzzCreateRequireOrModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = orFactory.createRequireOrModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));

        RequireOrModule module = RequireOrModule(moduleAddr);
        assertEq(module.owner(), _admin);
    }

    function testFuzzCreateRequireCompositeModule(address _admin, bytes32 _salt) public {
        vm.assume(_admin != address(0));

        address moduleAddr = compositeFactory.createRequireCompositeModule(_admin, _salt);
        assertTrue(moduleAddr != address(0));

        RequireCompositeModule module = RequireCompositeModule(moduleAddr);
        assertEq(module.owner(), _admin);
    }
}
