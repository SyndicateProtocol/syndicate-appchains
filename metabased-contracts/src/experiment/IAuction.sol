// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/**
 * @title IAuction
 * @dev Interface for a modular auction type monetization mechanism.
 */
interface IAuction { // Or IMonetizationMechanism?
    /**
     * @notice Initializes the auction with necessary parameters.
     */
    function initializeAuction() external; // idea to name this more generic to include other types of monetization mechanism: openAction(), start(), initProcess(), etc.

    /**
     * @notice Retrieves the type of auction.
     * @return The auction type as a string.
     */
    function getAuctionType() external view returns (string memory);

    /**
     * @notice Finalizes the auction, declaring the winner and assigning sequence epoch to winner.
     */
    function finalizeAuction() external;

    /**
     * @notice Checks if the auction is currently active.
     * @return True if the auction is active, false otherwise.
     */
    function isAuctionActive() external view returns (bool);

    /**
     * @notice Allows for placing a bid in the auction.
     */
    function bid() external payable;

    /**
     * @notice Allows for placing a sealed bid in the auction.
     * @param _sealedBid The sealed bid to place in the auction.
     */
    function bid(bytes32 _sealedBid) external payable; // idea to include sealed bid as a parameter

    /**
     * @notice Withdraws funds from the auction, incase of overbid or if auction requires deposit, others but the winner can withdraw funds.
     */
    function withdrawFunds() external;

    /**
     * @notice Retrieves the winner of the auction.
     * @return The address of the auction winner. The address that received sequence epoch
     */
    function getAuctionWinner() external view returns (address);

    /**
     * @notice Gets the current price in the auction.
     * @return The current highest bid or price in the auction.
     */
    function getCurrentPrice() external view returns (uint256);

    /**
     * @notice Gets the end time of the auction in case auction has it.
     * @return The timestamp when the auction ends.
     */
    function getAuctionEndTime() external view returns (uint256);
}
