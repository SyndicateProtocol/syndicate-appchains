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
    uint256 public constant EPOCH = 10; // matches the expected values in testConfirmEpochDataHashSuccess

    bytes32 public constant TEST_ETH_BLOCK_HASH = keccak256("eth_block");
    bytes32 public constant TEST_SETTLEMENT_BLOCK_HASH = keccak256("settlement_block");
    bytes32 public constant TEST_SEQ_BLOCK_HASH = keccak256("seq_block");

    event EpochDataValidated(uint256 indexed epoch, uint256 indexed seqChainID, bytes32 dataHash);
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

    /*//////////////////////////////////////////////////////////////
                    EPOCH DATA VALIDATION TESTS
    //////////////////////////////////////////////////////////////*/
    function testConfirmEpochDataHashSuccess() public {
        // TODO(ENG-2113): regenerate proof
        vm.skip(true);
        bytes memory seqChainHeader =
            hex"f90262a0605defa624498989bf665b3a40ae020f887dcfe2416d768c9d42a5f19b22fcc1a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a00d663178efa9bfb74511ae198171076765cdde527748f2b403dc0098f8b5a77ca07b6f777b47600b2184243dd7a8acd4718ac39b7cacff19d7cc7e4859d7b4babda0a4eb1fbd62f3905dbeead463382bd44cadbb8aab9c8ca947071cecded7cf7b51b901000000000400000000040000000000000040000000000000000080000000000000000000000000000000000000000000001000000000004020000000000004000100000000000000000000000000000200000100000004000000000000000000000000000002000000000000010080080000000480000000000000000400000040000000000000000000080000000000000000000000008000000000000080000000000000000000000000000200000000000000000000000000100000000000000000002000000020000000000000180000000000240c000100000008000060000000000000000000000000000000000000000000000000c0000000000000000080028401c9c3808325da7a8468b97c7980a01735d51a6bf99e813a40505ea196a5b79e0ab7d9d0dfb579ecee9499bccca784880000000000000000843455cb4aa056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b4218080a00000000000000000000000000000000000000000000000000000000000000000a0e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        // Setup: Set block hashes
        vm.prank(blockHashSender);
        gasArchive.sendBlockHashes(TEST_ETH_BLOCK_HASH, keccak256(seqChainHeader));

        // Setup: add the sequencing chain
        vm.warp(1754089200 + (EPOCH - 1) * 30 days);
        vm.prank(admin);
        gasArchive.addSettlementChainAsSequencingChain(address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0));
        vm.warp(1754089200 + EPOCH * 30 days);

        uint256[] memory appchains = new uint256[](2);
        appchains[0] = APPCHAIN_ID_1;
        appchains[1] = APPCHAIN_ID_2;

        uint256[] memory tokens = new uint256[](2);
        tokens[0] = 100;
        tokens[1] = 200;

        bytes[] memory mockAccountProof = new bytes[](1);
        mockAccountProof[0] = abi.encode("account_proof");
        bytes[] memory mockStorageProof = new bytes[](1);
        mockStorageProof[0] = abi.encode("storage_proof");

        // NOTE: the proof on `./fixtures/gasAggregatorEpochDataHashProof.json` was generated using a local anvil node and the following data:
        //Implementation: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
        // GasAggregator (Proxy): 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
        // Anvil chain id: 31337
        // Anvil block hash: 0x55c3e74a2dec0e3d150636b57e5c988c570215255b1b7670e9366914ba597018
        // appchain1 {id: 123, tokens: 100, emissionsReceiver: 0x123}
        // appchain2 {id: 456, tokens: 200, emissionsReceiver: 0x456}
        // EPOCH = 10

        // Load fixture data
        string memory seqProofJson = vm.readFile("./test/staking/fixtures/gasAggregatorEpochDataHashProof.json");

        // Parse JSON arrays directly
        bytes[] memory seqAccountProofArray = vm.parseJsonBytesArray(seqProofJson, ".accountProof");
        bytes[] memory seqStorageProofArray = vm.parseJsonBytesArray(seqProofJson, ".storageProof[0].proof");

        // RLP encoded block header obtained with the following rust code:
        //
        // use alloy::rlp::Encodable;
        // let provider =
        //     alloy::providers::ProviderBuilder::new().connect("http://localhost:8545").await.unwrap();
        // let block = provider.get_block_by_number(BlockNumberOrTag::Latest).await.unwrap().unwrap();
        // let mut buf = vec![];
        // block.header.encode(&mut buf);
        // println!("{}", alloy::hex::encode(&buf));

        vm.expectEmit(true, true, false, true);
        emit EpochDataValidated(
            EPOCH, SETTLEMENT_CHAIN_ID, bytes32(vm.parseJsonBytes(seqProofJson, ".storageProof[0].value"))
        );

        gasArchive.confirmSettlementChainEpochDataHash(seqChainHeader, seqAccountProofArray, seqStorageProofArray);

        // Test resubmission prevention for confirmEpochDataHash
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        gasArchive.confirmSettlementChainEpochDataHash(seqChainHeader, seqAccountProofArray, seqStorageProofArray);

        // At this point, the epoch data hash is verified but epoch is not yet completed
        assertFalse(gasArchive.epoch() > EPOCH, "Epoch should not be completed yet");

        gasArchive.submitEpochPreImageData(SETTLEMENT_CHAIN_ID, appchains, tokens);

        // Test resubmission prevention for submitEpochPreImageData
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        gasArchive.submitEpochPreImageData(SETTLEMENT_CHAIN_ID, appchains, tokens);

        // Epoch is not complete yet
        assertFalse(gasArchive.epoch() > EPOCH);

        // Remove the original sequencing chain
        vm.prank(admin);
        vm.expectEmit(true, false, false, false);
        emit GasArchive.EpochCompleted(EPOCH);
        gasArchive.removeSequencingChain(SEQ_CHAIN_ID);

        // Now the epoch should be completed
        assertTrue(gasArchive.epoch() > EPOCH, "Epoch should be marked as completed");

        // Check total gas fees
        assertEq(gasArchive.getTotalGasFees(EPOCH), 300, "Total gas fees should be 100 + 200 = 300");

        // Check individual appchain gas fees
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 100, "Appchain 123 should have 100 tokens");
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 200, "Appchain 456 should have 200 tokens");

        // Check active appchain IDs
        uint256[] memory activeAppchains = gasArchive.getAppchainIds(EPOCH);
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
        assertEq(gasArchive.AGGREGATED_EPOCH_DATA_HASH_SLOT(), 0);
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
}
