// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";

interface AppchainFactory {
    function getTotalAppchains() external view returns (uint256);
    function getContractsForAppchains(uint256[] memory chainIDs) external view returns (address[] memory);
    function getAppchainsAndContracts() external view returns (uint256[] memory chainIDs, address[] memory contracts);
}

interface StakingAppchain {
    function pushData(uint256[] memory chainIDs, uint256[] memory tokens, uint256 epoch) external;
}

contract DeployGasAggregator is Script {
    function run() public {
        vm.startBroadcast();

        // Read configuration from environment variables
        address factory = vm.envAddress("APPCHAIN_FACTORY_ADDRESS");
        address stakingAppchain = vm.envAddress("STAKING_APPCHAIN_ADDRESS");
        address admin = vm.envAddress("GAS_AGGREGATOR_ADMIN");
        uint256 challengeWindow = vm.envUint("CHALLENGE_WINDOW"); // Time in seconds, e.g., 86400 for 24 hours

        console2.log("Deploying GasAggregator with TransparentProxy pattern...");
        console2.log("Factory address:", factory);
        console2.log("Staking appchain address:", stakingAppchain);
        console2.log("Admin address:", admin);
        console2.log("Challenge window:", challengeWindow);

        // 1. Deploy ProxyAdmin contract
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        console2.log("ProxyAdmin deployed to:", address(proxyAdmin));

        // 2. Deploy GasAggregator implementation
        GasAggregator implementation = new GasAggregator();
        console2.log("GasAggregator implementation deployed to:", address(implementation));

        // 3. Prepare initialization data
        bytes memory initData = abi.encodeWithSelector(
            GasAggregator.initialize.selector,
            AppchainFactory(factory),
            StakingAppchain(stakingAppchain),
            admin,
            challengeWindow
        );

        // 4. Deploy TransparentUpgradeableProxy
        TransparentUpgradeableProxy proxy =
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);
        console2.log("GasAggregator proxy deployed to:", address(proxy));

        // 5. Optionally set maxAppchainsToQuery if provided
        uint256 maxAppchains = vm.envOr("MAX_APPCHAINS_TO_QUERY", uint256(0));
        if (maxAppchains > 0) {
            console2.log("Setting max appchains to query:", maxAppchains);
            GasAggregator gasAggregator = GasAggregator(address(proxy));
            gasAggregator.setMaxAppchainsToQuery(maxAppchains);
        }

        console2.log("=== Deployment Summary ===");
        console2.log("ProxyAdmin:", address(proxyAdmin));
        console2.log("Implementation:", address(implementation));
        console2.log("GasAggregator (Proxy):", address(proxy));
        console2.log("Admin (ProxyAdmin owner and GasAggregator admin):", admin);

        vm.stopBroadcast();
    }
}
