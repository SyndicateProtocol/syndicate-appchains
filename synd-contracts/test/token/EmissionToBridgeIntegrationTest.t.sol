// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
import {ArbitrumBridgeProxy} from "src/token/bridges/ArbitrumBridgeProxy.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title EmissionToBridgeIntegrationTest
 * @notice Integration tests verifying the complete flow from emission minting to bridge execution
 * @dev Uses mock bridge contracts to test the integration without mainnet dependencies
 */
contract EmissionToBridgeIntegrationTest is Test {
    // Mock bridge contracts
    MockArbitrumBridge public mockArbitrumBridge;
    MockOptimismBridge public mockOptimismBridge;

    // Core contracts
    SyndicateToken public token;
    SyndicateTokenEmissionScheduler public emissionScheduler;
    ArbitrumBridgeProxy public arbitrumBridge;
    OptimismBridgeProxy public optimismBridge;

    // Test addresses
    address public admin = address(0x1234);
    address public syndFoundation = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public l2Recipient = address(0x1111);

    // Bridge configuration
    address public constant ARB_L2_TOKEN = address(0x2222);
    address public constant OP_L2_TOKEN = address(0x3333);
    uint256 public constant MAX_SINGLE_TRANSFER = 20_000_000 * 10 ** 18; // 20M tokens
    uint256 public constant DAILY_LIMIT = 100_000_000 * 10 ** 18; // 100M tokens
    uint32 public constant OP_L2_GAS = 200000;
    uint256 public constant ARB_MAX_GAS = 300000;
    uint256 public constant ARB_GAS_PRICE_BID = 1e9;

    // Events to track
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);

    function setUp() public {
        // Deploy mock bridges
        mockArbitrumBridge = new MockArbitrumBridge();
        mockOptimismBridge = new MockOptimismBridge();

        vm.startPrank(admin);

        // Deploy core contracts
        token = new SyndicateToken(admin, syndFoundation);
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), admin, emissionsManager, pauser);

        // Grant emission minter role
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));

        // Deploy bridge proxies with mock bridge targets
        arbitrumBridge = new ArbitrumBridgeProxy(
            admin,
            address(emissionScheduler),
            address(mockArbitrumBridge),
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            l2Recipient,
            ARB_MAX_GAS,
            ARB_GAS_PRICE_BID
        );

        optimismBridge = new OptimismBridgeProxy(
            admin,
            address(emissionScheduler),
            address(mockOptimismBridge),
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            OP_L2_TOKEN,
            l2Recipient,
            OP_L2_GAS
        );

        // set max single transfer limit
        arbitrumBridge.setMaxSingleTransfer(MAX_SINGLE_TRANSFER);
        optimismBridge.setMaxSingleTransfer(MAX_SINGLE_TRANSFER);

        vm.stopPrank();
    }

    // ============ ARBITRUM INTEGRATION TESTS ============

    function test_EmissionToBridge_Arbitrum_CompleteFlow() public {
        _setupArbitrumBridge();
        _startEmissions();

        // Fast forward to first epoch
        vm.warp(block.timestamp + 30 days + 1);

        // Fund emission scheduler with ETH for Arbitrum gas
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID;
        vm.deal(address(emissionScheduler), ethNeeded);

        // Fund mock bridge with ETH so it can accept payments
        vm.deal(address(mockArbitrumBridge), 1 ether);

        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        // Execute emission
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify emission state
        assertEq(token.totalSupply(), initialSupply + expectedAmount, "Tokens should be minted");
        assertEq(emissionScheduler.currentEpoch(), 1, "Should be at epoch 1");
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount, "Total emissions tracking");

        // Verify tokens were bridged
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "Scheduler should have no tokens");
        assertEq(token.balanceOf(address(mockArbitrumBridge)), expectedAmount, "Bridge should have tokens");

        // Verify bridge call details
        MockArbitrumBridge.TransferCall memory call = mockArbitrumBridge.getLastTransferCall();
        assertEq(call.token, address(token), "Correct token bridged");
        assertEq(call.amount, expectedAmount, "Correct amount bridged");
        assertEq(call.to, l2Recipient, "Correct L2 recipient");
        assertEq(call.maxGas, ARB_MAX_GAS, "Correct gas limit");
        assertEq(call.gasPriceBid, ARB_GAS_PRICE_BID, "Correct gas price");
        assertEq(call.ethValue, ethNeeded, "Correct ETH value sent");
    }

    function test_EmissionToBridge_Arbitrum_MultipleEpochs() public {
        _setupArbitrumBridge();
        _startEmissions();

        // Fast forward to allow 2 epochs (to stay under single transfer limit)
        vm.warp(block.timestamp + 60 days + 1);

        // Fund with sufficient ETH for 2 epochs
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID * 2;
        vm.deal(address(emissionScheduler), ethNeeded);
        vm.deal(address(mockArbitrumBridge), 1 ether);

        uint256 expectedTotal = 6_780_550 * 10 ** 18 * 2;
        uint256 initialSupply = token.totalSupply();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify 2 epochs were processed (but as single bridge call)
        assertEq(emissionScheduler.currentEpoch(), 2, "Should process 2 epochs");
        assertEq(token.totalSupply(), initialSupply + expectedTotal, "Should mint 2 epochs worth");
        assertEq(token.balanceOf(address(mockArbitrumBridge)), expectedTotal, "All tokens should be bridged");
        assertEq(mockArbitrumBridge.getTransferCallCount(), 1, "Should make 1 bridge call for all epochs");
    }

    // ============ OPTIMISM INTEGRATION TESTS ============

    function test_EmissionToBridge_Optimism_CompleteFlow() public {
        _setupOptimismBridge();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify emission and bridge state
        assertEq(token.totalSupply(), initialSupply + expectedAmount, "Tokens minted");
        assertEq(emissionScheduler.currentEpoch(), 1, "Epoch updated");
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "No tokens in scheduler");
        assertEq(token.balanceOf(address(mockOptimismBridge)), expectedAmount, "Tokens in bridge");

        // Verify bridge call
        MockOptimismBridge.BridgeCall memory call = mockOptimismBridge.getLastBridgeCall();
        assertEq(call.l1Token, address(token), "Correct L1 token");
        assertEq(call.l2Token, OP_L2_TOKEN, "Correct L2 token");
        assertEq(call.to, l2Recipient, "Correct recipient");
        assertEq(call.amount, expectedAmount, "Correct amount");
        assertEq(call.l2Gas, OP_L2_GAS, "Correct L2 gas");
    }

    // ============ BRIDGE FAILURE SCENARIOS ============

    function test_EmissionToBridge_ArbitrumFailure_RevertEntireOperation() public {
        _setupArbitrumBridge();
        _startEmissions();

        // Configure mock bridge to fail
        mockArbitrumBridge.setShouldRevert(true);

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);

        // Entire operation should revert if bridge fails
        vm.expectRevert("Arbitrum bridge failed");
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify no state changes occurred
        assertEq(emissionScheduler.currentEpoch(), 0, "No epoch should be processed");
        assertEq(emissionScheduler.totalEmissionsMinted(), 0, "No emissions should be recorded");
        assertEq(token.balanceOf(address(mockArbitrumBridge)), 0, "No tokens should be bridged");
    }

    function test_EmissionToBridge_OptimismFailure_RevertEntireOperation() public {
        _setupOptimismBridge();
        _startEmissions();

        mockOptimismBridge.setShouldRevert(true);

        vm.warp(block.timestamp + 30 days + 1);

        vm.expectRevert("Optimism bridge failed");
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify atomic failure
        assertEq(emissionScheduler.currentEpoch(), 0, "No progress on failure");
        assertEq(token.balanceOf(address(mockOptimismBridge)), 0, "No tokens transferred on failure");
    }

    // ============ INSUFFICIENT ETH SCENARIOS ============

    function test_EmissionToBridge_Arbitrum_InsufficientETH() public {
        _setupArbitrumBridge();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        // Don't fund with ETH - should fail
        vm.expectRevert();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
    }

    // ============ DYNAMIC CONFIGURATION TESTS ============

    function test_EmissionToBridge_Arbitrum_DynamicParameters() public {
        _setupArbitrumBridge();

        // Override bridge data with custom parameters
        address customRecipient = address(0x9999);
        uint256 customMaxGas = 500000;
        uint256 customGasPrice = 2e9;

        vm.prank(admin);
        emissionScheduler.setBridgeData(abi.encode(customRecipient, customMaxGas, customGasPrice));

        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 customEthNeeded = customMaxGas * customGasPrice;
        vm.deal(address(emissionScheduler), customEthNeeded);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify custom parameters were used
        MockArbitrumBridge.TransferCall memory call = mockArbitrumBridge.getLastTransferCall();
        assertEq(call.to, customRecipient, "Custom recipient used");
        assertEq(call.maxGas, customMaxGas, "Custom gas limit used");
        assertEq(call.gasPriceBid, customGasPrice, "Custom gas price used");
        assertEq(call.ethValue, customEthNeeded, "Custom ETH value calculated");
    }

    function test_EmissionToBridge_Optimism_DynamicParameters() public {
        _setupOptimismBridge();

        address customRecipient = address(0x8888);
        uint32 customGas = 150000;

        vm.prank(admin);
        emissionScheduler.setBridgeData(abi.encode(customRecipient, customGas));

        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        MockOptimismBridge.BridgeCall memory call = mockOptimismBridge.getLastBridgeCall();
        assertEq(call.to, customRecipient, "Custom recipient used");
        assertEq(call.l2Gas, customGas, "Custom gas used");
    }

    // ============ RATE LIMITING TESTS ============

    function test_EmissionToBridge_DailyLimitEnforcement() public {
        _setupArbitrumBridge();

        // Set low daily limit
        vm.prank(admin);
        arbitrumBridge.setDailyLimit(1_000_000 * 10 ** 18); // 1M tokens

        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);

        // Should fail due to daily limit (emission is ~6.7M, limit is 1M)
        vm.expectRevert();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
    }

    // ============ HELPER FUNCTIONS ============

    function _setupArbitrumBridge() internal {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();
    }

    function _setupOptimismBridge() internal {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(optimismBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, OP_L2_GAS));
        vm.stopPrank();
    }

    function _startEmissions() internal {
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();
    }
}

