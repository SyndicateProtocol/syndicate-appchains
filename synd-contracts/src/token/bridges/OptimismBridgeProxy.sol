// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {BaseBridgeProxy} from "./BaseBridgeProxy.sol";

/**
 * @title IOptimismBridge
 * @notice Interface for the Optimism L1StandardBridge contract
 * @dev This interface defines the function used to deposit ERC20 tokens to Optimism L2
 */
interface IOptimismBridge {
    /**
     * @notice Deposit an ERC20 token to a recipient on L2
     * @param _l1Token Address of the L1 token being deposited
     * @param _l2Token Address of the corresponding L2 token
     * @param _to Address of the recipient on L2
     * @param _amount Amount of tokens to deposit
     * @param _l2Gas Gas limit for the L2 transaction
     * @param _data Optional data to pass to the L2 contract
     */
    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _l2Gas,
        bytes calldata _data
    ) external;
}

/**
 * @title OptimismBridgeProxy
 * @notice Bridge proxy implementation for Optimism (OP Mainnet) and OP Stack chains
 * @dev This contract implements the bridge logic for sending tokens from Ethereum L1
 *      to Optimism L2 using the official Optimism bridge infrastructure.
 *
 * Key Features:
 * - Integrates with Optimism's L1StandardBridge contract
 * - Supports configurable L2 token addresses and recipients
 * - Allows dynamic gas limit configuration for L2 transactions
 * - Inherits all security features from BaseBridgeProxy
 *
 * Usage:
 * The SyndicateToken contract calls executeBridge() with optional dynamicData
 * containing (recipient, gasLimit) parameters. If no dynamicData is provided,
 * default values are used.
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
contract OptimismBridgeProxy is BaseBridgeProxy {
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Address of the corresponding token on Optimism L2
    address public l2Token;

    /// @notice Default recipient address on L2 (usually the emissions distributor)
    address public recipient;

    /// @notice Default gas limit for L2 transactions
    uint32 public l2Gas;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when Optimism-specific configuration is updated
     * @param l2Token The new L2 token address
     * @param recipient The new default recipient address
     * @param l2Gas The new default L2 gas limit
     */
    event OptimismConfigUpdated(address l2Token, address recipient, uint32 l2Gas);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the Optimism bridge proxy
     * @param admin Address that will have admin privileges
     * @param caller Address that can execute bridge operations (SyndicateToken)
     * @param _bridgeTarget Address of the Optimism L1StandardBridge contract
     * @param _maxSingleTransfer Maximum amount per single transaction
     * @param _dailyLimit Maximum amount per 24-hour period
     * @param _l2Token Address of the token on Optimism L2
     * @param _recipient Default recipient address on L2
     * @param _l2Gas Default gas limit for L2 transactions
     */
    constructor(
        address admin, //#olympix-ignore-no-parameter-validation-in-constructor
        address caller, //#olympix-ignore-no-parameter-validation-in-constructor
        address _bridgeTarget, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _maxSingleTransfer, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _dailyLimit, //#olympix-ignore-no-parameter-validation-in-constructor
        address _l2Token, //#olympix-ignore-no-parameter-validation-in-constructor
        address _recipient, //#olympix-ignore-no-parameter-validation-in-constructor
        uint32 _l2Gas //#olympix-ignore-no-parameter-validation-in-constructor
    ) BaseBridgeProxy(admin, caller, "Optimism Bridge", _bridgeTarget, _maxSingleTransfer, _dailyLimit) {
        l2Token = _l2Token;
        recipient = _recipient;
        l2Gas = _l2Gas;
    }

    /*//////////////////////////////////////////////////////////////
                         BRIDGE IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Execute the Optimism bridge call
     * @dev This function implements the bridge-specific logic for Optimism.
     *      It approves the bridge contract to spend tokens and calls depositERC20To.
     *
     * Dynamic Data Format:
     * - If provided: abi.encode(address recipient, uint32 gasLimit)
     * - If empty: uses default recipient and l2Gas values
     *
     * @param token The L1 token address to bridge
     * @param amount The amount of tokens to bridge
     * @param dynamicData Optional ABI-encoded (recipient, gasLimit) parameters
     */
    function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal override {
        // Decode dynamic parameters or use defaults
        (address _recipient, uint32 _gas) =
            dynamicData.length > 0 ? abi.decode(dynamicData, (address, uint32)) : (recipient, l2Gas);

        // Approve the Optimism bridge to spend tokens
        IERC20(token).forceApprove(bridgeTarget, amount);

        // Call the Optimism bridge - if it fails, the entire transaction reverts automatically
        IOptimismBridge(bridgeTarget).depositERC20To(
            token, // L1 token address
            l2Token, // L2 token address
            _recipient, // Recipient on L2
            amount, // Amount to bridge
            _gas, // Gas limit for L2 transaction
            "" // No additional data
        );
    }

    /*//////////////////////////////////////////////////////////////
                        ADMIN CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Update Optimism-specific configuration
     * @dev Only callable by bridge admin. Allows updating L2 token address,
     *      default recipient, and default gas limit.
     *
     * @param _l2Token Address of the token on Optimism L2
     * @param _recipient Default recipient address on L2
     * @param _l2Gas Default gas limit for L2 transactions
     */
    function setOptimismConfig(address _l2Token, address _recipient, uint32 _l2Gas)
        external
        onlyRole(BRIDGE_ADMIN_ROLE)
    {
        l2Token = _l2Token;
        recipient = _recipient;
        l2Gas = _l2Gas;
        emit OptimismConfigUpdated(_l2Token, _recipient, _l2Gas);
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get current Optimism configuration
     * @return l2TokenAddr Address of the L2 token
     * @return recipientAddr Default recipient address
     * @return gasLimit Default L2 gas limit
     */
    function getOptimismConfig() external view returns (address l2TokenAddr, address recipientAddr, uint32 gasLimit) {
        return (l2Token, recipient, l2Gas);
    }
}
