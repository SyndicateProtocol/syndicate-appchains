// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
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

        console2.log("Deploying GasArchive...");
        console2.log("Block hash sender:", blockHashSender);
        console2.log("Block hash sender (ArbRollup alias):", blockHashSenderAliased);
        console2.log("Settlement chain ID:", settlementChainID);
        console2.log("Admin address:", admin);

        // Deploy GasArchive contract directly
        GasArchive gasArchive = new GasArchive(blockHashSenderAliased, settlementChainID, admin);
        console2.log("GasArchive deployed to:", address(gasArchive));

        console2.log("=== Deployment Summary ===");
        console2.log("GasArchive:", address(gasArchive));
        console2.log("Admin:", admin);

        vm.stopBroadcast();
    }
}
