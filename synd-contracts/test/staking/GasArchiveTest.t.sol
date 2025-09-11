pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {GasArchive} from "../../src/staking/GasArchive.sol";
import {MerklePatriciaProofVerifier} from "../../src/staking/lib/MerklePatriciaProofVerifier.sol";
import {RLPReader} from "../../src/staking/lib/RLPReader.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

contract MockGasAggregator {
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    function setEpochDataHash(uint256 epoch, bytes32 hash) external {
        aggregatedEpochDataHash[epoch] = hash;
    }
}

contract MockBridge {
    bytes32 public confirmedRollupHash;

    function setConfirmedRollupHash(bytes32 hash) external {
        confirmedRollupHash = hash;
    }
}

contract GasUsageArchiveTestHelper is GasArchive {
    function setArchivedEpochDataForTesting(
        uint256 epoch,
        uint256[] memory appchainIds,
        uint256[] memory tokensUsed,
        address[] memory emissionsReceivers
    ) external {
        require(appchainIds.length == tokensUsed.length, "Array length mismatch");
        require(appchainIds.length == emissionsReceivers.length, "Array length mismatch");

        uint256 totalTokensUsed = 0;
        epochAppchainIDs[epoch] = appchainIds;

        for (uint256 i = 0; i < appchainIds.length; i++) {
            totalTokensUsed += tokensUsed[i];
            epochAppchainTokensUsed[epoch][appchainIds[i]] = tokensUsed[i];
            epochAppchainEmissionsReceiver[epoch][appchainIds[i]] = emissionsReceivers[i];
        }

        epochTotalTokensUsed[epoch] = totalTokensUsed;
        epochCompleted[epoch] = true;
    }

    function setLastKnownSeqChainBlockHashForTesting(uint256 seqChainId, bytes32 blockHash) external {
        lastKnownSeqChainBlockHashes[seqChainId] = blockHash;
    }

    function setEpochChainDataSubmittedForTesting(uint256 epoch, uint256 chainId, bool submitted) external {
        epochChainDataSubmitted[epoch][chainId] = submitted;
    }

    function addExpectedChainForTesting(uint256 epoch, uint256 chainId) external {
        epochExpectedChains[epoch].push(chainId);
    }
}

