// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";

/**
 * @title UpgradeSyndicateFactory
 * @notice Upgrades the SyndicateFactory to a new implementation
 *
 * @dev Environment Variables Required:
 *      - FACTORY_ADDRESS: Address of SyndicateFactory proxy
 *      - ADMIN_ADDRESS: Address with DEFAULT_ADMIN_ROLE
 *
 * @dev Usage:
 *      make upgrade-factory
 *      or
 *      forge script script/upgrade/UpgradeContracts.s.sol:UpgradeSyndicateFactory \
 *        --rpc-url $RPC_URL --broadcast --env-file .env
 *
 * @dev IMPORTANT: Always run storage layout validation before upgrading:
 *      make storage-layout-check
 */
contract UpgradeSyndicateFactory is Script {
    function run() external {
        address factoryAddress = vm.envAddress("FACTORY_ADDRESS");

        console2.log("=== Upgrading SyndicateFactory ===");
        console2.log("Proxy address:", factoryAddress);
        console2.log("");

        vm.startBroadcast();

        // Deploy new implementation
        console2.log("Deploying new implementation...");
        SyndicateFactory newImplementation = new SyndicateFactory();
        console2.log("New implementation:", address(newImplementation));
        console2.log("");

        // Upgrade the proxy
        console2.log("Upgrading proxy to new implementation...");
        SyndicateFactory factory = SyndicateFactory(factoryAddress);
        factory.upgradeToAndCall(address(newImplementation), "");

        vm.stopBroadcast();

        // Verify upgrade
        uint256 version = factory.VERSION();
        console2.log("=== Upgrade Complete ===");
        console2.log("Proxy:", factoryAddress);
        console2.log("Implementation:", address(newImplementation));
        console2.log("Version:", version);
    }
}

/**
 * @title UpgradeGasAggregator
 * @notice Upgrades the GasAggregator to a new implementation
 *
 * @dev Environment Variables Required:
 *      - GAS_AGGREGATOR_ADDRESS: Address of GasAggregator proxy
 *      - ADMIN_ADDRESS: Address with DEFAULT_ADMIN_ROLE
 *
 * @dev Usage:
 *      make upgrade-gas-aggregator
 *      or
 *      forge script script/upgrade/UpgradeContracts.s.sol:UpgradeGasAggregator \
 *        --rpc-url $RPC_URL --broadcast --env-file .env
 *
 * @dev IMPORTANT: Always run storage layout validation before upgrading:
 *      make storage-layout-check
 */
contract UpgradeGasAggregator is Script {
    function run() external {
        address gasAggregatorAddress = vm.envAddress("GAS_AGGREGATOR_ADDRESS");

        console2.log("=== Upgrading GasAggregator ===");
        console2.log("Proxy address:", gasAggregatorAddress);
        console2.log("");

        vm.startBroadcast();

        // NOTE: GasAggregator is no longer upgradeable (changed from UUPS to regular Ownable pattern)
        // This script is deprecated and kept for reference only

        // Deploy new implementation
        console2.log("ERROR: GasAggregator is no longer upgradeable!");
        console2.log("Deploy a new GasAggregator contract instead of upgrading.");
        revert("GasAggregator is not upgradeable");

        vm.stopBroadcast();
    }
}
