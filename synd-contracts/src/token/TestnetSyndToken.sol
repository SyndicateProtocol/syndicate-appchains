// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit, Nonces} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";

/**
 * @title TestnetSyndToken
 * @notice Testnet version of Syndicate Token with testing utilities
 * @dev This contract is for testnet deployments. It is ERC20Votes and ERC20Permit compatible.
 */
contract TestnetSyndToken is ERC20, AccessControl, ERC20Permit, ERC20Votes {
    /// @notice Thrown when an address is zero
    error ZeroAddress();

    /// @notice Thrown when an amount is zero
    error ZeroAmount();

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor(address defaultAdmin, address minter)
        ERC20("Testnet Syndicate", "TestnetSYND")
        ERC20Permit("Testnet Syndicate")
        ERC20Votes()
    {
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (minter == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(MINTER_ROLE, minter);
    }

    /**
     * @notice Mint function for testing (uses MINTER_ROLE)
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        _mint(to, amount);
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
    function getPastVotingPowerToChange(address account, uint256 blockNumber) external view returns (uint256) {
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
