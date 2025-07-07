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
 * - Time-based transfer restrictions for airdrop tokens
 * - Airdrop manager controls for emergency actions during lock period
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

    /// @notice Thrown when transfers are locked and caller is not authorized
    error TransfersLocked();

    /// @notice Thrown when trying to set unlock timestamp beyond maximum allowed
    error UnlockTimestampTooLate();

    /// @notice Thrown when trying to set unlock timestamp in the past
    error UnlockTimestampInPast();

    /// @notice Thrown when trying to burn tokens outside of lock period
    error BurnOnlyDuringLockPeriod();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for minting emission tokens (typically the emission scheduler)
    bytes32 public constant EMISSION_MINTER_ROLE = keccak256("EMISSION_MINTER_ROLE");

    /// @notice Role for managing airdrop operations (transfer, burn during lock period)
    bytes32 public constant AIRDROP_MANAGER_ROLE = keccak256("AIRDROP_MANAGER_ROLE");

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Total token supply: 1 billion tokens
    uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;

    /// @notice Initial mint to foundation: 900 million tokens (90%)
    uint256 public constant INITIAL_MINT_SUPPLY = 900_000_000 * 10 ** 18;

    /// @notice Maximum lock duration: 90 days
    uint256 public constant MAX_LOCK_DURATION = 90 days;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Timestamp when tokens become transferable (0 = no restrictions)
    uint256 public unlockTimestamp;

    /// @notice Maximum timestamp until which tokens can be locked
    uint256 public immutable maxLockTimestamp;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when unlock timestamp is updated
     * @param oldTimestamp Previous unlock timestamp
     * @param newTimestamp New unlock timestamp
     * @param updatedBy Address that updated the timestamp
     */
    event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);

    /**
     * @notice Emitted when tokens are burned by airdrop manager
     * @param from Address tokens were burned from
     * @param amount Amount of tokens burned
     * @param burner Address that performed the burn
     */
    event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);

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

        // Set maximum lock timestamp (contract deployment + MAX_LOCK_DURATION)
        maxLockTimestamp = block.timestamp + MAX_LOCK_DURATION;

        // Initialize with no transfer restrictions (unlockTimestamp = 0)
        unlockTimestamp = 0;

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

    /**
     * @notice Burn tokens from a specific address (only during lock period)
     * @dev Only callable by addresses with AIRDROP_MANAGER_ROLE and only during lock period
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     */
    function burnFrom(address from, uint256 amount) external onlyRole(AIRDROP_MANAGER_ROLE) {
        if (from == address(0)) revert ZeroAddress();

        if (amount == 0) revert ZeroAmount();

        if (!transfersLocked()) revert BurnOnlyDuringLockPeriod();

        _burn(from, amount);
        emit TokensBurnedByManager(from, amount, msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                        LOCK MANAGEMENT FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the unlock timestamp for transfer restrictions
     * @dev Only callable by addresses with DEFAULT_ADMIN_ROLE
     * @param newUnlockTimestamp New timestamp when transfers become allowed (must be > 0 and future)
     */
    function setUnlockTimestamp(uint256 newUnlockTimestamp) external onlyRole(DEFAULT_ADMIN_ROLE) {
        // Must be a future timestamp (cannot be 0 to disable restrictions)
        if (newUnlockTimestamp <= block.timestamp) revert UnlockTimestampInPast();
        if (newUnlockTimestamp > maxLockTimestamp) revert UnlockTimestampTooLate();

        uint256 oldTimestamp = unlockTimestamp;
        unlockTimestamp = newUnlockTimestamp;

        emit UnlockTimestampUpdated(oldTimestamp, newUnlockTimestamp, msg.sender);
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

    /**
     * @notice Check if transfers are currently locked
     * @return True if transfers are locked, false otherwise
     */
    function transfersLocked() public view returns (bool) {
        return unlockTimestamp != 0 && block.timestamp < unlockTimestamp;
    }

    /**
     * @notice Get remaining lock time in seconds
     * @return Seconds remaining until unlock (0 if already unlocked)
     */
    function getRemainingLockTime() external view returns (uint256) {
        if (unlockTimestamp == 0 || block.timestamp >= unlockTimestamp) {
            return 0;
        }
        return unlockTimestamp - block.timestamp;
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
     * @dev Implements transfer restrictions based on lock timestamp and roles
     */
    function _update(address from, address to, uint256 value) internal virtual override(ERC20, ERC20Votes) {
        // Allow minting (from == address(0)) and burning (to == address(0))
        if (from != address(0) && to != address(0)) {
            // Check if transfers are locked and caller is not authorized
            if (transfersLocked() && !hasRole(AIRDROP_MANAGER_ROLE, msg.sender)) {
                revert TransfersLocked();
            }
        }

        // Call the parent implementation
        super._update(from, to, value);
    }

    /**
     * @dev Override required by Solidity for multiple inheritance
     */
    function nonces(address owner) public view virtual override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }
}
