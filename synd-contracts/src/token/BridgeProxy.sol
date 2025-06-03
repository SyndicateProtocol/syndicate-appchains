// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title BridgeProxy
 * @notice Generic bridge proxy that can call any bridge with arbitrary calldata
 * @dev This is a completely generic bridge that works with any protocol by using raw external calls
 */
contract BridgeProxy is AccessControl {
    // Custom errors
    error ZeroAddress();
    error ZeroAmount();
    error BridgeCallFailed();

    // Role definitions
    bytes32 public constant BRIDGE_ADMIN_ROLE = keccak256("BRIDGE_ADMIN_ROLE");

    // Bridge configuration
    address public bridgeTarget; // The contract to call
    bytes public bridgeCalldata; // The calldata template to execute

    // Events
    event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
    event BridgeCalldataUpdated(bytes oldCalldata, bytes newCalldata);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target, bool success);

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_ADMIN_ROLE, admin);
    }

    /**
     * @notice Execute bridge operation
     * @param token The token contract address
     * @param amount The amount to bridge
     * @param data Additional bridge data
     */
    function executeBridge(address token, uint256 amount, bytes calldata data) external {
        if (token == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (bridgeTarget == address(0)) revert ZeroAddress();

        // Transfer tokens from caller to this contract
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // Approve the bridge target to spend the tokens
        IERC20(token).approve(bridgeTarget, amount);

        // Call the bridge target with the configured function
        bytes memory callData = abi.encodeWithSelector(bytes4(bridgeCalldata), token, amount, data);

        (bool success,) = bridgeTarget.call(callData);

        emit BridgeExecuted(token, amount, bridgeTarget, success);

        if (!success) {
            revert BridgeCallFailed();
        }
    }

    /**
     * @notice Set the bridge target contract
     * @param target The contract address to call for bridging
     */
    function setBridgeTarget(address target) external onlyRole(BRIDGE_ADMIN_ROLE) {
        if (target == address(0)) revert ZeroAddress();

        address oldTarget = bridgeTarget;
        bridgeTarget = target;

        emit BridgeTargetUpdated(oldTarget, target);
    }

    /**
     * @notice Set the bridge calldata template
     * @param calldata_ The calldata template to use for bridge calls
     */
    function setBridgeCalldata(bytes calldata calldata_) external onlyRole(BRIDGE_ADMIN_ROLE) {
        bytes memory oldCalldata = bridgeCalldata;
        bridgeCalldata = calldata_;

        emit BridgeCalldataUpdated(oldCalldata, calldata_);
    }

    /**
     * @notice Get current bridge configuration
     * @return target The bridge target contract
     * @return calldata_ The bridge calldata template
     */
    function getBridgeConfiguration() external view returns (address target, bytes memory calldata_) {
        return (bridgeTarget, bridgeCalldata);
    }

    /**
     * @notice Emergency function to recover stuck tokens
     * @param token The token to recover
     * @param amount The amount to recover
     * @param to The address to send recovered tokens to
     */
    function recoverTokens(address token, uint256 amount, address to) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        IERC20(token).transfer(to, amount);
    }
}
