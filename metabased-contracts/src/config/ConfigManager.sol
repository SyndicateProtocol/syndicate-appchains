// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {BeaconProxy} from "@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";
import {ChainConfig} from "./ChainConfig.sol";

/**
 * @title ConfigManager
 * @dev Manages deployment of ChainConfig contracts using CREATE2 for deterministic addresses
 * Uses the Beacon Proxy pattern for upgradeable implementations
 */
contract ConfigManager is Ownable {
    // Events
    event ChainConfigCreated(uint256 indexed chainId, address configAddress);
    event ImplementationUpgraded(address newImplementation);

    // Beacon that holds the current implementation address
    UpgradeableBeacon public immutable beacon;

    // Mapping of chainId to deployed ChainConfig proxy address
    mapping(uint256 => address) public deployedConfigs;

    /**
     * @dev Constructor to deploy the implementation contract and beacon
     */
    constructor() Ownable(msg.sender) {
        // Deploy the implementation contract
        // No need to pass constructor arguments as they'll be handled by initialize()
        address implementation = address(new ChainConfig());

        // Deploy the beacon with the implementation and set the owner to this contract
        beacon = new UpgradeableBeacon(implementation, address(this));
    }

    /**
     * @dev Create a new ChainConfig contract for a specific chainId
     * @param chainId The chain ID for which to create a config
     * @return address The address of the deployed ChainConfig contract
     */
    function createChainConfig(uint256 chainId) external onlyOwner returns (address) {
        require(chainId != 0, "Chain ID cannot be zero");
        require(deployedConfigs[chainId] == address(0), "Config already exists for this chain ID");

        // Calculate the salt from the chainId
        bytes32 salt = keccak256(abi.encodePacked(chainId));

        // Deploy a beacon proxy using CREATE2
        // The BeaconProxy constructor expects only the beacon address and initialization data
        // The initialization data needs to be the encoded constructor arguments for the implementation
        BeaconProxy proxy = new BeaconProxy{salt: salt}(address(beacon), "");

        address proxyAddress = address(proxy);
        deployedConfigs[chainId] = proxyAddress;

        emit ChainConfigCreated(chainId, proxyAddress);

        return proxyAddress;
    }

    /**
     * @dev Get the deterministic address for a chain config given a chainId
     * @param chainId The chain ID
     * @return The deterministic address where the config would be deployed
     */
    function getChainConfigAddress(uint256 chainId) public view returns (address) {
        // Check if already deployed
        if (deployedConfigs[chainId] != address(0)) {
            return deployedConfigs[chainId];
        }

        // Require non-zero chainId
        require(chainId != 0, "Chain ID cannot be zero");

        // Calculate the salt from the chainId
        bytes32 salt = keccak256(abi.encodePacked(chainId));

        // Generate the bytecode for BeaconProxy with constructor args
        bytes memory constructorArgs = abi.encode(address(beacon));
        bytes memory bytecode = abi.encodePacked(type(BeaconProxy).creationCode);

        // Calculate CREATE2 address
        bytes32 hash = keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, keccak256(bytecode)));

        return address(uint160(uint256(hash)));
    }

    /**
     * @dev Upgrade the implementation for all proxies by updating the beacon
     * @param newImplementation The address of the new implementation
     * @notice This function can only be called by the beacon owner, which is transferred
     * to the deployer in the constructor
     */
    function upgradeImplementation(address newImplementation) external onlyOwner {
        require(newImplementation != address(0), "New implementation cannot be zero address");

        // Update the implementation in the beacon
        // This will automatically update all proxies
        beacon.upgradeTo(newImplementation);

        emit ImplementationUpgraded(newImplementation);
    }
}
