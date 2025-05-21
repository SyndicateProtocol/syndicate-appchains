// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {ArbConfigManagerFactory} from "src/config/ArbConfigManagerFactory.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";

contract DeployArbConfigManagerFactory is Script {
    function run() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        vm.startBroadcast(privateKey);
        new ArbConfigManagerFactory();
        vm.stopBroadcast();
    }
}

contract DeployArbConfigManagerForExitingChains is Script {
    struct ChainConfig {
        string name;
        uint256 chainId;
        address arbitrumBridgeAddress;
        address arbitrumInboxAddress;
        bool arbitrumIgnoreDelayedMessages;
        uint256 settlementDelay;
        uint256 settlementStartBlock;
        address sequencingContractAddress;
        uint256 sequencingStartBlock;
        address initialAppchainOwner;
        string appchainBlockExplorerUrl;
        address[] allowedSettlementAddresses;
    }

    function run() public {
        ArbConfigManager manager = ArbConfigManager(0xdf76aFe1057789d64e069C5cd1D3AfA5565c3d86);
        address owner = vm.addr(vm.envUint("DEPLOYER_PRIVATE_KEY"));
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        uint256 sequencingChainId = 5113;
        string memory sequencingChainRpcUrl = "";

        ChainConfig[11] memory chainConfigs;
        chainConfigs[0] = ChainConfig({
            name: "manchego",
            chainId: 510000,
            arbitrumBridgeAddress: 0x13Cb6F51e976287708D0ce926998ab717064b6C5,
            arbitrumInboxAddress: 0x141F33Fa64F669C71626beE5bFb1C3c5fB6fFbbD,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 21853620,
            sequencingContractAddress: 0x180972BF154c9Aea86c43149D83B7Ea078c33f48,
            sequencingStartBlock: 5527,
            initialAppchainOwner: 0x9a37E57d177c5Ff8817B55da36F2A2b3532CDE3F,
            appchainBlockExplorerUrl: "https://manchego-syndicate.cloud.blockscout.com/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[1] = ChainConfig({
            name: "burrata",
            chainId: 510001,
            arbitrumBridgeAddress: 0x32a725c440Ab3e855048C4620862754B7c51828C,
            arbitrumInboxAddress: 0x9ee2A0EAA2b6db13A462b8B0330a07EcCd57B9Ee,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 22106652,
            sequencingContractAddress: 0xC1cacFC14624c4E241286Ade61DF545b90f850B4,
            sequencingStartBlock: 51916,
            initialAppchainOwner: 0x9a37E57d177c5Ff8817B55da36F2A2b3532CDE3F,
            appchainBlockExplorerUrl: "",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[2] = ChainConfig({
            name: "cheddar",
            chainId: 63820,
            arbitrumBridgeAddress: 0x3AF7be703F48BcdB583d544F30bd365237f32dAD,
            arbitrumInboxAddress: 0x4A72956752b30711cAE859eB226b30939AD1B9Ec,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 24566776,
            sequencingContractAddress: 0xF9d9D097431c1f931D9Ea606fc7b118038F88590,
            sequencingStartBlock: 1048981,
            initialAppchainOwner: 0x49629dA1280DAb295E768d55fbc71D3AF51179B8,
            appchainBlockExplorerUrl: "https://cheddar.explorer.testnet.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[3] = ChainConfig({
            name: "dream",
            chainId: 63823,
            arbitrumBridgeAddress: 0x5f1a0E5A9bd9F5882Cff9B20042FAEF5d11dADD6,
            arbitrumInboxAddress: 0xD48f7ba1F470f6f163c889299B17E4a2f5242EAc,
            arbitrumIgnoreDelayedMessages: true,
            settlementDelay: 0,
            settlementStartBlock: 22160790,
            sequencingContractAddress: 0x62B82d1AF6D61DdfE5b4af38Eb5dE982A7f7565f,
            sequencingStartBlock: 90282,
            initialAppchainOwner: 0xC480Fc4694e5da3cF8257F9bF51eF1d01e1952Eb,
            appchainBlockExplorerUrl: "https://explorer.testnet.dream.syndicate.io/",
            // @note TODO: what values need to be passed here?
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[4] = ChainConfig({
            name: "commerce",
            chainId: 63822,
            arbitrumBridgeAddress: 0x1B1187A64729Ec53e57582A3363Ab112DF3D6749,
            arbitrumInboxAddress: 0xa4E2630072b983093644758ec2B712b20A02fea9,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 22074405,
            sequencingContractAddress: 0x7C8d3922298AbbEF7beE5F3dACC4238326482789,
            sequencingStartBlock: 32400,
            initialAppchainOwner: 0xb237e2b2E37cC486395B869D8dF106E269fDff4b,
            appchainBlockExplorerUrl: "https://explorer.testnet.commerce.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[5] = ChainConfig({
            name: "irl",
            chainId: 63821,
            arbitrumBridgeAddress: 0x9682E8DFfdadA5C5834ea0c905543798C72690F2,
            arbitrumInboxAddress: 0x5573d100711322FA0B28923A0786Cec221bb2e67,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 22074150,
            sequencingContractAddress: 0x536EA7C009ebE418501a1DB133b281a4a01d50f5,
            sequencingStartBlock: 422342,
            initialAppchainOwner: 0xee421B02A7AD4550511573f6e858F538B8906E92,
            appchainBlockExplorerUrl: "https://explorer.testnet.irl.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[6] = ChainConfig({
            name: "amino",
            chainId: 63824,
            arbitrumBridgeAddress: 0x6Fdfdac6B7Dab7C1E775bBaAb252375c09f1c44D,
            arbitrumInboxAddress: 0xd44570de730caB769f4FfaC427B7293e499a96e6,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 23361520,
            sequencingContractAddress: 0x8CcaC248CcFCA1283981678B7291F48f6e26ad39,
            sequencingStartBlock: 1042720,
            initialAppchainOwner: 0x5e2BAac714b9BF2680647F963996fF42228F7333,
            appchainBlockExplorerUrl: "https://explorer.testnet.amino.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[7] = ChainConfig({
            name: "eco",
            chainId: 63825,
            arbitrumBridgeAddress: 0x27F110e95315A9C780cB79D972A4d23154eCb34d,
            arbitrumInboxAddress: 0xc3e255EBcA0aAa45ab3E6A44b8d93403892df974,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 23920660,
            sequencingContractAddress: 0x47ec452FA5035C24217daCC66aA305802F1d0fbe,
            sequencingStartBlock: 1044655,
            initialAppchainOwner: 0xD35C73868A757711990cBa67Fc6330fFE6F9202E,
            appchainBlockExplorerUrl: "https://eco.explorer.testnet.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[8] = ChainConfig({
            name: "playground",
            chainId: 63826,
            arbitrumBridgeAddress: 0x6EC25a67B3200142A149bC02cA41377e72d7298b,
            arbitrumInboxAddress: 0x12FE97A3F116841e6139fB63447A5dB5eF69efb4,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 24010180,
            sequencingContractAddress: 0x4e001110D16bE154EB586e73d2da823721E1a9cD,
            sequencingStartBlock: 1045235,
            initialAppchainOwner: 0x66BE354275EAD6b3b9C205aEcC92Df9be2fBE55d,
            appchainBlockExplorerUrl: "https://playground.explorer.testnet.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[9] = ChainConfig({
            name: "sic",
            chainId: 63827,
            arbitrumBridgeAddress: 0xC769d9415144F896C65B8eD8beA1c425FdC184A4,
            arbitrumInboxAddress: 0xB8F7b603Ee0f4f1Ac6A842e53BF4b90c59cFF5BC,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 24574415,
            sequencingContractAddress: 0xDf8953cB55ac0a3cCDB8E6671db29353A4373306,
            sequencingStartBlock: 1049096,
            initialAppchainOwner: 0xa4E45A3FA9fA6164f7AA2caeA2c6c0D6a9B8eA5b,
            appchainBlockExplorerUrl: "https://sic.explorer.testnet.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        chainConfigs[10] = ChainConfig({
            name: "selene",
            chainId: 63888,
            arbitrumBridgeAddress: 0x5eeD1AAf542DCf3B08c67b1523D9Cd0E39B9de7F,
            arbitrumInboxAddress: 0x2910D25b0Ed24B6Cf60CA69DAc9A43fAfdeDfDCa,
            arbitrumIgnoreDelayedMessages: false,
            settlementDelay: 0,
            settlementStartBlock: 25184570,
            sequencingContractAddress: 0xb0F3aE2863c4d0Bed637b1dd4Fe5Ca87D15E7EBf,
            sequencingStartBlock: 1074096,
            initialAppchainOwner: 0x9334297A9c1B3c5cf96f8821385a629aC64AF154,
            appchainBlockExplorerUrl: "https://selene.explorer.testnet.syndicate.io/",
            allowedSettlementAddresses: new address[](0)
        });

        vm.startBroadcast(privateKey);
        for (uint256 i = 0; i < chainConfigs.length; i++) {
            ChainConfig memory chainConfig = chainConfigs[i];
            address arbChainConfigAddress = manager.createArbChainConfig(
                owner,
                chainConfig.chainId,
                sequencingChainId,
                chainConfig.arbitrumBridgeAddress,
                chainConfig.arbitrumInboxAddress,
                chainConfig.arbitrumIgnoreDelayedMessages,
                chainConfig.settlementDelay,
                chainConfig.settlementStartBlock,
                chainConfig.sequencingContractAddress,
                chainConfig.sequencingStartBlock,
                chainConfig.initialAppchainOwner,
                sequencingChainRpcUrl,
                chainConfig.appchainBlockExplorerUrl,
                chainConfig.allowedSettlementAddresses
            );
            console2.log("ArbChainConfig for chain", chainConfig.name, "deployed to:", arbChainConfigAddress);
        }
        vm.stopBroadcast();
    }
}

contract DeployArbConfigManager is Script {
    function run() public {
        // Get deployer information from environment
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);
        address owner = vm.envAddress("OWNER_ADDRESS");

        // Use a fixed salt to ensure consistency across chains
        bytes32 salt = keccak256("ARB_CONFIG_MANAGER");

        console2.log("Deployer address:", deployer);
        console2.log("Owner address:", owner);

        vm.startBroadcast(privateKey);

        // Step 1: Deploy the factory if it doesn't exist already
        // You could get this address from your environment if already deployed
        ArbConfigManagerFactory factory = new ArbConfigManagerFactory();
        console2.log("Factory deployed to:", address(factory));

        // Step 2: Get the predicted address
        address predictedAddress = factory.predictDeploymentAddress(owner, salt);
        console2.log("Predicted ArbConfigManager address:", predictedAddress);

        // Step 3: Deploy the ArbConfigManager through the factory
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        console2.log("ArbConfigManager deployed to:", deployedAddress);
        console2.log("Deployment successful:", deployedAddress == predictedAddress ? "Yes" : "No");

        require(deployedAddress == predictedAddress, "Deployed address does not match the predicted address");

        vm.stopBroadcast();
    }
}
