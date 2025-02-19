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

    // Test data
    bytes constant TEST_DATA = "test data";

    event ApplicantAdded(
        uint256 indexed applicantId,
        address indexed agentAddress,
        bytes additionalData,
        AgentApplication.ApplicationStatus status
    );
    event ApplicantStatusUpdated(uint256 indexed applicantId, AgentApplication.ApplicationStatus status);
    event AgentClaimNFTOwnerUpdated(address indexed oldOwner, address indexed newOwner);
    event AgentClaimNFTAddressUpdated(address indexed oldAddress, address indexed newAddress);

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

    function test_RevertWhen_ConstructorZeroAddress() public {
        vm.startPrank(admin);

        bytes memory errorData = abi.encodeWithSelector(Ownable.OwnableInvalidOwner.selector, address(0));

        vm.expectRevert(errorData);
        new AgentApplication(address(0));

        vm.stopPrank();
    }

    function test_AddApplicant() public {
        vm.startPrank(admin);

        vm.expectEmit(true, true, false, true);
        emit ApplicantAdded(1, agent, TEST_DATA, PENDING);
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);

        (
            uint256 returnedId,
            address returnedAddress,
            AgentApplication.ApplicationStatus returnedStatus,
            bytes memory returnedData
        ) = application.getApplicantById(applicantId);

        assertEq(returnedId, applicantId);
        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(PENDING));
        assertEq(returnedData, TEST_DATA);
        assertEq(applicantId, 1);
        vm.stopPrank();
    }

    function test_RevertWhen_AddApplicantWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.addApplicant(address(0), TEST_DATA);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerAddsApplicant() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.addApplicant(agent, TEST_DATA);
        vm.stopPrank();
    }

    function test_ApproveApplicant() public {
        vm.startPrank(admin);

        // First add applicant with PENDING status
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);

        // Then approve the applicant
        vm.expectEmit(true, false, false, true);
        emit ApplicantStatusUpdated(applicantId, APPROVED);
        uint256 returnedId = application.approveApplicant(agent);

        assertEq(returnedId, applicantId);

        // Verify status changed using getApplicantById
        (,, AgentApplication.ApplicationStatus status,) = application.getApplicantById(applicantId);
        assertEq(uint256(status), uint256(APPROVED));

        vm.stopPrank();
    }

    function test_RevertWhen_ApproveNonExistentApplicantByAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.approveApplicant(makeAddr("nonexistent"));
        vm.stopPrank();
    }

    function test_RevertWhen_ApproveApplicantWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.approveApplicant(address(0));
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerApprovesApplicant() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.approveApplicant(agent);
        vm.stopPrank();
    }

    function test_DenyApplicant() public {
        vm.startPrank(admin);
        // Add and approve applicant first
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);
        application.approveApplicant(agent);

        // Then deny the applicant
        vm.expectEmit(true, false, false, true);
        emit ApplicantStatusUpdated(applicantId, DENIED);

        uint256 returnedId = application.denyApplicant(agent);
        assertEq(returnedId, applicantId);

        // Verify status using getApplicantById
        (,, AgentApplication.ApplicationStatus status,) = application.getApplicantById(applicantId);
        assertEq(uint256(status), uint256(DENIED));
        vm.stopPrank();
    }

    function test_RevertWhen_DenyNonExistentApplicantByAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.denyApplicant(makeAddr("nonexistent"));
        vm.stopPrank();
    }

    function test_RevertWhen_DenyWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.denyApplicant(address(0));
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerDeniesApplicant() public {
        vm.prank(admin);
        application.addApplicant(agent, TEST_DATA);

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.denyApplicant(agent);
        vm.stopPrank();
    }

    function test_GetApplicantById() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);
        application.approveApplicant(agent);

        (
            uint256 returnedId,
            address returnedAddress,
            AgentApplication.ApplicationStatus returnedStatus,
            bytes memory returnedData
        ) = application.getApplicantById(applicantId);

        assertEq(returnedId, applicantId);
        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(APPROVED));
        assertEq(returnedData, TEST_DATA);
        vm.stopPrank();
    }

    function test_GetApplicantByAddress() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);
        application.approveApplicant(agent);

        (
            uint256 returnedId,
            address returnedAddress,
            AgentApplication.ApplicationStatus returnedStatus,
            bytes memory returnedData
        ) = application.getApplicantByAddress(agent);

        assertEq(returnedId, applicantId);
        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(APPROVED));
        assertEq(returnedData, TEST_DATA);
        vm.stopPrank();
    }

    function test_RevertWhen_GetApplicantByAddressWithZeroAddress() public {
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.getApplicantByAddress(address(0));
    }

    function test_RevertWhen_GetApplicantByAddressNonExistent() public {
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.getApplicantByAddress(makeAddr("nonexistent"));
    }

    function test_RevertWhen_GetNonExistentApplicantById() public {
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.getApplicantById(999);
    }

    function test_ApplicantCount() public {
        assertEq(application.applicantCount(), 0);

        vm.startPrank(admin);
        application.addApplicant(makeAddr("agent1"), TEST_DATA);
        assertEq(application.applicantCount(), 1);

        application.addApplicant(makeAddr("agent2"), TEST_DATA);
        assertEq(application.applicantCount(), 2);
        vm.stopPrank();
    }

    function test_IsPermittedById() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);
        application.approveApplicant(agent);
        assertTrue(application.isPermittedById(applicantId));

        application.denyApplicant(agent);
        assertFalse(application.isPermittedById(applicantId));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress() public {
        vm.startPrank(admin);
        application.addApplicant(agent, TEST_DATA);
        application.approveApplicant(agent);
        assertTrue(application.isPermittedByAddress(agent));

        application.denyApplicant(agent);
        assertFalse(application.isPermittedByAddress(agent));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress_ZeroAddress() public view {
        assertFalse(application.isPermittedByAddress(address(0)));
    }

    function test_RevertWhen_DuplicateAgentApproval() public {
        vm.startPrank(admin);
        application.addApplicant(agent, TEST_DATA);

        vm.expectRevert(AgentApplication.AgentAlreadyApplied.selector);
        application.addApplicant(agent, TEST_DATA);
        vm.stopPrank();
    }

    function test_ApplicantLifecycle() public {
        vm.startPrank(admin);

        // Add applicant (PENDING)
        uint256 applicantId = application.addApplicant(agent, TEST_DATA);
        assertEq(application.applicantCount(), 1);

        // Check status is PENDING
        (,, AgentApplication.ApplicationStatus status,) = application.getApplicantById(applicantId);
        assertEq(uint256(status), uint256(PENDING));

        // Applicant should not be permitted yet
        assertFalse(application.isPermittedById(applicantId));

        // Approve the applicant
        application.approveApplicant(agent);
        (,, status,) = application.getApplicantById(applicantId);
        assertEq(uint256(status), uint256(APPROVED));

        // Applicant should now be permitted
        assertTrue(application.isPermittedById(applicantId));

        // Deny the applicant
        application.denyApplicant(agent);
        (,, status,) = application.getApplicantById(applicantId);
        assertEq(uint256(status), uint256(DENIED));

        // Applicant should no longer be permitted
        assertFalse(application.isPermittedById(applicantId));

        vm.stopPrank();
    }
}
