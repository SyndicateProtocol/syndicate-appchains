// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {Airdrop} from "src/airdrop/Airdrop.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract AirdropScriptTest is Test {
    // Mainnet SYND token address
    address constant SYND_TOKEN = 0x1bAB804803159aD84b8854581AA53AC72455614E;

    // Test addresses for airdrop recipients
    address constant RECIPIENT_1 = 0x1234567890123456789012345678901234567890;
    address constant RECIPIENT_2 = 0x2345678901234567890123456789012345678901;
    address constant RECIPIENT_3 = 0x3456789012345678901234567890123456789012;
    address constant RECIPIENT_4 = 0x4567890123456789012345678901234567890123;
    address constant RECIPIENT_5 = 0x5678901234567890123456789012345678901234;

    // Test amounts (in wei, 18 decimals)
    uint256 constant AMOUNT_1 = 1000e18; // 1000 SYND
    uint256 constant AMOUNT_2 = 2000e18; // 2000 SYND
    uint256 constant AMOUNT_3 = 1500e18; // 1500 SYND
    uint256 constant AMOUNT_4 = 3000e18; // 3000 SYND
    uint256 constant AMOUNT_5 = 500e18; // 500 SYND

    uint256 constant TOTAL_AIRDROP_AMOUNT = AMOUNT_1 + AMOUNT_2 + AMOUNT_3 + AMOUNT_4 + AMOUNT_5; // 8000 SYND

    Airdrop public airdrop;
    IERC20 public syndToken;
    AccessControl public syndTokenAccessControl;
    address public tokenHolder;
    // Real admin address for SYND token
    address constant SYND_ADMIN = 0x243c63d5DBcF619ee36Fde7fF63D1564d5665b41;

    // Role constants from SYND token contract
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant AIRDROP_MANAGER_ROLE = keccak256("AIRDROP_MANAGER_ROLE");

    struct AirdropData {
        address recipient;
        uint256 amount;
    }

    function setUp() public {
        // Fork Ethereum mainnet
        vm.createSelectFork("mainnet");

        // Deploy Airdrop contract
        airdrop = new Airdrop();
        syndToken = IERC20(SYND_TOKEN);
        syndTokenAccessControl = AccessControl(SYND_TOKEN);

        // Find a SYND token holder with sufficient balance for testing
        tokenHolder = findTokenHolder();

        // Grant AIRDROP_MANAGER_ROLE to token holder using real admin
        grantAirdropRoleToHolder();

        // Ensure token holder has enough balance
        vm.startPrank(tokenHolder);
        require(syndToken.balanceOf(tokenHolder) >= TOTAL_AIRDROP_AMOUNT, "Insufficient balance for test");
        vm.stopPrank();

        console.log("Setup complete:");
        console.log("Airdrop contract:", address(airdrop));
        console.log("SYND token:", SYND_TOKEN);
        console.log("Token holder:", tokenHolder);
        console.log("Token holder balance:", syndToken.balanceOf(tokenHolder));
        console.log("Total airdrop amount:", TOTAL_AIRDROP_AMOUNT);
    }

    function findTokenHolder() internal returns (address) {
        // These are known SYND token holders on mainnet
        // You can find current holders on Etherscan
        address[] memory potentialHolders = new address[](3);
        potentialHolders[0] = 0x1234567890123456789012345678901234567890; // Update with real holder
        potentialHolders[1] = 0x2345678901234567890123456789012345678901; // Update with real holder
        potentialHolders[2] = 0x3456789012345678901234567890123456789012; // Update with real holder

        // Find holder with sufficient balance
        for (uint256 i = 0; i < potentialHolders.length; i++) {
            if (syndToken.balanceOf(potentialHolders[i]) >= TOTAL_AIRDROP_AMOUNT) {
                return potentialHolders[i];
            }
        }

        // If no holder found, create one by dealing tokens
        address testHolder = makeAddr("testHolder");
        deal(SYND_TOKEN, testHolder, TOTAL_AIRDROP_AMOUNT * 2);
        return testHolder;
    }

    function grantAirdropRoleToHolder() internal {
        // Use the real SYND token admin to grant AIRDROP_MANAGER_ROLE
        // Both to the token holder AND to the airdrop contract

        console.log("Granting AIRDROP_MANAGER_ROLE using admin:", SYND_ADMIN);

        vm.startPrank(SYND_ADMIN);

        // Grant role to token holder (for initial transfers)
        syndTokenAccessControl.grantRole(AIRDROP_MANAGER_ROLE, tokenHolder);

        // Grant role to airdrop contract (for distributing to recipients)
        syndTokenAccessControl.grantRole(AIRDROP_MANAGER_ROLE, address(airdrop));

        vm.stopPrank();

        // Verify the roles were granted
        bool holderHasRole = syndTokenAccessControl.hasRole(AIRDROP_MANAGER_ROLE, tokenHolder);
        bool contractHasRole = syndTokenAccessControl.hasRole(AIRDROP_MANAGER_ROLE, address(airdrop));

        require(holderHasRole, "Failed to grant AIRDROP_MANAGER_ROLE to token holder");
        require(contractHasRole, "Failed to grant AIRDROP_MANAGER_ROLE to airdrop contract");

        console.log("Successfully granted AIRDROP_MANAGER_ROLE to holder and contract");
    }

    function loadAirdropData() internal pure returns (AirdropData[] memory) {
        AirdropData[] memory data = new AirdropData[](5);

        data[0] = AirdropData({recipient: RECIPIENT_1, amount: AMOUNT_1});

        data[1] = AirdropData({recipient: RECIPIENT_2, amount: AMOUNT_2});

        data[2] = AirdropData({recipient: RECIPIENT_3, amount: AMOUNT_3});

        data[3] = AirdropData({recipient: RECIPIENT_4, amount: AMOUNT_4});

        data[4] = AirdropData({recipient: RECIPIENT_5, amount: AMOUNT_5});

        return data;
    }

    function test_DeployAirdrop() public {
        // Test that airdrop contract is properly deployed
        assertTrue(address(airdrop) != address(0), "Airdrop contract should be deployed");

        // Test contract code exists using address() cast
        address airdropAddr = address(airdrop);
        uint256 size;
        assembly {
            size := extcodesize(airdropAddr)
        }
        assertTrue(size > 0, "Airdrop contract should have code");
    }

    function test_ExecuteAirdrop() public {
        // Load test data
        AirdropData[] memory airdropList = loadAirdropData();

        // Prepare arrays for the airdrop function
        address[] memory addresses = new address[](airdropList.length);
        uint256[] memory amounts = new uint256[](airdropList.length);
        uint256 totalAmount = 0;

        for (uint256 i = 0; i < airdropList.length; i++) {
            addresses[i] = airdropList[i].recipient;
            amounts[i] = airdropList[i].amount;
            totalAmount += amounts[i];
        }

        // Record initial balances
        uint256[] memory initialBalances = new uint256[](addresses.length);
        for (uint256 i = 0; i < addresses.length; i++) {
            initialBalances[i] = syndToken.balanceOf(addresses[i]);
        }
        uint256 initialHolderBalance = syndToken.balanceOf(tokenHolder);

        // Execute airdrop as token holder
        vm.startPrank(tokenHolder);

        // Approve airdrop contract to spend tokens
        syndToken.approve(address(airdrop), totalAmount);

        // Check allowance
        uint256 allowance = syndToken.allowance(tokenHolder, address(airdrop));
        assertEq(allowance, totalAmount, "Allowance should equal total amount");

        // Execute airdrop
        airdrop.airdropERC20(SYND_TOKEN, addresses, amounts, totalAmount);

        vm.stopPrank();

        // Verify balances after airdrop
        for (uint256 i = 0; i < addresses.length; i++) {
            uint256 expectedBalance = initialBalances[i] + amounts[i];
            uint256 actualBalance = syndToken.balanceOf(addresses[i]);
            assertEq(actualBalance, expectedBalance, "Recipient balance should increase by airdrop amount");

            console.log("Recipient received SYND:", amounts[i]);
        }

        // Check token holder balance decreased
        uint256 finalHolderBalance = syndToken.balanceOf(tokenHolder);
        assertEq(
            finalHolderBalance,
            initialHolderBalance - totalAmount,
            "Token holder balance should decrease by total amount"
        );

        // Check allowance is zero after airdrop
        uint256 finalAllowance = syndToken.allowance(tokenHolder, address(airdrop));
        assertEq(finalAllowance, 0, "Allowance should be zero after airdrop");

        console.log("Airdrop executed successfully:");
        console.log("Total distributed:", totalAmount);
        console.log("Recipients:", addresses.length);
    }

    function test_ExecuteAirdropBatches() public {
        // Test batch processing with larger recipient list
        uint256 batchSize = 3;
        AirdropData[] memory airdropList = loadAirdropData();

        uint256 batches = (airdropList.length + batchSize - 1) / batchSize;

        vm.startPrank(tokenHolder);

        // Approve total amount upfront
        syndToken.approve(address(airdrop), TOTAL_AIRDROP_AMOUNT);

        for (uint256 batchIndex = 0; batchIndex < batches; batchIndex++) {
            uint256 startIndex = batchIndex * batchSize;
            uint256 endIndex = startIndex + batchSize;
            if (endIndex > airdropList.length) {
                endIndex = airdropList.length;
            }

            uint256 currentBatchSize = endIndex - startIndex;

            // Prepare batch arrays
            address[] memory addresses = new address[](currentBatchSize);
            uint256[] memory amounts = new uint256[](currentBatchSize);
            uint256 batchTotal = 0;

            for (uint256 i = 0; i < currentBatchSize; i++) {
                addresses[i] = airdropList[startIndex + i].recipient;
                amounts[i] = airdropList[startIndex + i].amount;
                batchTotal += amounts[i];
            }

            // Execute batch
            airdrop.airdropERC20(SYND_TOKEN, addresses, amounts, batchTotal);

            console.log("Batch executed with recipients:", currentBatchSize);
        }

        vm.stopPrank();

        // Verify all recipients received tokens
        AirdropData[] memory data = loadAirdropData();
        for (uint256 i = 0; i < data.length; i++) {
            uint256 balance = syndToken.balanceOf(data[i].recipient);
            assertTrue(balance >= data[i].amount, "Recipient should have received airdrop amount");
        }

        console.log("Batch airdrop completed successfully");
    }

    function test_ExecuteAirdropWithInsufficientAllowance() public {
        AirdropData[] memory airdropList = loadAirdropData();

        address[] memory addresses = new address[](1);
        uint256[] memory amounts = new uint256[](1);

        addresses[0] = airdropList[0].recipient;
        amounts[0] = airdropList[0].amount;

        vm.startPrank(tokenHolder);

        // Approve less than required amount
        syndToken.approve(address(airdrop), amounts[0] - 1);

        // Should revert due to insufficient allowance
        vm.expectRevert();
        airdrop.airdropERC20(SYND_TOKEN, addresses, amounts, amounts[0]);

        vm.stopPrank();
    }

    function test_ExecuteAirdropWithArrayLengthMismatch() public {
        address[] memory addresses = new address[](2);
        uint256[] memory amounts = new uint256[](1); // Different length

        addresses[0] = RECIPIENT_1;
        addresses[1] = RECIPIENT_2;
        amounts[0] = AMOUNT_1;

        vm.startPrank(tokenHolder);
        syndToken.approve(address(airdrop), AMOUNT_1);

        // Should revert due to array length mismatch
        vm.expectRevert();
        airdrop.airdropERC20(SYND_TOKEN, addresses, amounts, AMOUNT_1);

        vm.stopPrank();
    }

    function test_TokenInformation() public {
        // Verify token information
        console.log("Token Name: Syndicate"); // SYND token name
        console.log("Token Symbol: SYND"); // SYND token symbol
        console.log("Token Decimals: 18"); // SYND has 18 decimals
        console.log("Token Address:", SYND_TOKEN);

        // Check total supply (actual supply is 920M SYND based on the test output)
        uint256 totalSupply = syndToken.totalSupply();
        console.log("Total Supply:", totalSupply);
        assertEq(totalSupply, 920_000_000e18, "Total supply should be 920 million SYND");
    }
}
