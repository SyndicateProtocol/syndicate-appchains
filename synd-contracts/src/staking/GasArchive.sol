// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MerklePatriciaProofVerifier} from "./lib/MerklePatriciaProofVerifier.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {RLPReader} from "./lib/RLPReader.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/// @title GasArchive
/// @notice Lives on the staking appchain and trustlessly validates and stores gas usage data from multiple sequencing chains using storage proofs
/// @dev This contract supports arbitrum-based sequencing chains only (with the exception of the settlement chain, which can be any chain)
contract GasArchive is Initializable, OwnableUpgradeable, IGasDataProvider, UUPSUpgradeable {
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
                            IMMUTABLE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @dev The `BlockHashRelayer` contract is deployed on the settlement chain and is responsible for sending the block hashes to the `GasArchive` contract. Anyone can call `sendBlockHashes` on the relayer to send the block hashes.
    address public immutable blockHashSender;

    /// @notice when using the settlement chain as the sequencing chain, the rollup hash proof is not required
    uint256 public immutable settlementChainID;

    /*//////////////////////////////////////////////////////////////
                            STORAGE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice the current epoch
    uint256 public epoch;

    /// @notice list of sequencing chains
    EnumerableSet.UintSet seqChains;

    /// @notice tracks the remaining chains for the epoch
    uint256 public epochRemainingChains;

    /// @notice mapping of sequencing chain IDs to the address of the gas aggregator contract
    mapping(uint256 chainId => address aggregatorAddress) public seqChainGasAggregator;
    /// @notice mapping of sequencing chain IDs to the address of the Outbox contract for that sequencing chain (where the confirmed rollup hash can be found)
    mapping(uint256 chainId => address outboxAddress) public seqChainOutbox;
    mapping(uint256 chainId => bool) public seqChainSettlesToBase;
    /// @notice block hashes for l1 and settlement chains
    mapping(bytes32 blockHash => bool isPresent) public ethBlockHashes;
    mapping(bytes32 blockHash => bool isPresent) public setBlockHashes;

    /// @notice tracks which sequencing chains have submitted data for each epoch
    mapping(uint256 epoch => mapping(uint256 chainId => bool submitted)) public epochChainDataSubmitted;

    /// @notice Stores the verified epoch data hash
    mapping(uint256 epoch => mapping(uint256 seqChainID => bytes32 dataHash)) public epochVerifiedDataHash;

    /// @notice Validated epoch data
    mapping(uint256 epoch => uint256 totalTokens) public totalGasFees;
    mapping(uint256 epoch => EnumerableSet.UintSet appchainIds) internal appchainIDs;
    mapping(uint256 epoch => mapping(uint256 appchainId => uint256 tokens)) public appchainGasFees;

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    event EpochCompleted(uint256 indexed epoch);
    event KnownBlockHash(bytes32 ethBlockHash, bytes32 setBlockHash);
    event ChainAdded(
        uint256 indexed epoch, uint256 indexed chainID, address aggregator, address outbox, bool settlesToBase
    );
    event ChainRemoved(uint256 indexed epoch, uint256 indexed chainID);
    event ChainSubmitted(uint256 indexed epoch, uint256 indexed chainID);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroChainId();
    error ZeroAddress();
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
    error AlreadySubmitted();
    error EmptyDataHash();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Constructor that disables initializers
     * @dev Prevents direct initialization of implementation contract
     */
    constructor(address _blockHashSender, uint256 _settlementChainID) {
        require(_blockHashSender != address(0), ZeroAddress());
        require(_settlementChainID != 0, ZeroChainId());
        blockHashSender = _blockHashSender;
        settlementChainID = _settlementChainID;
        _disableInitializers();
    }

    function initialize(uint256 _epoch) external initializer {
        epoch = _epoch;
        __Ownable_init(msg.sender);
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
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) external {
        _confirmEpochDataHash(settlementChainID, blockHeader, accountProof, storageProof);
        require(setBlockHashes[keccak256(blockHeader)], InvalidSeqBlockHeader());
    }

    /// @notice Validates and stores the epochDataHash for a given sequencing chain / epoch using sequencing chain storage proofs
    /// @dev Verifies the proof data of the sequencing chain's proof against the confirmed seq chain block hash
    /// @param seqChainID The sequencing chain ID
    /// @param sendRoot The send root stored in the the Arbitrum Outbox contract that the eth proof was generated for, unused if seqChainID == settlementChainID
    /// @param ethBlockHeader RLP-encoded Ethereum block header, unused if seqChainID == settlementChainID
    /// @param ethAccountProof Merkle proof of the bridge contract account, unused if seqChainID == settlementChainID
    /// @param ethStorageProof Merkle proof of the storage slot containing the block hash, unused if seqChainID == settlementChainID
    /// @param seqBlockHeader RLP-encoded sequencing chain block header
    /// @param seqAccountProof Merkle proof of the GasAggregator account
    /// @param seqStorageProof Merkle proof of the epoch data storage slot
    function confirmEpochDataHash(
        uint256 seqChainID,
        bytes32 sendRoot,
        bytes calldata ethBlockHeader,
        bytes[] calldata ethAccountProof,
        bytes[] calldata ethStorageProof,
        bytes calldata seqBlockHeader,
        bytes[] calldata seqAccountProof,
        bytes[] calldata seqStorageProof
    ) external {
        _confirmEpochDataHash(seqChainID, seqBlockHeader, seqAccountProof, seqStorageProof);
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
        uint256 chainID,
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal {
        // prevent resubmission for the same epoch and chain
        require(epochVerifiedDataHash[epoch][chainID] == bytes32(0), AlreadySubmitted());

        // submissions are only allowed for active sequencing chains
        require(!seqChains.contains(chainID), InvalidSequencingChain());

        // verify that the provided epoch data is valid according to the sequencing chain proof
        bytes32 verifiedEpochDataHash = _getSlotValueFromProof({
            blockHeader: blockHeader,
            accountProof: accountProof,
            storageProof: storageProof,
            account: seqChainGasAggregator[chainID],
            storageSlot: keccak256(abi.encode(epoch, AGGREGATED_EPOCH_DATA_HASH_SLOT))
        });

        require(verifiedEpochDataHash != bytes32(0), EmptyDataHash());

        // data submitted is valid, store it
        epochVerifiedDataHash[epoch][chainID] = verifiedEpochDataHash;
    }

    /// @notice Receives the pre-image data for a verified epoch
    /// @param seqChainID The sequencing chain ID
    /// @param appchains Array of appchain IDs
    /// @param tokens Array of token amounts used to pay for gas by each appchain on the sequencing chain
    function submitEpochPreImageData(uint256 seqChainID, uint256[] calldata appchains, uint256[] calldata tokens)
        external
    {
        // prevent resubmission for the same epoch and chain
        require(!epochChainDataSubmitted[epoch][seqChainID], AlreadySubmitted());

        // note: we skip validating that appchains.length == tokens.length
        // because the GasAggregator already enforces this.
        // similarly we skip epoch validation because confirmEpochDataHash already enforces this.
        require(epochVerifiedDataHash[epoch][seqChainID] == keccak256(abi.encode(appchains, tokens)), InvalidData());

        for (uint256 i = 0; i < appchains.length; i++) {
            appchainIDs[epoch].add(appchains[i]);
            totalGasFees[epoch] += tokens[i];
            appchainGasFees[epoch][appchains[i]] += tokens[i];
        }

        epochChainDataSubmitted[epoch][seqChainID] = true;
        epochRemainingChains--;
        if (epochRemainingChains == 0) {
            emit EpochCompleted(epoch);
            epoch++;
            epochRemainingChains = seqChains.length();
        }
        emit ChainSubmitted(epoch, seqChainID);
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Authorize contract upgrades (admin only)
    /// @dev Required by UUPSUpgradeable. Only admins can upgrade this contract.
    function _authorizeUpgrade(address) internal override onlyOwner {}

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

    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256) {
        require(epochIndex < epoch, NotArchivedEpoch());
        return appchainGasFees[epochIndex][appchainId];
    }

    function getTotalGasFees(uint256 epochIndex) external view returns (uint256) {
        require(epochIndex < epoch, NotArchivedEpoch());
        return totalGasFees[epochIndex];
    }

    function getAppchainIds(uint256 epochIndex) external view returns (uint256[] memory chainIDs) {
        require(epochIndex < epoch, NotArchivedEpoch());
        bytes32[] memory ids = appchainIDs[epochIndex]._inner._values;
        assembly {
            chainIDs := ids
        }
    }

    function getAppchainIds(uint256 epochIndex, uint256 startIndex, uint256 pageSize)
        external
        view
        returns (uint256[] memory chainIDs)
    {
        require(epochIndex < epoch, NotArchivedEpoch());
        bytes32[] memory ids = appchainIDs[epochIndex]._inner._values;
        uint256 idsLength = ids.length;

        // Handle edge cases
        if (startIndex >= idsLength) {
            return new uint256[](0);
        }

        // Calculate actual size efficiently
        uint256 actualSize;
        unchecked {
            uint256 remaining = idsLength - startIndex;
            actualSize = pageSize == 0 || pageSize > remaining ? remaining : pageSize;
        }

        // Use assembly for zero-copy optimization when possible
        if (startIndex == 0 && actualSize == idsLength) {
            // Return entire array with zero-copy assembly trick
            assembly {
                chainIDs := ids
            }
            return chainIDs;
        }

        // For partial arrays, use assembly for efficient copying
        assembly {
            // Allocate memory for result array
            chainIDs := mload(0x40)
            let resultPtr := add(chainIDs, 0x20)

            // Store array length
            mstore(chainIDs, actualSize)

            // Calculate source pointer (skip array length + startIndex * 32)
            let sourcePtr := add(add(ids, 0x20), mul(startIndex, 0x20))

            // Copy data efficiently in 32-byte chunks
            let copySize := mul(actualSize, 0x20)
            let i := 0
            for {} lt(i, copySize) { i := add(i, 0x20) } { mstore(add(resultPtr, i), mload(add(sourcePtr, i))) }

            // Update free memory pointer
            mstore(0x40, add(resultPtr, copySize))
        }
    }

    function sequencingChainCount() external view returns (uint256) {
        return seqChains.length();
    }

    function getSequencingChainIds() external view returns (uint256[] memory chainIDs) {
        bytes32[] memory ids = seqChains._inner._values;
        assembly {
            chainIDs := ids
        }
    }

    function sequencingChainId(uint256 index) external view returns (uint256) {
        return seqChains.at(index);
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a new sequencing chain configuration or updates an existing one
    /// @dev Only admin can add sequencing chains. Special handling for settlement chain as sequencing chain
    /// @param chainID The chain ID of the sequencing chain
    /// @param aggregatorAddress Address of the GasAggregator contract on the sequencing chain
    /// @param outboxAddress Address of the sequencing chain outbox contract on Ethereum (not needed for settlement chain)
    function addSequencingChain(uint256 chainID, address aggregatorAddress, address outboxAddress, bool settlesToBase)
        public
        onlyOwner
    {
        require(aggregatorAddress != address(0), ZeroAddress());
        require(chainID != 0, ZeroChainId());

        require(seqChains.add(chainID), SequencingChainAlreadyExists());
        epochRemainingChains++;
        seqChainGasAggregator[chainID] = aggregatorAddress;

        if (chainID != settlementChainID) {
            require(outboxAddress != address(0), ZeroAddress());
            seqChainOutbox[chainID] = outboxAddress;
            seqChainSettlesToBase[chainID] = settlesToBase;
        }
        emit ChainAdded(epoch, chainID, aggregatorAddress, seqChainOutbox[chainID], seqChainSettlesToBase[chainID]);
    }

    function addSettlementChainAsSequencingChain(address aggregatorAddress) external {
        addSequencingChain(settlementChainID, aggregatorAddress, address(0), false);
    }

    /// @notice Removes an existing sequencing chain immediately
    /// @dev Only admin can remove sequencing chains
    function removeSequencingChain(uint256 chainID) external onlyOwner {
        require(seqChains.remove(chainID), SequencingChainDoesNotExist());
        seqChainGasAggregator[chainID] = address(0);
        if (chainID != settlementChainID) {
            seqChainOutbox[chainID] = address(0);
            seqChainSettlesToBase[chainID] = false;
        }
        if (!epochChainDataSubmitted[epoch][chainID]) {
            // clear the verified data hash in case it is set
            epochVerifiedDataHash[epoch][chainID] = bytes32(0);
            epochRemainingChains--;
            uint256 seqChainCount = seqChains.length();
            if (seqChainCount > 0 && epochRemainingChains == 0) {
                emit EpochCompleted(epoch);
                epoch++;
                epochRemainingChains = seqChainCount;
            }
        }
        emit ChainRemoved(epoch, chainID);
    }
}
