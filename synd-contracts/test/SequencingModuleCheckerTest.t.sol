// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "src/SequencingModuleChecker.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {Test} from "forge-std/Test.sol";

contract SequencingModuleCheckerMock is Initializable, SequencingModuleChecker {
    function initialize(address admin, address _permissionRequirementModule) external initializer {
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
    }
}

contract SequencingModuleCheckerTest is Test {
    SequencingModuleCheckerMock public manager;
    RequireAndModule public masterModule;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = msg.sender;
        nonAdmin = address(0x456);

        masterModule = new RequireAndModule(admin);
        manager = new SequencingModuleCheckerMock();
        manager.initialize(admin, address(masterModule));
    }

    function testUpdateMasterModule() public {
        address newModule = address(new RequireAndModule(admin));

        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit SequencingModuleChecker.RequirementModuleUpdated(newModule);
        manager.updateRequirementModule(newModule);

        assertEq(address(manager.permissionRequirementModule()), newModule);
    }

    function testUpdateMasterModuleNonAdmin() public {
        address newModule = address(new RequireAndModule(admin));

        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        manager.updateRequirementModule(newModule);
    }
}
