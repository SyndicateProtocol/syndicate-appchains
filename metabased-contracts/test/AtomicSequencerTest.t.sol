// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AtomicSequencer} from "src/AtomicSequencer.sol";
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

    function setUp() public {
        admin = address(0x1);
        uint256 l3ChainIdA = 10042001;
        uint256 l3ChainIdB = 10042002;

        vm.startPrank(admin);

        // Set up permission module and chains
        permissionModule = new RequireAllModule(admin);
        chainA = new MetabasedSequencerChain(l3ChainIdA, admin, address(permissionModule));
        chainB = new MetabasedSequencerChain(l3ChainIdB, admin, address(permissionModule));

        // Create atomic sequencer
        atomicSequencer = new AtomicSequencer();

        // Set up permissions
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);

        vm.stopPrank();
    }

    function testProcessTransactionsAtomicallyRaw() public {
        bytes memory txnA = abi.encode("transaction A");
        bytes memory txnB = abi.encode("transaction B");

        bytes32 expectedId = keccak256(abi.encodePacked(txnA, txnB));

        atomicSequencer.processTransactionsAtomically(
            chainA,
            chainB,
            txnA,
            txnB,
            true // isRaw
        );
    }

    function testProcessTransactionsAtomically() public {
        bytes memory txnA = abi.encode("transaction A");
        bytes memory txnB = abi.encode("transaction B");

        bytes32 expectedId = keccak256(abi.encodePacked(txnA, txnB));

        atomicSequencer.processTransactionsAtomically(
            chainA,
            chainB,
            txnA,
            txnB,
            false // not raw
        );
    }

    function testProcessBulkTransactionsAtomically() public {
        bytes[] memory txnsA = new bytes[](2);
        txnsA[0] = abi.encode("A1");
        txnsA[1] = abi.encode("A2");

        bytes[] memory txnsB = new bytes[](2);
        txnsB[0] = abi.encode("B1");
        txnsB[1] = abi.encode("B2");

        bytes32 expectedId = keccak256(abi.encodePacked(txnsA[0], txnsB[0]));

        atomicSequencer.processBulkTransactionsAtomically(chainA, chainB, txnsA, txnsB);
    }

    function testInvalidChainAddresses() public {
        bytes memory txn = abi.encode("transaction");

        // Test zero address for chainA
        vm.expectRevert(abi.encodeWithSelector(AtomicSequencer.InvalidChainAddresses.selector));
        atomicSequencer.processTransactionsAtomically(MetabasedSequencerChain(address(0)), chainB, txn, txn, true);

        // Test zero address for chainB
        vm.expectRevert(abi.encodeWithSelector(AtomicSequencer.InvalidChainAddresses.selector));
        atomicSequencer.processTransactionsAtomically(chainA, MetabasedSequencerChain(address(0)), txn, txn, true);

        // Test same address for both chains
        vm.expectRevert(abi.encodeWithSelector(AtomicSequencer.InvalidChainAddresses.selector));
        atomicSequencer.processTransactionsAtomically(chainA, chainA, txn, txn, true);
    }
}
