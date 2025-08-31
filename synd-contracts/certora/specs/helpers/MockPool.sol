// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import "../../../src/staking/IPool.sol";

/**
 * @title MockPool
 * @notice Mock contract for IPool interface to enable Certora formal verification
 * @dev This mock implements the full IPool interface needed for SyndStaking verification
 */
contract MockPool is IPool {
    /// @notice Track deposits per epoch
    mapping(uint256 => uint256) public deposits;
    
    /// @notice Track claimable amounts per user per epoch
    mapping(uint256 => mapping(address => uint256)) public claimableAmounts;
    
    /// @notice Track if user has claimed for epoch
    mapping(uint256 => mapping(address => bool)) public hasClaimed;

    /**
     * @notice Mock implementation of deposit
     * @param epochIndex The epoch to deposit to
     */
    function deposit(uint256 epochIndex) external payable override {
        require(msg.value > 0, "Must deposit something");
        deposits[epochIndex] += msg.value;
        
        // Mock: give claimable amount to sender for testing
        claimableAmounts[epochIndex][msg.sender] += msg.value;
    }

    /**
     * @notice Mock implementation of getClaimableAmount
     * @param epochIndex The epoch to check
     * @param user The user to check for
     * @return The claimable amount
     */
    function getClaimableAmount(uint256 epochIndex, address user) 
        external 
        view 
        override 
        returns (uint256) 
    {
        if (hasClaimed[epochIndex][user]) {
            return 0;
        }
        return claimableAmounts[epochIndex][user];
    }

    /**
     * @notice Mock implementation of claimFor
     * @param epochIndex The epoch to claim from
     * @param user The user claiming rewards
     * @param destination Where to send the rewards
     */
    function claimFor(uint256 epochIndex, address user, address destination) 
        external 
        override
    {
        require(user != address(0), "Invalid user");
        require(destination != address(0), "Invalid destination");
        require(!hasClaimed[epochIndex][user], "Already claimed");
        
        uint256 amount = claimableAmounts[epochIndex][user];
        require(amount > 0, "Nothing to claim");
        
        hasClaimed[epochIndex][user] = true;
        
        // Transfer ETH to destination (simplified for testing)
        payable(destination).transfer(amount);
    }
    
    // Allow contract to receive ETH
    receive() external payable {}
}