// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {SyndicateTokenCrosschain} from "src/token/SyndicateTokenCrosschain.sol";
import {TestnetSyndTokenCrosschain} from "src/token/testnet-synd-token/TestnetSyndTokenCrosschain.sol";

/**
 * @title Direct Crosschain Token Deployment
 * @notice Deploy crosschain tokens directly without factory to avoid contract size limits
 */
contract DeploySyndicateTokenCrosschainDirect is Script {
    function run() public {
        vm.startBroadcast();

        address admin = vm.envAddress("ADMIN_ADDR");
        address treasury = vm.envAddress("MANAGER_ADDR");

        require(admin != address(0), "ADMIN_ADDR not set");
        require(treasury != address(0), "MANAGER_ADDR not set");

        // Generate deterministic salt
        bytes32 salt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, block.chainid));

        // Deploy using CREATE2 for deterministic addressing
        SyndicateTokenCrosschain deployedTokenContract = new SyndicateTokenCrosschain{salt: salt}(admin, treasury);
        address deployedToken = address(deployedTokenContract);

        console.log("Deterministic salt used:", vm.toString(salt));

        console.log("SyndicateTokenCrosschain deployed at:", deployedToken);
        console.log("Admin address:", admin);
        console.log("Treasury address:", treasury);
        console.log("Salt used:", vm.toString(salt));

        // Verify token properties
        SyndicateTokenCrosschain token = SyndicateTokenCrosschain(deployedToken);
        console.log("Token name:", token.name());
        console.log("Token symbol:", token.symbol());
        console.log("Token decimals:", token.decimals());
        console.log("Initial supply:", token.totalSupply());

        vm.stopBroadcast();
    }
}

contract DeployTestnetSyndTokenCrosschainDirect is Script {
    function run() public {
        vm.startBroadcast();

        address admin = vm.envAddress("ADMIN_ADDR");
        address minter = vm.envAddress("MINTER_ADDR");

        require(admin != address(0), "ADMIN_ADDR not set");
        require(minter != address(0), "MINTER_ADDR not set");

        // Generate deterministic salt
        bytes32 salt = keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, block.chainid));

        // Deploy using CREATE2 for deterministic addressing
        TestnetSyndTokenCrosschain deployedTokenContract = new TestnetSyndTokenCrosschain{salt: salt}(admin, minter);
        address deployedToken = address(deployedTokenContract);

        console.log("Deterministic salt used:", vm.toString(salt));

        console.log("TestnetSyndTokenCrosschain deployed at:", deployedToken);
        console.log("Admin address:", admin);
        console.log("Minter address:", minter);
        console.log("Salt used:", vm.toString(salt));

        // Verify token properties
        TestnetSyndTokenCrosschain token = TestnetSyndTokenCrosschain(deployedToken);
        console.log("Token name:", token.name());
        console.log("Token symbol:", token.symbol());
        console.log("Token decimals:", token.decimals());
        console.log("Initial supply:", token.totalSupply());

        vm.stopBroadcast();
    }
}

contract VerifyCrosschainAddressConsistency is Script {
    function run() public pure {
        // Test configurations
        address admin = 0x1111111111111111111111111111111111111111;
        address treasury = 0x3333333333333333333333333333333333333333;
        address minter = 0x5555555555555555555555555555555555555555;

        bytes32 mainnetSalt = keccak256(abi.encodePacked("SYND_CROSSCHAIN", admin, treasury, uint256(1)));
        bytes32 testnetSalt = keccak256(abi.encodePacked("TESTNET_SYND_CROSSCHAIN", admin, minter, uint256(11155111)));

        console.log("Mainnet Salt:", vm.toString(mainnetSalt));
        console.log("Testnet Salt:", vm.toString(testnetSalt));
        console.log(
            "Using CREATE2 with same salt will produce consistent addresses when deployed from same deployer address"
        );
        console.log("Deploy on each chain with same environment variables to get consistent addresses");
    }
}
