// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title MockSyndicateToken
 * @notice Mock contract for SyndicateToken to enable Certora formal verification
 * @dev This mock implements the minimal interface needed for EmissionsCalculator verification
 */
contract MockSyndicateToken {
    /// @notice Total supply of tokens currently minted
    uint256 public totalSupply;
    
    /// @notice Maximum total supply (100M tokens)
    uint256 public constant TOTAL_SUPPLY = 100_000_000 * 10**18;
    
    /// @notice Initial supply (20M tokens) = TOTAL_SUPPLY - EMISSIONS_CAP
    uint256 public constant INITIAL_SUPPLY = 20_000_000 * 10**18;
    
    /// @notice Mapping of token balances
    mapping(address => uint256) public balanceOf;
    
    /// @notice Track total minted for verification
    uint256 public totalMinted;

    constructor() {
        // Start with initial supply minted to deployer
        totalSupply = INITIAL_SUPPLY;
        balanceOf[msg.sender] = INITIAL_SUPPLY;
        totalMinted = INITIAL_SUPPLY;
    }

    /**
     * @notice Mint tokens to specified address
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     * @dev This is called by EmissionsCalculator during formal verification
     */
    function mint(address to, uint256 amount) external {
        require(to != address(0), "Cannot mint to zero address");
        require(totalSupply + amount <= TOTAL_SUPPLY, "Would exceed total supply");
        
        totalSupply += amount;
        balanceOf[to] += amount;
        totalMinted += amount;
    }

    /**
     * @notice Get the total emissions that have been minted
     * @return Amount of emissions minted (total minted - initial supply)
     */
    function getEmissionsMinted() external view returns (uint256) {
        return totalMinted > INITIAL_SUPPLY ? totalMinted - INITIAL_SUPPLY : 0;
    }
}