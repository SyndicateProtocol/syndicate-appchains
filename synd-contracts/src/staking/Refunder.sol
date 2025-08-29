// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPool} from "./IPool.sol";
import {ISyndStaking} from "./ISyndStaking.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title Refunder
 * @notice A utility contract that recovers SYND balance and deposits it into a pool for the current epoch
 * @dev This contract is designed to be a simple recovery mechanism for any SYND that is leftover
 *      or gets refunded from the bridge. It uses the current epoch from the syndicate staking contract
 *      for the deposit to the pool
 */
contract Refunder is AccessControl {
    /// @notice The address of the pool contract where recovered funds are deposited
    address public pool;

    /// @notice The address of the syndicate staking contract used to get current epoch
    address public immutable syndStaking;

    /**
     * @notice Constructs the Refunder contract
     * @param _pool The address of the pool contract for deposits
     * @param _syndStaking The address of the syndicate staking contract
     * @dev Both addresses are set as immutable to ensure contract security and gas efficiency
     */
    constructor(address _pool, address _syndStaking, address _defaultAdmin) {
        pool = _pool;
        syndStaking = _syndStaking;

        _grantRole(DEFAULT_ADMIN_ROLE, _defaultAdmin);
    }

    /**
     * @notice Sets the address of the pool contract for future recoveries
     * @param _pool The new address of the pool contract
     * @dev Only callable by an account with the DEFAULT_ADMIN_ROLE
     */
    function setRecoverPool(address _pool) external onlyRole(DEFAULT_ADMIN_ROLE) {
        pool = _pool;
    }

    /**
     * @notice Recovers the contract's SYND balance and deposits it into the pool
     * @dev This function:
     *      - Gets the current contract balance
     *      - Queries the current epoch from the syndicate staking contract
     *      - Deposits the entire balance into the pool for the current epoch
     * @dev This function can be called by anyone, making it a public recovery mechanism
     */
    function recover() external {
        uint256 amount = address(this).balance;
        uint256 currentEpoch = ISyndStaking(syndStaking).getCurrentEpoch();

        IPool(pool).deposit{value: amount}(currentEpoch);
    }
}
