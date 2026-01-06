// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {L1Relayer} from "./L1Relayer.sol";

/**
 * @title RelayHelper
 * @dev Helper contract for using relayer contracts directly
 */
contract RelayHelper is AccessControl {
    /// @notice The address of the L1Relayer contract
    address public immutable l1Relayer;
    /// @notice The address of the L1 token
    address public immutable l1Token;

    /**
     * @notice Error thrown when the contract has insufficient token balance for a relay operation
     */
    error InsufficientBalance();

    /**
     * @notice Initializes the RelayHelper contract
     * @param _defaultAdmin The address of the default admin
     * @param _l1Relayer The address of the L1Relayer contract
     * @param _l1Token The address of the L1 token
     * @dev Sets the deployer as admin and configures the L1Relayer and L1Token contracts
     */
    constructor(address _defaultAdmin, address _l1Relayer, address _l1Token) {
        _grantRole(DEFAULT_ADMIN_ROLE, _defaultAdmin);
        l1Relayer = _l1Relayer;
        l1Token = _l1Token;
    }

    /**
     * @notice Sends the contract balance to L1Relayer and executes relay operation
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev This function sends the contract balance to the L1Relayer and executes the relay operation
     * @dev Only callable by admin
     */
    function relayContractBalance(address destination, uint256 epochIndex) external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 amount = IERC20(l1Token).balanceOf(address(this));
        if (amount == 0) revert InsufficientBalance();

        IERC20(l1Token).transfer(l1Relayer, amount);
        L1Relayer(l1Relayer).relay(destination, epochIndex);
    }

    /**
     * @notice Sends the allowance of the caller to L1Relayer and executes relay operation
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev This function transfers tokens from the caller to the L1Relayer and then executes the relay operation
     */
    function relaySenderAllowance(address destination, uint256 epochIndex) external {
        uint256 allowance = IERC20(l1Token).allowance(msg.sender, address(this));
        if (allowance == 0) revert InsufficientBalance();

        IERC20(l1Token).transferFrom(msg.sender, address(l1Relayer), allowance);
        L1Relayer(l1Relayer).relay(destination, epochIndex);
    }

    /**
     * @notice Withdraws tokens from the contract to the caller
     * @param amount The amount of tokens to withdraw
     * @param to The address to withdraw tokens to
     * @dev This function transfers tokens from the contract to the specified address
     */
    function withdraw(uint256 amount, address to) external onlyRole(DEFAULT_ADMIN_ROLE) {
        IERC20(l1Token).transfer(to, amount);
    }
}
