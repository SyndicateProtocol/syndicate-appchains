// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "../interfaces/PermissionModule.sol";

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);
}

/**
 * @title TokenBalanceSequencingModule
 * @dev This contract allows sequencing based on the caller's token balance.
 * @dev Useful in case Syndicate releases a token and wants to allow only token holders to sequence.
 */
// [Olympix Warning: unfuzzed variables, missing events assertion] These test-related warnings are not security critical
// as the contract uses standard unit tests and integration tests. Parameter validation is handled in constructor.
contract TokenBalanceSequencingModule is PermissionModule {
    /// @notice The address of the ERC20 token contract.
    address public immutable tokenAddress;
    /// @notice The minimum token balance required to be allowed.
    uint256 public immutable minimumBalance;

    /**
     * @dev Sets the token address and minimum balance required.
     * @param _tokenAddress The address of the ERC20 token contract.
     * @param _minimumBalance The minimum token balance required to be allowed.
     */
    constructor(address _tokenAddress, uint256 _minimumBalance) {
        require(_tokenAddress != address(0), "TokenBalanceSequencingModule: zero address");
        require(_minimumBalance > 0, "TokenBalanceSequencingModule: zero balance");

        tokenAddress = _tokenAddress;
        minimumBalance = _minimumBalance;
    }

    /**
     * @notice Checks if the caller is allowed based on their token balance.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address proposer) external view override returns (bool) {
        IERC20 token = IERC20(tokenAddress);
        return token.balanceOf(proposer) >= minimumBalance;
    }
}
