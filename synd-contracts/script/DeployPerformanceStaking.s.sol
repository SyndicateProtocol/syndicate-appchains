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

    address public seqChainAdmin = address(0xccc32F51073B0B1EB15746f5a2b1EE76add65CCC);

    // Ethereum Contracts
    address public seqChainOutbox = address(0xf555Bc86D1C953414F676479Bf7C979b1A737E8C);

    // Base Contracts
    address public arbInbox = address(0xAE824E2d20F21B222932aFC6079cDaA1EB5b2F00);
    address public syndToken = address(0x11dC28D01984079b7efE7763b533e6ed9E3722B9);

    // Commons Contracts
    uint256 public settlementChainID = baseChainID;
    address public commonsAdmin = address(0x03F8b8f48a3F22109bf1F4b54b54d0fdc96E7A67);
    address public staking = address(0xF9637B60f27AF139FC46EAa655cFBbe4E731BCdF);
    address public basePool = address(0x71cF8bf70Bb4f5ba8e4B4588bacB5ee108f3Ed10);

    // Filled in after deployment
    address public gasAggregatorDeployment = address(0x1aCc3a26FCB9751D5E3b698D009b9C944eb98F9e);
    address public blockHashSenderDeployment = address(0xD77Aa8b1743326Baeb548357f8334df911A4E58f);
    address public gasArchiveDeployment = address(0xAb5390d3708C78e84b82de12D3e07d94145a3C0b);
    address public splitterDeployment = address(0xF37a28C21A72eCf75fde856Ed18D28D1A49fA21d);


    uint160 constant offset = uint160(0x1111000000000000000000000000000000001111);
    function applyArbRollupAlias(address l1Address) internal pure returns (address l2Address) {
        unchecked {
            l2Address = address(uint160(l1Address) + offset);
        }
    }

    function run() public {
        vm.startBroadcast();

        // useTestnetValues();

        if (block.chainid == seqChainID) {
            console2.log("Deploying Performance Staking Contracts on Syndicate...");
            deploySeqChains();
        } else if (block.chainid == baseChainID) {
            console2.log("Deploying Performance Staking Contracts on Base...");
            deployBase();
        } else if (block.chainid == commonsChainID) {
            console2.log("Deploying Performance Staking Contracts on Commons...");
            // deployCommons();
            deployCommonsPools();
        } else if (block.chainid == 1) {
            actionsOnMainnet();
        } else {
            console2.log("Invalid chain ID:", block.chainid);
            revert("Invalid chain ID");
        }

        vm.stopBroadcast();
    }

    function useTestnetValues() public {
        seqChainID = 51014; // Risa
        baseChainID = 84532; // Base Sepolia
        commonsChainID = 510002; // Cheesesteak

        startingEpoch = 2;

        seqChainAdmin = address(0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A);

        // Ethereum Contracts
        seqChainOutbox = address(0x11A06E54971d7a61Ba7BCd47663Af3680E6582F9);

        // Base Contracts
        arbInbox = address(0x633de5dE5a1cDfF2e9Fb62Fc955618606366dbCe);
        syndToken = address(0x234Faa9cdeE5822767076495A9E258Dd8F21fFD8);

        // Commons Contracts
        settlementChainID = 84532;
        commonsAdmin = address(0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A);
        staking = address(0x503CF45e4376fC0c8d852f96c540fFF3c1487425);
        basePool = address(0x55040e6DB9BC79f158d8aF5d3Ed5BB62Ddf05d9f);

        // Filled in after deployment
        gasAggregatorDeployment = address(0xF37a28C21A72eCf75fde856Ed18D28D1A49fA21d);
        blockHashSenderDeployment = address(0x28F62871025e78a4a7C03c7287425b2eDE304E3A);
        gasArchiveDeployment = address(0x9D01b45D73fb63442509F3Ed757e176174811949);
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
        require(seqChainAdmin != address(0), "SeqChainAdmin not set");

        console2.log("Deploying Sequencing Chains...");
        GasAggregator gasAggregator = new GasAggregator(startingEpoch, 0, 0);
        console2.log("GasAggregator deployed to:", address(gasAggregator));

        if (seqChainID == 510) {
            console2.log("Adding mainnet chains to GasAggregator");
            // Only setup mainnet chains if we are on Syndicate mainnet
            gasAggregator.addLegacyChain(510003, address(0xbf4139c8332261d10A40b79274A4170a6B50Fc3A)); // Commons
            gasAggregator.addLegacyChain(63829, address(0xa8BDf301Fc4E8abC6857816220e77E4600A8C582)); // CMMT
            gasAggregator.addLegacyChain(510525, address(0x328dD1B8FA8ea7654520DC0C03B464aa5b7eAb89)); // Clankermon
            gasAggregator.addLegacyChain(574014, address(0x4F576256b2A9472677ebf271140429820a13A186)); // Stadium
        } else if (seqChainID == 51014) {
            console2.log("Adding testnet chains to GasAggregator");
            gasAggregator.addLegacyChain(510001, address(0x601530cF8595535746063423395b9DB1e98a801b)); // Burrata
            gasAggregator.addLegacyChain(510000, address(0xA9D785D1EcA06cB9Abc105a68F281044002FA903)); // Manchego
            gasAggregator.addLegacyChain(510002, address(0x6723cfa6de616e20e35dba0775c92FC53bB0fE15)); // Cheesesteak
        }

        console2.log("===  Transfer Ownership ===");

        gasAggregator.transferOwnership(seqChainAdmin);
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
        require(commonsAdmin != address(0), "CommonsAdmin not set");
        console2.log("Deploying Commons...");

        GasArchive gasArchiveImpl = new GasArchive(applyArbRollupAlias(blockHashSenderDeployment), settlementChainID);
        console2.log("GasArchive implementation deployed to:", address(gasArchiveImpl));

        bytes memory initData = abi.encodeCall(GasArchive.initialize, (startingEpoch));
        GasArchive gasArchive = GasArchive(address(new ERC1967Proxy(address(gasArchiveImpl), initData)));
        console2.log("GasArchive proxy deployed to:", address(gasArchive)); 

        console2.log("=== Setup ===");

        gasArchive.addSequencingChain(seqChainID, gasAggregatorDeployment, seqChainOutbox, false);
        console2.log("Sequencing Chain added to GasArchive");

        console2.log("===  Transfer Ownership ===");

        gasArchive.transferOwnership(commonsAdmin);
        console2.log("Gas Archive ownership transferred to admin");
    }

    function deployCommonsPools() public {
        require(gasArchiveDeployment != address(0), "GasArchive deployment not set");
        require(commonsAdmin != address(0), "CommonsAdmin not set");
        console2.log("Deploying Commons (Part 2)...");

        EmissionsReceiver emissionsReceiver = new EmissionsReceiver();
        console2.log("EmissionsReceiver deployed to:", address(emissionsReceiver));

        PerformancePool performancePool = new PerformancePool(commonsAdmin, staking, gasArchiveDeployment);
        console2.log("PerformancePool deployed to:", address(performancePool));

        AppchainPool appchainPool = new AppchainPool(commonsAdmin, staking, gasArchiveDeployment, address(emissionsReceiver));
        console2.log("AppchainPool deployed to:", address(appchainPool));

        Splitter splitter = new Splitter(basePool, address(performancePool), address(appchainPool));
        console2.log("Splitter deployed to:", address(splitter));

        console2.log("===  Add emission receivers ===");

        if (commonsChainID == 510003) {
            // Only setup mainnet chains if we are on commons
            // emissionsReceiver.setAppchainEmissionsReceiver(510003, address(0x0000000000000000000000000000000000000000)); // Commons
            // emissionsReceiver.setAppchainEmissionsReceiver(63829, address(0x0000000000000000000000000000000000000000)); // CMMT
            emissionsReceiver.setAppchainEmissionsReceiver(510525, address(0xec0e25aBc32e5dcee851133c59a0bE9Fe6BA452A)); // Clankermon
            emissionsReceiver.setAppchainEmissionsReceiver(574014, address(0x6c43da7CD9f68CfEfC626155D52805d66308835c)); // Stadium
            emissionsReceiver.setAppchainEmissionsReceiver(88899, address(0xa0055EFF3f0f17309C09685c1417Ec4Fb9899A31)); // Unite
            emissionsReceiver.setAppchainEmissionsReceiver(510530, address(0xF86883ab3E6341C51B67caFFdba3cE687a5c83ff)); // OpenLoot
        }


        console2.log("===  Transfer Ownership ===");

        emissionsReceiver.transferOwnership(commonsAdmin);
        console2.log("EmissionsReceiver ownership transferred to admin");

        //   EmissionsReceiver deployed to: 0xf8CA1551b6878779e4F4e60fffF07EF74Ac6051e
        //   PerformancePool deployed to: 0xD966439E91f41337D784BFFA0d38958fFECa54D2
        //   AppchainPool deployed to: 0xEd582132C33DE5B5A661De3D2dCe5FB8F2d8F33D
        //   Splitter deployed to: 0xF37a28C21A72eCf75fde856Ed18D28D1A49fA21d
    }
}
