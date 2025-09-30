// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/*
 * Deployment script for GasAggregator contract with TransparentUpgradeableProxy pattern.
 *
 * Required environment variables:
 * - APPCHAIN_FACTORY_ADDRESS: Address of the deployed AppchainFactory contract
 * - GAS_AGGREGATOR_ADMIN: Address that will have admin rights for the GasAggregator
 * - CHALLENGE_WINDOW: Time window in seconds for challenges (e.g., 86400 for 24 hours)
 *
 * Optional environment variables:
 * - ADD_CHAIN_FEE: Fee in wei for adding a chain (default: 0.1 ETH = 100000000000000000)
 * - MAX_APPCHAINS_TO_QUERY: Maximum number of chains to query before falling back to offchain (default: 0 = not set)
 *
 * Example usage:
 * forge script script/DeployGasAggregator.s.sol:DeployGasAggregator \
 *   --rpc-url $RPC_URL \
 *   --private-key $PRIVATE_KEY \
 *   --broadcast \
 *   --env-file .env
 */

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";

interface AppchainFactory {
    function isImplementationAllowed(address implementation) external view returns (bool);
    function computeSequencingChainAddress(uint256 chainId) external view returns (address);
    function getProxyBytecode() external view returns (bytes memory);
}

contract DeployGasAggregator is Script {
    function run() public {
        vm.startBroadcast();

        // Read configuration from environment variables
        address factory = vm.envAddress("APPCHAIN_FACTORY_ADDRESS");
        address admin = vm.envAddress("GAS_AGGREGATOR_ADMIN");
        uint256 challengeWindow = vm.envUint("CHALLENGE_WINDOW"); // Time in seconds, e.g., 86400 for 24 hours
        uint256 addChainFee = vm.envOr("ADD_CHAIN_FEE", uint256(0.1 ether)); // Default 0.1 ETH

        console2.log("Deploying GasAggregator with TransparentProxy pattern...");
        console2.log("Factory address:", factory);
        console2.log("Admin address:", admin);
        console2.log("Challenge window:", challengeWindow);
        console2.log("Add chain fee:", addChainFee);

        // 1. Deploy ProxyAdmin contract
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        console2.log("ProxyAdmin deployed to:", address(proxyAdmin));

        // 2. Deploy GasAggregator implementation
        GasAggregator implementation = new GasAggregator();
        console2.log("GasAggregator implementation deployed to:", address(implementation));

        // 3. Prepare initialization data
        bytes memory initData = abi.encodeWithSelector(
            GasAggregator.initialize.selector, AppchainFactory(factory), admin, challengeWindow, addChainFee
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
        console2.log("Challenge Window:", challengeWindow, "seconds");
        console2.log("Add Chain Fee:", addChainFee, "wei");

        vm.stopBroadcast();
    }
}
