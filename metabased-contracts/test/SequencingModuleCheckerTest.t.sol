// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker, Ownable} from "src/SequencingModuleChecker.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {Test} from "forge-std/Test.sol";

contract SequencingModuleCheckerMock is SequencingModuleChecker {}

contract SequencingModuleCheckerTest is Test {
    SequencingModuleChecker public manager;
    RequireAllModule public masterModule;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = msg.sender;
        nonAdmin = address(0x456);

        masterModule = new RequireAllModule(admin);
        manager = new SequencingModuleCheckerMock();
        manager.initialize(admin, address(masterModule));
    }

    function testUpdateMasterModule() public {
        address newModule = address(new RequireAllModule(admin));

        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit SequencingModuleChecker.RequirementModuleUpdated(newModule);
        manager.updateRequirementModule(newModule);

        assertEq(address(manager.permissionRequirementModule()), newModule);
    }

    function testUpdateMasterModuleNonAdmin() public {
        address newModule = address(new RequireAllModule(admin));

        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        manager.updateRequirementModule(newModule);
    }

    function testUpdateMasterModuleZeroAddress() public {
        vm.prank(admin);
        vm.expectRevert(SequencingModuleChecker.InvalidModuleAddress.selector);
        manager.updateRequirementModule(address(0));
    }

    function testCannotInitializeTwice() public {
        vm.prank(admin);
        vm.expectRevert(SequencingModuleChecker.AlreadyInitialized.selector);
        manager.initialize(admin, address(masterModule));
    }

    function testNotInitialized() public {
        SequencingModuleChecker uninitializedManager = new SequencingModuleCheckerMock();

        bytes memory emptyData = new bytes(0);

        assertFalse(uninitializedManager.isAllowed(address(0), address(0), emptyData));
    }

    function testIsAllowedAfterInitialization() public {
        SequencingModuleChecker initializedManager = new SequencingModuleCheckerMock();
        initializedManager.initialize(admin, address(masterModule));

        bytes memory emptyData = new bytes(0);

        assertTrue(initializedManager.isAllowed(address(0), address(0), emptyData));
    }
}
