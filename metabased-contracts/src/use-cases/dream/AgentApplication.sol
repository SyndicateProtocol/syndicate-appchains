// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title IMetabasedSequencerChain
/// @notice Interface for the MetabasedSequencerChain contract
interface IMetabasedSequencerChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external;
}

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

    IMetabasedSequencerChain public immutable sequencerChain;
    address public agentClaimNFTOwner;
    address public agentClaimNFTAddress;

    // Events
    event ApplicantAdded(
        uint256 indexed applicantId, address indexed agentAddress, bytes additionalData, ApplicationStatus status
    );
    event ApplicantStatusUpdated(uint256 indexed applicantId, ApplicationStatus status);
    event AgentClaimNFTOwnerUpdated(address indexed oldOwner, address indexed newOwner);
    event AgentClaimNFTAddressUpdated(address indexed oldAddress, address indexed newAddress);

    // Errors
    error ApplicantNotFound();
    error InvalidAddress();
    error AgentAlreadyApplied();

    /// @notice Initialize the contract with necessary addresses
    /// @param admin The admin address for the contract
    /// @param _sequencerChain The MetabasedSequencerChain contract address
    /// @param _agentClaimNFTOwner The owner address of the AgentClaimNFT contract
    /// @param _agentClaimNFTAddress The AgentClaimNFT contract address
    constructor(address admin, address _sequencerChain, address _agentClaimNFTOwner, address _agentClaimNFTAddress)
        Ownable(admin)
    {
        if (_sequencerChain == address(0) || _agentClaimNFTOwner == address(0) || _agentClaimNFTAddress == address(0)) {
            revert InvalidAddress();
        }

        sequencerChain = IMetabasedSequencerChain(_sequencerChain);
        agentClaimNFTOwner = _agentClaimNFTOwner;
        agentClaimNFTAddress = _agentClaimNFTAddress;
    }

    /// @notice Update the AgentClaimNFT owner address
    /// @param newOwner The new owner address
    function setAgentClaimNFTOwner(address newOwner) external onlyOwner {
        if (newOwner == address(0)) revert InvalidAddress();
        address oldOwner = agentClaimNFTOwner;
        agentClaimNFTOwner = newOwner;
        emit AgentClaimNFTOwnerUpdated(oldOwner, newOwner);
    }

    /// @notice Update the AgentClaimNFT contract address
    /// @param newAddress The new contract address
    function setAgentClaimNFTAddress(address newAddress) external onlyOwner {
        if (newAddress == address(0)) revert InvalidAddress();
        address oldAddress = agentClaimNFTAddress;
        agentClaimNFTAddress = newAddress;
        emit AgentClaimNFTAddressUpdated(oldAddress, newAddress);
    }

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
        if (existingId != 0 || applicants[0].agentAddress == agentAddress) {
            revert AgentAlreadyApplied();
        }

        applicantId = applicantCount++;
        applicants[applicantId] = Applicant({
            agentAddress: agentAddress,
            status: ApplicationStatus.APPROVED,
            isValid: true,
            additionalData: additionalData
        });
        agentToApplicantId[agentAddress] = applicantId;

        emit ApplicantAdded(applicantId, agentAddress, additionalData, ApplicationStatus.APPROVED);

        // Process the grantClaimPermission transaction through the sequencer chain
        bytes memory fullCalldata = _constructGrantClaimCalldata(agentAddress);
        sequencerChain.processTransaction(fullCalldata);
    }

    /// @notice Deny an applicant's application
    /// @param applicantId The ID of the applicant to deny
    function denyApplicant(uint256 applicantId) external onlyOwner {
        Applicant storage applicant = applicants[applicantId];
        if (!applicant.isValid) revert ApplicantNotFound();

        applicant.status = ApplicationStatus.DENIED;
        emit ApplicantStatusUpdated(applicantId, ApplicationStatus.DENIED);
    }

    /// @notice Constructs the calldata for granting claim permission
    /// @param agentAddress The address of the agent to grant permission to
    /// @return The constructed calldata for the sequencer
    function _constructGrantClaimCalldata(address agentAddress) internal view returns (bytes memory) {
        bytes memory grantCalldata = abi.encodeWithSignature("grantClaimPermission(address)", agentAddress);

        return abi.encodePacked(
            agentClaimNFTOwner, // from address
            agentClaimNFTAddress, // to address
            uint256(0), // value
            uint256(grantCalldata.length), // length of the calldata
            grantCalldata // actual calldata
        );
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