// ============ MOCK CONTRACTS ============

contract MockArbitrumBridge {
    struct TransferCall {
        address token;
        address refundTo;
        address to;
        uint256 amount;
        uint256 maxGas;
        uint256 gasPriceBid;
        bytes data;
        uint256 ethValue;
    }

    TransferCall[] public transferCalls;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function outboundTransfer(
        address _token,
        address _to,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes calldata _data
    ) external payable returns (bytes memory) {
        if (shouldRevert) {
            revert("Arbitrum bridge failed");
        }

        // Transfer tokens from caller
        IERC20(_token).transferFrom(msg.sender, address(this), _amount);

        // Record the call
        transferCalls.push(
            TransferCall({
                token: _token,
                refundTo: address(0), // Not used in outboundTransfer
                to: _to,
                amount: _amount,
                maxGas: _maxGas,
                gasPriceBid: _gasPriceBid,
                data: _data,
                ethValue: msg.value
            })
        );

        return "";
    }

    function getLastTransferCall() external view returns (TransferCall memory) {
        require(transferCalls.length > 0, "No transfer calls");
        return transferCalls[transferCalls.length - 1];
    }

    function getTransferCallCount() external view returns (uint256) {
        return transferCalls.length;
    }

    // Allow contract to receive ETH
    receive() external payable {}
}

contract MockOptimismBridge {
    struct BridgeCall {
        address l1Token;
        address l2Token;
        address to;
        uint256 amount;
        uint32 l2Gas;
        bytes data;
    }

    BridgeCall[] public bridgeCalls;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _l2Gas,
        bytes calldata _data
    ) external {
        if (shouldRevert) {
            revert("Optimism bridge failed");
        }

        // Transfer tokens from caller
        IERC20(_l1Token).transferFrom(msg.sender, address(this), _amount);

        // Record the call
        bridgeCalls.push(
            BridgeCall({
                l1Token: _l1Token,
                l2Token: _l2Token,
                to: _to,
                amount: _amount,
                l2Gas: _l2Gas,
                data: _data
            })
        );
    }

    function getLastBridgeCall() external view returns (BridgeCall memory) {
        require(bridgeCalls.length > 0, "No bridge calls");
        return bridgeCalls[bridgeCalls.length - 1];
    }

    function getBridgeCallCount() external view returns (uint256) {
        return bridgeCalls.length;
    }
}