// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AgentApplication, Ownable, IMetabasedSequencerChain} from "src/use-cases/dream/AgentApplication.sol";

contract AgentApplicationTest is Test {
    AgentApplication public application;
    AgentApplication.ApplicationStatus constant PENDING = AgentApplication.ApplicationStatus.PENDING;
    AgentApplication.ApplicationStatus constant APPROVED = AgentApplication.ApplicationStatus.APPROVED;
    AgentApplication.ApplicationStatus constant DENIED = AgentApplication.ApplicationStatus.DENIED;

    address public admin;
    address public agent;
    address public sequencerChain;
    address public agentClaimNFTOwner;
    address public agentClaimNFTAddress;

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
        sequencerChain = makeAddr("sequencerChain");
        agentClaimNFTOwner = makeAddr("agentClaimNFTOwner");
        agentClaimNFTAddress = makeAddr("agentClaimNFTAddress");

        // Mock the sequencer chain
        vm.etch(sequencerChain, hex"00");

        vm.startPrank(admin);
        application = new AgentApplication(admin, sequencerChain, agentClaimNFTOwner, agentClaimNFTAddress);
        vm.stopPrank();
    }

    function _getExpectedGrantClaimCalldata(address agentAddress) internal view returns (bytes memory) {
        bytes memory grantCalldata = abi.encodeWithSignature("grantClaimPermission(address)", agentAddress);
        return abi.encodePacked(
            agentClaimNFTOwner, agentClaimNFTAddress, uint256(0), uint256(grantCalldata.length), grantCalldata
        );
    }

    function test_Constructor() public view {
        assertEq(application.owner(), admin);
        assertEq(address(application.sequencerChain()), sequencerChain);
        assertEq(application.agentClaimNFTOwner(), agentClaimNFTOwner);
        assertEq(application.agentClaimNFTAddress(), agentClaimNFTAddress);
    }

    function test_RevertWhen_ConstructorZeroAddress() public {
        vm.startPrank(admin);

        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        new AgentApplication(admin, address(0), agentClaimNFTOwner, agentClaimNFTAddress);

        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        new AgentApplication(admin, sequencerChain, address(0), agentClaimNFTAddress);

        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        new AgentApplication(admin, sequencerChain, agentClaimNFTOwner, address(0));
        vm.stopPrank();
    }

    function test_SetAgentClaimNFTOwner() public {
        address newOwner = makeAddr("newOwner");

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit AgentClaimNFTOwnerUpdated(agentClaimNFTOwner, newOwner);

        application.setAgentClaimNFTOwner(newOwner);
        assertEq(application.agentClaimNFTOwner(), newOwner);
        vm.stopPrank();
    }

    function test_SetAgentClaimNFTAddress() public {
        address newAddress = makeAddr("newAddress");

        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit AgentClaimNFTAddressUpdated(agentClaimNFTAddress, newAddress);

        application.setAgentClaimNFTAddress(newAddress);
        assertEq(application.agentClaimNFTAddress(), newAddress);
        vm.stopPrank();
    }

    function test_RevertWhen_SetAgentClaimNFTOwnerZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.setAgentClaimNFTOwner(address(0));
        vm.stopPrank();
    }

    function test_RevertWhen_SetAgentClaimNFTAddressZeroAddress() public {
        vm.startPrank(admin);
        vm.expectRevert(AgentApplication.InvalidAddress.selector);
        application.setAgentClaimNFTAddress(address(0));
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerSetsAgentClaimNFTOwner() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.setAgentClaimNFTOwner(makeAddr("newOwner"));
        vm.stopPrank();
    }

    function test_RevertWhen_NonOwnerSetsAgentClaimNFTAddress() public {
        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.setAgentClaimNFTAddress(makeAddr("newAddress"));
        vm.stopPrank();
    }

    function test_ApproveApplicant() public {
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);

        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        vm.startPrank(admin);
        vm.expectCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata)
        );

        vm.expectEmit(true, true, false, true);
        emit ApplicantAdded(0, agent, TEST_DATA, APPROVED);

        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        (address returnedAddress, AgentApplication.ApplicationStatus returnedStatus, bytes memory returnedData) =
            application.getApplicant(applicantId);

        assertEq(returnedAddress, agent);
        assertEq(uint256(returnedStatus), uint256(APPROVED));
        assertEq(returnedData, TEST_DATA);
        assertEq(applicantId, 0);
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
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

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
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        vm.prank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);

        vm.startPrank(agent);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent));
        application.denyApplicant(applicantId);
        vm.stopPrank();
    }

    function test_GetApplicant() public {
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

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
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(makeAddr("agent1"));
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        assertEq(application.applicantCount(), 0);

        vm.startPrank(admin);
        application.approveApplicant(makeAddr("agent1"), TEST_DATA);
        assertEq(application.applicantCount(), 1);

        expectedCalldata = _getExpectedGrantClaimCalldata(makeAddr("agent2"));
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        application.approveApplicant(makeAddr("agent2"), TEST_DATA);
        assertEq(application.applicantCount(), 2);
        vm.stopPrank();
    }

    function test_IsPermittedById() public {
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        vm.startPrank(admin);
        uint256 applicantId = application.approveApplicant(agent, TEST_DATA);
        assertTrue(application.isPermittedById(applicantId));

        application.denyApplicant(applicantId);
        assertFalse(application.isPermittedById(applicantId));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress() public {
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        vm.startPrank(admin);
        application.approveApplicant(agent, TEST_DATA);
        assertTrue(application.isPermittedByAddress(agent));

        application.denyApplicant(0);
        assertFalse(application.isPermittedByAddress(agent));
        vm.stopPrank();
    }

    function test_IsPermittedByAddress_ZeroAddress() public view {
        assertFalse(application.isPermittedByAddress(address(0)));
    }

    function test_RevertWhen_DuplicateAgentApproval() public {
        bytes memory expectedCalldata = _getExpectedGrantClaimCalldata(agent);
        vm.mockCall(
            sequencerChain,
            abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, expectedCalldata),
            ""
        );

        vm.startPrank(admin);
        application.approveApplicant(agent, TEST_DATA);
        vm.expectRevert(AgentApplication.AgentAlreadyApplied.selector);
        application.approveApplicant(agent, TEST_DATA);
        vm.stopPrank();
    }
}
