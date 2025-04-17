// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

/// @title IAgentApplication
/// @notice Interface for agent application status checks
interface IAgentApplication {
    /// @notice Check if an agent address is permitted
    /// @param agentAddress The address to check
    /// @return bool indicating if the agent is permitted
    function isPermittedByAddress(address agentAddress) external view returns (bool);

    /// @notice The admin role for the AgentApplication contract
    function ADMIN_ROLE() external view returns (bytes32);

    /// @notice Check if an agent has a role
    /// @dev Returns `true` if `account` has been granted `role`.
    function hasRole(bytes32 role, address account) external view returns (bool);
}
