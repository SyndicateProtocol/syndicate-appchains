// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TeeModule} from "src/withdrawal/TeeModule.sol";
import {DummyKeyManager} from "src/withdrawal/DummyKeyManager.sol";
import {DummyPoster} from "src/withdrawal/DummyPoster.sol";
import {IBridge} from "@arbitrum/nitro-contracts/src/bridge/IBridge.sol";

address constant L1_BLOCK = address(0x4200000000000000000000000000000000000015);

contract DeployTestTeeModule is Script {
    function run() public {
        // Start broadcasting transactions
        vm.startBroadcast();
        IBridge bridge = IBridge(vm.envAddress("APP_BRIDGE"));
        address seqContract = vm.envAddress("SEQ_CONTRACT");
        address seqBridge = vm.envAddress("SEQ_BRIDGE");
        uint64 setDelay = uint64(vm.envUint("SET_DELAY"));
        bytes32 configHash = vm.envBytes32("CONFIG_HASH");
        bytes32 appBlockHash = vm.envBytes32("APP_BLOCK_HASH");
        bytes32 seqBlockHash = vm.envBytes32("SEQ_BLOCK_HASH");
        bytes32 l1BatchAcc = vm.envBytes32("L1_BATCH_ACC");
        address teePublicKey = vm.envAddress("TEE_PUBLIC_KEY");
        uint64 challengeWindowDuration = uint64(vm.envUint("CHALLENGE_WINDOW_DURATION"));

        DummyKeyManager keyManager = new DummyKeyManager();
        keyManager.addKey(teePublicKey);
        DummyPoster poster = new DummyPoster();
        TeeModule teeModule = new TeeModule(
            poster,
            bridge,
            keccak256(abi.encodePacked(seqContract, seqBridge, setDelay)),
            appBlockHash,
            seqBlockHash,
            l1BatchAcc,
            L1_BLOCK,
            false,
            challengeWindowDuration,
            keyManager
        );

        console2.log("TeeModule deployed to:", address(teeModule));
    }
}
