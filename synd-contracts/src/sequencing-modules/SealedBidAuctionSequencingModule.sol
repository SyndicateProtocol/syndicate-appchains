// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IPermissionModule} from "../interfaces/IPermissionModule.sol";

/// @title SealedBidAuctionSequencingModule
/// @notice A sealed bid auction sequencing module contract where all bidders simultaneously submit sealed bids so that no bidder knows how much the other auction participants have bid.
contract SealedBidAuctionSequencingModule is IPermissionModule {
    struct Bid {
        bytes32 sealedBid;
        uint256 deposit;
    }

    /// @notice The address to receive the highest bid of the auction.
    address public treasury;
    /// @notice The type of auction.
    string public auctionType;
    /// @notice Indicates if the auction is active.
    bool public auctionActive;
    /// @notice The end time of the auction.
    uint256 public endTime;
    /// @notice The address of the highest bidder.
    address public highestBidder;
    /// @notice The highest bid amount, initialized to 0 by design as no bids exist at contract creation.
    uint256 public highestBid; //#olympix-ignore-uninitialized-state-variable

    /// @notice Mapping to store bids of each participant.
    mapping(address => Bid) public bids;
    /// @notice Mapping to store refunds for bidders who didn't win.
    mapping(address => uint256) public refunds;

    error AddressNotAllowed();
    error AuctionNotActive();
    error InvalidBidDeposit();
    error NoBidFound();
    error InvalidBidReveal();
    error NoFundsToWithdraw();
    error AuctionNotEnded();
    error TransactionFailed();
    error BidExceedsDeposit();
    error InvalidDuration();

    event BidRevealed(address indexed bidder, uint256 bid, bool isHighestBid);

    modifier onlyActive() {
        if (!auctionActive) revert AuctionNotActive();
        _;
    }

    /**
     * @notice Constructor to initialize the auction.
     * @param _duration Duration of the auction in seconds.
     * @param _treasury The address to receive the auction highest bid.
     */
    constructor(uint256 _duration, address _treasury) {
        if (_treasury == address(0)) revert AddressNotAllowed();
        // [Olympix Warning: insufficient parameter validation] Adding duration validation
        if (_duration == 0) revert InvalidDuration();

        treasury = _treasury;
        auctionType = "SealedBid";
        auctionActive = true;
        endTime = block.timestamp + _duration;
    }

    /**
     * @notice Returns the type of the auction.
     * @return The auction type as a string.
     */
    function getAuctionType() external view returns (string memory) {
        return auctionType;
    }

    /**
     * @notice Finalizes the auction, transferring the highest bid to the treasury.
     */
    function finalizeAuction() external {
        if (block.timestamp < endTime) revert AuctionNotEnded();
        auctionActive = false;
        if (highestBidder != address(0)) {
            // Forward all available gas to treasury to handle ETH transfer
            // This is intentional as the treasury may be a contract that needs gas to process the payment
            //#olympix-ignore-call-without-gas-budget
            (bool success,) = payable(treasury).call{value: highestBid}("");
            if (!success) revert TransactionFailed();
        }
    }

    /**
     * @notice Checks if the auction is active.
     * @return Boolean indicating if the auction is active.
     */
    function isAuctionActive() external view returns (bool) {
        return auctionActive;
    }

    /**
     * @notice Places a bid with a sealed bid hash.
     * @param _sealedBid The hash of the bid and salt.
     */
    function bid(bytes32 _sealedBid) external payable onlyActive {
        if (msg.value == 0) revert InvalidBidDeposit();
        bids[msg.sender] = Bid(_sealedBid, msg.value);
    }

    /**
     * @notice Reveals the actual bid and salt, checking it against the sealed bid.
     * @param _bid The actual bid amount.
     * @param _salt The salt used to hash the bid.
     */
    //#olympix-ignore-signature-replay-attacks
    function revealBid(uint256 _bid, string memory _salt) external onlyActive {
        Bid memory bidData = bids[msg.sender];
        if (bidData.deposit == 0) revert NoBidFound();

        if (_bid > bidData.deposit) revert BidExceedsDeposit();

        bytes32 sealedBid = keccak256(abi.encodePacked(_bid, _salt));

        if (sealedBid != bidData.sealedBid) revert InvalidBidReveal();

        bool isHighest = false;
        if (_bid > highestBid) {
            if (highestBidder != address(0)) {
                refunds[highestBidder] = highestBid;
            }
            highestBid = _bid;
            highestBidder = msg.sender;
            isHighest = true;
        } else {
            refunds[msg.sender] = bidData.deposit;
        }

        emit BidRevealed(msg.sender, _bid, isHighest);

        bids[msg.sender].deposit = 0;
    }

    /**
     * @notice Withdraws the funds for a non-winning bid.
     */
    function withdrawFunds() external {
        uint256 refund = refunds[msg.sender];
        if (refund == 0) revert NoFundsToWithdraw();
        refunds[msg.sender] = 0;
        // Forward all available gas to handle ETH transfer
        // This is intentional as the recipient may be a contract that needs gas to process the refund
        //#olympix-ignore-call-without-gas-budget
        (bool success,) = payable(msg.sender).call{value: refund}("");
        if (!success) revert TransactionFailed();
    }

    /**
     * @notice Returns the address of the auction winner.
     * @return The address of the highest bidder.
     */
    function getAuctionWinner() external view returns (address) {
        return highestBidder;
    }

    /**
     * @notice Returns the current highest bid.
     * @return The highest bid amount.
     */
    function getCurrentPrice() external view returns (uint256) {
        return highestBid;
    }

    /**
     * @notice Returns the end time of the auction.
     * @return The auction end time as a timestamp.
     */
    function getAuctionEndTime() external view returns (uint256) {
        return endTime;
    }

    /**
     * @notice Checks if the caller is allowed to sequence.
     * @return Boolean indicating if the caller is the highest bidder.
     */
    function isAllowed(address proposer, address, bytes calldata) external view override returns (bool) {
        return proposer == highestBidder;
    }
}
