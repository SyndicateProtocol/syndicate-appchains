// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "src/SequencingModuleChecker.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {DataTooLarge} from "@arbitrum/nitro-contracts/src/libraries/Error.sol";
import {Test} from "forge-std/Test.sol";

contract SequencingModuleCheckerMock is Initializable, SequencingModuleChecker {
    function initialize(address admin, address _permissionRequirementModule) external initializer {
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
    }
}

contract MockPermissionModule is IPermissionModule {
    bool public shouldAllow;

    constructor(bool _shouldAllow) {
        shouldAllow = _shouldAllow;
    }

    function isAllowed(address, address, bytes memory) external view returns (bool) {
        return shouldAllow;
    }

    function setShouldAllow(bool _shouldAllow) external {
        shouldAllow = _shouldAllow;
    }
}

contract SequencingModuleCheckerTest is Test {
    SequencingModuleCheckerMock public manager;
    RequireAndModule public masterModule;
    MockPermissionModule public mockModule;
    address public admin;
    address public nonAdmin;
    address public user1;
    address public user2;

    event RequirementModuleUpdated(address indexed newModule);

    function setUp() public {
        admin = address(this);
        nonAdmin = address(0x456);
        user1 = address(0x123);
        user2 = address(0x789);

        masterModule = new RequireAndModule(admin);
        mockModule = new MockPermissionModule(true);
        manager = new SequencingModuleCheckerMock();
        manager.initialize(admin, address(masterModule));
    }

    // Initialization Tests
    function testInitialization() public {
        assertEq(manager.owner(), admin);
        assertEq(address(manager.permissionRequirementModule()), address(masterModule));
    }

    function testCannotInitializeTwice() public {
        vm.expectRevert(abi.encodeWithSelector(Initializable.InvalidInitialization.selector));
        manager.initialize(admin, address(masterModule));
    }

    function testInitializeWithDifferentModule() public {
        SequencingModuleCheckerMock newManager = new SequencingModuleCheckerMock();
        MockPermissionModule newMockModule = new MockPermissionModule(false);

        newManager.initialize(user1, address(newMockModule));

        assertEq(newManager.owner(), user1);
        assertEq(address(newManager.permissionRequirementModule()), address(newMockModule));
    }

    // Permission Requirement Module Update Tests
    function testUpdateRequirementModule() public {
        address newModule = address(new RequireAndModule(admin));

        vm.expectEmit(true, false, false, false);
        emit RequirementModuleUpdated(newModule);
        manager.updateRequirementModule(newModule);

        assertEq(address(manager.permissionRequirementModule()), newModule);
    }

    function testUpdateRequirementModuleNonAdmin() public {
        address newModule = address(new RequireAndModule(admin));

        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, nonAdmin));
        manager.updateRequirementModule(newModule);
    }

    function testUpdateRequirementModuleToZeroAddress() public {
        vm.expectEmit(true, false, false, false);
        emit RequirementModuleUpdated(address(0));
        manager.updateRequirementModule(address(0));

        assertEq(address(manager.permissionRequirementModule()), address(0));
    }

    function testUpdateRequirementModuleToAddressOne() public {
        vm.expectEmit(true, false, false, false);
        emit RequirementModuleUpdated(address(1));
        manager.updateRequirementModule(address(1));

        assertEq(address(manager.permissionRequirementModule()), address(1));
    }

    // isAllowed Function Tests
    function testIsAllowedWithAddressOne() public {
        manager.updateRequirementModule(address(1));

        bytes memory testData = abi.encode("test data");
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    function testIsAllowedWithZeroAddress() public {
        manager.updateRequirementModule(address(0));

        bytes memory testData = abi.encode("test data");
        // Zero address should cause a revert since there's no code there
        vm.expectRevert();
        manager.isAllowed(user1, user2, testData);
    }

    function testIsAllowedWithMockModuleAllow() public {
        manager.updateRequirementModule(address(mockModule));
        mockModule.setShouldAllow(true);

        bytes memory testData = abi.encode("test data");
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    function testIsAllowedWithMockModuleDeny() public {
        manager.updateRequirementModule(address(mockModule));
        mockModule.setShouldAllow(false);

        bytes memory testData = abi.encode("test data");
        assertFalse(manager.isAllowed(user1, user2, testData));
    }

    function testIsAllowedWithEmptyData() public {
        manager.updateRequirementModule(address(1));

        bytes memory emptyData = "";
        assertTrue(manager.isAllowed(user1, user2, emptyData));
    }

    function testIsAllowedWithLargeData() public {
        manager.updateRequirementModule(address(1));

        bytes memory largeData = new bytes(199999); // Just under the limit
        assertTrue(manager.isAllowed(user1, user2, largeData));
    }

    function testIsAllowedWithMaxSizeData() public {
        manager.updateRequirementModule(address(1));

        bytes memory maxSizeData = new bytes(200000); // Exactly the limit
        assertTrue(manager.isAllowed(user1, user2, maxSizeData));
    }

    function testIsAllowedWithOversizedData() public {
        manager.updateRequirementModule(address(1));

        bytes memory oversizedData = new bytes(200001); // Over the limit
        vm.expectRevert(abi.encodeWithSelector(DataTooLarge.selector, 200001, 200000));
        manager.isAllowed(user1, user2, oversizedData);
    }

    function testIsAllowedWithRealModule() public {
        RequireOrModule orModule = new RequireOrModule(admin);
        manager.updateRequirementModule(address(orModule));

        bytes memory testData = abi.encode("test");
        // RequireOrModule with no modules returns true by default
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    // Edge Cases and Error Conditions
    function testMaxDataSizeConstant() public {
        assertEq(manager.maxDataSize(), 200000);
    }

    function testStorageLocation() public {
        // Test that the storage location constant is correct
        bytes32 expected = 0x5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500;
        assertEq(manager.SEQUENCING_MODULE_STORAGE_LOCATION(), expected);
    }

    function testPermissionRequirementModuleGetter() public {
        assertEq(address(manager.permissionRequirementModule()), address(masterModule));

        manager.updateRequirementModule(address(mockModule));
        assertEq(address(manager.permissionRequirementModule()), address(mockModule));
    }

    // Ownership Tests
    function testOwnershipTransfer() public {
        assertEq(manager.owner(), admin);

        manager.transferOwnership(user1);
        assertEq(manager.owner(), user1);
    }

    function testRenounceOwnership() public {
        manager.renounceOwnership();
        assertEq(manager.owner(), address(0));
    }

    function testOnlyOwnerFunctions() public {
        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, nonAdmin));
        manager.updateRequirementModule(address(mockModule));
    }

    // Integration Tests with Different Module Types
    function testWithRequireAndModule() public {
        RequireAndModule andModule = new RequireAndModule(admin);
        manager.updateRequirementModule(address(andModule));

        bytes memory testData = abi.encode("test");
        // Empty AndModule returns true (no checks to fail)
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    function testWithRequireOrModule() public {
        RequireOrModule orModule = new RequireOrModule(admin);
        manager.updateRequirementModule(address(orModule));

        bytes memory testData = abi.encode("test");
        // Empty OrModule returns true by default
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    // Fuzz Tests
    function testFuzzIsAllowedWithVariousAddresses(address proposer, address originator) public {
        manager.updateRequirementModule(address(1));

        bytes memory testData = abi.encode("test");
        assertTrue(manager.isAllowed(proposer, originator, testData));
    }

    function testFuzzDataSizeLimit(uint256 dataSize) public {
        vm.assume(dataSize <= 200000);
        manager.updateRequirementModule(address(1));

        bytes memory testData = new bytes(dataSize);
        assertTrue(manager.isAllowed(user1, user2, testData));
    }

    function testFuzzDataSizeRevert(uint256 dataSize) public {
        vm.assume(dataSize > 200000 && dataSize < 1000000); // Cap to prevent OOM
        manager.updateRequirementModule(address(1));

        bytes memory testData = new bytes(dataSize);
        vm.expectRevert(abi.encodeWithSelector(DataTooLarge.selector, dataSize, 200000));
        manager.isAllowed(user1, user2, testData);
    }
}
