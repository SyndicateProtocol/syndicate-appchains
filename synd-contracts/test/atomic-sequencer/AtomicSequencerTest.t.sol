// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {AtomicSequencer, AtomicSequencerImplementation} from "src/atomic-sequencer/AtomicSequencer.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MockIsAllowed is IPermissionModule {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address, address, bytes calldata) external view override returns (bool) {
        return allowed;
    }
}

contract AtomicSequencerTest is Test {
    AtomicSequencer public atomicSequencer;
    SyndicateSequencingChain public chainA;
    SyndicateSequencingChain public chainB;
    RequireAndModule public permissionModule;
    address public admin;
    address public originalCaller;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        // Warp to START_TIMESTAMP to avoid underflow in epoch calculations
        vm.warp(1754089200); // START_TIMESTAMP from EpochTracker.sol

        admin = address(0x1);
        originalCaller = address(0x2);
        uint256 appchainIdA = 10042001;
        uint256 appchainIdB = 10042002;

        vm.startPrank(admin);
        permissionModule = new RequireAndModule(admin);
        chainA = deployFromFactory(appchainIdA);
        chainB = deployFromFactory(appchainIdB);
        atomicSequencer = new AtomicSequencer();
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();
    }

    function deployFromFactory(uint256 appchainId) public returns (SyndicateSequencingChain) {
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        SyndicateFactory factory = SyndicateFactory(address(proxy));
        (address chainAddress,) = factory.createSyndicateSequencingChain(appchainId, admin, permissionModule);
        return SyndicateSequencingChain(chainAddress);
    }

    function testMsgSenderPreservedInSingleTransaction() public {
        bytes memory txnA = abi.encode("transaction A");
        bytes memory txnB = abi.encode("transaction B");

        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[] memory txns = new bytes[](2);
        txns[0] = txnA;
        txns[1] = txnB;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[])", chains, txns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing transactions");
    }

    function testMsgSenderPreservedInBulkTransactions() public {
        bytes[] memory txnsA = new bytes[](2);
        txnsA[0] = abi.encode("A1");
        txnsA[1] = abi.encode("A2");

        bytes[] memory txnsB = new bytes[](2);
        txnsB[0] = abi.encode("B1");
        txnsB[1] = abi.encode("B2");

        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[][] memory allTxns = new bytes[][](2);
        allTxns[0] = txnsA;
        allTxns[1] = txnsB;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsBulkAtomically(address[],bytes[][])", chains, allTxns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing bulk transactions");
    }

    function testProcessMultipleChains() public {
        SyndicateSequencingChain chainC = deployFromFactory(10042003);

        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](3);
        chains[0] = chainA;
        chains[1] = chainB;
        chains[2] = chainC;

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction A");
        txns[1] = abi.encode("transaction B");
        txns[2] = abi.encode("transaction C");

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[])", chains, txns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing multiple transactions");
    }

    function testProcessSameChainMultipleTimes() public {
        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](2);
        chains[0] = chainA;
        chains[1] = chainA;

        bytes[] memory txns = new bytes[](2);
        txns[0] = abi.encode("transaction 1");
        txns[1] = abi.encode("transaction 2");

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[])", chains, txns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing same chain multiple times");
    }

    function testConstructorDeployment() public {
        AtomicSequencer newSequencer = new AtomicSequencer();
        assertTrue(address(newSequencer.implementation()) != address(0), "Implementation should be set");
        assertTrue(address(newSequencer) != address(0), "Sequencer should be deployed");
    }

    function testRevertOnInvalidCalls() public {
        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](1);
        chains[0] = chainA;

        bytes[] memory txns = new bytes[](1);
        txns[0] = abi.encode("transaction");

        bytes memory callData = abi.encodeWithSignature("WrongFunction(address[],bytes[])", chains, txns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertFalse(success, "invalid function call should revert");
    }

    function testInputLengthMismatch() public {
        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[] memory txns = new bytes[](1);
        txns[0] = abi.encode("tx1");

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[])", chains, txns);

        vm.prank(originalCaller);
        (bool success, bytes memory returnData) = address(atomicSequencer).call(callData);
        assertFalse(success);

        bytes4 errorSelector;
        assembly {
            errorSelector := mload(add(returnData, 0x20))
        }
        assertEq(errorSelector, AtomicSequencerImplementation.InputLengthMismatchError.selector);
    }
}
