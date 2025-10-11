// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

/**
 * @title UpgradeSequencingChain
 * @notice Upgrades a SyndicateSequencingChain to a new implementation
 *
 * @dev This script performs three actions:
 *      1. Deploys new SyndicateSequencingChain implementation
 *      2. Sets it as default in factory (for future chains)
 *      3. Upgrades the specific chain proxy
 *
 * @dev Environment Variables Required:
 *      - CHAIN_ADDRESS: Address of SyndicateSequencingChain proxy to upgrade
 *      - FACTORY_ADDRESS: Address of SyndicateFactory
 *
 * @dev Usage:
 *      make upgrade-sequencing-chain
 *      or
 *      forge script script/upgrade/UpgradeSequencingChain.s.sol \
 *        --rpc-url $RPC_URL --broadcast --env-file .env
 *
 * @dev IMPORTANT: Always run storage layout validation before upgrading:
 *      make storage-layout-check
 *
 * @dev Note: Setting new default implementation in factory requires DEFAULT_ADMIN_ROLE
 *            The GasAggregator will be notified of the new allowed implementation
 */
contract UpgradeSequencingChain is Script {
    function run() external {
        address chainAddress = vm.envAddress("CHAIN_ADDRESS");
        address factoryAddress = vm.envAddress("FACTORY_ADDRESS");

        console2.log("=== Upgrading SyndicateSequencingChain ===");
        console2.log("Chain proxy:", chainAddress);
        console2.log("Factory:", factoryAddress);
        console2.log("");

        vm.startBroadcast();

        // Deploy new implementation
        console2.log("Deploying new implementation...");
        SyndicateSequencingChain newImplementation = new SyndicateSequencingChain();
        console2.log("New implementation:", address(newImplementation));
        console2.log("");

        // Set as default in factory (notifies GasAggregator)
        console2.log("Setting as default implementation in factory...");
        SyndicateFactory factory = SyndicateFactory(factoryAddress);
        factory.setSyndicateSequencingChainImplementation(address(newImplementation));
        console2.log("Factory updated, GasAggregator notified");
        console2.log("");

        // Upgrade the specific chain
        console2.log("Upgrading chain proxy...");
        SyndicateSequencingChain chain = SyndicateSequencingChain(chainAddress);
        chain.upgradeToAndCall(address(newImplementation), "");

        vm.stopBroadcast();

        // Verify upgrade
        uint256 version = chain.VERSION();
        console2.log("=== Upgrade Complete ===");
        console2.log("Chain proxy:", chainAddress);
        console2.log("Implementation:", address(newImplementation));
        console2.log("Version:", version);
        console2.log("");
        console2.log("New chains created via factory will use this implementation");
    }
}
