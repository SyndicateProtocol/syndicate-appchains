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

    function test_ApproveApplicant() public {
        vm.startPrank(admin);

        vm.expectEmit(true, true, false, true);
        emit ApplicantAdded(1, agent, TEST_DATA, APPROVED);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        (address returnedAddress, AgentApplication.ApplicationStatus returnedStatus, bytes memory returnedData) =
            application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(APPROVED));
        assertEq(returnedData, TEST_DATA);
        assertEq(applicantId, 1);
        vm.stopPrank();
    }

    function test_RevertWhen_ApproveApplicantWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.approveApplicant(address(0), TEST_DATA);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerApprovesApplicant() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.approveApplicant(agent, TEST_DATA);
        vm.stopPrank();
    }

    function test_DenyApplicant() public {
        vm.startPrank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        vm.expectEmit(true, false, false, true);
        emit ApplicantStatusUpdated(applicantId, DENIED);

        application.denyApplicant(applicantId);

        (, AgentApplication.ApplicationStatus status,) = application.getApplicant(applicantId);
        assertEq(uint256(status), uint256(DENIED));
        vm.stopPrank();
    }

    function test_RevertWhen_DenyNonExistentApplicant() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.denyApplicant(999);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerDeniesApplicant() public {
        vm.prank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.denyApplicant(applicantId);
        vm.stopPrank();
    }

    function test_GetApplicant() public {
        vm.startPrank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        (address returnedAddress, AgentApplication.ApplicationStatus returnedStatus, bytes memory returnedData) =
            application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(APPROVED));
        assertEq(returnedData, TEST_DATA);
        vm.stopPrank();
    }

    function test_RevertWhen_GetNonExistentApplicant() public {
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.getApplicant(999);
    }

    function test_ApplicantCount() public {
        assertEq(application.applicantCount(), 0);

        vm.startPrank(admin);
        application.approveApplicant(makeAddr("agent1"), TEST_DATA);
        assertEq(application.applicantCount(), 1);

        application.approveApplicant(makeAddr("agent2"), TEST_DATA);
        assertEq(application.applicantCount(), 2);
        vm.stopPrank();
    }

    function test_IsPermittedById() public {
        vm.startPrank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);
        assertTrue(application.isPermittedById(applicantId));

        application.denyApplicant(applicantId);
        assertFalse(application.isPermittedById(applicantId));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress() public {
        vm.startPrank(admin);
        application.approveApplicant(agent, TEST_DATA);
        assertTrue(application.isPermittedByAddress(agent));

        application.denyApplicant(1);
        assertFalse(application.isPermittedByAddress(agent));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress_ZeroAddress() public view {
        assertFalse(application.isPermittedByAddress(address(0)));
    }

    function test_RevertWhen_DuplicateAgentApproval() public {
        vm.startPrank(admin);
        application.approveApplicant(agent, TEST_DATA);

        vm.expectRevert(AgentApplication.AgentAlreadyApplied.selector);
        application.approveApplicant(agent, TEST_DATA);
        vm.stopPrank();
    }
}
