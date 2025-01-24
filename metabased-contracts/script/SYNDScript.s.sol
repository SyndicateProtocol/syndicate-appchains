// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script, console} from "forge-std/Script.sol";

import {SynGasToken, AccessControl} from "src/token/SynGasToken.sol";

// Holesky SYND address
address constant SYND_ADDRESS = 0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba;

contract GrantMinterRole is Script {
    AccessControl public synd;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function run() public {
        vm.startBroadcast();

        // add minter addresses below
        address[] memory minters = new address[](2);
        minters[0] = 0x4E527486594696a7607Ff3379E21746689a3Fd6d;
        minters[1] = 0x37D911cBd7bB03521A975EC2dE03ce1dD0156883;

        synd = AccessControl(SYND_ADDRESS);

        for (uint256 i = 0; i < minters.length; i++) {
            require(minters[i] != address(0), "Invalid address");
            require(!synd.hasRole(MINTER_ROLE, minters[i]), "MINTER_ROLE already granted to address");

            synd.grantRole(MINTER_ROLE, minters[i]);
            console.log("Granted MINTER_ROLE to", minters[i]);
        }

        vm.stopBroadcast();
    }
}

contract MintSYNDToAddresses is Script {
    SynGasToken public synd;

    function run() public {
        vm.startBroadcast();

        // add recipient addresses and SYND amount below
        address[] memory recipients = new address[](1);
        recipients[0] = 0x28fAb3A5b69711cc64B09240d2694d9F0f07eBf6;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 10_000e18; // 10K SYND

        synd = SynGasToken(SYND_ADDRESS);

        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "Invalid address");

            synd.mint(recipients[i], amounts[i]);

            console.log("Minted", amounts[i], "SYND to", recipients[i]);
        }

        vm.stopBroadcast();
    }
}
