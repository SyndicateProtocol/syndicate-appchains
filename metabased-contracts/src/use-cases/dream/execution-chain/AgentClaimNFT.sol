// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

interface IDreamAgentNFT is IERC721 {
    function safeMint(address to) external;
}

/// @title AgentClaimNFT
/// @notice Manages the claiming process for DreamAgentNFTs by approved agents
/// @dev Ensures each agent can only claim one NFT and maintains a list of eligible claimers
contract AgentClaimNFT is Ownable {
    /// @notice The DreamAgentNFT contract that this claim contract can mint
    IDreamAgentNFT public immutable dreamAgentNFT;

    /// @notice Mapping to track which addresses are allowed to claim NFTs
    mapping(address => bool) public canClaim;

    /// @notice Mapping to track which addresses have already claimed their NFT
    mapping(address => bool) public hasClaimed;

    /// @notice Emitted when an agent is granted claim permission
    event ClaimPermissionGranted(address indexed agent);

    /// @notice Emitted when an agent's claim permission is revoked
    event ClaimPermissionRevoked(address indexed agent);

    /// @notice Emitted when an agent successfully claims their NFT
    event NFTClaimed(address indexed agent, uint256 tokenId);

    /// @notice Custom errors
    error NotAllowedToClaim();
    error AlreadyClaimed();

    /// @notice Initializes the contract with the DreamAgentNFT contract address
    /// @param _dreamAgentNFT Address of the DreamAgentNFT contract
    /// @param _admin Address of the admin who can grant claim permissions
    constructor(address _dreamAgentNFT, address _admin) Ownable(_admin) {
        dreamAgentNFT = IDreamAgentNFT(_dreamAgentNFT);
    }

    /// @notice Allows admin to grant claim permission to an agent
    /// @param agent Address of the agent to grant claim permission
    function grantClaimPermission(address agent) external onlyOwner {
        canClaim[agent] = true;
        emit ClaimPermissionGranted(agent);
    }

    /// @notice Allows admin to revoke claim permission from an agent
    /// @param agent Address of the agent to revoke claim permission
    function revokeClaimPermission(address agent) external onlyOwner {
        canClaim[agent] = false;
        emit ClaimPermissionRevoked(agent);
    }

    /// @notice Allows an approved agent to claim their NFT
    /// @dev Checks if agent is approved, hasn't claimed before
    function claimNFT() external {
        // Check if the sender is allowed to claim
        if (!canClaim[msg.sender]) revert NotAllowedToClaim();

        // Check if they've already claimed
        if (hasClaimed[msg.sender]) revert AlreadyClaimed();

        // Mark as claimed
        hasClaimed[msg.sender] = true;

        // Mint the NFT
        dreamAgentNFT.safeMint(msg.sender);

        emit NFTClaimed(msg.sender, dreamAgentNFT.balanceOf(msg.sender));
    }

    /// @notice Allows admin to batch grant claim permissions
    /// @param agents Array of addresses to grant claim permission to
    function batchGrantClaimPermission(address[] calldata agents) external onlyOwner {
        uint256 length = agents.length;
        for (uint256 i = 0; i < length; i++) {
            canClaim[agents[i]] = true;
            emit ClaimPermissionGranted(agents[i]);
        }
    }

    /// @notice Checks if an address can claim an NFT
    /// @param agent Address to check
    /// @return bool Whether the address can claim an NFT
    function isEligibleToClaim(address agent) external view returns (bool) {
        return canClaim[agent] && !hasClaimed[agent] && dreamAgentNFT.balanceOf(agent) == 0;
    }
}
