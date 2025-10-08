// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {MerklePatriciaProofVerifier} from "./lib/MerklePatriciaProofVerifier.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {RLPReader} from "./lib/RLPReader.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

/// @title GasArchive
/// @notice Lives on the staking appchain and trustlessly validates and stores gas usage data from multiple sequencing chains using storage proofs
/// @dev This contract supports arbitrum-based sequencing chains only (with the exception of the settlement chain, which can be any chain)
contract GasArchive is AccessControl, IGasDataProvider, EpochTracker {
    using EnumerableSet for EnumerableSet.UintSet;
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    /*//////////////////////////////////////////////////////////////
                            CONSTANTS
    //////////////////////////////////////////////////////////////*/
    /// @notice Storage slot of aggregatedEpochDataHash in GasAggregator (slot 0) (see `forge inspect GasAggregator storageLayout`)
    uint256 public constant AGGREGATED_EPOCH_DATA_HASH_SLOT = 0;
    uint256 public constant HEADER_STATE_ROOT_INDEX = 3;
    uint256 public constant STORAGE_ROOT_ACCOUNT_FIELDS_INDEX = 2;

    /// @notice Storage slot of roots in AbsOutbox (slot 3) (see `forge inspect AbsOutbox storageLayout`)
    uint256 public constant SEND_ROOT_STORAGE_SLOT = 3;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @dev The `BlockHashRelayer` contract is deployed on the settlement chain and is responsible for sending the block hashes to the `GasArchive` contract. Anyone can call `sendBlockHashes` on the relayer to send the block hashes.
    address public blockHashSender;

    /// @notice when using the settlement chain as the sequencing chain, the rollup hash proof is not required
    uint256 public immutable settlementChainID;

    /// @notice the latest epoch that the contract is aware of
    uint256 public latestEpoch;

    /// @notice the sequencing chain count for the latest epoch
    uint256 public seqChainCount;

    mapping(uint256 chainId => uint256 epoch) chainAdded;

    /// @notice mapping of sequencing chain IDs to the address of the gas aggregator contract
    mapping(uint256 chainId => address aggregatorAddress) public seqChainGasAggregatorAddresses;
    /// @notice mapping of sequencing chain IDs to the address of the Outbox contract for that sequencing chain (where the confirmed rollup hash can be found)
    mapping(uint256 chainId => address outboxAddress) public seqChainOutbox;
    mapping(uint256 chainId => bool) public seqChainSettlesToBase;
    /// @notice block hashes for l1 and settlement chains
    mapping(bytes32 blockHash => bool isPresent) public ethBlockHashes;
    mapping(bytes32 blockHash => bool isPresent) public setBlockHashes;

    /// @notice tracks which sequencing chains have submitted data for each epoch
    mapping(uint256 epoch => mapping(uint256 chainId => bool submitted)) public epochChainDataSubmitted;

    /// @notice tracks the remaining chains for the epoch - when the count hits zero, the epoch is completed
    mapping(uint256 epoch => uint256 count) epochRemainingChains;

    function epochCompleted(uint256 epoch) external view returns (bool) {
        return epoch < latestEpoch && epochRemainingChains[epoch] == 0;
    }

    /// @notice Stores the verified epoch data hash
    mapping(uint256 epoch => mapping(uint256 seqChainID => bytes32 dataHash)) public epochVerifiedDataHash;

    /// @notice Validated epoch data
    mapping(uint256 epoch => uint256 totalTokens) public epochTotalTokensUsed;
    mapping(uint256 epoch => EnumerableSet.UintSet appchainIds) internal epochAppchainIDs;
    mapping(uint256 epoch => mapping(uint256 appchainId => uint256 tokens)) public epochAppchainTokensUsed;
    mapping(uint256 epoch => mapping(uint256 appchainId => address receiver)) public epochAppchainEmissionsReceiver;
    mapping(uint256 appchainId => uint256 latestEpoch) public appchainLatestEpoch;
    // NOTE: if an appchain has different emissions receivers across different sequencing chains, the latest one to be validated will be used

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    event EpochDataValidated(uint256 indexed epoch, uint256 indexed seqChainID, bytes32 dataHash);
    event EpochCompleted(uint256 indexed epoch);
    event EpochExpectedChainsUpdated(uint256 indexed epoch, uint256[] chainIds);
    event GasAggregatorAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event KnownBlockHash(bytes32 ethBlockHash, bytes32 setBlockHash);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroChainId();
    error ZeroAddress();
    error InvalidProof();
    error AccountDoesNotExistInProof();
    error EmptySlot();
    error InvalidData();
    error InvalidSequencingChain();
    error NotBlockHashSender();
    error InvalidEthBlockHeader();
    error InvalidSetBlockHeader();
    error InvalidSeqBlockHeader();
    error SequencingChainAlreadyExists();
    error SequencingChainDoesNotExist();
    error NotArchivedEpoch();
    error ZeroLengthArray();
    error EpochAlreadyCompleted();
    error AlreadySubmitted();
    error EmptyDataHash();
    error OldSettlementChainBlockNumber();
    error EpochFromFuture();

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(address _blockHashSender, uint256 _settlementChainID, address admin) {
        require(_blockHashSender != address(0), ZeroAddress());
        require(_settlementChainID != 0, ZeroChainId());
        require(admin != address(0), ZeroAddress());
        blockHashSender = _blockHashSender;
        settlementChainID = _settlementChainID;
        latestEpoch = getCurrentEpoch();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS
    //////////////////////////////////////////////////////////////*/

    modifier onlyArchivedEpoch(uint256 epochIndex) {
        require(epochIndex < latestEpoch && epochRemainingChains[epochIndex] == 0, NotArchivedEpoch());
        _;
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Sets the last known block hashes for the ETH and SETTLEMENT chains
    /// @dev This function is called by the block hash sender on the settlement chain to share the last known block hashes
    /// @param ethBlockHash The last known block hash for the l1 chain
    /// @param setBlockHash The last known block hash for the settlement chain
    function sendBlockHashes(bytes32 ethBlockHash, bytes32 setBlockHash) external {
        require(msg.sender == blockHashSender, NotBlockHashSender());

        ethBlockHashes[ethBlockHash] = true;
        setBlockHashes[setBlockHash] = true;
        emit KnownBlockHash(ethBlockHash, setBlockHash);
    }

    function confirmSettlementChainEpochDataHash(
        uint256 epoch,
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) external {
        _confirmEpochDataHash(epoch, settlementChainID, blockHeader, accountProof, storageProof);
        require(setBlockHashes[keccak256(blockHeader)], InvalidSeqBlockHeader());
    }

    /// @notice Validates and stores the epochDataHash for a given sequencing chain / epoch using sequencing chain storage proofs
    /// @dev Verifies the proof data of the sequencing chain's proof against the confirmed seq chain block hash
    /// @param epoch The epoch number to validate
    /// @param seqChainID The sequencing chain ID
    /// @param sendRoot The send root stored in the the Arbitrum Outbox contract that the eth proof was generated for, unused if seqChainID == settlementChainID
    /// @param ethBlockHeader RLP-encoded Ethereum block header, unused if seqChainID == settlementChainID
    /// @param ethAccountProof Merkle proof of the bridge contract account, unused if seqChainID == settlementChainID
    /// @param ethStorageProof Merkle proof of the storage slot containing the block hash, unused if seqChainID == settlementChainID
    /// @param seqBlockHeader RLP-encoded sequencing chain block header
    /// @param seqAccountProof Merkle proof of the GasAggregator account
    /// @param seqStorageProof Merkle proof of the epoch data storage slot
    function confirmEpochDataHash(
        uint256 epoch,
        uint256 seqChainID,
        bytes32 sendRoot,
        bytes calldata ethBlockHeader,
        bytes[] calldata ethAccountProof,
        bytes[] calldata ethStorageProof,
        bytes calldata seqBlockHeader,
        bytes[] calldata seqAccountProof,
        bytes[] calldata seqStorageProof
    ) external {
        _confirmEpochDataHash(epoch, seqChainID, seqBlockHeader, seqAccountProof, seqStorageProof);
        if (seqChainID == settlementChainID) {
            require(setBlockHashes[keccak256(seqBlockHeader)], InvalidSeqBlockHeader());
            return;
        }

        if (seqChainSettlesToBase[seqChainID]) {
            require(setBlockHashes[keccak256(ethBlockHeader)], InvalidSetBlockHeader());
        } else {
            require(ethBlockHashes[keccak256(ethBlockHeader)], InvalidEthBlockHeader());
        }

        bytes32 verifiedSeqChainBlockHash = _getSlotValueFromProof({
            blockHeader: ethBlockHeader,
            accountProof: ethAccountProof,
            storageProof: ethStorageProof,
            account: seqChainOutbox[seqChainID],
            storageSlot: keccak256(abi.encode(sendRoot, SEND_ROOT_STORAGE_SLOT))
        });

        // seq chain header must match the block hash for this sequencing chain
        require(keccak256(seqBlockHeader) == verifiedSeqChainBlockHash, InvalidSeqBlockHeader());
    }

    function _confirmEpochDataHash(
        uint256 epoch,
        uint256 chainID,
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal {
        // prevent resubmission for the same epoch and chain
        require(epochVerifiedDataHash[epoch][chainID] == bytes32(0), AlreadySubmitted());

        // just in case, make sure the epoch is not from the future
        _updateLatestEpoch();
        require(epoch < latestEpoch, EpochFromFuture());

        // submissions are only allowed for active sequencing chains
        require(chainAdded[chainID] > 0 && chainAdded[chainID] <= epoch, InvalidSequencingChain());

        // verify that the provided epoch data is valid according to the sequencing chain proof
        bytes32 verifiedEpochDataHash = _getSlotValueFromProof({
            blockHeader: blockHeader,
            accountProof: accountProof,
            storageProof: storageProof,
            account: seqChainGasAggregatorAddresses[chainID],
            storageSlot: keccak256(abi.encode(epoch, AGGREGATED_EPOCH_DATA_HASH_SLOT))
        });

        require(verifiedEpochDataHash != bytes32(0), EmptyDataHash());

        // data submitted is valid, store it
        emit EpochDataValidated(epoch, chainID, verifiedEpochDataHash);

        epochVerifiedDataHash[epoch][chainID] = verifiedEpochDataHash;
    }

    /// @notice Receives the pre-image data for a verified epoch
    /// @param epoch The epoch number to validate
    /// @param seqChainID The sequencing chain ID
    /// @param appchains Array of appchain IDs
    /// @param tokens Array of token amounts used to pay for gas by each appchain on the sequencing chain
    /// @param emissionsReceivers Array of emissions receiver addresses for each appchain
    function submitEpochPreImageData(
        uint256 epoch,
        uint256 seqChainID,
        uint256[] calldata appchains,
        uint256[] calldata tokens,
        address[] calldata emissionsReceivers
    ) external {
        // prevent resubmission for the same epoch and chain
        require(!epochChainDataSubmitted[epoch][seqChainID], AlreadySubmitted());

        // note: we skip validating that appchains.length == tokens.length == emissionsReceivers.length
        // because the GasAggregator already enforces this.
        // similarly we skip epoch validation because confirmEpochDataHash already enforces this.

        bytes32 epochDataHash = keccak256(abi.encode(appchains, tokens, emissionsReceivers));
        require(epochVerifiedDataHash[epoch][seqChainID] == epochDataHash, InvalidData());

        uint256 totalTokensUsed = 0;
        for (uint256 i = 0; i < appchains.length; i++) {
            epochAppchainIDs[epoch].add(appchains[i]);
            totalTokensUsed += tokens[i];
            epochAppchainTokensUsed[epoch][appchains[i]] += tokens[i];
            epochAppchainEmissionsReceiver[epoch][appchains[i]] = emissionsReceivers[i];
            if (epoch > appchainLatestEpoch[appchains[i]]) {
                appchainLatestEpoch[appchains[i]] = epoch;
            }
        }
        epochTotalTokensUsed[epoch] += totalTokensUsed;

        epochChainDataSubmitted[epoch][seqChainID] = true;
        _decrementEpochRemainingChains(epoch);
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _decrementEpochRemainingChains(uint256 epoch) internal {
        if (--epochRemainingChains[epoch] == 0) {
            emit EpochCompleted(epoch);
        }
    }

    function _updateLatestEpoch() internal {
        uint256 currentEpoch = getCurrentEpoch();
        while (latestEpoch < currentEpoch) {
            epochRemainingChains[latestEpoch] = seqChainCount;
            latestEpoch++;
        }
    }

    /// @notice Retrieves a storage slot value using Merkle Patricia proofs (can be obtained from `eth_getProof`)
    /// @dev First verifies the account proof to get the storage root, then verifies the storage proof
    ///      to extract the value at the specified storage slot
    /// @param blockHeader The RLP-encoded block header to verify against
    /// @param accountProof Merkle proof of the account in the state trie
    /// @param storageProof Merkle proof of the storage slot in the account's storage trie
    /// @param account The account address containing the storage slot
    /// @param storageSlot The storage slot to retrieve the value from
    /// @return The value stored in the specified storage slot
    function _getSlotValueFromProof(
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof,
        address account,
        bytes32 storageSlot
    ) internal pure returns (bytes32) {
        RLPReader.RLPItem memory accountRlp = MerklePatriciaProofVerifier.extractProofValue({
            rootHash: bytes32(blockHeader.toRlpItem().toList()[HEADER_STATE_ROOT_INDEX].toUint()),
            path: abi.encodePacked(keccak256(abi.encodePacked(account))),
            stack: _RLPItemsFromProofBytes(accountProof)
        }).toRlpItem();

        // If the account does not exist, return the hash of an empty trie.
        require(accountRlp.len > 0, AccountDoesNotExistInProof());

        RLPReader.RLPItem memory slotContents = MerklePatriciaProofVerifier.extractProofValue({
            rootHash: bytes32(accountRlp.toList()[STORAGE_ROOT_ACCOUNT_FIELDS_INDEX].toUint()),
            path: abi.encodePacked(keccak256(abi.encodePacked(storageSlot))),
            stack: _RLPItemsFromProofBytes(storageProof)
        }).toRlpItem();

        require(slotContents.len > 0, EmptySlot());
        return bytes32(slotContents.toUint());
    }

    /// @notice creates RLP items from the given proof bytes.
    ///
    /// @param proof The proof bytes.
    ///
    /// @return The RLP items.
    function _RLPItemsFromProofBytes(bytes[] memory proof) internal pure returns (RLPReader.RLPItem[] memory) {
        RLPReader.RLPItem[] memory proofItems = new RLPReader.RLPItem[](proof.length);
        for (uint256 i; i < proof.length; i++) {
            proofItems[i] = proof[i].toRlpItem();
        }
        return proofItems;
    }

    /*//////////////////////////////////////////////////////////////
                             VIEWS
    //////////////////////////////////////////////////////////////*/

    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId)
        external
        view
        onlyArchivedEpoch(epochIndex)
        returns (uint256)
    {
        return epochAppchainTokensUsed[epochIndex][appchainId];
    }

    function getTotalGasFees(uint256 epochIndex) external view onlyArchivedEpoch(epochIndex) returns (uint256) {
        return epochTotalTokensUsed[epochIndex];
    }

    function getActiveAppchainIds(uint256 epochIndex)
        external
        view
        onlyArchivedEpoch(epochIndex)
        returns (uint256[] memory _chainIDs)
    {
        bytes32[] memory ids = epochAppchainIDs[epochIndex]._inner._values;
        assembly {
            _chainIDs := ids
        }
    }

    function getAppchainRewardsReceiver(uint256 appchainId) external view returns (address) {
        return epochAppchainEmissionsReceiver[appchainLatestEpoch[appchainId]][appchainId];
    }

    /// @notice Checks if a specific sequencing chain has submitted data for an epoch
    /// @param epochIndex The epoch to check
    /// @param chainId The chain ID to check
    /// @return Whether the chain has submitted data for this epoch
    function hasChainSubmittedForEpoch(uint256 epochIndex, uint256 chainId) external view returns (bool) {
        return epochChainDataSubmitted[epochIndex][chainId];
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a new sequencing chain configuration
    /// @dev Only admin can add sequencing chains. Special handling for settlement chain as sequencing chain
    /// @param chainID The chain ID of the sequencing chain
    /// @param aggregatorAddress Address of the GasAggregator contract on the sequencing chain
    /// @param outboxAddress Address of the sequencing chain outbox contract on Ethereum (not needed for settlement chain)
    function addSequencingChain(uint256 chainID, address aggregatorAddress, address outboxAddress, bool settlesToBase)
        public
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(aggregatorAddress != address(0), ZeroAddress());
        require(chainID != 0, ZeroChainId());
        require(chainAdded[chainID] == 0, SequencingChainAlreadyExists());

        _updateLatestEpoch();
        seqChainCount++;
        chainAdded[chainID] = latestEpoch;
        seqChainGasAggregatorAddresses[chainID] = aggregatorAddress;

        if (chainID != settlementChainID) {
            require(outboxAddress != address(0), ZeroAddress());
            seqChainOutbox[chainID] = outboxAddress;
            seqChainSettlesToBase[chainID] = settlesToBase;
        }
    }

    /// @notice overload of addSequencingChain for sequencing chains that settle to ethereum
    function addSequencingChain(uint256 chainID, address aggregatorAddress, address outboxAddress) external {
        addSequencingChain(chainID, aggregatorAddress, outboxAddress, false);
    }

    function addSettlementChainAsSequencingChain(address aggregatorAddress) external {
        addSequencingChain(settlementChainID, aggregatorAddress, address(0), false);
    }

    /// @notice Removes an existing sequencing chain immediately
    /// @dev Only admin can remove sequencing chains
    function removeSequencingChain(uint256 chainID) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(chainAdded[chainID] != 0, SequencingChainDoesNotExist());
        for (uint256 epoch = chainAdded[chainID]; epoch < latestEpoch; epoch++) {
            // do not remove epoch data for already submitted chains
            if (!epochChainDataSubmitted[epoch][chainID]) {
                // clear the verified data hash in case it is set
                epochVerifiedDataHash[epoch][chainID] = bytes32(0);
                _decrementEpochRemainingChains(epoch);
            }
        }
        seqChainCount--;
        chainAdded[chainID] = 0;
        seqChainGasAggregatorAddresses[chainID] = address(0);
        if (chainID != settlementChainID) {
            seqChainOutbox[chainID] = address(0);
            seqChainSettlesToBase[chainID] = false;
        }
    }

    /// @notice Updates the authorized block hash sender address
    /// @dev Only admin can change the block hash sender
    /// @param newBlockHashSender The new address authorized to send block hashes
    function setBlockHashSender(address newBlockHashSender) external onlyRole(DEFAULT_ADMIN_ROLE) {
        blockHashSender = newBlockHashSender;
    }
}
