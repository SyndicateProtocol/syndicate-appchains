// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {Airdrop} from "src/airdrop/Airdrop.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract ExecuteAirdrop is Script {
    // Update these addresses for your deployment
    address constant AIRDROP_CONTRACT = address(0); // Deploy Airdrop contract first
    address constant TOKEN_ADDRESS = address(0); // ERC20 token to airdrop

    // Maximum recipients per batch to avoid gas limits
    uint256 constant MAX_BATCH_SIZE = 200;

    struct AirdropData {
        address recipient;
        uint256 amount;
    }

    function run() public {
        require(AIRDROP_CONTRACT != address(0), "Update AIRDROP_CONTRACT address");
        require(TOKEN_ADDRESS != address(0), "Update TOKEN_ADDRESS");

        vm.startBroadcast();

        // Load airdrop data from CSV
        AirdropData[] memory airdropList = loadAirdropData();

        // Execute airdrop in batches
        executeAirdropBatches(airdropList);

        vm.stopBroadcast();
    }

    function loadAirdropData() internal pure returns (AirdropData[] memory) {
        // TODO: Replace with your CSV data
        // Format: address,amount (in wei)
        // Example CSV content:
        // 0x1234567890123456789012345678901234567890,1000000000000000000
        // 0x2345678901234567890123456789012345678901,2000000000000000000

        AirdropData[] memory data = new AirdropData[](2);

        // Example data - replace with actual addresses and amounts
        data[0] = AirdropData({
            recipient: 0x1234567890123456789012345678901234567890,
            amount: 1000e18 // 1000 tokens
        });

        data[1] = AirdropData({
            recipient: 0x2345678901234567890123456789012345678901,
            amount: 2000e18 // 2000 tokens
        });

        return data;
    }

    function executeAirdropBatches(AirdropData[] memory airdropList) internal {
        uint256 totalRecipients = airdropList.length;
        uint256 batches = (totalRecipients + MAX_BATCH_SIZE - 1) / MAX_BATCH_SIZE;

        console.log("Total recipients:", totalRecipients);
        console.log("Processing in", batches, "batches");

        IERC20 token = IERC20(TOKEN_ADDRESS);
        uint256 skippedAddresses = 0;

        for (uint256 batchIndex = 0; batchIndex < batches; batchIndex++) {
            uint256 startIndex = batchIndex * MAX_BATCH_SIZE;
            uint256 endIndex = startIndex + MAX_BATCH_SIZE;
            if (endIndex > totalRecipients) {
                endIndex = totalRecipients;
            }

            // First pass: filter out addresses with existing balances
            AirdropData[] memory filteredBatch = new AirdropData[](endIndex - startIndex);
            uint256 validCount = 0;

            for (uint256 i = startIndex; i < endIndex; i++) {
                address recipient = airdropList[i].recipient;
                uint256 currentBalance = token.balanceOf(recipient);

                if (currentBalance > 0) {
                    console.log("SKIPPING - Address already has SYND tokens:");
                    console.log("Address:", recipient);
                    console.log("Current balance:", currentBalance);
                    skippedAddresses++;
                } else {
                    filteredBatch[validCount] = airdropList[i];
                    validCount++;
                }
            }

            // Skip batch if no valid recipients
            if (validCount == 0) {
                console.log("Batch", batchIndex + 1, "skipped - no valid recipients");
                continue;
            }

            // Prepare batch arrays with only valid recipients
            address[] memory addresses = new address[](validCount);
            uint256[] memory amounts = new uint256[](validCount);
            uint256 totalAmount = 0;

            for (uint256 i = 0; i < validCount; i++) {
                addresses[i] = filteredBatch[i].recipient;
                amounts[i] = filteredBatch[i].amount;
                totalAmount += amounts[i];
            }

            // Check allowance
            uint256 allowance = token.allowance(msg.sender, AIRDROP_CONTRACT);
            require(allowance >= totalAmount, "Insufficient token allowance");

            // Execute airdrop batch
            console.log("Executing batch", batchIndex + 1, "of", batches);
            console.log("Valid recipients in batch:", validCount);
            console.log("Total amount:", totalAmount);

            Airdrop(AIRDROP_CONTRACT).airdropERC20(TOKEN_ADDRESS, addresses, amounts, totalAmount);

            console.log("Batch", batchIndex + 1, "completed successfully");
        }

        console.log("All batches completed successfully!");
        console.log("Total addresses skipped due to existing balance:", skippedAddresses);
    }
}

contract DeployAirdrop is Script {
    function run() public {
        vm.startBroadcast();

        Airdrop airdrop = new Airdrop();

        console.log("Airdrop contract deployed at:", address(airdrop));

        vm.stopBroadcast();
    }
}
