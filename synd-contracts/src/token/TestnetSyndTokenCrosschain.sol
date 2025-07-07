// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateTokenCrosschain, IBridgeRateLimiter, IERC7802} from "./SyndicateTokenCrosschain.sol";
import {TestnetSyndToken} from "./TestnetSyndToken.sol";

/**
 * @title TestnetSyndTokenCrosschain
 * @notice Testnet crosschain-compatible Syndicate Protocol token
 * @dev Combines TestnetSyndToken functionality with SyndicateTokenCrosschain capabilities
 *
 * This contract provides all the features of both parent contracts:
 * - TestnetSyndToken: Flexible minting for testing, no initial supply constraints
 * - SyndicateTokenCrosschain: Crosschain capabilities, rate limiting, emission budgets
 *
 * Key Features:
 * - All TestnetSyndToken functionality (flexible minting for testing)
 * - All SyndicateTokenCrosschain functionality (crosschain capabilities, rate limiting, emission budgets)
 * - Secure bridge management and sliding window rate limiting
 * - Emission budget controls for crosschain minting
 * - Same contract address across all chains via deterministic deployment
 *
 */
contract TestnetSyndTokenCrosschain is SyndicateTokenCrosschain {
    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for minting tokens during testnet development
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the crosschain testnet Syndicate token
     * @param defaultAdmin Address that will have default admin privileges
     * @param minter Address that will have minter privileges for testnet
     */
    constructor(address defaultAdmin, address minter)
        SyndicateTokenCrosschain(defaultAdmin, defaultAdmin) // Use admin as treasury for testnet
    {
        // Override token metadata for testnet
        // Note: We can't override name/symbol in constructor due to ERC20 limitations
        // but this contract will have testnet-specific behavior

        // Grant testnet minter role
        _grantRole(MINTER_ROLE, minter);
        _grantRole(MINTER_ROLE, defaultAdmin);
    }

    /*//////////////////////////////////////////////////////////////
                        TESTNET MINTING FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Mint tokens for testnet purposes
     * @dev This function allows flexible minting during testnet development
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     */
    function mint(address to, uint256 amount) external override onlyRole(MINTER_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        // For testnet, we don't enforce total supply restrictions
        // This allows flexible testing without emission schedule constraints
        _mint(to, amount);
    }

    /*//////////////////////////////////////////////////////////////
                        METADATA OVERRIDES
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Returns the name of the token
     * @return The name of the testnet token
     */
    function name() public pure override returns (string memory) {
        return "Testnet Syndicate";
    }

    /**
     * @notice Returns the symbol of the token
     * @return The symbol of the testnet token
     */
    function symbol() public pure override returns (string memory) {
        return "TestnetSYND";
    }

    /*//////////////////////////////////////////////////////////////
                        INTERFACE SUPPORT
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Check if contract supports an interface
     * @param interfaceId Interface identifier to check
     * @return true if interface is supported
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IERC7802).interfaceId || interfaceId == type(IBridgeRateLimiter).interfaceId
            || super.supportsInterface(interfaceId);
    }
}
