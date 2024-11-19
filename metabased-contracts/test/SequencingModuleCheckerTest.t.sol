// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker, Ownable} from "src/SequencingModuleChecker.sol";
import {RequirementChainModule} from "src/requirement-modules/RequirementChainModule.sol";
import {Test} from "forge-std/Test.sol";

contract SequencingModuleCheckerMock is SequencingModuleChecker {
    constructor(address _admin, address _masterModule) SequencingModuleChecker(_admin, _masterModule) {}
}

contract SequencingModuleCheckerTest is Test {
    SequencingModuleChecker public manager;
    RequirementChainModule public masterModule;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = msg.sender;
        nonAdmin = address(0x456);

        masterModule = new RequirementChainModule(admin);
        manager = new SequencingModuleCheckerMock(admin, address(masterModule));
    }

    function testUpdateMasterModule() public {
        address newModule = address(new RequirementChainModule(admin));

        vm.prank(admin);
        manager.updateRequirementModule(newModule);

        assertEq(address(manager.requirementModule()), newModule);
    }

    function testUpdateMasterModuleNonAdmin() public {
        address newModule = address(new RequirementChainModule(admin));

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
