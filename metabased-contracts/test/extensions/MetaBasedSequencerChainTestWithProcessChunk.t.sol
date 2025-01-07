// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {
    MetabasedSequencerChainWithProcessChunk,
    MetabasedSequencerChain
} from "src/extensions/MetabasedSequencerChainWithProcessChunk.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";
import {PermissionModule} from "src/interfaces/PermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowed is PermissionModule {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address) external view override returns (bool) {
        return allowed;
    }
}

contract MetabasedSequencerChainWithProcessChunkTestSetUp is Test {
    MetabasedSequencerChainWithProcessChunk public chain;
    RequireAllModule public permissionModule;
    RequireAnyModule public permissionModuleAny;
    address public admin;

    function setUp() public virtual {
        admin = address(0x1);
        uint256 l3ChainId = 10042001;

        vm.startPrank(admin);
        permissionModule = new RequireAllModule(admin);
        permissionModuleAny = new RequireAnyModule(admin);
        chain = new MetabasedSequencerChainWithProcessChunk(l3ChainId, admin, address(permissionModule));
        vm.stopPrank();
    }
}

contract MetabasedSequencerChainWithProcessChunkTest is MetabasedSequencerChainWithProcessChunkTestSetUp {
    function testProcessFirstChunk() public {
        bytes memory chunk = abi.encode("chunk1");
        bytes32 txHash = bytes32(keccak256(chunk));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChain.TransactionProcessed(address(this), chunk);

        vm.expectEmit(true, true, true, true);
        emit MetabasedSequencerChainWithProcessChunk.TransactionChunkProcessed(chunk, 0, 3, txHash);

        chain.processChunk(chunk, 0, 3, txHash);

        // Verify chunk state
        bytes32 txHashProcessId = keccak256(abi.encode(txHash, address(this), tx.origin));
        (uint256 chunkMask, uint256 totalChunks, address owner, bool isComplete) = chain.chunkStates(txHashProcessId);

        assertEq(owner, address(this));
        assertEq(chunkMask, 1); // First bit set (2^0 = 1)
        assertEq(totalChunks, 3);
        assertFalse(isComplete);
    }

    function testProcessChunksOutOfOrder() public {
        bytes memory chunk1 = abi.encode("chunk1");
        bytes memory chunk2 = abi.encode("chunk2");
        bytes memory chunk3 = abi.encode("chunk3");
        bytes32 txHash = bytes32(keccak256(abi.encodePacked(chunk1, chunk2, chunk3)));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Process chunks out of order: 2 -> 0
        chain.processChunk(chunk3, 2, 3, txHash); // Sets bit 2 (0b0100 = 4)
        chain.processChunk(chunk1, 0, 3, txHash); // Sets bit 0 (0b0001 = 1)

        // Verify intermediate state
        bytes32 txHashProcessId = keccak256(abi.encode(txHash, address(this), tx.origin));
        (uint256 chunkMask,,, bool isComplete) = chain.chunkStates(txHashProcessId);

        assertEq(chunkMask, 5); // Should be 0b0101 (bits 0 and 2 set = 1 + 4 = 5)
        assertFalse(isComplete);

        // Process final chunk and expect completion event
        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChainWithProcessChunk.ChunkProcessingCompleted(txHash);

        chain.processChunk(chunk2, 1, 3, txHash); // Sets bit 1 (0b0010 = 2)

        // Verify final state
        (chunkMask,,, isComplete) = chain.chunkStates(txHashProcessId);
        assertEq(chunkMask, 7); // Should be 0b0111 (all bits set = 1 + 2 + 4 = 7)
        assertTrue(isComplete);
    }

    function testProcessChunkInvalidConfigs() public {
        bytes memory chunk = abi.encode("chunk");
        bytes32 txHash = bytes32(keccak256(chunk));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Test zero chunks
        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChainWithProcessChunk.InvalidChunkConfig.selector));
        chain.processChunk(chunk, 0, 0, txHash);

        // Test too many chunks (> 256)
        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChainWithProcessChunk.InvalidChunkConfig.selector));
        chain.processChunk(chunk, 0, 257, txHash);

        // Test invalid index
        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChainWithProcessChunk.InvalidChunkIndex.selector));
        chain.processChunk(chunk, 5, 3, txHash);
    }

    function testIncompleteChunks() public {
        bytes memory chunk = abi.encode("chunk");
        bytes32 txHash = bytes32(keccak256(chunk));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Process first and third chunks of a 4-chunk sequence
        chain.processChunk(chunk, 0, 4, txHash);
        chain.processChunk(chunk, 2, 4, txHash);

        // Verify state shows incomplete
        bytes32 txHashProcessId = keccak256(abi.encode(txHash, address(this), tx.origin));
        (uint256 chunkMask, uint256 totalChunks, address owner, bool isComplete) = chain.chunkStates(txHashProcessId);

        assertEq(owner, address(this));
        assertEq(chunkMask, 5); // Bits 0 and 2 set (2^0 + 2^2 = 1 + 4 = 5)
        assertEq(totalChunks, 4);
        assertFalse(isComplete); // Should not be complete as chunks 1 and 3 are missing

        // Verify expected mask
        uint256 expectedMask = (1 << 4) - 1; // Should be 15 (2^4 - 1)
        assertFalse(chunkMask == expectedMask);
    }

    function testProcessDuplicateChunk() public {
        bytes memory chunk = abi.encode("chunk");
        bytes32 txHash = bytes32(keccak256(chunk));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Process first chunk (setting bit 1)
        chain.processChunk(chunk, 1, 3, txHash); // Sets bit 1 (0b0010 = 2)

        // Verify initial state
        bytes32 txHashProcessId = keccak256(abi.encode(txHash, address(this), tx.origin));
        (uint256 chunkMask_,,,) = chain.chunkStates(txHashProcessId);
        assertEq(chunkMask_, 2); // Should be 0b0010 (bit 1 set = 2)

        // Process same chunk again
        chain.processChunk(chunk, 1, 3, txHash);

        // Verify state has not changed
        (uint256 chunkMask,,, bool isComplete) = chain.chunkStates(txHashProcessId);
        assertEq(chunkMask, 2); // Should still be 0b0010 (bit 1 set = 2)
        assertFalse(isComplete);
    }

    function testProcessChunkAfterCompletion() public {
        bytes memory chunk = abi.encode("chunk");
        bytes32 txHash = bytes32(keccak256(chunk));

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Process first chunk (with totalChunks = 1)
        chain.processChunk(chunk, 0, 1, txHash);

        // Attempt to process more chunks after completion
        vm.expectRevert(
            abi.encodeWithSelector(MetabasedSequencerChainWithProcessChunk.ChunkProcessingAlreadyComplete.selector)
        );
        chain.processChunk(chunk, 0, 1, txHash);
    }
}

contract MetabasedSequencerChainWithProcessChunkViewRequireAllTest is
    MetabasedSequencerChainWithProcessChunkTestSetUp
{
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        permissionModule.addCheck(address(mockRequireAll1), false);
        permissionModule.addCheck(address(mockRequireAll2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = permissionModule.getAllChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }
}

contract MetabasedSequencerChainWithProcessChunkViewRequireAnyTest is
    MetabasedSequencerChainWithProcessChunkTestSetUp
{
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();

        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));

        permissionModuleAny.addCheck(address(mockRequireAny1), false);
        permissionModuleAny.addCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = permissionModuleAny.getAllChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
