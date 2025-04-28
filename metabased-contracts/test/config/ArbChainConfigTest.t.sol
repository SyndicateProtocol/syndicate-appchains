// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {ArbChainConfig, Initializable} from "src/config/ArbChainConfig.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract ArbChainConfigTest is Test {
    ArbChainConfig public chainConfig;
    address public owner = address(1);
    address public rollupOwner = address(2);
    address public newRollupOwner = address(3);

    uint256 public constant CHAIN_ID = 123456;
    uint256 public constant SEQUENCING_CHAIN_ID = 654321;
    address public constant ARBITRUM_BRIDGE_ADDRESS = address(0x1234);
    address public constant ARBITRUM_INBOX_ADDRESS = address(0x5678);
    bool public constant ARBITRUM_IGNORE_DELAYED_MESSAGES = false;
    uint256 public constant SETTLEMENT_DELAY = 10;
    uint256 public constant SETTLEMENT_START_BLOCK = 100;
    address public constant SEQUENCING_CONTRACT_ADDRESS = address(0x9ABC);
    uint256 public constant SEQUENCING_START_BLOCK = 200;
    string public constant DEFAULT_RPC_URL = "https://example.com/rpc";
    string public constant APPCHAIN_BLOCK_EXPLORER_URL = "https://example.com/explorer";

    function setUp() public {
        vm.startPrank(owner);
        // Create a new ArbChainConfig and initialize it
        chainConfig = new ArbChainConfig();
        chainConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );
        vm.stopPrank();
    }

    function testImmutableValues() public view {
        assertEq(chainConfig.CHAIN_ID(), CHAIN_ID);
        assertEq(chainConfig.SEQUENCING_CHAIN_ID(), SEQUENCING_CHAIN_ID);
        assertEq(chainConfig.ARBITRUM_BRIDGE_ADDRESS(), ARBITRUM_BRIDGE_ADDRESS);
        assertEq(chainConfig.ARBITRUM_INBOX_ADDRESS(), ARBITRUM_INBOX_ADDRESS);
        assertEq(chainConfig.ARBITRUM_IGNORE_DELAYED_MESSAGES(), ARBITRUM_IGNORE_DELAYED_MESSAGES);
        assertEq(chainConfig.SETTLEMENT_DELAY(), SETTLEMENT_DELAY);
        assertEq(chainConfig.SETTLEMENT_START_BLOCK(), SETTLEMENT_START_BLOCK);
        assertEq(chainConfig.SEQUENCING_CONTRACT_ADDRESS(), SEQUENCING_CONTRACT_ADDRESS);
        assertEq(chainConfig.SEQUENCING_START_BLOCK(), SEQUENCING_START_BLOCK);
    }

    function testInitialMutableValues() public view {
        assertEq(chainConfig.ROLLUP_OWNER(), rollupOwner);
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), DEFAULT_RPC_URL);
        assertEq(chainConfig.APPCHAIN_BLOCK_EXPLORER_URL(), APPCHAIN_BLOCK_EXPLORER_URL);
    }

    function testConstructorRequirements() public {
        vm.startPrank(owner);

        ArbChainConfig newConfig = new ArbChainConfig();

        // Test chain ID requirement
        vm.expectRevert("Chain ID cannot be zero");
        newConfig.initialize(
            owner,
            0, // Zero chain ID
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        // Test Arbitrum bridge address requirement
        newConfig = new ArbChainConfig();
        vm.expectRevert("Arbitrum bridge address cannot be zero");
        newConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            address(0), // Zero address
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        // Test Arbitrum inbox address requirement
        newConfig = new ArbChainConfig();
        vm.expectRevert("Arbitrum inbox address cannot be zero");
        newConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            address(0), // Zero address
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        // Test sequencing contract address requirement
        newConfig = new ArbChainConfig();
        vm.expectRevert("Sequencing contract address cannot be zero");
        newConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            address(0), // Zero address
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        // Test rollup owner requirement
        newConfig = new ArbChainConfig();
        vm.expectRevert("Rollup owner cannot be zero address");
        newConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            address(0), // Zero address
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        vm.stopPrank();
    }

    function testCannotReinitialize() public {
        vm.startPrank(owner);

        // Try to reinitialize the same contract
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        chainConfig.initialize(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL
        );

        vm.stopPrank();
    }

    function testUpdateRollupOwner() public {
        vm.startPrank(owner);

        vm.expectEmit(true, false, false, true);
        emit ArbChainConfig.RollupOwnerUpdated(newRollupOwner);

        chainConfig.updateRollupOwner(newRollupOwner);
        assertEq(chainConfig.ROLLUP_OWNER(), newRollupOwner);

        vm.stopPrank();
    }

    function testUpdateRollupOwnerRevertOnZeroAddress() public {
        vm.startPrank(owner);

        vm.expectRevert("New rollup owner cannot be zero address");
        chainConfig.updateRollupOwner(address(0));

        vm.stopPrank();
    }

    function testUpdateRollupOwnerRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        chainConfig.updateRollupOwner(newRollupOwner);
    }

    function testUpdateDefaultSequencingChainRpcUrl() public {
        vm.startPrank(owner);

        string memory newRpcUrl = "https://new-example.com/rpc";

        vm.expectEmit(true, false, false, true);
        emit ArbChainConfig.DefaultSequencingChainRpcUrlUpdated(newRpcUrl);

        chainConfig.updateDefaultSequencingChainRpcUrl(newRpcUrl);
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), newRpcUrl);

        vm.stopPrank();
    }

    function testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        chainConfig.updateDefaultSequencingChainRpcUrl("https://new-example.com/rpc");
    }

    function testEmptyDefaultRpcUrl() public {
        vm.startPrank(owner);

        // Test with empty RPC URL
        chainConfig.updateDefaultSequencingChainRpcUrl("");
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), "");

        vm.stopPrank();
    }
}
