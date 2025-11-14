// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import {GasAggregator} from "../src/staking/GasAggregator.sol";
import {BlockHashRelayer, IArbInbox, IERC20} from "../src/staking/BlockHashRelayer.sol";
import {GasArchive} from "../src/staking/GasArchive.sol";
import {PerformancePool} from "../src/staking/PerformancePool.sol";
import {AppchainPool} from "../src/staking/AppchainPool.sol";
import {Splitter} from "../src/staking/Splitter.sol";
import {EmissionsReceiver} from "../src/staking/EmissionsReceiver.sol";

contract DeployPerformanceStaking is Script {
    uint256 public seqChainID = 510;
    uint256 public baseChainID = 8453;
    uint256 public commonsChainID = 510003;

    uint256 public startingEpoch = 4;

    // Ethereum Contracts
    address public seqChainOutbox = address(0xf555Bc86D1C953414F676479Bf7C979b1A737E8C);

    // Base Contracts
    address public arbInbox = address(0xAE824E2d20F21B222932aFC6079cDaA1EB5b2F00);
    address public syndToken = address(0x11dC28D01984079b7efE7763b533e6ed9E3722B9);

    // Commons Contracts
    uint256 public settlementChainID = baseChainID;
    address public admin = address(0x0000000000000000000000000000000000000000);
    address public staking = address(0xF9637B60f27AF139FC46EAa655cFBbe4E731BCdF);
    address public basePool = address(0x71cF8bf70Bb4f5ba8e4B4588bacB5ee108f3Ed10);

    // Filled in after deployment
    address public gasAggregatorDeployment = address(0x0000000000000000000000000000000000000000);
    address public blockHashSenderDeployment = address(0x0000000000000000000000000000000000000000);

    function run() public {
        vm.startBroadcast();

        useTestnetValues();

        if (block.chainid == seqChainID) {
            console2.log("Deploying Performance Staking Contracts on Syndicate...");
            deploySeqChains();
        } else if (block.chainid == baseChainID) {
            console2.log("Deploying Performance Staking Contracts on Base...");
            deployBase();
        } else if (block.chainid == commonsChainID) {
            console2.log("Deploying Performance Staking Contracts on Commons...");
            deployCommons();
        } else if (block.chainid == 1) {
            actionsOnMainnet();
        } else {
            revert("Invalid chain ID");
        }

        vm.stopBroadcast();
    }

    function useTestnetValues() public {
        seqChainID = 51014; // Risa
        baseChainID = 84532; // Base Sepolia
        commonsChainID = 510002; // Cheesesteak

        startingEpoch = 4;

        // Ethereum Contracts
        seqChainOutbox = address(0x11A06E54971d7a61Ba7BCd47663Af3680E6582F9);

        // Base Contracts
        arbInbox = address(0xf324b8d22a73Ebc59537c7666F72aD5229B81b0f);
        syndToken = address(0x234Faa9cdeE5822767076495A9E258Dd8F21fFD8);

        // Commons Contracts
        settlementChainID = 84532;
        admin = address(0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A);
        staking = address(0x503CF45e4376fC0c8d852f96c540fFF3c1487425);
        basePool = address(0x55040e6DB9BC79f158d8aF5d3Ed5BB62Ddf05d9f);

        // Filled in after deployment
        gasAggregatorDeployment = address(0x0000000000000000000000000000000000000000);
        blockHashSenderDeployment = address(0x0000000000000000000000000000000000000000);
        splitterDeployment = address(0x0000000000000000000000000000000000000000);
    }

    function actionsOnMainnet() public {
        require(splitterDeployment != address(0), "Splitter deployment not set");
        console2.log("Actions on Mainnet...");
        console2.log("These need to be done using the admin Gnosis Safe");

        console2.log("1. Call 'setRelayDestinationL3(address)' on the EmissionsScheduler contract to set the relay destination to:", splitterDeployment);
        console2.log("2. Call 'unpause()' on EmissionsScheduler contract");
    }

    function deploySeqChains() public {
        console2.log("Deploying Sequencing Chains...");
        GasAggregator gasAggregator = new GasAggregator(startingEpoch, 0, 0);
        console2.log("GasAggregator deployed to:", address(gasAggregator));

        if (seqChainID == 510) {
            // Only setup mainnet chains if we are on Syndicate mainnet
            gasAggregator.addLegacyChain(510003, address(0xbf4139c8332261d10A40b79274A4170a6B50Fc3A)); // Commons
            gasAggregator.addLegacyChain(63829, address(0xa8BDf301Fc4E8abC6857816220e77E4600A8C582)); // CMMT
            gasAggregator.addLegacyChain(510525, address(0x328dD1B8FA8ea7654520DC0C03B464aa5b7eAb89)); // Clankermon
            gasAggregator.addLegacyChain(574014, address(0x4F576256b2A9472677ebf271140429820a13A186)); // Stadium
        }

        console2.log("===  Transfer Ownership ===");

        gasAggregator.transferOwnership(admin);
        console2.log("GasAggregator ownership transferred to admin");
    }

    function deployBase() public {
        console2.log("Deploying Base...");

        BlockHashRelayer blockHashRelayer = new BlockHashRelayer(IArbInbox(arbInbox), IERC20(syndToken));
        console2.log("BlockHashRelayer deployed to:", address(blockHashRelayer));
    }

    function deployCommons() public {
        require(gasAggregatorDeployment != address(0), "GasAggregator deployment not set");
        require(blockHashSenderDeployment != address(0), "BlockHashSender deployment not set");
        console2.log("Deploying Commons...");

        GasArchive gasArchiveImpl = new GasArchive(blockHashSenderDeployment, settlementChainID);
        console2.log("GasArchive implementation deployed to:", address(gasArchiveImpl));

        bytes memory initData = abi.encodeCall(GasArchive.initialize, (startingEpoch));
        GasArchive gasArchive = GasArchive(address(new ERC1967Proxy(address(gasArchiveImpl), initData)));
        console2.log("GasArchive proxy deployed to:", address(gasArchive));

        EmissionsReceiver emissionsReceiver = new EmissionsReceiver();
        console2.log("EmissionsReceiver deployed to:", address(emissionsReceiver));

        PerformancePool performancePool = new PerformancePool(admin, staking, address(gasArchive));
        console2.log("PerformancePool deployed to:", address(performancePool));

        AppchainPool appchainPool = new AppchainPool(admin, staking, address(gasArchive), address(emissionsReceiver));
        console2.log("AppchainPool deployed to:", address(appchainPool));

        Splitter splitter = new Splitter(basePool, address(performancePool), address(appchainPool));
        console2.log("Splitter deployed to:", address(splitter));   

        console2.log("=== Setup ===");

        gasArchive.addSequencingChain(seqChainID, gasAggregatorDeployment, seqChainOutbox, false);
        console2.log("Sequencing Chain added to GasArchive");

        console2.log("===  Transfer Ownership ===");

        gasArchive.transferOwnership(admin);
        console2.log("Gas Archive ownership transferred to admin");

        emissionsReceiver.transferOwnership(admin);
        console2.log("EmissionsReceiver ownership transferred to admin");

        performancePool.transferOwnership(admin);
        console2.log("Performance Pool ownserhip transfer to admin");

        appchainPool.transferOwnership(admin);
        console2.log("Appchain Pool ownership transferred to admin");
    }
}
