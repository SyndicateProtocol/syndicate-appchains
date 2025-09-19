// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";

/**
 * @title DeploySyndicateFactory
 * @notice Forge script to deploy SyndicateFactory using CREATE2
 * @dev Deploys upgradeable SyndicateFactory with proper initialization
 */
contract DeploySyndicateFactory is Script {
    function run() public {
        // The fixed salt for CREATE2 deployment
        bytes32 factorySalt = keccak256("SYNDICATE_FACTORY_v1");

        // MUST use the same private key (resulting in the same deployer address) across all chains
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(privateKey);

        // Deploy implementation first
        SyndicateFactory implementation = new SyndicateFactory();

        // Deploy proxy with initialization
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (deployer));
        bytes memory proxyBytecode =
            abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(address(implementation), initData));
        bytes32 proxyBytecodeHash = keccak256(proxyBytecode);

        // Calculate the expected proxy address
        address expectedAddress = Create2.computeAddress(factorySalt, proxyBytecodeHash, deployer);
        console2.log("Expected SyndicateFactory proxy address:", expectedAddress);

        // Start broadcasting transactions
        vm.startBroadcast(privateKey);

        console2.log("Implementation deployed to:", address(implementation));

        // Deploy proxy using CREATE2
        address factoryAddress;
        assembly {
            factoryAddress := create2(0, add(proxyBytecode, 0x20), mload(proxyBytecode), factorySalt)
            if iszero(factoryAddress) {
                mstore(0x00, "Proxy deployment failed")
                revert(0x00, 0x20)
            }
        }

        console2.log("SyndicateFactory proxy deployed to:", factoryAddress);
        console2.log("Deployment successful:", factoryAddress == expectedAddress ? "Yes" : "No");

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
