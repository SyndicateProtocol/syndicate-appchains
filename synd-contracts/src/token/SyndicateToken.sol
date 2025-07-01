// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit, Nonces} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";

/**
 * @title SyndicateToken
 * @notice The main Syndicate Protocol token
 * @dev This contract implements the core SYND token deployed on Ethereum L1.
 *      It provides basic ERC20 functionality with governance capabilities.
 *      Emissions are handled by a separate SyndicateTokenEmissionScheduler contract.
 *
 * Key Features:
 * - ERC20 token with 1B total supply (900M initial + 100M emissions)
 * - Voting functionality via ERC20Votes for governance
 * - Permit functionality for gasless approvals
 * - Controlled minting for emissions (only by emission scheduler)
 * - Comprehensive access controls
 *
 * Supply Distribution:
 * - 900M tokens (90%): Initial mint to foundation
 * - 100M tokens (10%): Available for emissions via emission scheduler
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
contract SyndicateToken is ERC20, AccessControl, ERC20Permit, ERC20Votes {
    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when an address is zero
    error ZeroAddress();

    /// @notice Thrown when an amount is zero
    error ZeroAmount();

    /// @notice Thrown when trying to mint more than the total supply allows
    error ExceedsTotalSupply();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for minting emission tokens (typically the emission scheduler)
    bytes32 public constant EMISSION_MINTER_ROLE = keccak256("EMISSION_MINTER_ROLE");

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Total token supply: 1 billion tokens
    uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;

    /// @notice Initial mint to foundation: 900 million tokens (90%)
    uint256 public constant INITIAL_MINT_SUPPLY = 900_000_000 * 10 ** 18;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the Syndicate token contract
     * @param defaultAdmin Address that will have default admin privileges
     * @param syndTreasuryAddress Address to receive the initial token mint
     */
    constructor(address defaultAdmin, address syndTreasuryAddress)
        ERC20("Syndicate", "SYND")
        ERC20Permit("Syndicate")
        ERC20Votes()
    {
        // Input validation
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (syndTreasuryAddress == address(0)) revert ZeroAddress();

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);

        // Mint initial supply to foundation
        _mint(syndTreasuryAddress, INITIAL_MINT_SUPPLY);
    }

    /*//////////////////////////////////////////////////////////////
                            EMISSION MINTING
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Mint tokens
     * @dev This function can only be called by addresses with EMISSION_MINTER_ROLE.
     *      It is designed to be called by the SyndicateTokenEmissionScheduler contract.
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) external onlyRole(EMISSION_MINTER_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        // Check that we don't exceed the total supply
        if (totalSupply() + amount > TOTAL_SUPPLY) {
            revert ExceedsTotalSupply();
        }

        // Mint the tokens
        _mint(to, amount);
    }

    /*//////////////////////////////////////////////////////////////
                           BURN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Burn tokens from caller's balance
     * @param amount Amount of tokens to burn
     */
    function burn(uint256 amount) external {
        if (amount == 0) revert ZeroAmount();
        _burn(msg.sender, amount);
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get remaining tokens that can be minted
     * @return The amount of tokens still available to mint to reach TOTAL_SUPPLY
     */
    function getRemainingEmissions() external view returns (uint256) {
        return TOTAL_SUPPLY - totalSupply();
    }

    /*//////////////////////////////////////////////////////////////
                          GOVERNANCE HELPER FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get voting power of an account at current block
     * @param account The account to check
     * @return The current voting power
     */
    function getVotingPower(address account) external view returns (uint256) {
        return getVotes(account);
    }

    /**
     * @notice Get past voting power of an account at a specific block
     * @param account The account to check
     * @param blockNumber The block number to check
     * @return The voting power at that block
     */
    function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256) {
        return getPastVotes(account, blockNumber);
    }

    /**
     * @notice Get total supply at current block
     * @return Current total supply
     */
    function getCurrentTotalSupply() external view returns (uint256) {
        return totalSupply();
    }

    /*//////////////////////////////////////////////////////////////
                    REQUIRED OVERRIDES FOR MULTIPLE INHERITANCE
    //////////////////////////////////////////////////////////////*/

    /**
     * @dev Override required by Solidity for multiple inheritance
     */
    function _update(address from, address to, uint256 value) internal virtual override(ERC20, ERC20Votes) {
        super._update(from, to, value);
    }

    /**
     * @dev Override required by Solidity for multiple inheritance
     */
    function nonces(address owner) public view virtual override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }
}
