// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.29;

import {Test} from "forge-std/Test.sol";
import {AtomicSequencer, AtomicSequencerImplementation} from "src/atomic-sequencer/AtomicSequencer.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

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
    RequireAllModule public permissionModule;
    address public admin;
    address public originalCaller;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        admin = address(0x1);
        originalCaller = address(0x2);
        uint256 appChainIdA = 10042001;
        uint256 appChainIdB = 10042002;

        vm.startPrank(admin);
        permissionModule = new RequireAllModule(admin);
        chainA = new SyndicateSequencingChain(appChainIdA);
        chainA.initialize(admin, address(permissionModule));
        chainB = new SyndicateSequencingChain(appChainIdB);
        chainB.initialize(admin, address(permissionModule));
        atomicSequencer = new AtomicSequencer();
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();
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

        bool[] memory isRaw = new bool[](2);
        isRaw[0] = true;
        isRaw[1] = true;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRaw);

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
        SyndicateSequencingChain chainC = new SyndicateSequencingChain(10042003);
        chainC.initialize(admin, address(permissionModule));

        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](3);
        chains[0] = chainA;
        chains[1] = chainB;
        chains[2] = chainC;

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction A");
        txns[1] = abi.encode("transaction B");
        txns[2] = abi.encode("transaction C");

        bool[] memory isRaw = new bool[](3);
        isRaw[0] = true;
        isRaw[1] = false;
        isRaw[2] = true;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRaw);

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

        bool[] memory isRaw = new bool[](2);
        isRaw[0] = true;
        isRaw[1] = false;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRaw);

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

        bool[] memory isRaw = new bool[](1);
        isRaw[0] = true;

        bytes memory callData = abi.encodeWithSignature("WrongFunction(address[],bytes[],bool[])", chains, txns, isRaw);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertFalse(success, "invalid function call should revert");
    }

    function testInputLengthMismatch() public {
        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[] memory txns = new bytes[](2);
        txns[0] = abi.encode("tx1");
        txns[1] = abi.encode("tx2");

        bool[] memory isRaw = new bool[](1); // Mismatched length

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRaw);

        vm.prank(originalCaller);
        (bool success, bytes memory returnData) = address(atomicSequencer).call(callData);
        assertFalse(success);

        bytes4 errorSelector;
        assembly {
            errorSelector := mload(add(returnData, 0x20))
        }
        assertEq(errorSelector, AtomicSequencerImplementation.InputLengthMismatchError.selector);
    }

    function testMixedRawProcessing() public {
        SyndicateSequencingChain[] memory chains = new SyndicateSequencingChain[](3);
        chains[0] = chainA;
        chains[1] = chainB;
        chains[2] = chainA;

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("tx1");
        txns[1] = abi.encode("tx2");
        txns[2] = abi.encode("tx3");

        bool[] memory isRaw = new bool[](3);
        isRaw[0] = true; // Raw processing
        isRaw[1] = false; // Standard processing
        isRaw[2] = true; // Raw processing

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRaw);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing mixed raw transactions");
    }
}
