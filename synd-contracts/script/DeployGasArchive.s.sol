// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {GasArchive} from "src/staking/GasArchive.sol";

contract DeployGasArchive is Script {
    uint160 constant offset = uint160(0x1111000000000000000000000000000000001111);

    function applyArbRollupAlias(address l1Address) internal pure returns (address l2Address) {
        unchecked {
            l2Address = address(uint160(l1Address) + offset);
        }
    }

    function run() public {
        vm.startBroadcast();

        // Read configuration from environment variables
        address blockHashSender = vm.envAddress("BLOCK_HASH_SENDER");
        uint256 settlementChainID = vm.envUint("SETTLEMENT_CHAIN_ID");
        address admin = vm.envAddress("GAS_ARCHIVE_ADMIN");

        address blockHashSenderAliased = applyArbRollupAlias(blockHashSender);

        console2.log("Deploying GasArchive with TransparentProxy pattern...");
        console2.log("Block hash sender:", blockHashSender);
        console2.log("Block hash sender (ArbRollup alias):", blockHashSenderAliased);
        console2.log("Settlement chain ID:", settlementChainID);
        console2.log("Admin address:", admin);

        // 1. Deploy ProxyAdmin contract
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        console2.log("ProxyAdmin deployed to:", address(proxyAdmin));

        // 2. Deploy GasArchive implementation
        GasArchive implementation = new GasArchive();
        console2.log("GasArchive implementation deployed to:", address(implementation));

        // 3. Prepare initialization data
        bytes memory initData =
            abi.encodeWithSelector(GasArchive.initialize.selector, blockHashSenderAliased, settlementChainID, admin);

        // 4. Deploy TransparentUpgradeableProxy
        TransparentUpgradeableProxy proxy =
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);
        console2.log("GasArchive proxy deployed to:", address(proxy));

        console2.log("=== Deployment Summary ===");
        console2.log("ProxyAdmin:", address(proxyAdmin));
        console2.log("Implementation:", address(implementation));
        console2.log("GasArchive (Proxy):", address(proxy));
        console2.log("Admin (ProxyAdmin owner and GasArchive admin):", admin);

        vm.stopBroadcast();
    }
}
