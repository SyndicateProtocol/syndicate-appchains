// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker, Ownable} from "src/SequencingModuleChecker.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {Test} from "forge-std/Test.sol";

contract SequencingModuleCheckerMock is SequencingModuleChecker {
    constructor(address _admin, address _masterModule) SequencingModuleChecker() {
        initialize(_admin, _masterModule);
    }
}

contract SequencingModuleCheckerTest is Test {
    SequencingModuleChecker public manager;
    RequireAllModule public masterModule;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = msg.sender;
        nonAdmin = address(0x456);

        masterModule = new RequireAllModule(admin);
        manager = new SequencingModuleCheckerMock(admin, address(masterModule));
    }

    function testUpdateMasterModule() public {
        address newModule = address(new RequireAllModule(admin));

        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit SequencingModuleChecker.RequirementModuleUpdated(newModule);
        manager.updateRequirementModule(newModule);

        assertEq(address(manager.requirementModule()), newModule);
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
}
