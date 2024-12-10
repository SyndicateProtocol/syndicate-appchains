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

        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address,address,bytes,bytes,bool)", chainA, chainB, txnA, txnB, true
        );

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

        bytes memory callData = abi.encodeWithSignature(
            "processBulkTransactionsAtomically(address,address,bytes[],bytes[])", chainA, chainB, txnsA, txnsB
        );

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing bulk transactions");
    }

    function testInvalidChainAddresses() public {
        bytes memory txn = abi.encode("transaction");

        // Test zero address for chainA
        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address,address,bytes,bytes,bool)", address(0), chainB, txn, txn, true
        );

        vm.prank(originalCaller);
        (bool success, bytes memory returnData) = address(atomicSequencer).call(callData);
        assertFalse(success);

        // Extract the error selector from the revert data
        bytes4 errorSelector;
        assembly {
            errorSelector := mload(add(returnData, 0x20))
        }
        assertEq(errorSelector, AtomicSequencerImplementation.InvalidChainAddresses.selector);

        // Test same address for both chains
        callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address,address,bytes,bytes,bool)", chainA, chainA, txn, txn, true
        );

        vm.prank(originalCaller);
        (success, returnData) = address(atomicSequencer).call(callData);
        assertFalse(success);

        assembly {
            errorSelector := mload(add(returnData, 0x20))
        }
        assertEq(errorSelector, AtomicSequencerImplementation.InvalidChainAddresses.selector);
    }
}
