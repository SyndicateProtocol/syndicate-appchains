// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {TestnetSyndTokenCrosschain} from "../TestnetSyndTokenCrosschain.sol";
import {CREATE3} from "./libraries/CREATE3.sol";

/**
 * @title TestnetSyndTokenCrosschainFactory
 * @notice Factory for deploying TestnetSyndTokenCrosschain with deterministic addresses
 * @dev Uses Solmate CREATE3 to ensure the same contract address across all chains.
 *      This enables seamless cross-chain token bridging without address confusion.
 *
 * Key Features:
 * - Deterministic deployment across all chains using Solmate CREATE3
 * - Same token address on L1, L2, and L3 testnets
 * - Configurable bridge setup during deployment
 * - Emergency controls and ownership management
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
contract TestnetSyndTokenCrosschainFactory {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a new TestnetSyndTokenCrosschain is deployed
    /// @param token Address of the deployed token
    /// @param admin Address of the token admin
    /// @param minter Address of the token minter
    /// @param salt Salt used for deployment
    event TestnetSyndTokenCrosschainDeployed(
        address indexed token, address indexed admin, address indexed minter, bytes32 salt
    );

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when deployment fails
    error DeploymentFailed();

    /*//////////////////////////////////////////////////////////////
                            DEPLOYMENT FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Deploy TestnetSyndTokenCrosschain with deterministic address
     * @param admin Address that will have admin privileges
     * @param minter Address that will have minter privileges
     * @param salt Deployment salt for deterministic address
     * @return token Address of deployed token contract
     */
    function deployTestnetSyndTokenCrosschain(address admin, address minter, bytes32 salt)
        external
        returns (address token)
    {
        // Generate creation code
        bytes memory creationCode =
            abi.encodePacked(type(TestnetSyndTokenCrosschain).creationCode, abi.encode(admin, minter));

        // Deploy with Solmate CREATE3 (no value sent)
        token = CREATE3.deploy(salt, creationCode, 0);

        emit TestnetSyndTokenCrosschainDeployed(token, admin, minter, salt);
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Predict the address of a TestnetSyndTokenCrosschain deployment
     * @dev With CREATE3, address only depends on salt and deployer, not constructor args
     * @param salt Deployment salt
     * @return predicted Predicted contract address
     */
    function predictTokenAddress(bytes32 salt) external view returns (address predicted) {
        // With CREATE3, the address doesn't depend on constructor args
        // It only depends on the salt and deployer
        return CREATE3.getDeployed(salt, address(this));
    }

    /**
     * @notice Generate deterministic salt for consistent deployment
     * @param admin Admin address
     * @param minter Minter address
     * @param chainId Chain ID where token will be deployed
     * @return salt Generated salt
     */
    function generateSalt(address admin, address minter, uint256 chainId) external pure returns (bytes32 salt) {
        return keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, chainId));
    }

    /**
     * @notice Generate salt with additional entropy
     * @param admin Admin address
     * @param minter Minter address
     * @param chainId Chain ID where token will be deployed
     * @param additionalData Additional data for salt generation
     * @return salt Generated salt
     */
    function generateSaltWithData(address admin, address minter, uint256 chainId, bytes calldata additionalData)
        external
        pure
        returns (bytes32 salt)
    {
        return keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, chainId, additionalData));
    }

    /**
     * @notice Check if a token is already deployed at predicted address
     * @dev With CREATE3, address only depends on salt and deployer, not constructor args
     * @param salt Deployment salt
     * @return deployed True if contract is deployed
     */
    function isTokenDeployed(bytes32 salt) external view returns (bool deployed) {
        address predicted = CREATE3.getDeployed(salt, address(this));
        uint256 size;
        assembly {
            size := extcodesize(predicted)
        }
        return size > 0;
    }

    /*//////////////////////////////////////////////////////////////
                            UTILITY FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get deployment info for a token
     * @param tokenAddress Address of deployed token
     * @return bridgeCount Number of configured bridges
     */
    function getTokenInfo(address tokenAddress) external view returns (uint256 bridgeCount) {
        TestnetSyndTokenCrosschain token = TestnetSyndTokenCrosschain(tokenAddress);
        bridgeCount = token.getBridgeCount();
    }
}
