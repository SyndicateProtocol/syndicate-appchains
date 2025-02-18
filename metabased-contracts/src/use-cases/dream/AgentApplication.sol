// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title AgentApplication
/// @notice Manages the application process for Dream agents
/// @dev Controls agent permissions and status with admin management
contract AgentApplication is Ownable {
    enum ApplicationStatus {
        PENDING,
        APPROVED,
        DENIED
    }

    struct Applicant {
        // Pack these together in first storage slot
        address agentAddress;
        ApplicationStatus status;
        bool isValid;
        // This uses its own slot
        bytes additionalData;
    }

    // Mapping from applicant ID to Applicant
    mapping(uint256 => Applicant) public applicants;
    // Mapping from agent address to applicant ID
    mapping(address => uint256) public agentToApplicantId;
    uint256 public applicantCount;

    // Events
    event ApplicantAdded(
        uint256 indexed applicantId, address indexed agentAddress, bytes additionalData, ApplicationStatus status
    );
    event ApplicantStatusUpdated(uint256 indexed applicantId, ApplicationStatus status);

    // Errors
    error ApplicantNotFound();
    error InvalidAddress();
    error AgentAlreadyApplied();

    /// @notice Initialize the contract with necessary addresses
    /// @param admin The admin address for the contract
    constructor(address admin) Ownable(admin) {}

    /// @notice Approves a new agent and grants claim permission
    /// @param agentAddress The address of the agent to approve
    /// @param additionalData Any additional data about the agent
    /// @return applicantId The ID assigned to the new applicant
    function approveApplicant(address agentAddress, bytes calldata additionalData)
        external
        onlyOwner
        returns (uint256 applicantId)
    {
        if (agentAddress == address(0)) revert InvalidAddress();

        uint256 existingId = agentToApplicantId[agentAddress];

        if (existingId != 0 && applicants[existingId].isValid) {
            revert AgentAlreadyApplied();
        }

        applicantCount++;
        applicantId = applicantCount;

        applicants[applicantId] = Applicant({
            agentAddress: agentAddress,
            status: ApplicationStatus.APPROVED,
            isValid: true,
            additionalData: additionalData
        });

        agentToApplicantId[agentAddress] = applicantId;

        emit ApplicantAdded(applicantId, agentAddress, additionalData, ApplicationStatus.APPROVED);
    }

    /// @notice Deny an applicant's application
    /// @param applicantId The ID of the applicant to deny
    function denyApplicant(uint256 applicantId) external onlyOwner {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();

        applicant.status = ApplicationStatus.DENIED;
        emit ApplicantStatusUpdated(applicantId, ApplicationStatus.DENIED);
    }

    /// @notice Check if an applicant is permitted by their ID
    /// @param applicantId The ID of the applicant
    /// @return bool indicating if the applicant is permitted
    function isPermittedById(uint256 applicantId) external view returns (bool) {
        Applicant storage applicant = applicants[applicantId];
        return applicant.isValid && applicant.status == ApplicationStatus.APPROVED;
    }

    /// @notice Check if an agent address is permitted
    /// @param agentAddress The address to check
    /// @return bool indicating if the agent is permitted
    function isPermittedByAddress(address agentAddress) external view returns (bool) {
        if (agentAddress == address(0)) return false;
        uint256 applicantId = agentToApplicantId[agentAddress];
        Applicant storage applicant = applicants[applicantId];
        return applicant.isValid && applicant.status == ApplicationStatus.APPROVED;
    }

    /// @notice Get all applicant information
    /// @param applicantId The ID of the applicant
    /// @return agentAddress The address of the applicant
    /// @return status The application status
    /// @return additionalData Any additional data about the applicant
    function getApplicant(uint256 applicantId)
        external
        view
        returns (address agentAddress, ApplicationStatus status, bytes memory additionalData)
    {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();

        return (applicant.agentAddress, applicant.status, applicant.additionalData);
    }
}
