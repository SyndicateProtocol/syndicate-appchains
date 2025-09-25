// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";

// Note: to deploy the deterministic deployment proxy, see: Makefile
// command:  make preview-syndicate-factory:  preview the deployment addresses
// command:  make deploy-syndicate-factory:   deploy the SyndicateFactory using deterministic deployment proxy

/**
 * @title IDeterministicDeploymentProxy
 * @notice Interface for Arachnid's deterministic deployment proxy
 * @dev The proxy deploys contracts using CREATE2 with a provided salt
 */
interface IDeterministicDeploymentProxy {
    /**
     * @dev Deploy a contract using CREATE2
     * @param salt The salt for deterministic deployment
     * @param bytecode The contract bytecode to deploy
     * @return The address of the deployed contract
     */
    function deploy(bytes32 salt, bytes calldata bytecode) external returns (address);
}

// Hardcoded initial owner to prevent squatting (replace with actual owner address)
address constant INITIAL_OWNER = address(0); // TODO: Replace with actual owner

// Arachnid's deterministic deployment proxy address (same on all chains)
address constant DETERMINISTIC_DEPLOYMENT_PROXY = 0x4e59b44847b379578588920cA78FbF26c0B4956C;

// Fixed salt for deterministic deployment
bytes32 constant DEPLOYMENT_SALT = keccak256("SYNDICATE_FACTORY_DETERMINISTIC_v1");

/**
 * @dev Compute the deterministic address for a contract
 * @param bytecodeHash The hash of the contract bytecode
 * @param salt The salt for deterministic deployment
 * @return The computed address
 */
function _computeDeterministicAddress(bytes32 bytecodeHash, bytes32 salt) pure returns (address) {
    return Create2.computeAddress(salt, bytecodeHash, DETERMINISTIC_DEPLOYMENT_PROXY);
}

/**
 * @title DeploySyndicateFactoryDeterministic
 * @notice Forge script to deploy SyndicateFactory using Arachnid's deterministic deployment proxy
 * @dev Ensures same contract address across all chains by using deterministic deployment
 */
contract DeploySyndicateFactoryDeterministic is Script {
    function run() external {
        vm.startBroadcast();

        if (INITIAL_OWNER == address(0)) {
            console2.log("ERROR: INITIAL_OWNER is not set. Please update the script with a valid owner address.");
            revert("INITIAL_OWNER not set");
        }

        // Check if deterministic deployment proxy exists
        if (DETERMINISTIC_DEPLOYMENT_PROXY.code.length == 0) {
            console2.log("WARNING: Deterministic deployment proxy not found at:", DETERMINISTIC_DEPLOYMENT_PROXY);
            console2.log("Please deploy it first using: https://github.com/Arachnid/deterministic-deployment-proxy");
            revert("Deterministic deployment proxy not available");
        }

        console2.log("Using deterministic deployment proxy at:", DETERMINISTIC_DEPLOYMENT_PROXY);
        console2.log("Initial owner hardcoded as:", INITIAL_OWNER);

        // Step 1: Deploy implementation contract deterministically
        bytes memory implementationBytecode = type(SyndicateFactory).creationCode;
        bytes32 implementationSalt = keccak256(abi.encodePacked(DEPLOYMENT_SALT, "implementation"));

        address expectedImplementationAddress =
            _computeDeterministicAddress(keccak256(implementationBytecode), implementationSalt);

        console2.log("Expected implementation address:", expectedImplementationAddress);

        address implementationAddress = _deployDeterministic(implementationBytecode, implementationSalt);

        console2.log("Implementation deployed to:", implementationAddress);
        require(implementationAddress == expectedImplementationAddress, "Implementation address mismatch");

        // Step 2: Deploy proxy contract deterministically
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (INITIAL_OWNER));
        bytes memory proxyBytecode =
            abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(implementationAddress, initData));
        bytes32 proxySalt = keccak256(abi.encodePacked(DEPLOYMENT_SALT, "proxy"));

        address expectedProxyAddress = _computeDeterministicAddress(keccak256(proxyBytecode), proxySalt);

        console2.log("Expected proxy address:", expectedProxyAddress);

        address proxyAddress = _deployDeterministic(proxyBytecode, proxySalt);

        console2.log("SyndicateFactory proxy deployed to:", proxyAddress);
        require(proxyAddress == expectedProxyAddress, "Proxy address mismatch");

        // Verify the deployment
        SyndicateFactory factory = SyndicateFactory(proxyAddress);
        bool isOwner = factory.hasRole(factory.DEFAULT_ADMIN_ROLE(), INITIAL_OWNER);
        require(isOwner, "Owner verification failed");

        console2.log("Deterministic deployment successful!");
        console2.log("SyndicateFactory address (consistent across all chains):", proxyAddress);

        vm.stopBroadcast();
    }

    /**
     * @dev Deploy a contract using the deterministic deployment proxy
     * @param bytecode The contract bytecode to deploy
     * @param salt The salt for deterministic deployment
     * @return deployedAddress The address of the deployed contract
     */
    function _deployDeterministic(bytes memory bytecode, bytes32 salt) internal returns (address deployedAddress) {
        // Check if contract already exists
        address expectedAddress = _computeDeterministicAddress(keccak256(bytecode), salt);
        if (expectedAddress.code.length > 0) {
            console2.log("Contract already deployed at:", expectedAddress);
            return expectedAddress;
        }

        console2.log("Deploying contract to expected address:", expectedAddress);
        console2.log("Bytecode length:", bytecode.length);
        console2.log("Salt:", vm.toString(salt));

        if (DETERMINISTIC_DEPLOYMENT_PROXY.code.length > 0) {
            // Arachnid's proxy expects: salt (32 bytes) + bytecode
            bytes memory deploymentData = abi.encodePacked(salt, bytecode);

            console2.log("Attempting deployment via proxy...");
            (bool success, bytes memory returnData) = DETERMINISTIC_DEPLOYMENT_PROXY.call(deploymentData);

            if (success) {
                deployedAddress = expectedAddress;
                if (deployedAddress.code.length > 0) {
                    return deployedAddress;
                } else {
                    console2.log("Proxy call succeeded but no code deployed");
                }
            } else {
                console2.log("Proxy deployment failed");
                if (returnData.length > 0) {
                    console2.logBytes(returnData);
                }
            }
        }
    }
}

contract PreviewSyndicateFactoryAddresses is Script {
    /**
     * @dev Preview the addresses that will be deployed
     * @notice Call this function to preview addresses before actual deployment
     */
    function run() external pure {
        require(INITIAL_OWNER != address(0), "INITIAL_OWNER not set");

        console2.log("=== Address Preview ===");

        // Implementation address
        bytes memory implementationBytecode = type(SyndicateFactory).creationCode;
        bytes32 implementationSalt = keccak256(abi.encodePacked(DEPLOYMENT_SALT, "implementation"));
        address implementationAddress =
            _computeDeterministicAddress(keccak256(implementationBytecode), implementationSalt);
        console2.log("Implementation will deploy to:", implementationAddress);

        // Proxy address
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (INITIAL_OWNER));
        bytes memory proxyBytecode =
            abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(implementationAddress, initData));
        bytes32 proxySalt = keccak256(abi.encodePacked(DEPLOYMENT_SALT, "proxy"));
        address proxyAddress = _computeDeterministicAddress(keccak256(proxyBytecode), proxySalt);
        console2.log("Proxy will deploy to:", proxyAddress);

        console2.log("=====================");
    }
}
