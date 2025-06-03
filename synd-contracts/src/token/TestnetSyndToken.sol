// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AbstractXERC20} from "./AbstractXERC20.sol";

/**
 * @title TestnetSyndToken
 * @notice Testnet version of Syndicate Token with XERC20 and testing utilities
 * @dev This contract is for testnet deployments. It is ERC20Votes, ERC20Permit, and XERC20 compatible.
 */
contract TestnetSyndToken is AbstractXERC20 {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor(address defaultAdmin, address minter) AbstractXERC20("Testnet Syndicate", "TestnetSYND") {
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (minter == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(MINTER_ROLE, minter);
        _grantRole(BRIDGE_MANAGER_ROLE, defaultAdmin);
    }

    /**
     * @notice Traditional mint function for testing (uses MINTER_ROLE)
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     */
    function adminMint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        if (amount == 0) revert ZeroAmount();

        _mint(to, amount);
    }
}
