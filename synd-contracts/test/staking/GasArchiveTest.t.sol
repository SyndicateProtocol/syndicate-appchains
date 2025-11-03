pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {GasArchive} from "../../src/staking/GasArchive.sol";
import {MerklePatriciaProofVerifier} from "../../src/staking/lib/MerklePatriciaProofVerifier.sol";
import {RLPReader} from "../../src/staking/lib/RLPReader.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract GasArchiveTestHelper is GasArchive {
    constructor(address _blockHashSender, uint256 _settlementChainID)
        GasArchive(_blockHashSender, _settlementChainID)
    {}

    function setEpochDataHashForTesting(uint256 newEpoch, uint256 seqChainId, bytes32 hash) external {
        require(newEpoch <= epoch, "cannot set future epoch data hash");
        epochVerifiedDataHash[epoch][seqChainId] = hash;
    }
}

contract GasArchiveTest is Test {
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    address public gasArchiveImpl;
    GasArchiveTestHelper public gasArchive;

    address public admin;
    address public blockHashSender;
    address public user;

    uint256 public constant SETTLEMENT_CHAIN_ID = 1;
    uint256 public constant SEQ_CHAIN_ID = 31337; // matches the expected values in testConfirmEpochDataHashSuccess
    uint256 public constant APPCHAIN_ID_1 = 123;
    uint256 public constant APPCHAIN_ID_2 = 456;
    uint256 public constant EPOCH = 10;

    bytes32 public constant TEST_ETH_BLOCK_HASH = keccak256("eth_block");
    bytes32 public constant TEST_SETTLEMENT_BLOCK_HASH = keccak256("settlement_block");
    bytes32 public constant TEST_SEQ_BLOCK_HASH = keccak256("seq_block");

    event EpochCompleted(uint256 indexed epoch);
    event EpochExpectedChainsUpdated(uint256 indexed epoch, uint256[] chainIds);
    event GasAggregatorAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event LastKnownBlockHashesUpdated(bytes32 ethBlockHash, bytes32 settlementBlockHash, uint256 settlementBlockNumber);

    function setUp() public {
        // Start at the epoch start
        vm.warp(1754089200 + (EPOCH - 1) * 30 days);

        admin = makeAddr("admin");
        blockHashSender = makeAddr("blockHashSender");
        user = makeAddr("user");

        // Deploy GasArchive implementation
        gasArchiveImpl = address(new GasArchiveTestHelper(blockHashSender, SETTLEMENT_CHAIN_ID));

        // Prepare initialization data
        bytes memory initData = abi.encodeCall(GasArchive.initialize, (EPOCH));

        // Deploy GasArchive proxy
        vm.prank(admin);
        gasArchive = GasArchiveTestHelper(address(new ERC1967Proxy(gasArchiveImpl, initData)));

        assertEq(gasArchive.blockHashSender(), blockHashSender);
        assertEq(gasArchive.settlementChainID(), SETTLEMENT_CHAIN_ID);
        assertEq(gasArchive.owner(), admin);

        // Set up sequencing chain
        vm.prank(admin);
        gasArchive.addSequencingChain(SEQ_CHAIN_ID, address(2), address(1), false);

        // Wait until end of epoch
        vm.warp(1754089200 + EPOCH * 30 days);
    }

    /*//////////////////////////////////////////////////////////////
                    BLOCK HASH MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function testSetBlockHashes() public {
        vm.prank(blockHashSender);
        gasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

        assertTrue(gasArchive.ethBlockHashes(TEST_ETH_BLOCK_HASH));
        assertTrue(gasArchive.setBlockHashes(TEST_SETTLEMENT_BLOCK_HASH));
    }

    function testSetBlockHashesUnauthorized() public {
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);
    }

    /*//////////////////////////////////////////////////////////////
                    SEQUENCING CHAIN MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function testAddSequencingChain() public {
        uint256 newChainId = 789;
        address newAggregator = makeAddr("newAggregator");
        address newBridge = makeAddr("newBridge");

        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, newAggregator, newBridge, false);

        assertEq(gasArchive.seqChainGasAggregator(newChainId), newAggregator);
        assertEq(gasArchive.seqChainOutbox(newChainId), newBridge);
    }

    function testAddSettlementChainAsSequencingChain() public {
        address settlementAggregator = makeAddr("settlementAggregator");

        vm.prank(admin);
        gasArchive.addSettlementChainAsSequencingChain(settlementAggregator);

        assertEq(gasArchive.seqChainGasAggregator(SETTLEMENT_CHAIN_ID), settlementAggregator);
        assertEq(gasArchive.seqChainOutbox(SETTLEMENT_CHAIN_ID), address(0));
    }

    function testAddSequencingChainAlreadyExists() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.SequencingChainAlreadyExists.selector);
        gasArchive.addSequencingChain(SEQ_CHAIN_ID, address(1), address(1), false);
    }

    function testAddSequencingChainZeroAggregator() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(0), address(1), false);
    }

    function testAddSequencingChainZeroBridge() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(1), address(0), false);
    }

    function testAddSequencingChainUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(1), address(1), false);
    }

    function testRemoveSequencingChain() public {
        // First add a new chain to remove
        uint256 newChainId = 789;
        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, address(1), address(1), false);

        // Remove it
        vm.prank(admin);
        gasArchive.removeSequencingChain(newChainId);

        assertEq(gasArchive.seqChainGasAggregator(newChainId), address(0));
        assertEq(gasArchive.seqChainOutbox(newChainId), address(0));
    }

    function testRemoveSettlementChainAsSequencing() public {
        // First add settlement chain as sequencing
        vm.prank(admin);
        gasArchive.addSettlementChainAsSequencingChain(address(1));

        assertEq(gasArchive.seqChainGasAggregator(SETTLEMENT_CHAIN_ID), address(1));

        // Remove it
        vm.prank(admin);
        gasArchive.removeSequencingChain(SETTLEMENT_CHAIN_ID);

        assertEq(gasArchive.seqChainGasAggregator(SETTLEMENT_CHAIN_ID), address(0));
        assertEq(gasArchive.seqChainGasAggregator(SETTLEMENT_CHAIN_ID), address(0));
    }

    function testRemoveSequencingChainNotFound() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.SequencingChainDoesNotExist.selector);
        gasArchive.removeSequencingChain(999);
    }

    function testRemoveSequencingChainUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.removeSequencingChain(SEQ_CHAIN_ID);
    }

    function testRemoveSequencingChainTriggersEpochCompletionWithCorrectChainCount() public {
        uint256 chain2 = 888;
        uint256 chain3 = 999;

        vm.startPrank(admin);
        gasArchive.addSequencingChain(chain2, address(3), address(4), false);
        gasArchive.addSequencingChain(chain3, address(5), address(6), false);
        vm.stopPrank();

        // At this point:
        // - epoch = EPOCH (10)
        // - epochRemainingChains = 3 (SEQ_CHAIN_ID, chain2, chain3)
        // - None of the chains have submitted data

        // Setup verified data and submit pre-image for SEQ_CHAIN_ID
        uint256[] memory appchainIds = new uint256[](1);
        appchainIds[0] = APPCHAIN_ID_1;
        uint256[] memory gasUsageAmounts = new uint256[](1);
        gasUsageAmounts[0] = 100;

        gasArchive.setEpochDataHashForTesting(EPOCH, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // Setup verified data and submit pre-image for chain2
        gasArchive.setEpochDataHashForTesting(EPOCH, chain2, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(chain2, appchainIds, gasUsageAmounts);

        // At this point:
        // - SEQ_CHAIN_ID has submitted data
        // - chain2 has submitted data
        // - chain3 has NOT submitted data (epochRemainingChains should be 1)

        // Now remove chain3 which hasn't submitted data
        // This should trigger epoch completion because it's the last remaining chain
        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit EpochCompleted(EPOCH);
        gasArchive.removeSequencingChain(chain3);

        // Verify epoch has incremented
        assertEq(gasArchive.epoch(), EPOCH + 1, "Epoch should have incremented");

        // Verify chain3 was removed
        assertEq(gasArchive.seqChainGasAggregator(chain3), address(0), "Chain3 should be removed");

        uint256 newEpoch = EPOCH + 1;

        // Add verified data for SEQ_CHAIN_ID in the new epoch
        gasArchive.setEpochDataHashForTesting(newEpoch, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // Epoch should NOT complete yet because we still have chain2 to submit
        assertEq(gasArchive.epoch(), newEpoch, "Epoch should not complete after first submission");

        // Now submit for chain2 - this should complete the epoch
        gasArchive.setEpochDataHashForTesting(newEpoch, chain2, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        vm.expectEmit(true, false, false, false);
        emit EpochCompleted(newEpoch);
        gasArchive.submitEpochPreImageData(chain2, appchainIds, gasUsageAmounts);

        // Epoch should now be incremented and NOT REVERT
        assertEq(gasArchive.epoch(), newEpoch + 1, "Epoch should complete after all chains submit");
    }

    /*//////////////////////////////////////////////////////////////
                    EPOCH DATA VALIDATION TESTS
    //////////////////////////////////////////////////////////////*/
    /// @notice Tests the complete flow of epoch data validation and submission with cryptographic proofs
    /// @dev This test validates:
    ///      1. Setting block hashes via sendBlockHashes()
    ///      2. Adding settlement chain as a sequencing chain
    ///      3. Confirming epoch data hash with Merkle Patricia storage proofs
    ///      4. Submitting epoch pre-image data
    ///      5. Completing an epoch and verifying gas fee tracking
    ///
    function testConfirmEpochDataHashSuccess() public {
        // NOTE: the proof on `./fixtures/gasAggregatorEpochDataHashProof.json` was generated using a local anvil node (as if it were a sequencing chain) and the following data:
        // Implementation: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
        // appchain 1: 0x5FbDB2315678afecb367f032d93F642f64180aa3 tokensUsed: 100 chainId: 123
        // appchain 2: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 tokensUsed: 200 chainId 456
        // GasAggregator: 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
        // epoch = 1

        // this simple mock appchain contract is used
        //
        // // SPDX-License-Identifier: UNLICENSED
        // pragma solidity 0.8.28;
        // contract MockAppchain {
        //     uint256 gasUsed;
        //     function setGasUsed(uint256 gas) external {
        //         gasUsed = gas;
        //     }
        //     function tokensUsedPerEpoch(uint256 epoch) external view returns (uint256){
        //         return gasUsed;
        //     }
        // }

        // and each appchain is added using the `addLegacyChain` function

        // RLP encoded block header obtained with the following rust code:
        //
        // use alloy::rlp::Encodable;
        // let provider =
        //     alloy::providers::ProviderBuilder::new().connect("http://localhost:8545").await.unwrap();
        // let block = provider.get_block_by_number(BlockNumberOrTag::Latest).await.unwrap().unwrap();
        // let mut buf = vec![];
        // block.header.encode(&mut buf);
        // println!("{}", alloy::hex::encode(&buf));

        uint256 testEpoch = 1; // Fixture was generated for epoch 1

        // Deploy a fresh GasArchive instance for this test with epoch 1
        address testGasArchiveImpl = address(new GasArchiveTestHelper(blockHashSender, SETTLEMENT_CHAIN_ID));
        bytes memory testInitData = abi.encodeCall(GasArchive.initialize, (testEpoch));
        vm.prank(admin);
        GasArchiveTestHelper testGasArchive =
            GasArchiveTestHelper(address(new ERC1967Proxy(testGasArchiveImpl, testInitData)));

        bytes memory seqChainHeader =
            hex"f90262a07167e95bf5aba056c95ef955b97b2b2f15d8be3c7e34f1dfae6fcb8e89aedf00a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0e8f92fe4bb1e858b314d387a1931771d8ff99c37aef301e5f9757cb2ceea49eca0580e5fa979108b49feef86ad187d78d31952c2bea1b7e19045b6e7d8daec449aa0895a85eb5d12606d376b22c85f5be4f78061100c77503cb88ed2fb6174f5af79b901000000000000000000000000000000000000000000000000008000000000000000000000000000000000000200000000000000000000004020000000000004000000000000000000000000000000000000000800000004040000000000800000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000040000000000000040080088401c9c380830164d78468f7757a80a0b43699eab41f0370ee4d80cb949263979b52d6b68934f335a0aa55506186606c8800000000000000008417eb804ca056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b4218080a00000000000000000000000000000000000000000000000000000000000000000a0e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        // Setup: Set block hashes
        vm.prank(blockHashSender);
        testGasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, keccak256(seqChainHeader));

        // Setup: add the sequencing chain
        vm.warp(1754089200 + (testEpoch - 1) * 30 days);
        vm.prank(admin);
        testGasArchive.addSettlementChainAsSequencingChain(address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0));
        vm.warp(1754089200 + testEpoch * 30 days);

        uint256[] memory appchains = new uint256[](2);
        appchains[0] = APPCHAIN_ID_1;
        appchains[1] = APPCHAIN_ID_2;

        uint256[] memory tokens = new uint256[](2);
        tokens[0] = 100;
        tokens[1] = 200;

        // setup: add a secondary sequencing chain which will be removed later (so we can test the "already submitted" error, otherwise the epoch gets closed immediatelly)
        vm.prank(admin);
        uint256 mockSeqChainId = 111;
        testGasArchive.addSequencingChain(mockSeqChainId, makeAddr("someGasAggr"), makeAddr("someOutbox"), false);

        // Load fixture data
        string memory seqProofJson = vm.readFile("./test/staking/fixtures/gasAggregatorEpochDataHashProof.json");
        bytes[] memory seqAccountProofArray = vm.parseJsonBytesArray(seqProofJson, ".accountProof");
        bytes[] memory seqStorageProofArray = vm.parseJsonBytesArray(seqProofJson, ".storageProof[0].proof");

        testGasArchive.confirmSettlementChainEpochDataHash(seqChainHeader, seqAccountProofArray, seqStorageProofArray);

        // Test resubmission prevention for confirmEpochDataHash
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        testGasArchive.confirmSettlementChainEpochDataHash(seqChainHeader, seqAccountProofArray, seqStorageProofArray);

        // At this point, the epoch data hash is verified but epoch is not yet completed
        assertFalse(testGasArchive.epoch() > testEpoch, "Epoch should not be completed yet");

        testGasArchive.submitEpochPreImageData(SETTLEMENT_CHAIN_ID, appchains, tokens);

        // Test resubmission prevention for submitEpochPreImageData
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        testGasArchive.submitEpochPreImageData(SETTLEMENT_CHAIN_ID, appchains, tokens);

        // Epoch is not complete yet
        assertFalse(testGasArchive.epoch() > testEpoch);

        // Remove the settlement chain (added as sequencing chain)
        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit GasArchive.EpochCompleted(testEpoch);
        testGasArchive.removeSequencingChain(mockSeqChainId);

        // Now the epoch should be completed
        assertTrue(testGasArchive.epoch() > testEpoch, "Epoch should be marked as completed");

        // Check total gas fees
        assertEq(testGasArchive.getTotalGasFees(testEpoch), 300, "Total gas fees should be 100 + 200 = 300");

        // Check individual appchain gas fees
        assertEq(
            testGasArchive.getAppchainGasFees(testEpoch, APPCHAIN_ID_1), 100, "Appchain 123 should have 100 tokens"
        );
        assertEq(
            testGasArchive.getAppchainGasFees(testEpoch, APPCHAIN_ID_2), 200, "Appchain 456 should have 200 tokens"
        );

        // Check active appchain IDs
        uint256[] memory activeAppchains = testGasArchive.getAppchainIds(testEpoch);
        assertEq(activeAppchains.length, 2, "Should have 2 active appchains");
        assertEq(activeAppchains[0], APPCHAIN_ID_1, "First appchain should be 123");
        assertEq(activeAppchains[1], APPCHAIN_ID_2, "Second appchain should be 456");
    }

    function testSubmitEpochPreImageDataWithoutVerifiedHash() public {
        uint256[] memory appchains = new uint256[](1);
        appchains[0] = APPCHAIN_ID_1;

        uint256[] memory tokens = new uint256[](1);
        tokens[0] = 100;

        // Should revert if no verified hash exists
        vm.expectRevert(GasArchive.InvalidData.selector);
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchains, tokens);
    }

    /*//////////////////////////////////////////////////////////////
                        GETTER TESTS
    //////////////////////////////////////////////////////////////*/

    function testConstants() public view {
        assertEq(gasArchive.HEADER_STATE_ROOT_INDEX(), 3);
        assertEq(gasArchive.STORAGE_ROOT_ACCOUNT_FIELDS_INDEX(), 2);
    }

    function testInitialState() public view {
        assertEq(gasArchive.blockHashSender(), blockHashSender);
        assertEq(gasArchive.settlementChainID(), SETTLEMENT_CHAIN_ID);
        assertEq(gasArchive.seqChainGasAggregator(SETTLEMENT_CHAIN_ID), address(0));
    }

    function testSeqChainConfiguration() public view {
        assertEq(gasArchive.seqChainGasAggregator(SEQ_CHAIN_ID), address(2));
        assertEq(gasArchive.seqChainOutbox(SEQ_CHAIN_ID), address(1));
    }

    function testEpochDataInitiallyEmpty() public {
        assertFalse(gasArchive.epoch() > EPOCH);
        assertEq(gasArchive.totalGasFees(EPOCH), 0);
        assertEq(gasArchive.appchainGasFees(EPOCH, APPCHAIN_ID_1), 0);
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getTotalGasFees(EPOCH);
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1);
    }

    /*//////////////////////////////////////////////////////////////
                        VIEW FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function testGetAppchainGasFeesNotArchivedEpoch() public {
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1);
    }

    function testGetTotalGasFeesNotArchivedEpoch() public {
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getTotalGasFees(EPOCH);
    }

    function testGetActiveAppchainIdsNotArchivedEpoch() public {
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainIds(EPOCH);
    }

    function testViewFunctionsWithArchivedData() public {
        // Create test data
        uint256[] memory appchainIds = new uint256[](2);
        appchainIds[0] = APPCHAIN_ID_1;
        appchainIds[1] = APPCHAIN_ID_2;

        uint256[] memory gasUsageAmounts = new uint256[](2);
        gasUsageAmounts[0] = 1000;
        gasUsageAmounts[1] = 2000;

        // Set archived data using helper contract
        gasArchive.setEpochDataHashForTesting(EPOCH, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // Test getAppchainGasFees
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 1000);
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 2000);

        // Test getTotalGasFees
        assertEq(gasArchive.getTotalGasFees(EPOCH), 3000);

        // Test getActiveAppchainIds
        uint256[] memory activeAppchains = gasArchive.getAppchainIds(EPOCH);
        assertEq(activeAppchains.length, 2);
        assertEq(activeAppchains[0], APPCHAIN_ID_1);
        assertEq(activeAppchains[1], APPCHAIN_ID_2);
    }

    function testGetAppchainGasFeesZeroForNonExistentAppchain() public {
        // Create test data with only one appchain
        uint256[] memory appchainIds = new uint256[](1);
        appchainIds[0] = APPCHAIN_ID_1;

        uint256[] memory gasUsageAmounts = new uint256[](1);
        gasUsageAmounts[0] = 1500;

        gasArchive.setEpochDataHashForTesting(EPOCH, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // Test existing appchain
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 1500);

        // Test non-existent appchain returns 0
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 0);
    }

    /*//////////////////////////////////////////////////////////////
                    EPOCH COMPLETION TRACKING TESTS
    //////////////////////////////////////////////////////////////*/

    function testGetNewViewFunctions() public view {
        uint256 epoch = 200;

        assertFalse(gasArchive.epochChainDataSubmitted(epoch, SEQ_CHAIN_ID));
    }

    function testViewFunctionsRevertForIncompleteEpoch() public {
        uint256 chain2 = 999;

        // Warp to start of epoch
        vm.warp(1754089200 + (EPOCH - 1) * 30 days);
        // Add another sequencing chain
        vm.prank(admin);
        gasArchive.addSequencingChain(chain2, address(1), address(1), false);
        // Warp to end of epoch
        vm.warp(1754089200 + EPOCH * 30 days);

        uint256[] memory appchainIds = new uint256[](0);
        uint256[] memory gasUsageAmounts = new uint256[](0);

        // Manually set up partial epoch data (one chain submitted, one hasn't)
        gasArchive.setEpochDataHashForTesting(EPOCH, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // IGasDataProvider view functions should revert
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1);

        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getTotalGasFees(EPOCH);

        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainIds(EPOCH);

        assertTrue(gasArchive.epochChainDataSubmitted(EPOCH, SEQ_CHAIN_ID));
        assertFalse(gasArchive.epochChainDataSubmitted(EPOCH, chain2));
    }

    /*//////////////////////////////////////////////////////////////
                         OTHER TESTS
    //////////////////////////////////////////////////////////////*/

    function testMultipleSequencingChains() public {
        uint256 chainId2 = 999;
        uint256 chainId3 = 888;

        // Add multiple chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chainId2, address(1), address(1), false);
        gasArchive.addSequencingChain(chainId3, address(1), address(1), false);
        vm.stopPrank();

        // Verify they're all configured
        assertEq(gasArchive.seqChainGasAggregator(SEQ_CHAIN_ID), address(2));
        assertEq(gasArchive.seqChainGasAggregator(chainId2), address(1));
        assertEq(gasArchive.seqChainGasAggregator(chainId3), address(1));

        // Remove middle chain
        vm.prank(admin);
        gasArchive.removeSequencingChain(chainId2);

        // Verify removal
        assertEq(gasArchive.seqChainGasAggregator(chainId2), address(0));
        // Others should still exist
        assertEq(gasArchive.seqChainGasAggregator(SEQ_CHAIN_ID), address(2));
        assertEq(gasArchive.seqChainGasAggregator(chainId3), address(1));
    }

    function testAccessControl() public {
        // Test that only admin can perform admin functions
        vm.startPrank(user);

        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(1), address(1), false);

        vm.expectRevert();
        gasArchive.removeSequencingChain(SEQ_CHAIN_ID);

        vm.stopPrank();

        // Test that only blockHashSender can set block hashes
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);
    }

    /*//////////////////////////////////////////////////////////////
                    UUPS PROXY IMMUTABLE VARIABLES TESTS
    //////////////////////////////////////////////////////////////*/

    /// @dev CONTEXT: Why GasArchive has both constructor AND initialize()
    ///
    /// This test suite validates an important pattern for UUPS upgradeable contracts:
    /// Using BOTH a constructor (for immutables) and initialize() (for storage variables).
    ///
    /// KEY CONCEPTS:
    /// 1. Immutable variables are NOT stored in contract storage - they're compiled directly
    ///    into the contract's bytecode at deployment time.
    ///
    /// 2. When a proxy delegates to an implementation:
    ///    - Storage operations (SLOAD/SSTORE) affect the PROXY's storage
    ///    - Code execution (including immutable value reads) uses the IMPLEMENTATION's bytecode
    ///
    /// 3. Therefore:
    ///    - Constructor sets immutables → becomes part of implementation bytecode
    ///    - Initialize sets storage variables → affects proxy storage when called through proxy
    ///
    /// PATTERN BENEFITS:
    /// - Immutables are gas-efficient (no SLOAD, values inlined in bytecode)
    /// - Values that shouldn't change between upgrades can be immutable
    /// - Storage variables remain upgradeable and isolated to proxy
    ///
    /// These tests prove that when any address calls the proxy:
    /// → Proxy delegatecalls to implementation
    /// → Implementation's bytecode executes (including immutable values)
    /// → Immutables are readable and work correctly in business logic

    /// @notice Tests that immutable variables set in the implementation constructor are readable through the proxy
    /// @dev This validates that immutables in UUPS pattern work as expected - they're part of the implementation bytecode
    function testProxyCanReadImmutableVariables() public view {
        // Call through the proxy to read immutable variables
        address retrievedBlockHashSender = gasArchive.blockHashSender();
        uint256 retrievedSettlementChainID = gasArchive.settlementChainID();

        // Verify the proxy returns the correct immutable values
        assertEq(retrievedBlockHashSender, blockHashSender, "blockHashSender should be readable through proxy");
        assertEq(retrievedSettlementChainID, SETTLEMENT_CHAIN_ID, "settlementChainID should be readable through proxy");
    }

    /// @notice Tests that immutable variables are consistent between proxy and implementation
    /// @dev Verifies that reading from proxy and implementation returns the same values
    function testImmutableVariablesConsistentBetweenProxyAndImplementation() public view {
        // Read from proxy
        address proxyBlockHashSender = gasArchive.blockHashSender();
        uint256 proxySettlementChainID = gasArchive.settlementChainID();

        // Read directly from implementation
        GasArchiveTestHelper impl = GasArchiveTestHelper(gasArchiveImpl);
        address implBlockHashSender = impl.blockHashSender();
        uint256 implSettlementChainID = impl.settlementChainID();

        // Both should return the same values since immutables are in the bytecode
        assertEq(
            proxyBlockHashSender,
            implBlockHashSender,
            "blockHashSender should be the same when read from proxy or implementation"
        );
        assertEq(
            proxySettlementChainID,
            implSettlementChainID,
            "settlementChainID should be the same when read from proxy or implementation"
        );
    }

    /// @notice Tests that immutable variables work correctly in business logic through proxy
    /// @dev Verifies that immutables are used correctly in delegatecall context
    function testImmutableVariablesUsedInBusinessLogic() public {
        // Test that blockHashSender immutable is used correctly in access control
        vm.prank(blockHashSender);
        gasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

        assertTrue(gasArchive.ethBlockHashes(TEST_ETH_BLOCK_HASH), "Block hash should be set");

        // Test that wrong sender is rejected (verifies immutable is checked correctly)
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.sendBlockHashes(keccak256("another_hash"), keccak256("another_hash"));
    }

    /// @notice Tests that a fresh proxy deployment with different immutable values works correctly
    /// @dev This proves that each deployment can have unique immutable values
    function testMultipleProxyDeploymentsWithDifferentImmutables() public {
        // Deploy a new implementation with different immutable values
        address newBlockHashSender = makeAddr("newBlockHashSender");
        uint256 newSettlementChainID = 42161; // Arbitrum One

        address newImpl = address(new GasArchiveTestHelper(newBlockHashSender, newSettlementChainID));

        // Deploy a new proxy pointing to the new implementation
        bytes memory initData = abi.encodeCall(GasArchive.initialize, (1));
        GasArchiveTestHelper newProxy = GasArchiveTestHelper(address(new ERC1967Proxy(newImpl, initData)));

        // Verify the new proxy has the new immutable values
        assertEq(newProxy.blockHashSender(), newBlockHashSender, "New proxy should have new blockHashSender");
        assertEq(newProxy.settlementChainID(), newSettlementChainID, "New proxy should have new settlementChainID");

        // Verify original proxy still has original values
        assertEq(gasArchive.blockHashSender(), blockHashSender, "Original proxy should retain original blockHashSender");
        assertEq(
            gasArchive.settlementChainID(),
            SETTLEMENT_CHAIN_ID,
            "Original proxy should retain original settlementChainID"
        );

        // Verify they're different
        assertTrue(
            newProxy.blockHashSender() != gasArchive.blockHashSender(),
            "Different proxies should have different blockHashSender"
        );
        assertTrue(
            newProxy.settlementChainID() != gasArchive.settlementChainID(),
            "Different proxies should have different settlementChainID"
        );
    }

    /// @notice Tests that any address can call the proxy and read immutable variables
    /// @dev Proves immutables are publicly accessible through delegatecall
    function testAnyAddressCanReadImmutablesThroughProxy() public {
        // Test with multiple different addresses
        address[] memory callers = new address[](3);
        callers[0] = admin;
        callers[1] = user;
        callers[2] = makeAddr("randomAddress");

        for (uint256 i = 0; i < callers.length; i++) {
            vm.prank(callers[i]);
            address retrievedSender = gasArchive.blockHashSender();
            uint256 retrievedChainID = gasArchive.settlementChainID();

            assertEq(retrievedSender, blockHashSender, "Any caller should read correct blockHashSender");
            assertEq(retrievedChainID, SETTLEMENT_CHAIN_ID, "Any caller should read correct settlementChainID");
        }
    }

    function testSeqChainRemoveAndAddAgainDeadlock() public {
        uint256 chainId2 = 999;

        // Add multiple chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chainId2, address(1), address(1), false);
        vm.stopPrank();

        // Create test data
        uint256[] memory appchainIds = new uint256[](2);
        appchainIds[0] = APPCHAIN_ID_1;
        appchainIds[1] = APPCHAIN_ID_2;

        uint256[] memory gasUsageAmounts = new uint256[](2);
        gasUsageAmounts[0] = 1000;
        gasUsageAmounts[1] = 2000;

        // Set archived data using helper contract
        gasArchive.setEpochDataHashForTesting(EPOCH, chainId2, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(chainId2, appchainIds, gasUsageAmounts);

        // Remove chainId2
        vm.prank(admin);
        gasArchive.removeSequencingChain(chainId2);

        // Add chainId2 back
        vm.prank(admin);
        gasArchive.addSequencingChain(chainId2, address(1), address(1), false);

        // Submit data for SEQ_CHAIN_ID
        gasArchive.setEpochDataHashForTesting(EPOCH, SEQ_CHAIN_ID, keccak256(abi.encode(appchainIds, gasUsageAmounts)));
        gasArchive.submitEpochPreImageData(SEQ_CHAIN_ID, appchainIds, gasUsageAmounts);

        // Check epoch completion
        assertEq(gasArchive.epoch(), EPOCH + 1);
    }
}
