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
        archivedEpochData[epoch] = true;
    }

    function setLastKnownSeqChainBlockHashForTesting(uint256 seqChainId, bytes32 blockHash) external {
        lastKnownSeqChainBlockHashes[seqChainId] = blockHash;
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
    uint256 public constant SEQ_CHAIN_ID = 31337; // matches the expcted values in testConfirmEpochDataHashSuccess
    uint256 public constant APPCHAIN_ID_1 = 123;
    uint256 public constant APPCHAIN_ID_2 = 456;
    uint256 public constant EPOCH = 10; // matches the expcted values in testConfirmEpochDataHashSuccess

    bytes32 public constant TEST_ETH_BLOCK_HASH = keccak256("eth_block");
    bytes32 public constant TEST_SETTLEMENT_BLOCK_HASH = keccak256("settlement_block");
    bytes32 public constant TEST_SEQ_BLOCK_HASH = keccak256("seq_block");
    bytes32 public constant TEST_STORAGE_SLOT = keccak256("storage_slot");

    event EpochDataValidated(uint256 indexed epoch, uint256 indexed seqChainID, bytes32 dataHash);
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
        address gasArchiveAddress = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0); // matches the expcted values in testConfirmEpochDataHashSuccess
        gasArchive.addSequencingChain(SEQ_CHAIN_ID, gasArchiveAddress, address(mockBridge), TEST_STORAGE_SLOT);
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
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

        assertEq(gasArchive.lastKnownEthereumBlockHash(), TEST_ETH_BLOCK_HASH);
        assertEq(gasArchive.lastKnownSettlementChainBlockHash(), TEST_SETTLEMENT_BLOCK_HASH);
    }

    function testSetLastKnownBlockHashesUnauthorized() public {
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);
    }

    /*//////////////////////////////////////////////////////////////
                    SEQUENCING CHAIN MANAGEMENT TESTS
    //////////////////////////////////////////////////////////////*/

    function testAddSequencingChain() public {
        uint256 newChainId = 789;
        address newAggregator = makeAddr("newAggregator");
        address newBridge = makeAddr("newBridge");
        bytes32 newStorageSlot = keccak256("new_slot");

        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, newAggregator, newBridge, newStorageSlot);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(newChainId), newAggregator);
        assertEq(gasArchive.ethereumSeqChainBridges(newChainId), newBridge);
        assertEq(gasArchive.ethereumSeqChainStorageSlots(newChainId), newStorageSlot);
    }

    function testAddSequencingChainAsSettlementChain() public {
        address settlementAggregator = makeAddr("settlementAggregator");

        vm.prank(admin);
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, settlementAggregator, address(0), bytes32(0));

        assertEq(gasArchive.seqChainGasAggregatorAddresses(SETTLEMENT_CHAIN_ID), settlementAggregator);
        assertTrue(gasArchive.useSettlementChainAsSequencingChain());
    }

    function testAddSequencingChainAlreadyExists() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.SequencingChainAlreadyExists.selector);
        gasArchive.addSequencingChain(SEQ_CHAIN_ID, address(mockGasAggregator), address(mockBridge), TEST_STORAGE_SLOT);
    }

    function testAddSequencingChainZeroAggregator() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(0), address(mockBridge), TEST_STORAGE_SLOT);
    }

    function testAddSequencingChainZeroBridge() public {
        vm.prank(admin);
        vm.expectRevert(GasArchive.ZeroAddress.selector);
        gasArchive.addSequencingChain(999, address(0x1), address(0), TEST_STORAGE_SLOT);
    }

    function testAddSequencingChainUnauthorized() public {
        vm.prank(user);
        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(0x1), address(mockBridge), TEST_STORAGE_SLOT);
    }

    function testRemoveSequencingChain() public {
        // First add a new chain to remove
        uint256 newChainId = 789;
        vm.prank(admin);
        gasArchive.addSequencingChain(newChainId, address(0x1), address(mockBridge), TEST_STORAGE_SLOT);

        // Remove it
        vm.prank(admin);
        gasArchive.removeSeqChain(newChainId);

        assertEq(gasArchive.seqChainGasAggregatorAddresses(newChainId), address(0));
        assertEq(gasArchive.ethereumSeqChainBridges(newChainId), address(0));
        assertEq(gasArchive.ethereumSeqChainStorageSlots(newChainId), bytes32(0));
    }

    function testRemoveSettlementChainAsSequencing() public {
        // First add settlement chain as sequencing
        vm.prank(admin);
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, address(0x1), address(0), bytes32(0));

        assertTrue(gasArchive.useSettlementChainAsSequencingChain());

        // Remove it
        vm.prank(admin);
        gasArchive.removeSeqChain(SETTLEMENT_CHAIN_ID);

        assertFalse(gasArchive.useSettlementChainAsSequencingChain());
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
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

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
            EPOCH,
            SEQ_CHAIN_ID,
            abi.encode("mock_header"),
            mockAccountProof,
            mockStorageProof,
            appchains,
            tokens,
            emissionsReceivers
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
        string memory proofJson = vm.readFile("./test/staking/fixtures/arbRollupProof.json");

        // Parse JSON arrays directly
        bytes[] memory accountProofArray = vm.parseJsonBytesArray(proofJson, ".accountProof");
        bytes[] memory storageProofArray = vm.parseJsonBytesArray(proofJson, ".storageProof[0].proof");

        // RLP encoded block header obtained with the followin rust code:
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

        // Now the test should pass with valid proofs
        vm.expectEmit(true, false, false, true);
        emit EpochDataValidated(EPOCH, SEQ_CHAIN_ID, bytes32(vm.parseJsonBytes(proofJson, ".storageProof[0].value")));

        gasArchive.confirmEpochDataHash(
            EPOCH,
            SEQ_CHAIN_ID,
            seqChainHeader,
            accountProofArray,
            storageProofArray,
            appchains,
            tokens,
            emissionsReceivers
        );

        // Assert the epoch data was stored correctly
        assertTrue(gasArchive.archivedEpochData(EPOCH), "Epoch should be marked as archived");

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
    }

    function testConfirmEpochDataHashInvalidSeqChainBlockHeader() public {
        uint256[] memory appchains = new uint256[](1);
        appchains[0] = APPCHAIN_ID_1;

        uint256[] memory tokens = new uint256[](1);
        tokens[0] = 100;

        address[] memory emissionsReceivers = new address[](1);
        emissionsReceivers[0] = makeAddr("receiver1");

        bytes memory mockHeader = abi.encode("invalid_header");
        bytes[] memory mockAccountProof = new bytes[](0);
        bytes[] memory mockStorageProof = new bytes[](0);

        vm.expectRevert(GasArchive.InvalidSeqChainBlockHeader.selector);
        gasArchive.confirmEpochDataHash(
            EPOCH, SEQ_CHAIN_ID, mockHeader, mockAccountProof, mockStorageProof, appchains, tokens, emissionsReceivers
        );
    }

    /*//////////////////////////////////////////////////////////////
                    SEQ CHAIN BLOCK HASH TESTS
    //////////////////////////////////////////////////////////////*/

    function testConfirmSequencingChainBlockHashWithValidProof() public {
        // proof generated using Arbitrum Nova deployment on Ethereum with the following info:
        // block number: 23297133 hash: 0x5df3e2c7aafac99082a4538843b361f2502e48d034677dce776e7d7c9587cf35
        // Rollup Contract: https://etherscan.io/address/0xE7E8cCC7c381809BDC4b213CE44016300707B7Bd
        // storage_slot: 116 - see nitro-contracts at v2.1.0 (RollupCore contract)
        // arbitrum nova assertion: 0x6e4b03dae0c2f93a95ad7eb04805564a345c2f200a87694eac0eefea9740a4fd
        // arbitrum nova block hash: ?

        // Setup: Add Arbitrum Nova as a sequencing chain
        uint256 arbNovaChainId = 42170; // Arbitrum Nova chain ID
        address arbNovaRollupContract = 0xE7E8cCC7c381809BDC4b213CE44016300707B7Bd;
        bytes32 storageSlot = bytes32(uint256(116)); // slot for confirmedNodeHash

        vm.prank(admin);
        gasArchive.addSequencingChain(arbNovaChainId, makeAddr("gasAggregator"), arbNovaRollupContract, storageSlot);

        // Load fixture data
        string memory proofJson = vm.readFile("./test/staking/fixtures/arbRollupProof.json");

        // Parse the proof data
        bytes[] memory accountProofArray = vm.parseJsonBytesArray(proofJson, ".accountProof");
        bytes[] memory storageProofArray = vm.parseJsonBytesArray(proofJson, ".storageProof[0].proof");
        string memory storageValueStr = vm.parseJsonString(proofJson, ".storageProof[0].value");

        // Set the last known Ethereum block hash (this would come from the bridge in practice)
        // We need to calculate the block header hash that would match the account proof's root
        bytes32 ethereumBlockHash = 0x5df3e2c7aafac99082a4538843b361f2502e48d034677dce776e7d7c9587cf35;
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(ethereumBlockHash, TEST_SETTLEMENT_BLOCK_HASH);

        // obtained using rust code, same method as `testConfirmEpochDataHashSuccess`
        bytes memory ethereumBlockHeader =
            hex"f9027ea04155c48e3ccc5b913d13722a9d2c4274097c11f9abe4d1649800cd8a75ba1994a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347945995510b29924a0c68e5e266687c81af6a06abb43268378b2c475de61a46a90538e8a0efdac8fbf57e76e6d5905f0fac94a5783e7402e675a50efee418f0039617bd33b9010028ff328352b344aa028b24adca61b622f360e2a15115c5099b63283252009a1bb8636c59be951c54cb42a00e2c4b279a86e64110c12f2070a0c0e8edc65d89afa05ade68bbc41c73c0dcd4800d28fccfff87582e25e713ab3517336644680a472d67279aba4cad4ce88cc0c29b2081d54a41ee4a7070306b5256b4213b19300390c54480b1a53b008660e256be8604515503808c55a225fc808401637c6d8402aea540839037008468bae223924e65746865726d696e6420e454a0f4da304510dddcdf8186909d7cb74e2e07a1e703dd7baa27eeeada9a014629ad8312000083040000a004dad78a8451df8a2911991c5a6bcadb0fd9d8f8ff9d8135e6fa0de127ffa7dfa0e3b0c442";

        // The test should revert with EmptySlot since the proven storage value is 0
        vm.expectRevert(GasArchive.EmptySlot.selector);
        gasArchive.confirmSequencingChainBlockHash(
            arbNovaChainId,
            0x6e4b03dae0c2f93a95ad7eb04805564a345c2f200a87694eac0eefea9740a4fd,
            ethereumBlockHeader,
            accountProofArray,
            storageProofArray
        );
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
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

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
        gasArchive.addSequencingChain(SETTLEMENT_CHAIN_ID, address(mockGasAggregator), address(0), bytes32(0));

        // Set ethereum block hash first to avoid InvalidEthereumBlockHeader
        vm.prank(blockHashSender);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);

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
        assertFalse(gasArchive.useSettlementChainAsSequencingChain());
    }

    function testSeqChainConfiguration() public view {
        assertEq(gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x1));
        assertEq(gasArchive.ethereumSeqChainBridges(SEQ_CHAIN_ID), address(mockBridge));
        assertEq(gasArchive.ethereumSeqChainStorageSlots(SEQ_CHAIN_ID), TEST_STORAGE_SLOT);
    }

    function testEpochDataInitiallyEmpty() public view {
        assertFalse(gasArchive.archivedEpochData(EPOCH));
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
                         OTHER TESTS
    //////////////////////////////////////////////////////////////*/

    function testMultipleSequencingChains() public {
        uint256 chainId2 = 999;
        uint256 chainId3 = 888;

        // Add multiple chains
        vm.startPrank(admin);
        gasArchive.addSequencingChain(chainId2, address(0x1), address(mockBridge), TEST_STORAGE_SLOT);
        gasArchive.addSequencingChain(chainId3, address(0x1), address(mockBridge), TEST_STORAGE_SLOT);
        vm.stopPrank();

        // Verify they're all configured
        assertEq(gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x1));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId2), address(0x1));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId3), address(0x1));

        // Remove middle chain
        vm.prank(admin);
        gasArchive.removeSeqChain(chainId2);

        // Verify removal
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId2), address(0));
        // Others should still exist
        assertEq(gasArchive.seqChainGasAggregatorAddresses(SEQ_CHAIN_ID), address(0x1));
        assertEq(gasArchive.seqChainGasAggregatorAddresses(chainId3), address(0x1));
    }

    function testAccessControl() public {
        // Test that only admin can perform admin functions
        vm.startPrank(user);

        vm.expectRevert();
        gasArchive.addSequencingChain(999, address(0x1), address(mockBridge), TEST_STORAGE_SLOT);

        vm.expectRevert();
        gasArchive.removeSeqChain(SEQ_CHAIN_ID);

        vm.expectRevert();
        gasArchive.setBlockHashSender(user);

        vm.stopPrank();

        // Test that blockHashSender can only set block hashes
        vm.prank(user);
        vm.expectRevert(GasArchive.NotBlockHashSender.selector);
        gasArchive.setLastKnownBlockHashes(TEST_ETH_BLOCK_HASH, TEST_SETTLEMENT_BLOCK_HASH);
    }
}
