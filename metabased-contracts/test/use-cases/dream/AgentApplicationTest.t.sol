// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AgentApplication, Ownable} from "src/use-cases/dream/AgentApplication.sol";

contract AgentApplicationTest is Test {
    AgentApplication public application;

    address public admin;
    address public agent;

    event ApplicantAdded(
        uint256 indexed applicantId, string name, address indexed agentAddress, bytes additionalData, bool isPermitted
    );
    event ApplicantPermissionUpdated(uint256 indexed applicantId, bool isPermitted);

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
        bool isPermitted = true;

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, true);
        emit ApplicantAdded(0, name, agent, additionalData, isPermitted);

        uint256 applicantId = application.addApplicant(name, agent, additionalData, isPermitted);

        (address returnedAddress, bool returnedPermission, string memory returnedName, bytes memory returnedData) =
            application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(returnedPermission, isPermitted);
        assertEq(returnedName, name);
        assertEq(returnedData, additionalData);
        assertEq(applicantId, 0);
        vm.stopPrank();
    }

    function test_RevertWhen_AddApplicantWithZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.addApplicant("Test", address(0), "", true);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerAddsApplicant() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.addApplicant("Test", agent, "", true);
        vm.stopPrank();
    }

    function test_UpdateApplicantPermission() public {
        // First add an applicant
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "", false);

        // Then update their permission
        vm.expectEmit(true, false, false, true);
        emit ApplicantPermissionUpdated(applicantId, true);

        application.updateApplicantPermission(applicantId, true);

        (, bool isPermitted,,) = application.getApplicant(applicantId);
        assertTrue(isPermitted);
        vm.stopPrank();
    }

    function test_RevertWhen_UpdateNonExistentApplicant() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.ApplicantNotFound.selector);
        application.updateApplicantPermission(999, true);
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerUpdatesPermission() public {
        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant("Test", agent, "", false);
        vm.stopPrank();

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.updateApplicantPermission(applicantId, true);
        vm.stopPrank();
    }

    function test_GetApplicant() public {
        string memory name = "Test Agent";
        bytes memory additionalData = "test data";
        bool isPermitted = true;

        vm.startPrank(admin);
        uint256 applicantId = application.addApplicant(name, agent, additionalData, isPermitted);

        (address returnedAddress, bool returnedPermission, string memory returnedName, bytes memory returnedData) =
            application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(returnedPermission, isPermitted);
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
        application.addApplicant("Test1", makeAddr("agent1"), "", true);
        assertEq(application.applicantCount(), 1);

        application.addApplicant("Test2", makeAddr("agent2"), "", false);
        assertEq(application.applicantCount(), 2);
        vm.stopPrank();
    }
}
