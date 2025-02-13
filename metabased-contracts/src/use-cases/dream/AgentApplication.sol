// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @notice Interface for Dream chain Sequencer NFT contract
interface IDreamSequencer {
    function addAllowedMinter(address minter) external;
}

/// @title AgentApplication
/// @notice Manages the application process for Dream agents, allowing them to apply and be approved/denied
/// @dev Maintains a registry of agent applications and their statuses, with admin controls for approval management
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
        // These use their own slots
        string name;
        bytes additionalData;
    }

    // Mapping from applicant ID to Applicant
    mapping(uint256 => Applicant) public applicants;
    // Mapping from agent address to applicant ID
    mapping(address => uint256) public agentToApplicantId;
    uint256 public applicantCount;

    IDreamSequencer public dreamSequencer;

    // Events
    event ApplicantAdded(
        uint256 indexed applicantId,
        string name,
        address indexed agentAddress,
        bytes additionalData,
        ApplicationStatus status
    );
    event ApplicantStatusUpdated(uint256 indexed applicantId, ApplicationStatus status);

    // Errors
    error ApplicantNotFound();
    error InvalidAddress();
    error AgentAlreadyApplied();
    error DreamSequencerNotSet();

    constructor(address admin) Ownable(admin) {}

    /// @notice Set the Dream Sequencer contract address
    /// @param _dreamSequencer The address of the Dream Sequencer contract
    function setDreamSequencer(address _dreamSequencer) external onlyOwner {
        if (_dreamSequencer == address(0)) revert InvalidAddress();
        dreamSequencer = IDreamSequencer(_dreamSequencer);
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

    /// @notice Add a new agent applicant
    /// @param name The name of the applicant
    /// @param agentAddress The address of the applicant
    /// @param additionalData Any additional data about the applicant
    /// @return applicantId The ID assigned to the new applicant
    function addApplicant(string calldata name, address agentAddress, bytes calldata additionalData)
        external
        onlyOwner
        returns (uint256 applicantId)
    {
        if (agentAddress == address(0)) revert InvalidAddress();
        uint256 existingId = agentToApplicantId[agentAddress];
        if (existingId != 0 || applicants[0].agentAddress == agentAddress) {
            revert AgentAlreadyApplied();
        }

        applicantId = applicantCount++;
        applicants[applicantId] = Applicant({
            agentAddress: agentAddress,
            status: ApplicationStatus.PENDING,
            isValid: true,
            name: name,
            additionalData: additionalData
        });
        agentToApplicantId[agentAddress] = applicantId;

        emit ApplicantAdded(applicantId, name, agentAddress, additionalData, ApplicationStatus.PENDING);
    }

    /// @notice Approve an applicant's application and add them as an allowed minter
    /// @param applicantId The ID of the applicant to approve
    function approveApplicant(uint256 applicantId) external onlyOwner {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();
        if (address(dreamSequencer) == address(0)) revert DreamSequencerNotSet();

        applicant.status = ApplicationStatus.APPROVED;
        emit ApplicantStatusUpdated(applicantId, ApplicationStatus.APPROVED);

        // Update Dream chain Sequencer
        dreamSequencer.addAllowedMinter(applicant.agentAddress);
    }

    /// @notice Deny an applicant's application
    /// @param applicantId The ID of the applicant to deny
    function denyApplicant(uint256 applicantId) external onlyOwner {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();

        applicant.status = ApplicationStatus.DENIED;
        emit ApplicantStatusUpdated(applicantId, ApplicationStatus.DENIED);
    }

    /// @notice Get all applicant information
    /// @param applicantId The ID of the applicant
    /// @return agentAddress The address of the applicant
    /// @return status The application status
    /// @return name The name of the applicant
    /// @return additionalData Any additional data about the applicant
    function getApplicant(uint256 applicantId)
        external
        view
        returns (address agentAddress, ApplicationStatus status, string memory name, bytes memory additionalData)
    {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();

        return (applicant.agentAddress, applicant.status, applicant.name, applicant.additionalData);
    }
}
