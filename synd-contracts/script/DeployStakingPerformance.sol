// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {PerformancePool} from "src/staking/PerformancePool.sol";
import {AppchainPool} from "src/staking/AppchainPool.sol";
import {MockGasDataProvider} from "src/staking/MockGasDataProvider.sol";
/**
 * @title Deploy Staking Performance Contracts
 * @notice Deploy the staking performance contracts
 */

contract DeployStakingPerformance is Script {
    function run() public {
        vm.startBroadcast();

        address staking = address(0x1aCc3a26FCB9751D5E3b698D009b9C944eb98F9e);
        address admin = address(0x4834a3c778F2d005B28a18c68d580Cc7F68c5Cbf);

        MockGasDataProvider gasDataProvider = new MockGasDataProvider();

        PerformancePool performancePool = new PerformancePool(admin, staking, address(gasDataProvider));
        AppchainPool appchainPool = new AppchainPool(admin, staking, address(gasDataProvider));

        console.log("GasDataProvider deployed to:", address(gasDataProvider));
        console.log("PerformancePool deployed to:", address(performancePool));
        console.log("AppchainPool deployed to:", address(appchainPool));

        vm.stopBroadcast();
    }
}
