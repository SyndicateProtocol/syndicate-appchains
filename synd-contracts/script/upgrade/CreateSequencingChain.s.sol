// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

/**
 * @title CreateSequencingChain
 * @notice Creates a new sequencing chain via the SyndicateFactory
 *
 * @dev Environment Variables Required:
 *      - FACTORY_ADDRESS: Address of deployed SyndicateFactory
 *      - NONCE: User-provided nonce for chain creation (e.g., 1, 2, 3)
 *      - ADMIN_ADDRESS: Admin address for the new chain
 *      - PERMISSION_MODULE (optional): Permission module address
 *                          If not set, deploys new AlwaysAllowedModule
 *
 * @dev Usage:
 *      NONCE=1 make create-sequencing-chain
 *      or
 *      forge script script/upgrade/CreateSequencingChain.s.sol \
 *        --rpc-url $RPC_URL --broadcast --env-file .env
 *
 * @dev Note: The chain ID is computed from the nonce by the factory
 */
contract CreateSequencingChain is Script {
    function run() external {
        // Get required parameters from environment
        address factoryAddress = vm.envAddress("FACTORY_ADDRESS");
        uint256 nonce = vm.envUint("NONCE");
        address admin = vm.envAddress("ADMIN_ADDRESS");

        console2.log("=== Creating Sequencing Chain ===");
        console2.log("Factory:", factoryAddress);
        console2.log("Nonce:", nonce);
        console2.log("Admin:", admin);
        console2.log("");

        // Get or deploy permission module
        address permissionModule;
        try vm.envAddress("PERMISSION_MODULE") returns (address module) {
            permissionModule = module;
            console2.log("Using existing permission module:", permissionModule);
        } catch {
            console2.log("No PERMISSION_MODULE set, deploying AlwaysAllowedModule...");
            vm.startBroadcast();
            permissionModule = address(new AlwaysAllowedModule());
            vm.stopBroadcast();
            console2.log("AlwaysAllowedModule deployed:", permissionModule);
        }

        console2.log("");

        vm.startBroadcast();
        // Create the sequencing chain
        SyndicateFactory factory = SyndicateFactory(factoryAddress);
        (address chainAddress, uint256 chainId) =
            factory.createSyndicateSequencingChain(nonce, admin, IRequirementModule(permissionModule));
        vm.stopBroadcast();

        console2.log("=== Sequencing Chain Created ===");
        console2.log("Chain Address:", chainAddress);
        console2.log("Chain ID:", chainId);
        console2.log("");
        console2.log("Save this address to your .env file:");
        console2.log("CHAIN_ADDRESS=%s", chainAddress);
    }
}
