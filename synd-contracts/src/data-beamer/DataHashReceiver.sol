// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {MerklePatriciaProofVerifier} from "./lib/MerklePatriciaProofVerifier.sol";
import {RLPReader} from "./lib/RLPReader.sol";

contract DataHashReceiver {
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    /*//////////////////////////////////////////////////////////////
                            CONSTANTS
    //////////////////////////////////////////////////////////////*/

    // Generated using: cast index-erc7201 syndicate.storage.DataHashStore
    bytes32 public constant DATA_HASH_STORAGE_LOCATION =
        0xdefd86032ebaaca80e3baa1b4281b4518ac2b830a870f52678dcf4c66353e100;

    uint256 public constant HEADER_STATE_ROOT_INDEX = 3;
    uint256 public constant STORAGE_ROOT_ACCOUNT_FIELDS_INDEX = 2;

    /// @notice Storage slot of roots in AbsOutbox (slot 3) (see `forge inspect AbsOutbox storageLayout`)
    uint256 public constant SEND_ROOT_STORAGE_SLOT = 3;

    /*//////////////////////////////////////////////////////////////
                            IMMUTABLE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @dev The `BlockHashRelayer` contract is deployed on the settlement chain and is responsible for sending the block hashes to the `GasArchive` contract. Anyone can call `sendBlockHashes` on the relayer to send the block hashes.
    /// @dev IMPORTANT: Immutable variables are set in the constructor and become part of the implementation contract's bytecode.
    ///      When the proxy delegates calls to the implementation, these immutable values are read from the implementation's bytecode.
    ///      This is why we can use both a constructor (for immutables) and initialize() (for storage variables) in UUPS upgradeable contracts.
    address public immutable blockHashSender;

    /*//////////////////////////////////////////////////////////////
                            STORAGE VARIABLES
    //////////////////////////////////////////////////////////////*/


    /// @notice mapping of data hash keys to the address of the data hash store contract
    address public dataHashStore;

    /// @notice mapping of sequencing chain IDs to the address of the Outbox contract for that sequencing chain (where the confirmed rollup hash can be found)
    address public chainOutbox;

    /// @notice block hashes for l1 and settlement chains
    mapping(bytes32 blockHash => bool isPresent) public ethBlockHashes;

    /// @notice Stores the verified data hash
    mapping(bytes32 key => bytes32 dataHash) public verifiedDataHash;

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    event KnownBlockHash(bytes32 ethBlockHash);
    event DataHash(bytes32 key, bytes32 dataHash);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    error AccountDoesNotExistInProof();
    error EmptySlot();
    error NotBlockHashSender();
    error InvalidEthBlockHeader();
    error AlreadySubmitted();
    error EmptyDataHash();

    constructor(address _blockHashSender) {
        require(_blockHashSender != address(0), ZeroAddress());
        blockHashSender = _blockHashSender;
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Sets the last known block hashes for the ETH and SETTLEMENT chains
    /// @dev This function is called by the block hash sender on the settlement chain to share the last known block hashes
    /// @param ethBlockHash The last known block hash for the l1 chain
    /// @param setBlockHash The last known block hash for the settlement chain
    function sendBlockHash(bytes32 ethBlockHash) external {
        require(msg.sender == blockHashSender, NotBlockHashSender());

        ethBlockHashes[ethBlockHash] = true;
        emit KnownBlockHash(ethBlockHash);
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
    function confirmDataHash(
        bytes32 key,
        bytes32 sendRoot,
        bytes calldata ethBlockHeader,
        bytes[] calldata ethAccountProof,
        bytes[] calldata ethStorageProof,
        bytes calldata seqBlockHeader,
        bytes[] calldata seqAccountProof,
        bytes[] calldata seqStorageProof
    ) external {
        _confirmDataHash(key, seqBlockHeader, seqAccountProof, seqStorageProof);
        require(ethBlockHashes[keccak256(ethBlockHeader)], InvalidEthBlockHeader());

        bytes32 dataHash = _getSlotValueFromProof({
            blockHeader: ethBlockHeader,
            accountProof: ethAccountProof,
            storageProof: ethStorageProof,
            account: chainOutbox,
            storageSlot: keccak256(abi.encode(sendRoot, SEND_ROOT_STORAGE_SLOT))
        });

        // seq chain header must match the block hash for this sequencing chain
        require(keccak256(seqBlockHeader) == dataHash, InvalidSeqBlockHeader());
    }

    function _confirmDataHash(
        bytes32 key,
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal {
        // prevent resubmission for the same epoch and chain
        require(verifiedDataHash[key] == bytes32(0), AlreadySubmitted());

        // verify that the provided epoch data is valid according to the sequencing chain proof
        bytes32 dataHash = _getSlotValueFromProof({
            blockHeader: blockHeader,
            accountProof: accountProof,
            storageProof: storageProof,
            account: dataHashStore,
            storageSlot: keccak256(abi.encode(key, DATA_HASH_STORAGE_LOCATION))
        });

        require(dataHash != bytes32(0), EmptyDataHash());

        // data submitted is valid, store it
        verifiedDataHash[key] = dataHash;
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

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
}
