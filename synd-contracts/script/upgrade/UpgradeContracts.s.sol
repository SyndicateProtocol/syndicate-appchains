// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";

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
        uint256 version = factory.version();
        console2.log("=== Upgrade Complete ===");
        console2.log("Proxy:", factoryAddress);
        console2.log("Implementation:", address(newImplementation));
        console2.log("Version:", version);
    }
}
