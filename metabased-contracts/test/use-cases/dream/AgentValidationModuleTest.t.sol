// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AgentApplication} from "src/use-cases/dream/AgentApplication.sol";
import {AgentValidationModule} from "src/use-cases/dream/AgentValidationModule.sol";

contract AgentValidationModuleTest is Test {
    AgentValidationModule public validationModule;
    AgentApplication public agentApplication;

    address public admin;
    address public agent;
    address public nonAgent;

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);

    function setUp() public {
        admin = makeAddr("admin");
        agent = makeAddr("agent");
        nonAgent = makeAddr("nonAgent");

        vm.startPrank(admin);
        agentApplication = new AgentApplication(admin);
        validationModule = new AgentValidationModule(admin, address(agentApplication));
        vm.stopPrank();
    }

    function test_Constructor() public view {
        assertEq(validationModule.admin(), admin);
        assertEq(address(validationModule.agentApplication()), address(agentApplication));
    }

    function test_RevertWhen_ConstructorZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentValidationModule.InvalidAddress.selector);
        new AgentValidationModule(address(0), address(agentApplication));

        vm.expectRevert(AgentValidationModule.InvalidAddress.selector);
        new AgentValidationModule(admin, address(0));
        vm.stopPrank();
    }

    function test_TransferAdmin() public {
        address newAdmin = makeAddr("newAdmin");

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit AdminTransferred(admin, newAdmin);

        validationModule.transferAdmin(newAdmin);
        assertEq(validationModule.admin(), newAdmin);
        vm.stopPrank();
    }

    function test_RevertWhen_NonAdminTransfersAdmin() public {
        address newAdmin = makeAddr("newAdmin");

        vm.startPrank(nonAgent);
        vm.expectRevert(AgentValidationModule.NotAdmin.selector);
        validationModule.transferAdmin(newAdmin);
        vm.stopPrank();
    }

    function test_RevertWhen_TransferAdminToZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentValidationModule.InvalidAddress.selector);
        validationModule.transferAdmin(address(0));
        vm.stopPrank();
    }

    function test_IsAllowedForApprovedAgent() public {
        address dreamSequencer = makeAddr("dreamSequencer");
        // Add an agent and approve them
        vm.startPrank(admin);
        uint256 applicantId = agentApplication.addApplicant("Test Agent", agent, "");

        // Mock the Dream Sequencer contract
        vm.etch(dreamSequencer, hex"00");
        vm.mockCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent), abi.encode());
        agentApplication.setDreamSequencer(dreamSequencer);
        agentApplication.approveApplicant(applicantId);
        vm.stopPrank();

        assertTrue(validationModule.isAllowed(agent));
    }

    function test_IsAllowedForDeniedAgent() public {
        // Add an agent and deny them
        vm.startPrank(admin);
        uint256 applicantId = agentApplication.addApplicant("Test Agent", agent, "");
        agentApplication.denyApplicant(applicantId);
        vm.stopPrank();

        assertFalse(validationModule.isAllowed(agent));
    }

    function test_IsAllowedForPendingAgent() public {
        // Add an agent (defaults to pending)
        vm.startPrank(admin);
        agentApplication.addApplicant("Test Agent", agent, "");
        vm.stopPrank();

        assertFalse(validationModule.isAllowed(agent));
    }

    function test_IsAllowedForNonExistentAgent() public view {
        assertFalse(validationModule.isAllowed(nonAgent));
    }
}
