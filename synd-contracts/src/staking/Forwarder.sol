// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IUserPool} from "./IUserPool.sol";
import {SyndStaking} from "./SyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title Forwarder
 * @notice Contract for batching staking operations to reduce gas costs and improve UX
 * @dev This contract allows users to:
 *      1. Claim all rewards from multiple appchains and pools in a single transaction
 *      2. Restake all claimed rewards to the same or different appchains in a single transaction
 *
 * The contract acts as a forwarder that calls internal functions on the staking
 * contracts with the original caller's address as a parameter.
 */
contract Forwarder is ReentrancyGuard {
    /// @notice Reference to the SyndStaking contract
    SyndStaking public immutable stakingContract;

    /**
     * @notice Struct for claiming rewards from multiple pools
     * @param epochIndex The epoch index to claim rewards from
     * @param poolAddress The address of the pool to claim from
     * @param destination The address where rewards should be sent
     */
    struct ClaimRequest {
        uint256 epochIndex;
        address poolAddress;
        address destination;
    }

    /**
     * @notice Struct for restaking rewards to multiple appchains
     * @param fromAppchainId The source appchain ID (0 for direct ETH restake)
     * @param toAppchainId The destination appchain ID
     * @param amount The amount to restake
     */
    struct RestakeRequest {
        uint256 fromAppchainId;
        uint256 toAppchainId;
        uint256 amount;
    }

    /**
     * @notice Emitted when rewards are claimed from multiple pools
     * @param user The address of the user who claimed rewards
     * @param totalClaimed The total amount of rewards claimed
     * @param claimCount The number of successful claims
     */
    event BatchClaimCompleted(address user, uint256 totalClaimed, uint256 claimCount);

    /**
     * @notice Emitted when rewards are restaked to multiple appchains
     * @param user The address of the user who restaked rewards
     * @param totalRestaked The total amount of rewards restaked
     * @param restakeCount The number of successful restakes
     */
    event BatchRestakeCompleted(address user, uint256 totalRestaked, uint256 restakeCount);

    /// @notice Error thrown when no claims are provided
    error NoClaimsProvided();
    /// @notice Error thrown when no restakes are provided
    error NoRestakesProvided();
    /// @notice Error thrown when insufficient ETH is provided for restaking
    error InsufficientETH();

    /**
     * @notice Constructor to initialize the forwarder
     * @param _stakingContract Address of the SyndStaking contract
     */
    constructor(address _stakingContract) {
        stakingContract = SyndStaking(_stakingContract);
    }

    /**
     * @notice Claim rewards from multiple pools for the caller
     * @dev This function calls the claimFor function on each pool contract
     * @param claims Array of ClaimRequest structs containing claim details
     * @return totalClaimed The total amount of rewards claimed
     * @return claimCount The number of successful claims
     */
    function claimAllRewards(ClaimRequest[] calldata claims)
        external
        nonReentrant
        returns (uint256 totalClaimed, uint256 claimCount)
    {
        if (claims.length == 0) {
            revert NoClaimsProvided();
        }

        for (uint256 i = 0; i < claims.length; i++) {
            ClaimRequest memory claim = claims[i];

            // Call the claimFor function on the pool contract
            IUserPool pool = IUserPool(claim.poolAddress);
            uint256 claimableAmount = pool.getClaimableAmount(claim.epochIndex, claim.destination);

            if (claimableAmount > 0) {
                pool.claimFor(claim.epochIndex, claim.destination);
                totalClaimed += claimableAmount;
                claimCount++;
            }
        }

        emit BatchClaimCompleted(msg.sender, totalClaimed, claimCount);
    }

    /**
     * @notice Restake rewards to multiple appchains for the caller
     * @dev This function calls the stakeSyndFor function on the staking contract
     * @param restakes Array of RestakeRequest structs containing restake details
     * @return totalRestaked The total amount of rewards restaked
     * @return restakeCount The number of successful restakes
     */
    function restakeAllRewards(RestakeRequest[] calldata restakes)
        external
        payable
        nonReentrant
        returns (uint256 totalRestaked, uint256 restakeCount)
    {
        if (restakes.length == 0) {
            revert NoRestakesProvided();
        }

        for (uint256 i = 0; i < restakes.length; i++) {
            RestakeRequest memory restake = restakes[i];

            if (restake.amount > 0) {
                // Call the stakeSyndFor function on the staking contract
                stakingContract.stakeSyndFor{value: restake.amount}(msg.sender, restake.toAppchainId);
                totalRestaked += restake.amount;
                restakeCount++;
            }
        }

        emit BatchRestakeCompleted(msg.sender, totalRestaked, restakeCount);
    }

    /**
     * @notice Claim and restake rewards in a single transaction
     * @dev This function combines claimAllRewards and restakeAllRewards
     * @param claims Array of ClaimRequest structs containing claim details
     * @param restakes Array of RestakeRequest structs containing restake details
     * @return totalClaimed The total amount of rewards claimed
     * @return totalRestaked The total amount of rewards restaked
     */
    function claimAndRestake(ClaimRequest[] calldata claims, RestakeRequest[] calldata restakes)
        external
        payable
        nonReentrant
        returns (uint256 totalClaimed, uint256 totalRestaked)
    {
        // First claim all rewards
        (totalClaimed,) = this.claimAllRewards(claims);

        // Then restake all rewards
        (totalRestaked,) = this.restakeAllRewards{value: msg.value}(restakes);
    }
}
