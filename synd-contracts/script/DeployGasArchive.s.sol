// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {GasArchive} from "src/staking/GasArchive.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";

contract DeployGasArchive is Script, EpochTracker {
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

        address blockHashSenderAliased = applyArbRollupAlias(blockHashSender);
        uint256 epochStart = getCurrentEpoch();

        console2.log("Deploying GasArchive...");
        console2.log("Block hash sender:", blockHashSender);
        console2.log("Block hash sender (ArbRollup alias):", blockHashSenderAliased);
        console2.log("Settlement chain ID:", settlementChainID);
        console2.log("Epoch start:", epochStart);

        // 1. Deploy GasArchive implementation
        GasArchive implementation = new GasArchive(blockHashSenderAliased, settlementChainID);
        console2.log("GasArchive implementation deployed to:", address(implementation));

        // 2. Prepare initialization data
        bytes memory initData = abi.encodeCall(GasArchive.initialize, (epochStart));

        // 3. Deploy ERC1967Proxy (UUPS)
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        console2.log("GasArchive proxy deployed to:", address(proxy));

        console2.log("=== Deployment Summary ===");
        console2.log("GasArchive (Proxy):", address(proxy));

        vm.stopBroadcast();
    }
}
