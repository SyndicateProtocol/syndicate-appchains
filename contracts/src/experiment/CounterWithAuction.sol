// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IAuction} from "./IAuction.sol";

contract CounterWithAuction {
    uint256 public count;
    address public auctionAddress;

    error OnlyWinnerError();

    constructor(address _auctionAddress) {
        auctionAddress = _auctionAddress;
    }

    modifier onlyWinner() {
        IAuction auction = IAuction(auctionAddress);
        if (msg.sender != auction.getAuctionWinner()) revert OnlyWinnerError();
        _;
    }

    /// @notice Bids for a chance to increment the counter. Probably not needed as the bid can be done directly in the auction contract.
    /// @param sealedBid The sealed bid to place in the auction.
    function bidForChanceToIncrement(bytes32 sealedBid) external payable {
        IAuction auction = IAuction(auctionAddress);
        auction.bid{value: msg.value}(sealedBid);
    }

    /// @notice Only auction winner is able to increment the counter.
    function increment() external onlyWinner {
        count += 1;
    }
}
