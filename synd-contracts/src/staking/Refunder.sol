// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPool} from "./interfaces/IPool.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {EpochTracker} from "./EpochTracker.sol";

/**
 * @title Refunder
 * @notice A utility contract that recovers SYND balance and deposits it into a pool for the current epoch
 * @dev This contract is designed to be a simple recovery mechanism for any SYND that is leftover
 *      or gets refunded from the bridge. It automatically deposits recovered funds to the current epoch.
 * @dev Inherits from AccessControl for admin functionality
 */
contract Refunder is Ownable, EpochTracker {
    /// @notice The address of the pool contract where recovered funds are deposited
    /// @dev Admin can change this address to redirect recovered funds to different pools
    address public pool;

    /**
     * @notice Constructs the Refunder contract
     * @param _pool The address of the pool contract for deposits
     */
    constructor(address _pool) Ownable(msg.sender) {
        require(_pool != address(0), "pool cannot be zero");
        pool = _pool;
    }

    /**
     * @notice Sets the address of the pool contract for future recoveries
     * @param _pool The new address of the pool contract
     * @dev Only callable by the owner
     */
    function setRecoverPool(address _pool) external onlyOwner {
        require(_pool != address(0), "pool cannot be zero");
        pool = _pool;
    }

    /**
     * @notice Recovers the contract's SYND balance and deposits it into the pool
     * @dev This function:
     *      - Gets the current contract balance
     *      - Queries the current epoch from the syndicate staking contract
     *      - Deposits the entire balance into the pool for the current epoch
     * @dev This function can be called by anyone, making it a public recovery mechanism.
     *      This allows for decentralized recovery of funds without requiring admin intervention.
     * @custom:example If contract has 1000 SYND and current epoch is 5, deposits 1000 SYND to epoch 5
     */
    function recover() external {
        uint256 amount = address(this).balance;

        IPool(pool).deposit{value: amount}(getCurrentEpoch());
    }
}
