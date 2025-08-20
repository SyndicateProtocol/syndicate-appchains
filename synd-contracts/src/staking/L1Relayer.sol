// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IOPBridge {
    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes calldata _extraData
    ) external;
}

interface IOPMessageRelayer {
    function sendMessage(address _target, bytes memory _message, uint32 _minGasLimit) external;
}

interface IL2Relayer {
    function relay(uint256 amount, uint256 epochIndex) external;
}

contract L1Relayer {
    address public opBridge;
    address public opMessageRelayer;

    address public l1Token;
    address public l2Token;

    address public l2Relayer;

    constructor(address _opBridge, address _opMessageRelayer, address _l1Token, address _l2Token, address _l2Relayer) {
        opBridge = _opBridge;
        opMessageRelayer = _opMessageRelayer;
        l1Token = _l1Token;
        l2Token = _l2Token;
        l2Relayer = _l2Relayer;

        IERC20(l1Token).approve(opBridge, type(uint256).max);
    }

    function relay(uint256 amount, uint256 epochIndex) external {
        require(IERC20(l1Token).balanceOf(address(this)) >= amount, "L1Relayer: Insufficient balance");

        _deposit(amount);
        _relay(amount, epochIndex);
    }

    function _deposit(uint256 amount) internal {
        IOPBridge(opBridge).depositERC20To(l1Token, l2Token, l2Relayer, amount, 200000, bytes(""));
    }

    function _relay(uint256 amount, uint256 epochIndex) internal {
        IOPMessageRelayer(opMessageRelayer).sendMessage(
            l2Relayer, abi.encodeWithSelector(IL2Relayer.relay.selector, amount, epochIndex), 200000
        );
    }
}
