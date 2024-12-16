// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AtomicSequencer, AtomicSequencerImplementation} from "src/atomic-sequencer/AtomicSequencer.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {PermissionModule} from "src/interfaces/PermissionModule.sol";

contract MockIsAllowed is PermissionModule {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address) external view override returns (bool) {
        return allowed;
    }
}

contract AtomicSequencerTest is Test {
    AtomicSequencer public atomicSequencer;
    MetabasedSequencerChain public chainA;
    MetabasedSequencerChain public chainB;
    RequireAllModule public permissionModule;
    address public admin;
    address public originalCaller;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        admin = address(0x1);
        originalCaller = address(0x2);
        uint256 l3ChainIdA = 10042001;
        uint256 l3ChainIdB = 10042002;

        vm.startPrank(admin);
        permissionModule = new RequireAllModule(admin);
        chainA = new MetabasedSequencerChain(l3ChainIdA, admin, address(permissionModule));
        chainB = new MetabasedSequencerChain(l3ChainIdB, admin, address(permissionModule));
        atomicSequencer = new AtomicSequencer();
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();
    }

    function testMsgSenderPreservedInSingleTransaction() public {
        bytes memory txnA = abi.encode("transaction A");
        bytes memory txnB = abi.encode("transaction B");

        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[] memory txns = new bytes[](2);
        txns[0] = txnA;
        txns[1] = txnB;

        bool[] memory isRawArray = new bool[](2);
        isRawArray[0] = true;
        isRawArray[1] = true;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRawArray);

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

        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[][] memory allTxns = new bytes[][](2);
        allTxns[0] = txnsA;
        allTxns[1] = txnsB;

        bytes memory callData =
            abi.encodeWithSignature("processBulkTransactionsAtomically(address[],bytes[][])", chains, allTxns);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing bulk transactions");
    }

    function testProcessMultipleChains() public {
        MetabasedSequencerChain chainC = new MetabasedSequencerChain(10042003, admin, address(permissionModule));

        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](3);
        chains[0] = chainA;
        chains[1] = chainB;
        chains[2] = chainC;

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction A");
        txns[1] = abi.encode("transaction B");
        txns[2] = abi.encode("transaction C");

        bool[] memory isRawArray = new bool[](3);
        isRawArray[0] = true;
        isRawArray[1] = false;
        isRawArray[2] = true;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRawArray);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing multiple transactions");
    }

    function testProcessSameChainMultipleTimes() public {
        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](2);
        chains[0] = chainA;
        chains[1] = chainA;

        bytes[] memory txns = new bytes[](2);
        txns[0] = abi.encode("transaction 1");
        txns[1] = abi.encode("transaction 2");

        bool[] memory isRawArray = new bool[](2);
        isRawArray[0] = true;
        isRawArray[1] = false;

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRawArray);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing same chain multiple times");
    }

    function testRevertOnInvalidCalls() public {
        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](1);
        chains[0] = chainA;

        bytes[] memory txns = new bytes[](1);
        txns[0] = abi.encode("transaction");

        bool[] memory isRawArray = new bool[](1);
        isRawArray[0] = true;

        bytes memory callData =
            abi.encodeWithSignature("WrongFunction(address[],bytes[],bool[])", chains, txns, isRawArray);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertFalse(success, "invalid function call should revert");
    }

    function testInputLengthMismatch() public {
        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](2);
        chains[0] = chainA;
        chains[1] = chainB;

        bytes[] memory txns = new bytes[](2);
        txns[0] = abi.encode("tx1");
        txns[1] = abi.encode("tx2");

        bool[] memory isRawArray = new bool[](1); // Mismatched length

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRawArray);

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
        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](3);
        chains[0] = chainA;
        chains[1] = chainB;
        chains[2] = chainA;

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("tx1");
        txns[1] = abi.encode("tx2");
        txns[2] = abi.encode("tx3");

        bool[] memory isRawArray = new bool[](3);
        isRawArray[0] = true; // Raw processing
        isRawArray[1] = false; // Standard processing
        isRawArray[2] = true; // Raw processing

        bytes memory callData =
            abi.encodeWithSignature("processTransactionsAtomically(address[],bytes[],bool[])", chains, txns, isRawArray);

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing mixed raw transactions");
    }
}
