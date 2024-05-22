// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {BasedSequencerChain, IsAllowed} from "src/BasedSequencerChain.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowed is IsAllowed {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed() external view override returns (bool) {
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
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)));
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,
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
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)));
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,
            transaction_list: new bytes[](0)
        });

        chain.sequenceNextBatch(userProvidedBatch);

        assertEq(chain.lastNonEmptyEpochNumber(), chain.INITIAL_EPOCH_NUMBER());
    }

    function testSequenceNextBatchInvalidParentHash() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)));
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: 0x1111111111111111111111111111111111111111111111111111111111111111,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                BasedSequencerChain.ParentHashDoesNotMatch.selector,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x1111111111111111111111111111111111111111111111111111111111111111
            )
        );
        chain.sequenceNextBatch(userProvidedBatch);
    }

    function testSequenceNextBatchRequireAllFailure() public {
        address mockRequireAny = address(new MockIsAllowed(false));
        vm.startPrank(admin);
        chain.addRequireAllCheck(mockRequireAny);
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                BasedSequencerChain.RequireAllCheckFailed.selector, address(mockRequireAny), address(this)
            )
        );
        chain.sequenceNextBatch(userProvidedBatch);
    }

    function testSequenceNextBatchRequireAnyFailure() public {
        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(false)));
        vm.stopPrank();

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,
            transaction_list: new bytes[](0)
        });

        vm.expectRevert(abi.encodeWithSelector(BasedSequencerChain.RequireAnyCheckFailed.selector, address(this)));
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
        chain.addRequireAllCheck(address(mockRequireAll1));
        chain.addRequireAllCheck(address(mockRequireAll2));
        chain.addRequireAnyCheck(address(mockRequireAny1));
        chain.addRequireAnyCheck(address(mockRequireAny2));
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

contract BasedSequencerChainAdminFunctions is BasedSequencerChainTestSetUp {
    function testAddRequireAllCheck() public {
        MockIsAllowed mockRequireAll = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAllCheck(address(mockRequireAll));
        vm.stopPrank();

        address[] memory allChecks = chain.getAllRequirements(true);
        assertEq(allChecks.length, 1);
        assertEq(allChecks[0], address(mockRequireAll));
    }

    function testAddRequireAnyCheck() public {
        MockIsAllowed mockRequireAny = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(mockRequireAny));
        vm.stopPrank();

        address[] memory allChecks = chain.getAllRequirements(false);
        assertEq(allChecks.length, 1);
        assertEq(allChecks[0], address(mockRequireAny));
    }

    function testRemoveRequireAllCheck() public {
        MockIsAllowed mockRequireAll = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAllCheck(address(mockRequireAll));
        vm.stopPrank();

        address[] memory allChecks = chain.getAllRequirements(true);
        assertEq(allChecks.length, 1);
        assertEq(allChecks[0], address(mockRequireAll));

        vm.startPrank(admin);
        chain.removeRequireAllCheck(address(mockRequireAll));
        vm.stopPrank();

        allChecks = chain.getAllRequirements(true);
        assertEq(allChecks.length, 0);
    }

    function testRemoveRequireAnyCheck() public {
        MockIsAllowed mockRequireAny = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(mockRequireAny));
        vm.stopPrank();

        address[] memory allChecks = chain.getAllRequirements(false);
        assertEq(allChecks.length, 1);
        assertEq(allChecks[0], address(mockRequireAny));

        vm.startPrank(admin);
        chain.removeRequireAnyCheck(address(mockRequireAny));
        vm.stopPrank();

        allChecks = chain.getAllRequirements(false);
        assertEq(allChecks.length, 0);
    }
}
