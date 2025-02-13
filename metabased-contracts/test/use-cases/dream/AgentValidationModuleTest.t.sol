// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AgentApplication, IMetabasedSequencerChain} from "src/use-cases/dream/AgentApplication.sol";
import {AgentValidationModule} from "src/use-cases/dream/AgentValidationModule.sol";

contract AgentValidationModuleTest is Test {
    AgentValidationModule public validationModule;
    AgentApplication public agentApplication;

    address public admin;
    address public agent;
    address public nonAgent;
    address public sequencerChain;
    address public agentClaimNFTOwner;
    address public agentClaimNFTAddress;

    bytes constant TEST_DATA = "test data";

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);

    function setUp() public {
        admin = makeAddr("admin");
        agent = makeAddr("agent");
        nonAgent = makeAddr("nonAgent");
        sequencerChain = makeAddr("sequencerChain");
        agentClaimNFTOwner = makeAddr("agentClaimNFTOwner");
        agentClaimNFTAddress = makeAddr("agentClaimNFTAddress");

        // Mock the sequencer chain
        vm.etch(sequencerChain, hex"00");

        vm.startPrank(admin);
        agentApplication = new AgentApplication(admin, sequencerChain, agentClaimNFTOwner, agentClaimNFTAddress);
        validationModule = new AgentValidationModule(admin, address(agentApplication));
        vm.stopPrank();
    }

    function _mockSequencerCall(address agentAddress) internal {
        bytes memory grantCalldata = abi.encodeWithSignature("grantClaimPermission(address)", agentAddress);
        bytes memory expectedCalldata = abi.encodePacked(
            agentClaimNFTOwner, agentClaimNFTAddress, uint256(0), uint256(grantCalldata.length), grantCalldata
        );

        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );
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
        // Approve an agent
        vm.startPrank(admin);
        _mockSequencerCall(agent);
        agentApplication.approveApplicant(agent, TEST_DATA);
        vm.stopPrank();

        assertTrue(validationModule.isAllowed(agent));
    }

    function test_IsAllowedForDeniedAgent() public {
        // First approve then deny an agent
        vm.startPrank(admin);
        _mockSequencerCall(agent);
        uint256 applicantId = agentApplication.approveApplicant(agent, TEST_DATA);
        agentApplication.denyApplicant(applicantId);
        vm.stopPrank();

        assertFalse(validationModule.isAllowed(agent));
    }

    function test_IsAllowedForNonExistentAgent() public view {
        assertFalse(validationModule.isAllowed(nonAgent));
    }

    function test_IsAllowedForZeroAddress() public view {
        assertFalse(validationModule.isAllowed(address(0)));
    }
}
