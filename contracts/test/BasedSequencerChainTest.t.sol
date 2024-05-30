// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {BasedSequencerChain, RequireListManager} from "src/BasedSequencerChain.sol";
import {IsAllowed} from "src/interfaces/IsAllowed.sol";
import {Test} from "forge-std/Test.sol";
import {EMPTY_PARENT_HASH, INVALID_PARENT_HASH} from "./utils/Constants.sol";

contract MockIsAllowed is IsAllowed {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address) external view override returns (bool) {
        return allowed;
    }
}

contract BasedSequencerChainTestSetUp is Test {
    BasedSequencerChain public chain;
    address public admin;

    function setUp() public virtual {
        admin = address(0x1);

        vm.startPrank(admin); // Prank the admin to allow contract creation
        chain = new BasedSequencerChain();
        vm.stopPrank();
    }
}

contract BasedSequencerChainTest is BasedSequencerChainTestSetUp {
    function testInitialEpochNumber() public view {
        uint256 expectedEpochNumber = block.timestamp / chain.EPOCH_LENGTH();
        assertEq(chain.INITIAL_EPOCH_NUMBER(), expectedEpochNumber);
    }

    function testEpochLength() public view {
        assertEq(chain.EPOCH_LENGTH(), 1);
    }

    function testMaxBidListSize() public view {
        assertEq(chain.MAX_BID_LIST_SIZE(), 5);
    }

    function testCalculateEpochNumber() public view {
        uint256 timestamp = 1625097600; // Example timestamp
        uint256 expectedEpochNumber = timestamp / chain.EPOCH_LENGTH();
        assertEq(chain.calculateEpochNumber(timestamp), expectedEpochNumber);
    }

    function testCalculateCurrentEpochNumber() public view {
        uint256 currentTimestamp = block.timestamp;
        uint256 expectedEpochNumber = currentTimestamp / chain.EPOCH_LENGTH();
        assertEq(chain.calculateCurrentEpochNumber(), expectedEpochNumber);
    }

    function testCheckParentHash() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: new bytes[](0)
        });

        chain.sequenceNextBatch(userProvidedBatch);

        (,,, bytes32 lastNonEmptyEpochHash) = chain.batches(chain.lastNonEmptyEpochNumber());
        assertTrue(chain.checkParentHash(lastNonEmptyEpochHash), "Parent hash should match");
    }
}

contract BasedSequencerChainBatchTest is BasedSequencerChainTestSetUp {
    function testSequenceNextBatch() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: new bytes[](0)
        });

        chain.sequenceNextBatch(userProvidedBatch);

        assertEq(chain.lastNonEmptyEpochNumber(), chain.INITIAL_EPOCH_NUMBER());
    }

    function testSequenceNextBatchInvalidParentHash() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: INVALID_PARENT_HASH,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                BasedSequencerChain.ParentHashDoesNotMatch.selector, EMPTY_PARENT_HASH, INVALID_PARENT_HASH
            )
        );
        chain.sequenceNextBatch(userProvidedBatch);
    }

    function testSequenceNextBatchRequireAllFailure() public {
        address mockRequireAny = address(new MockIsAllowed(false));
        vm.startPrank(admin);
        chain.addRequireAllCheck(mockRequireAny, false);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                RequireListManager.RequireAllCheckFailed.selector, address(mockRequireAny), address(this)
            )
        );
        chain.sequenceNextBatch(userProvidedBatch);
    }

    function testSequenceNextBatchRequireAnyFailure() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(abi.encodeWithSelector(RequireListManager.RequireAnyCheckFailed.selector, address(this)));
        chain.sequenceNextBatch(userProvidedBatch);
    }
}

contract BasedSequencerChainViewTest is BasedSequencerChainTestSetUp {
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);
        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAllCheck(address(mockRequireAll1), false);
        chain.addRequireAllCheck(address(mockRequireAll2), false);
        chain.addRequireAnyCheck(address(mockRequireAny1), false);
        chain.addRequireAnyCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = chain.getAllRequirements(true);
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = chain.getAllRequirements(false);
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
