// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

/// @title DreamAgentNFT
/// @notice An ERC721 contract for minting Dream Agent NFTs with role-based access control
/// @dev Inherits from ERC721 and AccessControl to implement NFT functionality with role-based permissions
contract DreamAgentNFT is ERC721, AccessControl {
    /// @notice Role identifier for addresses allowed to mint tokens
    /// @dev keccak256("MINTER_ROLE")
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    /// @notice Counter to track the next token ID to be minted
    /// @dev Increments by 1 for each new token minted
    uint256 private _nextTokenId;

    /// @notice Base URI for token metadata
    /// @dev Used as a prefix for all token URIs
    string public baseAgentNFTURI;

    /// @notice Initializes the contract with default admin and minter roles
    /// @dev Sets up the ERC721 metadata and grants initial roles
    /// @param defaultAdmin Address to be granted the default admin role
    /// @param minter Address to be granted the minter role
    constructor(address defaultAdmin, address minter) ERC721("DreamAgentNFT", "DREAM") {
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(MINTER_ROLE, minter);
    }

    /// @notice Safely mints a new token to the specified address
    /// @dev Can only be called by addresses with MINTER_ROLE
    /// @param to The address that will own the minted token
    function safeMint(address to) public onlyRole(MINTER_ROLE) {
        uint256 tokenId = _nextTokenId++;
        _safeMint(to, tokenId);
    }

    /// @notice Updates the base URI for all token metadata
    /// @dev Can only be called by addresses with DEFAULT_ADMIN_ROLE
    /// @param newBaseAgentNFTURI The new base URI to be set
    function setBaseAgentNFTURI(string memory newBaseAgentNFTURI) public onlyRole(DEFAULT_ADMIN_ROLE) {
        baseAgentNFTURI = newBaseAgentNFTURI;
    }

    /// @notice Returns the base URI for computing token URIs
    /// @dev Internal view function overriding ERC721 _baseURI
    /// @return The base URI string
    function _baseURI() internal view override returns (string memory) {
        return baseAgentNFTURI;
    }

    /// @notice Checks if the contract implements an interface
    /// @dev Required override to support both ERC721 and AccessControl interfaces
    /// @param interfaceId The interface identifier, as specified in ERC-165
    /// @return bool True if the contract implements `interfaceId`
    function supportsInterface(bytes4 interfaceId) public view override(ERC721, AccessControl) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
}
