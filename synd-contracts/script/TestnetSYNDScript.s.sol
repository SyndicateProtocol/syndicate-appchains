// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";

import {TestnetSyndToken} from "src/token/TestnetSyndToken.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Holesky SYND address Testnet EXO
// address constant SYND_ADDRESS = 0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba;

// // Holesky SYND address Devnet
// address constant SYND_ADDRESS = 0x19aaf160dA8985c54bb97adAF9304B5aC7890421;

// // Sepolia SYND address Testnet Old
// address constant SYND_ADDRESS = 0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23;

// Sepolia SYND address Testnet RISA
address constant SYND_ADDRESS = 0xb80c82702791664f59dF773e8b50dE921fC026fE;

contract GrantMinterRole is Script {
    AccessControl public synd;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function run() public {
        vm.startBroadcast();

        // add minter addresses below
        address[] memory minters = new address[](4);
        minters[0] = 0x4E527486594696a7607Ff3379E21746689a3Fd6d;
        minters[1] = 0x37D911cBd7bB03521A975EC2dE03ce1dD0156883;
        minters[2] = 0x9c2F68B133286CFcc8677BD342bc724A0F2E2546;
        minters[3] = 0x18F33CEf45817C428d98C4E188A770191fDD4B79;

        synd = AccessControl(SYND_ADDRESS);

        for (uint256 i = 0; i < minters.length; i++) {
            require(minters[i] != address(0), "Invalid address");
            // require(!synd.hasRole(MINTER_ROLE, minters[i]), "MINTER_ROLE already granted to address");

            synd.grantRole(MINTER_ROLE, minters[i]);
            console.log("Granted MINTER_ROLE to", minters[i]);
        }

        vm.stopBroadcast();
    }
}

contract MintSYNDToAddresses is Script {
    TestnetSyndToken public synd;

    function run() public {
        vm.startBroadcast();

        // add recipient addresses and SYND amount below
        address[] memory recipients = new address[](4);
        recipients[0] = 0x19aaf160dA8985c54bb97adAF9304B5aC7890421;
        recipients[1] = 0x37D911cBd7bB03521A975EC2dE03ce1dD0156883;
        recipients[2] = 0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba;
        recipients[3] = 0x28fAb3A5b69711cc64B09240d2694d9F0f07eBf6; // alchemy

        uint256[] memory amounts = new uint256[](4);
        // 10K SYND
        amounts[0] = 200e18;
        amounts[1] = 200e18;
        amounts[2] = 200e18;
        amounts[3] = 200e18;

        synd = TestnetSyndToken(SYND_ADDRESS);

        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "Invalid address");

            synd.mint(recipients[i], amounts[i]);

            console.log("Minted", amounts[i], "SYND to", recipients[i]);
        }

        vm.stopBroadcast();
    }
}
