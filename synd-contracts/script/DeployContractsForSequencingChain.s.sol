// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {RequireAndModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";
import {IGasAggregator} from "src/interfaces/IGasAggregator.sol";
import {MinimalUUPSStub} from "src/factory/MinimalUUPSStub.sol";

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeploySyndicateFactory is Script {
    SyndicateFactory public syndicateFactory;
    RequireAndModuleFactory public requireAndModuleFactory;
    uint256 public appchainId;

    function run() public {
        vm.startBroadcast();

        appchainId = 0; // TODO: Set the App chain ID

        // syndicate admin and manager
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy implementation and proxy
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        syndicateFactory = SyndicateFactory(address(proxy));
        console.log("Deployed SyndicateFactory", address(syndicateFactory));

        // Deploy and set GasAggregator
        GasAggregator gasAggImpl = new GasAggregator();
        MinimalUUPSStub stub = new MinimalUUPSStub();
        ERC1967Proxy gasAggProxy = new ERC1967Proxy(address(stub), "");
        bytes memory gasAggInitData = abi.encodeWithSignature(
            "initialize(address,address,address,uint256)",
            admin,
            address(syndicateFactory),
            syndicateFactory.syndicateChainImpl(),
            1
        );
        (bool success,) = address(gasAggProxy).call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", address(gasAggImpl), gasAggInitData)
        );
        require(success, "GasAgg init failed");
        syndicateFactory.setGasAggregator(IGasAggregator(address(gasAggProxy)));
        console.log("Deployed GasAggregator", address(gasAggProxy));

        requireAndModuleFactory = new RequireAndModuleFactory(admin);
        console.log("Deployed RequireAndModuleFactory", address(requireAndModuleFactory));

        address module = requireAndModuleFactory.createRequireAndModule(admin, bytes32(appchainId));
        console.log("Deployed RequireAndModule", module);

        // create SyndicateSequencingChain with the permission module (using nonce 0)
        (address sequencingChain, uint256 chainId) =
            syndicateFactory.createSyndicateSequencingChain(0, admin, IRequirementModule(module));

        console.log("Deployed SyndicateSequencingChain", sequencingChain);
        console.log("Deployed RequireAndModule", address(module));
        console.log("Sequencing Chain ID", chainId);

        vm.stopBroadcast();
    }
}

contract DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule is Script {
    SyndicateSequencingChain public sequencingChain;
    RequireAndModule public permissionModule;
    uint256 public appchainId;

    function run() public {
        vm.startBroadcast();

        appchainId = 0; // TODO: Set the App chain ID
        address gasAggregator = vm.envOr("GAS_AGGREGATOR_ADDR", address(0));
        address admin = vm.envOr("ADMIN_ADDR", msg.sender);

        // Deploy permission module first
        permissionModule = new RequireAndModule(admin);
        console.log("Deployed RequireAndModule", address(permissionModule));

        // Deploy sequencer with permission module
        sequencingChain = new SyndicateSequencingChain();
        sequencingChain.initialize(admin, address(0), gasAggregator, address(permissionModule), appchainId, 0);
        console.log("Deployed SyndicateSequencingChain", address(sequencingChain));

        // Deploy and add always allowed module
        AlwaysAllowedModule alwaysAllowedModule = new AlwaysAllowedModule();
        console.log("Deployed AlwaysAllowedModule", address(alwaysAllowedModule));

        permissionModule.addPermissionCheck(address(alwaysAllowedModule), true);
        console.log("Added alwaysAllowedModule to permission checks", address(alwaysAllowedModule));

        vm.stopBroadcast();
    }
}
