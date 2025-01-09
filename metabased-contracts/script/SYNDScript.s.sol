// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script, console} from "forge-std/Script.sol";

import {SynGasToken, AccessControl} from "src/token/SynGasToken.sol";

contract GrantMinterRole is Script {
    AccessControl public synd;
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function run() public {
        vm.startBroadcast();

        require(block.chainid == 17000, "This script is for Holesky testnet");

        // add minter addresses below
        address[] memory minters = new address[](1);
        minters[0] = address(0); // CHANGE THIS

        synd = AccessControl(0x19aaf160dA8985c54bb97adAF9304B5aC7890421);

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

        require(block.chainid == 17000, "This script is for Holesky testnet");

        // add recipient addresses and SYND amount below
        address[] memory recipients = new address[](1);
        recipients[0] = address(0); // CHANGE THIS

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 0; // CHANGE THIS

        synd = SynGasToken(0x19aaf160dA8985c54bb97adAF9304B5aC7890421);

        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "Invalid address");

            synd.mint(recipients[i], amounts[i]);

            console.log("Minted", amounts[i], "SYND to", recipients[i]);
        }

        vm.stopBroadcast();
    }
}
