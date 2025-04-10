// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {MetabasedFactory} from "src/MetabasedFactory.sol";

/**
 * @title DeployMetabasedFactoryDirect
 * @notice Forge script to deploy MetabasedFactory directly using CREATE2
 * @dev Alternative approach that doesn't require a separate deployer contract
 */
contract DeployMetabasedFactoryDirect is Script {
    function run() public {
        // The fixed salt for CREATE2 deployment
        bytes32 factorySalt = keccak256("METABASED_FACTORY_v1");

        // MUST use the same private key (resulting in the same deployer address) across all chains
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);

        // Get the bytecode for the factory
        bytes memory bytecode = type(MetabasedFactory).creationCode;
        bytes32 bytecodeHash = keccak256(bytecode);

        // Calculate the expected address
        address expectedAddress = Create2.computeAddress(factorySalt, bytecodeHash, deployer);
        console2.log("Expected MetabasedFactory address:", expectedAddress);

        // Start broadcasting transactions
        vm.startBroadcast(privateKey);

        // Deploy using CREATE2
        address factoryAddress;
        assembly {
            factoryAddress := create2(0, add(bytecode, 0x20), mload(bytecode), factorySalt)
            if iszero(factoryAddress) {
                mstore(0x00, "Deployment failed")
                revert(0x00, 0x20)
            }
        }

        console2.log("MetabasedFactory deployed to:", factoryAddress);
        console2.log("Deployment successful:", factoryAddress == expectedAddress ? "Yes" : "No");

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
