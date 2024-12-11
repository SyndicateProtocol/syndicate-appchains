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

        // Create arrays for chains and transactions
        address[] memory chainAddresses = new address[](2);
        chainAddresses[0] = address(chainA);
        chainAddresses[1] = address(chainB);

        bytes[] memory txns = new bytes[](2);
        txns[0] = txnA;
        txns[1] = txnB;

        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address[],bytes[],bool)",
            chainAddresses,
            txns,
            true
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

        // Create arrays for chains and transactions
        address[] memory chainAddresses = new address[](2);
        chainAddresses[0] = address(chainA);
        chainAddresses[1] = address(chainB);

        bytes[][] memory allTxns = new bytes[][](2);
        allTxns[0] = txnsA;
        allTxns[1] = txnsB;

        bytes memory callData = abi.encodeWithSignature(
            "processBulkTransactionsAtomically(address[],bytes[][])",
            chainAddresses,
            allTxns
        );

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

        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address[],bytes[],bool)",
            chains,
            txns,
            true
        );

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

        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address[],bytes[],bool)",
            chains,
            txns,
            true
        );

        vm.prank(originalCaller);
        (bool success,) = address(atomicSequencer).call(callData);
        assertTrue(success, "failure in processing same chain multiple times");
    }

    function testInvalidChainAddresses() public {
        MetabasedSequencerChain[] memory chains = new MetabasedSequencerChain[](1);
        chains[0] = MetabasedSequencerChain(address(0));

        bytes[] memory txns = new bytes[](1);
        txns[0] = abi.encode("transaction");

        bytes memory callData = abi.encodeWithSignature(
            "processTransactionsAtomically(address[],bytes[],bool)",
            chains,
            txns,
            true
        );

        vm.prank(originalCaller);
        (bool success, bytes memory returnData) = address(atomicSequencer).call(callData);
        assertFalse(success);

        bytes4 errorSelector;
        assembly {
            errorSelector := mload(add(returnData, 0x20))
        }
        assertEq(errorSelector, AtomicSequencerImplementation.InvalidChainAddresses.selector);
    }
}
