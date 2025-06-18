// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {BaseBridgeProxy} from "./BaseBridgeProxy.sol";

/**
 * @title IArbitrumBridge
 * @notice Interface for the Arbitrum L1GatewayRouter contract
 * @dev This interface defines the function used to transfer ERC20 tokens to Arbitrum L2
 */
interface IArbitrumBridge {
    /**
     * @notice Transfer tokens to L2 with custom refund address
     * @param _token Address of the L1 token being transferred
     * @param _refundTo Address to refund excess gas fees on L1
     * @param _to Address of the recipient on L2
     * @param _amount Amount of tokens to transfer
     * @param _maxGas Maximum gas to use for the L2 transaction
     * @param _gasPriceBid Gas price bid for the L2 transaction
     * @param _data Optional data to pass to the L2 contract
     * @return Unique identifier for the transfer
     */
    function outboundTransferCustomRefund(
        address _token,
        address _refundTo,
        address _to,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes calldata _data
    ) external payable returns (bytes memory);
}

/**
 * @title ArbitrumBridgeProxy
 * @notice Bridge proxy implementation for Arbitrum One and Arbitrum Nova
 * @dev This contract implements the bridge logic for sending tokens from Ethereum L1
 *      to Arbitrum L2 using the official Arbitrum bridge infrastructure.
 *
 * Key Features:
 * - Integrates with Arbitrum's L1GatewayRouter contract
 * - Supports configurable gas parameters for L2 transactions
 * - Handles ETH payments for L2 gas fees
 * - Allows dynamic recipient and gas configuration
 * - Inherits all security features from BaseBridgeProxy
 *
 * Usage:
 * The SyndicateToken contract calls executeBridge() with optional dynamicData
 * containing (recipient, maxGas, gasPriceBid) parameters. If no dynamicData is provided,
 * default values are used. ETH must be sent with the transaction to pay for L2 gas.
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
contract ArbitrumBridgeProxy is BaseBridgeProxy {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Default recipient address on L2 (usually the emissions distributor)
    address public recipient;

    /// @notice Default maximum gas for L2 transactions
    uint256 public maxGas;

    /// @notice Default gas price bid for L2 transactions
    uint256 public gasPriceBid;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when Arbitrum-specific configuration is updated
     * @param recipient The new default recipient address
     * @param maxGas The new default maximum gas
     * @param gasPriceBid The new default gas price bid
     */
    event ArbitrumConfigUpdated(address recipient, uint256 maxGas, uint256 gasPriceBid);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the Arbitrum bridge proxy
     * @param admin Address that will have admin privileges
     * @param caller Address that can execute bridge operations (SyndicateToken)
     * @param _bridgeTarget Address of the Arbitrum L1GatewayRouter contract
     * @param _maxSingleTransfer Maximum amount per single transaction
     * @param _dailyLimit Maximum amount per 24-hour period
     * @param _recipient Default recipient address on L2
     * @param _maxGas Default maximum gas for L2 transactions
     * @param _gasPriceBid Default gas price bid for L2 transactions
     */
    constructor(
        address admin, //#olympix-ignore-no-parameter-validation-in-constructor
        address caller, //#olympix-ignore-no-parameter-validation-in-constructor
        address _bridgeTarget, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _maxSingleTransfer, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _dailyLimit, //#olympix-ignore-no-parameter-validation-in-constructor
        address _recipient, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _maxGas, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _gasPriceBid //#olympix-ignore-no-parameter-validation-in-constructor
    ) BaseBridgeProxy(admin, caller, "Arbitrum Bridge", _bridgeTarget, _maxSingleTransfer, _dailyLimit) {
        recipient = _recipient;
        maxGas = _maxGas;
        gasPriceBid = _gasPriceBid; //#olympix-ignore-events-price-change
    }

    /*//////////////////////////////////////////////////////////////
                         BRIDGE IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Execute the Arbitrum bridge call
     * @dev This function implements the bridge-specific logic for Arbitrum.
     *      It approves the bridge contract to spend tokens and calls outboundTransferCustomRefund.
     *      ETH value is calculated as maxGas * gasPriceBid to pay for L2 execution.
     *
     * Dynamic Data Format:
     * - If provided: abi.encode(address recipient, uint256 maxGas, uint256 gasPriceBid)
     * - If empty: uses default recipient, maxGas, and gasPriceBid values
     *
     * @param token The L1 token address to bridge
     * @param amount The amount of tokens to bridge
     * @param dynamicData Optional ABI-encoded (recipient, maxGas, gasPriceBid) parameters
     */
    function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal override {
        // Decode dynamic parameters or use defaults
        (address _recipient, uint256 _maxGas, uint256 _gasPriceBid) = dynamicData.length > 0
            ? abi.decode(dynamicData, (address, uint256, uint256))
            : (recipient, maxGas, gasPriceBid);

        // Calculate ETH value needed for L2 gas
        uint256 ethValue = _maxGas * _gasPriceBid;

        // Approve the Arbitrum bridge to spend tokens
        IERC20(token).approve(bridgeTarget, amount);

        // Call the Arbitrum bridge - if it fails, the entire transaction reverts automatically
        IArbitrumBridge(bridgeTarget).outboundTransferCustomRefund{value: ethValue}(
            token, // L1 token address
            address(this), // Refund address (this contract)
            _recipient, // Recipient on L2
            amount, // Amount to bridge
            _maxGas, // Maximum gas for L2 transaction
            _gasPriceBid, // Gas price bid
            "" // No additional data
        );
    }

    /*//////////////////////////////////////////////////////////////
                        ADMIN CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Update Arbitrum-specific configuration
     * @dev Only callable by bridge admin. Allows updating default recipient
     *      and gas parameters for L2 transactions.
     *
     * @param _recipient Default recipient address on L2
     * @param _maxGas Default maximum gas for L2 transactions
     * @param _gasPriceBid Default gas price bid for L2 transactions
     */
    function setArbitrumConfig(address _recipient, uint256 _maxGas, uint256 _gasPriceBid)
        external
        onlyRole(BRIDGE_ADMIN_ROLE)
    {
        recipient = _recipient;
        maxGas = _maxGas;
        gasPriceBid = _gasPriceBid;
        emit ArbitrumConfigUpdated(_recipient, _maxGas, _gasPriceBid);
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get current Arbitrum configuration
     * @return recipientAddr Default recipient address
     * @return maxGasLimit Default maximum gas
     * @return gasPriceBidAmount Default gas price bid
     */
    function getArbitrumConfig()
        external
        view
        returns (address recipientAddr, uint256 maxGasLimit, uint256 gasPriceBidAmount)
    {
        return (recipient, maxGas, gasPriceBid);
    }

    /**
     * @notice Calculate the ETH value needed for a bridge operation
     * @param _maxGas Maximum gas for the L2 transaction
     * @param _gasPriceBid Gas price bid for the L2 transaction
     * @return ethValue The amount of ETH needed
     */
    function calculateEthValue(uint256 _maxGas, uint256 _gasPriceBid) external pure returns (uint256 ethValue) {
        return _maxGas * _gasPriceBid;
    }

    /*//////////////////////////////////////////////////////////////
                          RECEIVE FUNCTION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Allow contract to receive ETH for gas payments and refunds
     * @dev The contract needs to accept ETH for:
     *      1. Paying L2 gas fees during bridge operations
     *      2. Receiving refunds from Arbitrum bridge if gas is overestimated
     */
    //#olympix-ignore
    receive() external payable {
        // ETH can be received for gas payments and refunds
    }
}
