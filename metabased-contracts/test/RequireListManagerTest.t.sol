// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager, Ownable} from "src/RequireListManager.sol";
import {MasterPermissionModule} from "src/MasterPermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract RequireListManagerMock is RequireListManager {
    constructor(address _admin, address _masterModule) RequireListManager(_admin, _masterModule) {}
}

contract RequireListManagerTest is Test {
    RequireListManager public manager;
    MasterPermissionModule public masterModule;
    address public admin;
    address public nonAdmin;

    function setUp() public {
        admin = msg.sender;
        nonAdmin = address(0x456);

        masterModule = new MasterPermissionModule(admin);
        manager = new RequireListManagerMock(admin, address(masterModule));
    }

    function testUpdateMasterModule() public {
        address newModule = address(new MasterPermissionModule(admin));

        vm.prank(admin);
        manager.updateMasterModule(newModule);

        assertEq(address(manager.masterPermissionModule()), newModule);
    }

    function testUpdateMasterModuleNonAdmin() public {
        address newModule = address(new MasterPermissionModule(admin));

        vm.prank(nonAdmin);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonAdmin));
        manager.updateMasterModule(newModule);
    }

    function testUpdateMasterModuleZeroAddress() public {
        vm.prank(admin);
        vm.expectRevert(RequireListManager.InvalidModuleAddress.selector);
        manager.updateMasterModule(address(0));
    }
}