contract GasArchiveTest is Test {
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    GasUsageArchiveTestHelper public gasArchive;
    MockGasAggregator public mockGasAggregator;
    MockBridge public mockBridge;

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
    uint256 public constant TEST_STORAGE_SLOT_INDEX = 1;

    event EpochDataValidated(uint256 indexed epoch, uint256 indexed seqChainID, bytes32 dataHash);
    event EpochCompleted(uint256 indexed epoch);
    event EpochExpectedChainsUpdated(uint256 indexed epoch, uint256[] chainIds);
    event GasAggregatorAddressUpdated(address indexed oldAddress, address indexed newAddress);

    function setUp() public {
        admin = makeAddr("admin");
        blockHashSender = makeAddr("blockHashSender");
        user = makeAddr("user");

        mockBridge = new MockBridge();

        // Deploy using TransparentUpgradeableProxy pattern
        GasUsageArchiveTestHelper implementation = new GasUsageArchiveTestHelper();
        bytes memory initData =
            abi.encodeWithSelector(GasArchive.initialize.selector, blockHashSender, SETTLEMENT_CHAIN_ID, admin);
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        TransparentUpgradeableProxy proxy =
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);
        gasArchive = GasUsageArchiveTestHelper(address(proxy));

        // Set up sequencing chain
        vm.startPrank(admin);
        address gasArchiveAddress = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0); // matches the expected values in testConfirmEpochDataHashSuccess
        gasArchive.addSequencingChain(SEQ_CHAIN_ID, gasArchiveAddress, address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                        INITIALIZATION TESTS
    //////////////////////////////////////////////////////////////*/

    function testInitialize() public {
        // Deploy new instances to test initialization with different parameters
        // Test zero blockHashSender
        {
            ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
            GasArchive implementation = new GasArchive();
            bytes memory badInitData = abi.encodeWithSelector(
                GasArchive.initialize.selector,
                address(0), // zero address
                SETTLEMENT_CHAIN_ID,
                admin
            );

            vm.expectRevert(GasArchive.ZeroAddress.selector);
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), badInitData);
        }

        // Test zero admin
        {
            ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
            GasArchive implementation = new GasArchive();
            bytes memory badInitData = abi.encodeWithSelector(
                GasArchive.initialize.selector,
                blockHashSender,
                SETTLEMENT_CHAIN_ID,
                address(0) // zero address
            );

            vm.expectRevert(GasArchive.ZeroAddress.selector);
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), badInitData);
        }

        // Test successful initialization (already tested in setUp, but let's verify)
        assertEq(gasArchive.blockHashSender(), blockHashSender);
        assertEq(gasArchive.settlementChainID(), SETTLEMENT_CHAIN_ID);
        assertTrue(gasArchive.hasRole(gasArchive.DEFAULT_ADMIN_ROLE(), admin));
    }

    function testCannotInitializeTwice() public {
        vm.expectRevert();
        gasArchive.initialize(blockHashSender, SETTLEMENT_CHAIN_ID, admin);
    }

    /*//////////////////////////////////////////////////////////////
                    BLOCK HASH MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function testSetLastKnownBlockHashes() public {
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);

        assertEq(gasArchive.lastKnownEthereumBlockHash(), TEST_ETH_BLOCK_HASH);
        assertEq(gasArchive.lastKnownSettlementChainBlockHash(), TEST_SETTLEMENT_BLOCK_HASH);
    }

    function testSetLastKnownBlockHashesUnauthorized() public {
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);
    }

    function testSetLastKnownBlockHashesInvalidBlockNumber() public {
        // First set with block number 5
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 5);

        assertEq(gasArchive.lastKnownSettlementChainBlockNumber(), 5);

        // Try to set with same block number - should revert
        vm.prank(blockHashSender);
        vm.expectRevert(GasArchive.OldSettlementChainBlockNumber.selector);
        gasArchive.setLastKnownBlockHashes(keccak256("new_eth"), keccak256("new_settlement"), 5);

        // Try to set with lower block number - should revert
        vm.prank(blockHashSender);
        vm.expectRevert(GasArchive.OldSettlementChainBlockNumber.selector);
        gasArchive.setLastKnownBlockHashes(keccak256("new_eth"), keccak256("new_settlement"), 3);

        // Setting with higher block number should succeed
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(keccak256("new_eth"), keccak256("new_settlement"), 10);

        assertEq(gasArchive.lastKnownSettlementChainBlockNumber(), 10);
        assertEq(gasArchive.lastKnownEthereumBlockHash(), keccak256("new_eth"));
        assertEq(gasArchive.lastKnownSettlementChainBlockHash(), keccak256("new_settlement"));
    }

    /*//////////////////////////////////////////////////////////////
                    SEQUENCING CHAIN MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function testAddSequencingChain() public {
        uint256 newChainId = 789;
        address newAggregator = makeAddr("newAggregator");
        address newBridge = makeAddr("newBridge");
        uint256 newStorageSlotIndex = 2;

        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, newAggregator, newBridge, newStorageSlotIndex);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(newChainId), newAggregator);
        assertEq(gasArchive.seqChainEthOutbox(newChainId), newBridge);
        assertEq(gasArchive.seqChainEthSendRootStorageSlot(newChainId), newStorageSlotIndex);
    }

    function testAddSettlementChainAsSequencingChain() public {
        address settlementAggregator = makeAddr("settlementAggregator");

        vm.prank(admin);
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, settlementAggregator, address(0), 0);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), settlementAggregator);
        assertEq(gasArchive.seqChainEthOutbox(SETTLEMENT_CHAIN_ID), address(0));
        assertEq(gasArchive.seqChainEthSendRootStorageSlot(SETTLEMENT_CHAIN_ID), 0);
    }

    function testAddSequencingChainAlreadyExists() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.SequencingChainAlreadyExists.selector);
        gasArchive.addSequencingChain(
            SEQ_CHAIN_ID, address(mockGasAggregator), address(mockBridge), TEST_STORAGE_SLOT_INDEX
        );
    }

    function testAddSequencingChainZeroAggregator() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(0), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
    }

    function testAddSequencingChainZeroBridge() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(0x1), address(0), TEST_STORAGE_SLOT_INDEX);
    }

    function testAddSequencingChainUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
    }

    function testRemoveSequencingChain() public {
        // First add a new chain to remove
        uint256 newChainId = 789;
        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);

        // Remove it
        vm.prank(admin);
        gasArchive.removeSeqChain(newChainId);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(newChainId), address(0));
        assertEq(gasArchive.seqChainEthOutbox(newChainId), address(0));
        assertEq(gasArchive.seqChainEthSendRootStorageSlot(newChainId), 0);
    }

    function testRemoveSettlementChainAsSequencing() public {
        // First add settlement chain as sequencing
        vm.prank(admin);
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, address(0x1), address(0), 0);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), address(0x1));

        // Remove it
        vm.prank(admin);
        gasArchive.removeSeqChain(SETTLEMENT_CHAIN_ID);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), address(0));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), address(0));
    }

    function testRemoveSequencingChainNotFound() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ChainIDNotFound.selector);
        gasArchive.removeSeqChain(999);
    }

    function testRemoveSequencingChainUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.removeSeqChain(SEQ_CHAIN_ID);
    }

    /*//////////////////////////////////////////////////////////////
                    BLOCK HASH SENDER MANAGEMENT
    //////////////////////////////////////////////////////////////*/

    function testSetBlockHashSender() public {
        address newSender = makeAddr("newSender");

        vm.prank(admin);
        gasArchive.setBlockHashSender(newSender);

        assertEq(gasArchive.blockHashSender(), newSender);
    }

    function testSetBlockHashSenderUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.setBlockHashSender(user);
    }

    /*//////////////////////////////////////////////////////////////
                    EPOCH DATA VALIDATION TESTS
    //////////////////////////////////////////////////////////////*/

    function testConfirmEpochDataHashSuccess() public {
        // Setup: Set block hashes
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);

        uint256[] memory appchains = new uint256[](2);
        appchains[0] = APPCHAIN_ID_1;
        appchains[1] = APPCHAIN_ID_2;

        uint256[] memory tokens = new uint256[](2);
        tokens[0] = 100;
        tokens[1] = 200;

        address[] memory emissionsReceivers = new address[](2);
        emissionsReceivers[0] = address(0x123);
        emissionsReceivers[1] = address(0x456);

        bytes[] memory mockAccountProof = new bytes[](1);
        mockAccountProof[0] = abi.encode("account_proof");
        bytes[] memory mockStorageProof = new bytes[](1);
        mockStorageProof[0] = abi.encode("storage_proof");

        // This will revert because we don't have a valid block header
        vm.expectRevert(GasArchive.InvalidSeqChainBlockHeader.selector);
        gasArchive.confirmEpochDataHash(
            EPOCH, SEQ_CHAIN_ID, abi.encode("mock_header"), mockAccountProof, mockStorageProof
        );

        // NOTE: the proof on `./fixtures/epochDataHash.json` was generated using a local anvil node and the following data:
        //Implementation: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
        // GasAggregator (Proxy): 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
        // Anvil chain id: 31337
        // Anvil block hash: 0x55c3e74a2dec0e3d150636b57e5c988c570215255b1b7670e9366914ba597018
        // appchain1 {id: 123, tokens: 100, emissionsReceiver: 0x123}
        // appchain2 {id: 456, tokens: 200, emissionsReceiver: 0x456}
        // EPOCH = 10

        // Load fixture data
        string memory proofJson = vm.readFile("./test/staking/fixtures/gasAggregatorEpochDataHashProof.json");

        // Parse JSON arrays directly
        bytes[] memory accountProofArray = vm.parseJsonBytesArray(proofJson, ".accountProof");
        bytes[] memory storageProofArray = vm.parseJsonBytesArray(proofJson, ".storageProof[0].proof");

        // RLP encoded block header obtained with the following rust code:
        //
        // use alloy::rlp::Encodable;
        // let provider =
        //     alloy::providers::ProviderBuilder::new().connect("http://localhost:8545").await.unwrap();
        // let block = provider.get_block_by_number(BlockNumberOrTag::Latest).await.unwrap().unwrap();
        // let mut buf = vec![];
        // block.header.encode(&mut buf);
        // println!("{}", alloy::hex::encode(&buf));

        bytes memory seqChainHeader =
            hex"f90262a0605defa624498989bf665b3a40ae020f887dcfe2416d768c9d42a5f19b22fcc1a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a00d663178efa9bfb74511ae198171076765cdde527748f2b403dc0098f8b5a77ca07b6f777b47600b2184243dd7a8acd4718ac39b7cacff19d7cc7e4859d7b4babda0a4eb1fbd62f3905dbeead463382bd44cadbb8aab9c8ca947071cecded7cf7b51b901000000000400000000040000000000000040000000000000000080000000000000000000000000000000000000000000001000000000004020000000000004000100000000000000000000000000000200000100000004000000000000000000000000000002000000000000010080080000000480000000000000000400000040000000000000000000080000000000000000000000008000000000000080000000000000000000000000000200000000000000000000000000100000000000000000002000000020000000000000180000000000240c000100000008000060000000000000000000000000000000000000000000000000c0000000000000000080028401c9c3808325da7a8468b97c7980a01735d51a6bf99e813a40505ea196a5b79e0ab7d9d0dfb579ecee9499bccca784880000000000000000843455cb4aa056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b4218080a00000000000000000000000000000000000000000000000000000000000000000a0e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        // Set the last known sequencing chain block hash
        gasArchive.setLastKnownSeqChainBlockHashForTesting(
            SEQ_CHAIN_ID, 0x55c3e74a2dec0e3d150636b57e5c988c570215255b1b7670e9366914ba597018
        );

        vm.expectEmit(true, false, false, true);
        emit EpochDataValidated(EPOCH, SEQ_CHAIN_ID, bytes32(vm.parseJsonBytes(proofJson, ".storageProof[0].value")));

        gasArchive.confirmEpochDataHash(EPOCH, SEQ_CHAIN_ID, seqChainHeader, accountProofArray, storageProofArray);

        // Test resubmission prevention for confirmEpochDataHash
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        gasArchive.confirmEpochDataHash(EPOCH, SEQ_CHAIN_ID, seqChainHeader, accountProofArray, storageProofArray);

        // At this point, the epoch data hash is verified but epoch is not yet completed
        assertFalse(gasArchive.epochCompleted(EPOCH), "Epoch should not be completed yet");

        gasArchive.submitEpochPreImageData(EPOCH, SEQ_CHAIN_ID, appchains, tokens, emissionsReceivers);

        // Now the epoch should be completed
        assertTrue(gasArchive.epochCompleted(EPOCH), "Epoch should be marked as completed");

        // Check total gas fees
        assertEq(gasArchive.getTotalGasFees(EPOCH), 300, "Total gas fees should be 100 + 200 = 300");

        // Check individual appchain gas fees
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 100, "Appchain 123 should have 100 tokens");
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 200, "Appchain 456 should have 200 tokens");

        // Check active appchain IDs
        uint256[] memory activeAppchains = gasArchive.getActiveAppchainIds(EPOCH);
        assertEq(activeAppchains.length, 2, "Should have 2 active appchains");
        assertEq(activeAppchains[0], APPCHAIN_ID_1, "First appchain should be 123");
        assertEq(activeAppchains[1], APPCHAIN_ID_2, "Second appchain should be 456");

        // Check emissions receivers
        assertEq(
            gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_1),
            address(0x123),
            "Appchain 123 receiver should be 0x123"
        );
        assertEq(
            gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_2),
            address(0x456),
            "Appchain 456 receiver should be 0x456"
        );

        // Test resubmission prevention for confirmEpochDataHash (after the data has been submitted and the datahash deleted from storage)
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        gasArchive.confirmEpochDataHash(EPOCH, SEQ_CHAIN_ID, seqChainHeader, accountProofArray, storageProofArray);

        // Test resubmission prevention for submitEpochPreImageData
        vm.expectRevert(GasArchive.AlreadySubmitted.selector);
        gasArchive.submitEpochPreImageData(EPOCH, SEQ_CHAIN_ID, appchains, tokens, emissionsReceivers);

        // Verify that the numbers haven't been inflated after the failed resubmissions
        assertEq(gasArchive.getTotalGasFees(EPOCH), 300, "Total gas fees should still be 100 + 200 = 300");
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 100, "Appchain 123 should still have 100 tokens");
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 200, "Appchain 456 should still have 200 tokens");
    }

    function testConfirmEpochDataHashInvalidSeqChainBlockHeader() public {
        bytes memory mockHeader = abi.encode("invalid_header");
        bytes[] memory mockAccountProof = new bytes[](0);
        bytes[] memory mockStorageProof = new bytes[](0);

        vm.expectRevert(GasArchive.InvalidSeqChainBlockHeader.selector);
        gasArchive.confirmEpochDataHash(EPOCH, SEQ_CHAIN_ID, mockHeader, mockAccountProof, mockStorageProof);
    }

    function testSubmitEpochPreImageDataWithoutVerifiedHash() public {
        uint256[] memory appchains = new uint256[](1);
        appchains[0] = APPCHAIN_ID_1;

        uint256[] memory tokens = new uint256[](1);
        tokens[0] = 100;

        address[] memory emissionsReceivers = new address[](1);
        emissionsReceivers[0] = makeAddr("receiver1");

        // Should revert if no verified hash exists
        vm.expectRevert(GasArchive.InvalidData.selector);
        gasArchive.submitEpochPreImageData(EPOCH, SEQ_CHAIN_ID, appchains, tokens, emissionsReceivers);
    }

    function testSubmitEpochPreImageDataWithIncorrectData() public {
        // First, set up a verified epoch data hash
        string memory proofJson = vm.readFile("./test/staking/fixtures/gasAggregatorEpochDataHashProof.json");
        bytes[] memory accountProofArray = vm.parseJsonBytesArray(proofJson, ".accountProof");
        bytes[] memory storageProofArray = vm.parseJsonBytesArray(proofJson, ".storageProof[0].proof");
        bytes memory seqChainHeader =
            hex"f90262a0605defa624498989bf665b3a40ae020f887dcfe2416d768c9d42a5f19b22fcc1a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a00d663178efa9bfb74511ae198171076765cdde527748f2b403dc0098f8b5a77ca07b6f777b47600b2184243dd7a8acd4718ac39b7cacff19d7cc7e4859d7b4babda0a4eb1fbd62f3905dbeead463382bd44cadbb8aab9c8ca947071cecded7cf7b51b901000000000400000000040000000000000040000000000000000080000000000000000000000000000000000000000000001000000000004020000000000004000100000000000000000000000000000200000100000004000000000000000000000000000002000000000000010080080000000480000000000000000400000040000000000000000000080000000000000000000000008000000000000080000000000000000000000000000200000000000000000000000000100000000000000000002000000020000000000000180000000000240c000100000008000060000000000000000000000000000000000000000000000000c0000000000000000080028401c9c3808325da7a8468b97c7980a01735d51a6bf99e813a40505ea196a5b79e0ab7d9d0dfb579ecee9499bccca784880000000000000000843455cb4aa056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b4218080a00000000000000000000000000000000000000000000000000000000000000000a0e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);
        gasArchive.setLastKnownSeqChainBlockHashForTesting(
            SEQ_CHAIN_ID, 0x55c3e74a2dec0e3d150636b57e5c988c570215255b1b7670e9366914ba597018
        );

        // Confirm epoch data hash
        gasArchive.confirmEpochDataHash(EPOCH, SEQ_CHAIN_ID, seqChainHeader, accountProofArray, storageProofArray);

        // Now try to submit incorrect pre-image data
        uint256[] memory wrongAppchains = new uint256[](1);
        wrongAppchains[0] = 999; // Different from the fixture data

        uint256[] memory wrongTokens = new uint256[](1);
        wrongTokens[0] = 999; // Different from the fixture data

        address[] memory wrongEmissionsReceivers = new address[](1);
        wrongEmissionsReceivers[0] = makeAddr("wrongReceiver");

        vm.expectRevert(GasArchive.InvalidData.selector);
        gasArchive.submitEpochPreImageData(EPOCH, SEQ_CHAIN_ID, wrongAppchains, wrongTokens, wrongEmissionsReceivers);
    }

    /*//////////////////////////////////////////////////////////////
                    SEQ CHAIN BLOCK HASH TESTS
    //////////////////////////////////////////////////////////////*/

    function testConfirmSequencingChainBlockHashWithValidProof() public {
        // proof generated using the following rust code:
        //

        // use alloy::{
        //     primitives::{keccak256, Address, Bytes, FixedBytes, StorageKey},
        //     providers::ProviderBuilder,
        //     rlp::Encodable,
        //     sol,
        //     sol_types::{SolEvent, SolValue},
        // };
        //
        // // the rollup Outbox contract
        // let outbox_contract: Address = "0xD4B80C3D7240325D18E645B49e6535A3Bf95cc58".parse().unwrap();
        // let roots_mapping_storage_slot_index = U256::from(3);
        // let rpc_url = "<ETHEREUM_RPC_URL>";
        //
        // let provider = ProviderBuilder::new().connect(rpc_url).await.unwrap();
        // let eth_block = provider.get_block(BlockId::latest()).await.unwrap().unwrap();
        // let mut header_rlp = vec![];
        // eth_block.header.encode(&mut header_rlp);
        //
        // sol! {
        //     event SendRootUpdated(bytes32 indexed outputRoot, bytes32 indexed l2BlockHash);
        // }
        //
        // // search the 1000 previous blocks for the SendRootUpdated event
        // let filter = alloy::rpc::types::Filter::new()
        //     .address(outbox_contract)
        //     .event_signature(SendRootUpdated::SIGNATURE_HASH)
        //     .from_block(BlockNumberOrTag::Number(eth_block.number() - 1000))
        //     .to_block(BlockNumberOrTag::Number(eth_block.number()));
        //
        // let logs = provider.get_logs(&filter).await.unwrap();
        //
        // let last_log = logs.last().unwrap_or_else(|| panic!("No events found"));
        // let parsed = SendRootUpdated::decode_log_data(last_log.data()).unwrap();
        //
        // let storage_key: StorageKey =
        //     keccak256((parsed.outputRoot, roots_mapping_storage_slot_index).abi_encode());
        //
        // let proof = provider
        //     .get_proof(outbox_contract, vec![storage_key])
        //     .block_id(eth_block.hash().into())
        //     .await
        //     .unwrap();
        //
        // let storage_proof_value: FixedBytes<32> = proof.storage_proof.first().unwrap().value.into();
        // assert_eq!(storage_proof_value, parsed.l2BlockHash); //sanity check
        //
        // #[derive(serde::Serialize)]
        // struct TestFixture {
        //     ethereum_block_hash: FixedBytes<32>,
        //     outbox_contract_address: Address,
        //     send_root_storage_slot_index: U256,
        //     rollup_block_hash: FixedBytes<32>,
        //     send_root: Bytes,
        //     eth_block_header_rlp: Bytes,
        //     account_proof: Vec<Bytes>,
        //     storage_proof: Vec<Bytes>,
        // }
        //
        // let fixture = TestFixture {
        //     send_root: parsed.outputRoot.into(),
        //     eth_block_header_rlp: header_rlp.into(),
        //     account_proof: proof.account_proof.clone(),
        //     storage_proof: proof.storage_proof.first().unwrap().proof.clone(),
        //     ethereum_block_hash: eth_block.hash(),
        //     outbox_contract_address: outbox_contract,
        //     send_root_storage_slot_index: roots_mapping_storage_slot_index,
        //     rollup_block_hash: parsed.l2BlockHash,
        // };
        // println!("{}", serde_json::to_string_pretty(&fixture).unwrap());

        string memory proofJson = vm.readFile("./test/staking/fixtures/arbRollupProof.json");

        uint256 storageSlot = vm.parseJsonUint(proofJson, ".send_root_storage_slot_index");
        address arbNovaOutboxContract = vm.parseJsonAddress(proofJson, ".outbox_contract_address");
        bytes32 sendRoot = vm.parseJsonBytes32(proofJson, ".send_root");
        bytes32 ethereumBlockHash = vm.parseJsonBytes32(proofJson, ".ethereum_block_hash");
        bytes32 rollupBlockHash = vm.parseJsonBytes32(proofJson, ".rollup_block_hash");
        bytes memory ethBlockHeaderRLP = vm.parseJsonBytes(proofJson, ".eth_block_header_rlp");
        bytes[] memory accountProofArray = vm.parseJsonBytesArray(proofJson, ".account_proof");
        bytes[] memory storageProofArray = vm.parseJsonBytesArray(proofJson, ".storage_proof");

        // Add Arbitrum Nova as a sequencing chain
        uint256 arbNovaChainId = 42170; // Arbitrum Nova chain ID
        vm.prank(admin);
        gasArchive.addSequencingChain(arbNovaChainId, makeAddr("gasAggregator"), arbNovaOutboxContract, storageSlot);

        // Set the last known Ethereum block hash (this would come from the bridge in practice)
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(ethereumBlockHash, keccak256("set chain bloch hash"), 1);

        gasArchive.confirmSequencingChainBlockHash(
            arbNovaChainId, sendRoot, ethBlockHeaderRLP, accountProofArray, storageProofArray
        );
        // Assert the proof has successfully confimed the sequencing chain block hash
        assertEq(gasArchive.lastKnownSeqChainBlockHashes(arbNovaChainId), rollupBlockHash);
    }

    function testConfirmSequencingChainBlockHashInvalidEthereumBlockHeader() public {
        bytes memory mockEthHeader = abi.encode("invalid_eth_header");
        bytes[] memory mockAccountProof = new bytes[](0);
        bytes[] memory mockStorageProof = new bytes[](0);

        vm.expectRevert(GasArchive.InvalidEthereumBlockHeader.selector);
        gasArchive.confirmSequencingChainBlockHash(
            SEQ_CHAIN_ID, TEST_SEQ_BLOCK_HASH, mockEthHeader, mockAccountProof, mockStorageProof
        );
    }

    function testConfirmSequencingChainBlockHashChainIDNotFound() public {
        uint256 invalidChainId = 999;

        // Set ethereum block hash first to avoid InvalidEthereumBlockHeader
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);

        // We need to create a header that when hashed equals TEST_ETH_BLOCK_HASH
        // Since TEST_ETH_BLOCK_HASH = keccak256("eth_block"), we use that exact data
        bytes memory mockEthHeader = "eth_block";
        bytes[] memory mockAccountProof = new bytes[](0);
        bytes[] memory mockStorageProof = new bytes[](0);

        vm.expectRevert(GasArchive.ChainIDNotFound.selector);
        gasArchive.confirmSequencingChainBlockHash(
            invalidChainId, TEST_SEQ_BLOCK_HASH, mockEthHeader, mockAccountProof, mockStorageProof
        );
    }

    function testCannotSubmitBlockHashProofForSettlementChain() public {
        // Setup settlement chain as sequencing chain
        vm.prank(admin);
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, makeAddr("settlementAggregator"), address(0), 0);

        // Set ethereum block hash first to avoid InvalidEthereumBlockHeader
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);

        bytes memory mockEthHeader = "eth_block";
        bytes[] memory mockAccountProof = new bytes[](0);
        bytes[] memory mockStorageProof = new bytes[](0);

        vm.expectRevert(GasArchive.CannotSubmitProofForSettlementChain.selector);
        gasArchive.confirmSequencingChainBlockHash(
            SETTLEMENT_CHAIN_ID, TEST_SEQ_BLOCK_HASH, mockEthHeader, mockAccountProof, mockStorageProof
        );
    }

    /*//////////////////////////////////////////////////////////////
                        GETTER TESTS
    //////////////////////////////////////////////////////////////*/

    function testConstants() public view {
        assertEq(gasArchive.AGGREGATED_EPOCH_DATA_HASH_SLOT(), 7);
        assertEq(gasArchive.HEADER_STATE_ROOT_INDEX(), 3);
        assertEq(gasArchive.STORAGE_ROOT_ACCOUNT_FIELDS_INDEX(), 2);
    }

    function testInitialState() public view {
        assertEq(gasArchive.blockHashSender(), blockHashSender);
        assertEq(gasArchive.settlementChainID(), SETTLEMENT_CHAIN_ID);
        assertEq(gasArchive.lastKnownEthereumBlockHash(), bytes32(0));
        assertEq(gasArchive.lastKnownSettlementChainBlockHash(), bytes32(0));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), address(0));
    }

    function testSeqChainConfiguration() public view {
        assertEq(
            gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0)
        );
        assertEq(gasArchive.seqChainEthOutbox(SEQ_CHAIN_ID), address(mockBridge));
        assertEq(gasArchive.seqChainEthSendRootStorageSlot(SEQ_CHAIN_ID), TEST_STORAGE_SLOT_INDEX);
    }

    function testEpochDataInitiallyEmpty() public view {
        assertFalse(gasArchive.epochCompleted(EPOCH));
        assertEq(gasArchive.epochTotalTokensUsed(EPOCH), 0);
        assertEq(gasArchive.epochAppchainTokensUsed(EPOCH, APPCHAIN_ID_1), 0);
        assertEq(gasArchive.epochAppchainEmissionsReceiver(EPOCH, APPCHAIN_ID_1), address(0));
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
        gasArchive.getActiveAppchainIds(EPOCH);
    }

    function testGetAppchainRewardsReceiverNotArchivedEpoch() public {
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_1);
    }

    function testViewFunctionsWithArchivedData() public {
        // Create test data
        uint256[] memory appchainIds = new uint256[](2);
        appchainIds[0] = APPCHAIN_ID_1;
        appchainIds[1] = APPCHAIN_ID_2;

        uint256[] memory gasUsageAmounts = new uint256[](2);
        gasUsageAmounts[0] = 1000;
        gasUsageAmounts[1] = 2000;

        address[] memory rewardsReceivers = new address[](2);
        rewardsReceivers[0] = makeAddr("receiver1");
        rewardsReceivers[1] = makeAddr("receiver2");

        // Set archived data using helper contract
        gasArchive.setArchivedEpochDataForTesting(EPOCH, appchainIds, gasUsageAmounts, rewardsReceivers);

        // Test getAppchainGasFees
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 1000);
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 2000);

        // Test getTotalGasFees
        assertEq(gasArchive.getTotalGasFees(EPOCH), 3000);

        // Test getActiveAppchainIds
        uint256[] memory activeAppchains = gasArchive.getActiveAppchainIds(EPOCH);
        assertEq(activeAppchains.length, 2);
        assertEq(activeAppchains[0], APPCHAIN_ID_1);
        assertEq(activeAppchains[1], APPCHAIN_ID_2);

        // Test getAppchainRewardsReceiver
        assertEq(gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_1), rewardsReceivers[0]);
        assertEq(gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_2), rewardsReceivers[1]);
    }

    function testGetAppchainGasFeesZeroForNonExistentAppchain() public {
        // Create test data with only one appchain
        uint256[] memory appchainIds = new uint256[](1);
        appchainIds[0] = APPCHAIN_ID_1;

        uint256[] memory gasUsageAmounts = new uint256[](1);
        gasUsageAmounts[0] = 1500;

        address[] memory rewardsReceivers = new address[](1);
        rewardsReceivers[0] = makeAddr("receiver1");

        gasArchive.setArchivedEpochDataForTesting(EPOCH, appchainIds, gasUsageAmounts, rewardsReceivers);

        // Test existing appchain
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_1), 1500);

        // Test non-existent appchain returns 0
        assertEq(gasArchive.getAppchainGasFees(EPOCH, APPCHAIN_ID_2), 0);
    }

    function testGetAppchainRewardsReceiverZeroForNonExistentAppchain() public {
        // Create test data with only one appchain
        uint256[] memory appchainIds = new uint256[](1);
        appchainIds[0] = APPCHAIN_ID_1;

        uint256[] memory gasUsageAmounts = new uint256[](1);
        gasUsageAmounts[0] = 1500;

        address[] memory rewardsReceivers = new address[](1);
        rewardsReceivers[0] = makeAddr("receiver1");

        gasArchive.setArchivedEpochDataForTesting(EPOCH, appchainIds, gasUsageAmounts, rewardsReceivers);

        // Test existing appchain
        assertEq(gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_1), rewardsReceivers[0]);

        // Test non-existent appchain returns zero address
        assertEq(gasArchive.getAppchainRewardsReceiver(EPOCH, APPCHAIN_ID_2), address(0));
    }

    /*//////////////////////////////////////////////////////////////
                    EPOCH COMPLETION TRACKING TESTS
    //////////////////////////////////////////////////////////////*/

    function testEpochCompletionWithMultipleSequencingChains() public {
        uint256 epoch = 100;
        uint256 chain2 = 999;
        uint256 chain3 = 888;

        // Add additional sequencing chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chain2, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        gasArchive.addSequencingChain(chain3, address(0x2), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        vm.stopPrank();

        // Set up block hashes for all chains
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);
        gasArchive.setLastKnownSeqChainBlockHashForTesting(SEQ_CHAIN_ID, TEST_SEQ_BLOCK_HASH);
        gasArchive.setLastKnownSeqChainBlockHashForTesting(chain2, keccak256("chain2_block"));
        gasArchive.setLastKnownSeqChainBlockHashForTesting(chain3, keccak256("chain3_block"));

        // Initially epoch should not be completed
        assertFalse(gasArchive.epochCompleted(epoch));

        // Test data for each chain
        uint256[] memory appchains = new uint256[](1);
        appchains[0] = APPCHAIN_ID_1;
        uint256[] memory tokens = new uint256[](1);
        tokens[0] = 100;
        address[] memory emissionsReceivers = new address[](1);
        emissionsReceivers[0] = address(0x123);

        // Simulate confirmEpochDataHash for first chain (SEQ_CHAIN_ID)
        // This will create the snapshot of expected chains
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, SEQ_CHAIN_ID, true);
        gasArchive.addExpectedChainForTesting(epoch, SEQ_CHAIN_ID);
        gasArchive.addExpectedChainForTesting(epoch, chain2);
        gasArchive.addExpectedChainForTesting(epoch, chain3);

        // Check that the chain submitted but epoch is not yet complete
        assertTrue(gasArchive.hasChainSubmittedForEpoch(epoch, SEQ_CHAIN_ID));
        assertFalse(gasArchive.epochCompleted(epoch));

        // Get epoch progress
        (bool completed, uint256 totalExpected, uint256 totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertFalse(completed);
        assertEq(totalExpected, 3);
        assertEq(totalSubmitted, 1);

        // Submit for second chain
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, chain2, true);

        // Still should not be completed
        assertFalse(gasArchive.epochCompleted(epoch));
        (completed, totalExpected, totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertFalse(completed);
        assertEq(totalExpected, 3);
        assertEq(totalSubmitted, 2);

        // Submit for third chain - now should be completed
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, chain3, true);

        // Manually call the completion check (normally done automatically in confirmEpochDataHash)
        uint256[] memory expectedChains = gasArchive.getEpochExpectedChains(epoch);
        bool allSubmitted = true;
        for (uint256 i = 0; i < expectedChains.length; i++) {
            if (!gasArchive.hasChainSubmittedForEpoch(epoch, expectedChains[i])) {
                allSubmitted = false;
                break;
            }
        }

        // Verify all chains have submitted
        assertTrue(allSubmitted);
        assertTrue(gasArchive.hasChainSubmittedForEpoch(epoch, SEQ_CHAIN_ID));
        assertTrue(gasArchive.hasChainSubmittedForEpoch(epoch, chain2));
        assertTrue(gasArchive.hasChainSubmittedForEpoch(epoch, chain3));

        // Check expected chains were saved correctly
        uint256[] memory expectedChainsFromContract = gasArchive.getEpochExpectedChains(epoch);
        assertEq(expectedChainsFromContract.length, 3);
        assertEq(expectedChainsFromContract[0], SEQ_CHAIN_ID);
        assertEq(expectedChainsFromContract[1], chain2);
        assertEq(expectedChainsFromContract[2], chain3);
    }

    function testGetNewViewFunctions() public {
        uint256 epoch = 200;

        // Initially, no data should be available
        uint256[] memory expectedChains = gasArchive.getEpochExpectedChains(epoch);
        assertEq(expectedChains.length, 0);

        assertFalse(gasArchive.hasChainSubmittedForEpoch(epoch, SEQ_CHAIN_ID));

        (bool completed, uint256 totalExpected, uint256 totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertFalse(completed);
        assertEq(totalExpected, 0);
        assertEq(totalSubmitted, 0);
    }

    function testViewFunctionsRevertForIncompleteEpoch() public {
        uint256 epoch = 300;
        uint256 chain2 = 999;

        // Add another sequencing chain
        vm.prank(admin);
        gasArchive.addSequencingChain(chain2, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);

        // Manually set up partial epoch data (one chain submitted, one hasn't)
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, SEQ_CHAIN_ID, true);
        gasArchive.addExpectedChainForTesting(epoch, SEQ_CHAIN_ID);
        gasArchive.addExpectedChainForTesting(epoch, chain2);
        // Note: epochCompleted[epoch] remains false

        // IGasDataProvider view functions should revert
        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainGasFees(epoch, APPCHAIN_ID_1);

        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getTotalGasFees(epoch);

        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getActiveAppchainIds(epoch);

        vm.expectRevert(GasArchive.NotArchivedEpoch.selector);
        gasArchive.getAppchainRewardsReceiver(epoch, APPCHAIN_ID_1);

        // But our new utility functions should work
        assertTrue(gasArchive.hasChainSubmittedForEpoch(epoch, SEQ_CHAIN_ID));
        assertFalse(gasArchive.hasChainSubmittedForEpoch(epoch, chain2));

        (bool completed, uint256 totalExpected, uint256 totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertFalse(completed);
        assertEq(totalExpected, 2);
        assertEq(totalSubmitted, 1);
    }

    /*//////////////////////////////////////////////////////////////
                    ADMIN OVERRIDE FUNCTIONALITY TESTS
    //////////////////////////////////////////////////////////////*/

    function testSetEpochExpectedChains() public {
        uint256 epoch = 500;
        uint256[] memory chainIds = new uint256[](3);
        chainIds[0] = SEQ_CHAIN_ID;
        chainIds[1] = 999;
        chainIds[2] = 888;

        // Initially no expected chains
        uint256[] memory initialExpected = gasArchive.getEpochExpectedChains(epoch);
        assertEq(initialExpected.length, 0);

        // Admin sets expected chains
        vm.expectEmit(true, false, false, true);
        emit EpochExpectedChainsUpdated(epoch, chainIds);

        vm.prank(admin);
        gasArchive.setEpochExpectedChains(epoch, chainIds);

        // Verify expected chains were set
        uint256[] memory expected = gasArchive.getEpochExpectedChains(epoch);
        assertEq(expected.length, 3);
        assertEq(expected[0], SEQ_CHAIN_ID);
        assertEq(expected[1], 999);
        assertEq(expected[2], 888);

        // Epoch should not be completed yet (no submissions)
        assertFalse(gasArchive.epochCompleted(epoch));
    }

    function testSetEpochExpectedChainsUnauthorized() public {
        uint256 epoch = 500;
        uint256[] memory chainIds = new uint256[](1);
        chainIds[0] = SEQ_CHAIN_ID;

        vm.prank(user);
        vm.expectRevert();
        gasArchive.setEpochExpectedChains(epoch, chainIds);
    }

    function testSetEpochExpectedChainsTriggersCompletion() public {
        uint256 epoch = 500;
        uint256[] memory chainIds = new uint256[](1);
        chainIds[0] = SEQ_CHAIN_ID;

        // First submit data for the chain
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, SEQ_CHAIN_ID, true);

        // Now set expected chains - this should trigger completion
        vm.expectEmit(true, false, false, false);
        emit EpochCompleted(epoch);

        vm.prank(admin);
        gasArchive.setEpochExpectedChains(epoch, chainIds);

        // Epoch should be completed
        assertTrue(gasArchive.epochCompleted(epoch));
    }

    function testAdminCanOverrideSnapshotLogic() public {
        uint256 epoch = 700;
        uint256 chain2 = 999;
        uint256 chain3 = 888;

        // Add additional sequencing chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chain2, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        gasArchive.addSequencingChain(chain3, address(0x2), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        vm.stopPrank();

        // Simulate first submission creating snapshot of all 3 chains (SEQ_CHAIN_ID + chain2 + chain3)
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, SEQ_CHAIN_ID, true);
        gasArchive.addExpectedChainForTesting(epoch, SEQ_CHAIN_ID);
        gasArchive.addExpectedChainForTesting(epoch, chain2);
        gasArchive.addExpectedChainForTesting(epoch, chain3);

        // Verify snapshot was created
        uint256[] memory expectedBefore = gasArchive.getEpochExpectedChains(epoch);
        assertEq(expectedBefore.length, 3);

        // Only one chain has submitted so far
        (bool completed, uint256 totalExpected, uint256 totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertFalse(completed);
        assertEq(totalExpected, 3);
        assertEq(totalSubmitted, 1);

        // Admin decides chain3 won't submit data for this epoch, so removes it
        uint256[] memory newExpected = new uint256[](2);
        newExpected[0] = SEQ_CHAIN_ID;
        newExpected[1] = chain2;

        vm.prank(admin);
        gasArchive.setEpochExpectedChains(epoch, newExpected);

        // Verify expected chains were updated
        uint256[] memory expectedAfter = gasArchive.getEpochExpectedChains(epoch);
        assertEq(expectedAfter.length, 2);
        assertEq(expectedAfter[0], SEQ_CHAIN_ID);
        assertEq(expectedAfter[1], chain2);

        // Now submit data for chain2
        gasArchive.setEpochChainDataSubmittedForTesting(epoch, chain2, true);

        // Manually check completion (since we can't call the internal function directly)
        (completed, totalExpected, totalSubmitted) = gasArchive.getEpochProgress(epoch);
        assertEq(totalExpected, 2);
        assertEq(totalSubmitted, 2);
    }

    /*//////////////////////////////////////////////////////////////
                         OTHER TESTS
    //////////////////////////////////////////////////////////////*/

    function testMultipleSequencingChains() public {
        uint256 chainId2 = 999;
        uint256 chainId3 = 888;

        // Add multiple chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chainId2, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        gasArchive.addSequencingChain(chainId3, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);
        vm.stopPrank();

        // Verify they're all configured
        assertEq(
            gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0)
        );
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId2), address(0x1));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId3), address(0x1));

        // Remove middle chain
        vm.prank(admin);
        gasArchive.removeSeqChain(chainId2);

        // Verify removal
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId2), address(0));
        // Others should still exist
        assertEq(
            gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0)
        );
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId3), address(0x1));
    }

    function testAccessControl() public {
        // Test that only admin can perform admin functions
        vm.startPrank(user);

        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(0x1), address(mockBridge), TEST_STORAGE_SLOT_INDEX);

        vm.expectRevert();
        gasArchive.removeSeqChain(SEQ_CHAIN_ID);

        vm.expectRevert();
        gasArchive.setBlockHashSender(user);

        vm.stopPrank();

        // Test that blockHashSender can only set block hashes
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH, 1);
    }
}
