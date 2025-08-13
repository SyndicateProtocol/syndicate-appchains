// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./SyndStaking.sol";

contract BasePool {
    address public immutable depositor;

    ISyndStaking public stakingContract;

    // Epoch index => emission total for that epoch
    mapping(uint256 => uint256) public epochTotal;
    // Epoch index => user => claimed
    mapping(uint256 => mapping(address => bool)) public claimed;

    event EpochDeposit(uint256 epochIndex, uint256 amount);
    event ClaimSuccess(uint256 epochIndex, address user, address destination, uint256 amount);

    error NotStaked();
    error AlreadyClaimed();
    error ClaimNotAvailable();
    error DepositNotAllowed();

    constructor(address _stakingContract, address _depositor) {
        stakingContract = ISyndStaking(_stakingContract);
        depositor = _depositor;
    }

    function deposit(uint256 epochIndex) external payable {
        if (msg.sender != depositor) {
            revert DepositNotAllowed();
        }

        uint256 amount = msg.value;
        epochTotal[epochIndex] += amount;

        emit EpochDeposit(epochIndex, amount);
    }

    function claim(uint256 epochIndex, address destination) external {
        if (epochTotal[epochIndex] == 0) {
            revert ClaimNotAvailable();
        }

        if (claimed[epochIndex][msg.sender]) {
            revert AlreadyClaimed();
        }
        claimed[epochIndex][msg.sender] = true;

        uint256 amount = stakingContract.getUserStake(epochIndex, msg.sender);
        if (amount == 0) {
            revert NotStaked();
        }
        uint256 total = stakingContract.getTotalStake(epochIndex);

        // Calculate the amount of synd to claim
        uint256 claimAmount = (epochTotal[epochIndex] * amount) / total;

        // Send synd to user
        payable(destination).transfer(claimAmount);

        emit ClaimSuccess(epochIndex, msg.sender, destination, claimAmount);
    }
}
