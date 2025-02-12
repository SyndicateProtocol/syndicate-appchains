// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract AgentApplication is Ownable {
    struct Applicant {
        // Pack these together in first storage slot
        address agentAddress;
        bool isPermitted;
        // These use their own slots
        string name;
        bytes additionalData;
    }

    // Mapping from applicant ID to Applicant
    mapping(uint256 => Applicant) public applicants;
    uint256 public applicantCount;

    // Events
    event ApplicantAdded(
        uint256 indexed applicantId, string name, address indexed agentAddress, bytes additionalData, bool isPermitted
    );
    event ApplicantPermissionUpdated(uint256 indexed applicantId, bool isPermitted);

    // Errors
    error ApplicantNotFound();
    error InvalidAddress();

    constructor(address admin) Ownable(admin) {}

    /// @notice Add a new agent applicant
    /// @param name The name of the applicant
    /// @param agentAddress The address of the applicant
    /// @param additionalData Any additional data about the applicant
    /// @param isPermitted Initial permission status
    /// @return applicantId The ID assigned to the new applicant
    function addApplicant(string calldata name, address agentAddress, bytes calldata additionalData, bool isPermitted)
        external
        onlyOwner
        returns (uint256 applicantId)
    {
        if (agentAddress == address(0)) revert InvalidAddress();

        applicantId = applicantCount++;
        applicants[applicantId] = Applicant({
            agentAddress: agentAddress,
            isPermitted: isPermitted,
            name: name,
            additionalData: additionalData
        });

        emit ApplicantAdded(applicantId, name, agentAddress, additionalData, isPermitted);
    }

    /// @notice Update an applicant's permission status
    /// @param applicantId The ID of the applicant
    /// @param isPermitted The new permission status
    function updateApplicantPermission(uint256 applicantId, bool isPermitted) external onlyOwner {
        // Will revert with default values if applicant doesn't exist
        address applicantAddr = applicants[applicantId].agentAddress;
        if (applicantAddr == address(0)) revert ApplicantNotFound();

        applicants[applicantId].isPermitted = isPermitted;
        emit ApplicantPermissionUpdated(applicantId, isPermitted);
    }

    /// @notice Get all applicant information
    /// @param applicantId The ID of the applicant
    /// @return agentAddress The address of the applicant
    /// @return isPermitted The permission status of the applicant
    /// @return name The name of the applicant
    /// @return additionalData Any additional data about the applicant
    function getApplicant(uint256 applicantId)
        external
        view
        returns (address agentAddress, bool isPermitted, string memory name, bytes memory additionalData)
    {
        Applicant storage applicant = applicants[applicantId];
        if (applicant.agentAddress == address(0)) revert ApplicantNotFound();

        return (applicant.agentAddress, applicant.isPermitted, applicant.name, applicant.additionalData);
    }
}
