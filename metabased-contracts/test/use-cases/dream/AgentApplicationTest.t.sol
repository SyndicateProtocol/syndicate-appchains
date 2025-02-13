// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AgentApplication, Ownable} from "src/use-cases/dream/AgentApplication.sol";

contract AgentApplicationTest is Test {
    AgentApplication public application;
    AgentApplication.ApplicationStatus constant PENDING = AgentApplication.ApplicationStatus.PENDING;
    AgentApplication.ApplicationStatus constant APPROVED = AgentApplication.ApplicationStatus.APPROVED;
    AgentApplication.ApplicationStatus constant DENIED = AgentApplication.ApplicationStatus.DENIED;

    address public admin;
    address public agent;

    event ApplicantAdded(
        uint256 indexed applicantId,
        string name,
        address indexed agentAddress,
        bytes additionalData,
        AgentApplication.ApplicationStatus status
    );
    event ApplicantStatusUpdated(uint256 indexed applicantId, AgentApplication.ApplicationStatus status);

    function setUp() public {
        admin = makeAddr("admin");
        agent = makeAddr("agent");

        vm.startPrank(admin);
        application = new AgentApplication(admin);
        vm.stopPrank();
    }

    function test_Constructor() public view {
        assertEq(application.owner(), admin);
    }

    function test_AddApplicant() public {
        string memory name = "Test Agent";
        bytes memory additionalData = "test data";

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, true);
        emit ApplicantAdded(0, name, agent, additionalData, PENDING);

        uint256 applicantId = application.addApplicant(name, agent, additionalData);

        (
            address returnedAddress,
            AgentApplication.ApplicationStatus returnedStatus,
            string memory returnedName,
            bytes memory returnedData
        ) = application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(PENDING));
        assertEq(returnedName, name);
        assertEq(returnedData, additionalData);
        assertEq(applicantId, 0);
        vm.stopPrank();
    }

    function test_RevertWhen_AddApplicantWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.addApplicant("Test", address(0), "");
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerAddsApplicant() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.addApplicant("Test", agent, "");
        vm.stopPrank();
    }

    function test_ApproveApplicant() public {
        address dreamSequencer = makeAddr("dreamSequencer");
        // First add an applicant
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");

        // Mock the Dream Sequencer contract
        vm.etch(dreamSequencer, hex"00");
        vm.mockCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent), abi.encode());
        application.setDreamSequencer(dreamSequencer);

        // Then approve them
        vm.expectEmit(true, false, false, true);
        emit ApplicantStatusUpdated(applicantId, APPROVED);

        application.approveApplicant(applicantId);

        (, AgentApplication.ApplicationStatus status,,) = application.getApplicant(applicantId);
        assertEq(uint256(status), uint256(APPROVED));
        vm.stopPrank();
    }

    function test_DenyApplicant() public {
        // First add an applicant
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");

        // Then deny them
        vm.expectEmit(true, false, false, true);
        emit ApplicantStatusUpdated(applicantId, DENIED);

        application.denyApplicant(applicantId);

        (, AgentApplication.ApplicationStatus status,,) = application.getApplicant(applicantId);
        assertEq(uint256(status), uint256(DENIED));
        vm.stopPrank();
    }

    function test_RevertWhen_ApproveNonExistentApplicant() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.approveApplicant(999);
        vm.stopPrank();
    }

    function test_RevertWhen_DenyNonExistentApplicant() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.denyApplicant(999);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerApprovesApplicant() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");
        vm.stopPrank();

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.approveApplicant(applicantId);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerDeniesApplicant() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");
        vm.stopPrank();

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.denyApplicant(applicantId);
        vm.stopPrank();
    }

    function test_GetApplicant() public {
        string memory name = "Test Agent";
        bytes memory additionalData = "test data";

        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant(name, agent, additionalData);

        (
            address returnedAddress,
            AgentApplication.ApplicationStatus returnedStatus,
            string memory returnedName,
            bytes memory returnedData
        ) = application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(PENDING));
        assertEq(returnedName, name);
        assertEq(returnedData, additionalData);
        vm.stopPrank();
    }

    function test_RevertWhen_GetNonExistentApplicant() public {
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.getApplicant(999);
    }

    function test_ApplicantCount() public {
        assertEq(application.applicantCount(), 0);

        vm.startPrank(admin);
        application.addApplicant("Test1", makeAddr("agent1"), "");
        assertEq(application.applicantCount(), 1);

        application.addApplicant("Test2", makeAddr("agent2"), "");
        assertEq(application.applicantCount(), 2);
        vm.stopPrank();
    }

    function test_IsPermittedById() public {
        address dreamSequencer = makeAddr("dreamSequencer");
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");

        // Mock the Dream Sequencer contract
        vm.etch(dreamSequencer, hex"00");
        vm.mockCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent), abi.encode());
        application.setDreamSequencer(dreamSequencer);

        application.approveApplicant(applicantId);
        assertTrue(application.isPermittedById(applicantId));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress() public {
        address dreamSequencer = makeAddr("dreamSequencer");
        vm.startPrank(admin);
        application.addApplicant("Test", agent, "");

        // Mock the Dream Sequencer contract
        vm.etch(dreamSequencer, hex"00");
        vm.mockCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent), abi.encode());
        application.setDreamSequencer(dreamSequencer);

        application.approveApplicant(0);
        assertTrue(application.isPermittedByAddress(agent));
        vm.stopPrank();
    }

    function test_RevertWhen_DuplicateAgentApplication() public {
        vm.startPrank(admin);
        application.addApplicant("Test1", agent, "");
        vm.expectRevert(AgentApplication.AgentAlreadyApplied.selector);
        application.addApplicant("Test2", agent, "");
        vm.stopPrank();
    }

    function test_GetApplicantById() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");
        (address returnedAddress, AgentApplication.ApplicationStatus status, string memory name,) =
            application.getApplicant(applicantId);
        assertEq(returnedAddress, agent);
        assertEq(uint256(status), uint256(PENDING));
        assertEq(name, "Test");
        vm.stopPrank();
    }

    function test_RevertWhen_ApproveWithoutDreamSequencer() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "");
        vm.expectRevert(AgentApplication.DreamSequencerNotSet.selector);
        application.approveApplicant(applicantId);
        vm.stopPrank();
    }

    function test_DreamSequencerIntegration() public {
        address dreamSequencer = makeAddr("dreamSequencer");
        vm.startPrank(admin);

        // Mock the Dream Sequencer contract
        vm.etch(dreamSequencer, hex"00");
        vm.mockCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent), abi.encode());
        vm.expectCall(dreamSequencer, abi.encodeWithSignature("addAllowedMinter(address)", agent));

        // Set up and approve applicant
        uint256 applicantId = application.addApplicant("Test", agent, "");
        application.setDreamSequencer(dreamSequencer);
        application.approveApplicant(applicantId);

        vm.stopPrank();
    }
}
