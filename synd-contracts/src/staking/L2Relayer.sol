// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IArbBridge {
    function depositERC20(uint256 amount) external returns (uint256);

    function sendContractTransaction(
        uint256 gasLimit,
        uint256 maxFeePerGas,
        address to,
        uint256 value,
        bytes calldata data
    ) external returns (uint256);
}

interface IPool {
    function deposit(uint256 epochIndex) external;
}

contract L2Relayer {
    address public arbBridge;
    address public tokenAddress;
    address public poolDestination;

    constructor(address _arbBridge, address _tokenAddress, address _poolDestination) {
        arbBridge = _arbBridge;
        tokenAddress = _tokenAddress;
        poolDestination = _poolDestination;

        IERC20(tokenAddress).approve(arbBridge, type(uint256).max);
    }

    function relay(uint256 amount, uint256 epochIndex) external {
        require(IERC20(tokenAddress).balanceOf(address(this)) >= amount, "L2Relayer: Insufficient balance");

        _deposit(amount);
        _relay(amount, epochIndex);
    }

    function _deposit(uint256 amount) internal {
        IArbBridge(arbBridge).depositERC20(amount);
    }

    function _relay(uint256 amount, uint256 epochIndex) internal {
        uint256 gasLimit = 210000; // 21k gas for transfer x 10
        uint256 maxFeePerGas = 1000000000; // 1 gwei
        IArbBridge(arbBridge).sendContractTransaction(
            gasLimit, maxFeePerGas, poolDestination, amount, abi.encodeWithSelector(IPool.deposit.selector, epochIndex)
        );
    }
}
