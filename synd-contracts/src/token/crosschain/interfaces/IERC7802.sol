// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

/// @title ERC-7802 Crosschain Fungibility Extension for ERC-20
/// @dev See https://eips.ethereum.org/EIPS/eip-7802
/// @notice Interface for crosschain token minting and burning.
/// @dev Used in SuperchainERC20 contract
interface IERC7802 is IERC165 {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when tokens are minted by a bridge
    /// @param to The address of the recipient
    /// @param amount The amount of tokens minted
    /// @param bridge The address of the bridge that minted the tokens
    event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);

    /// @notice Emitted when tokens are burned by a bridge
    /// @param from The address of the sender
    /// @param amount The amount of tokens burned
    /// @param bridge The address of the bridge that burned the tokens
    event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);

    /*//////////////////////////////////////////////////////////////
                            CORE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Mints tokens to the recipient (called by authorized bridges)
    /// @param to The address of the recipient
    /// @param amount The amount of tokens to mint
    function crosschainMint(address to, uint256 amount) external;

    /// @notice Burns tokens from the sender (called by authorized bridges)
    /// @param from The address of the sender
    /// @param amount The amount of tokens to burn
    function crosschainBurn(address from, uint256 amount) external;
}
