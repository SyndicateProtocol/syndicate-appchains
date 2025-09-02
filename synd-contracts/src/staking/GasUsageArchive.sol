// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {MerklePatriciaProofVerifier} from "./lib/MerklePatriciaProofVerifier.sol";
import {RLPReader} from "./lib/RLPReader.sol";

/// @title GasUsageArchive
/// @notice Trustlessly validates and stores gas usage data from chainA using storage proofs
contract GasUsageArchive is Initializable, AccessControlUpgradeable {
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    /*//////////////////////////////////////////////////////////////
                            CONSTANTS
    //////////////////////////////////////////////////////////////*/
    uint256 constant HEADER_STATE_ROOT_INDEX = 3;
    uint256 constant STORAGE_ROOT_ACCOUNT_FIELDS_INDEX = 2;

    /*//////////////////////////////////////////////////////////////
                               STRUCTS
    //////////////////////////////////////////////////////////////*/

    struct ProofData {
        bytes appchainBlockHeader;
        bytes[] seqchainAccountProof;
        bytes[] seqchainStorageProof;
        bytes ethereumBlockHeader;
        bytes[] ethereumAccountProof;
        bytes[] ethereumStorageProof;
    }

    struct EpochData {
        uint256[] appchains;
        uint256[] tokens;
        address[] emissionsReceivers;
    }

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Storage slot of aggregatedEpochDataHash in GasAggregator (slot 7) (see `forge inspect GasAggregator storageLayout`)
    uint256 public constant AGGREGATED_EPOCH_DATA_HASH_SLOT = 7;

    /// @notice chainIDs of the sequencing chains to expect data from
    uint256[] public seqChainIDs;
    mapping(uint256 => address) public seqChainGasAggregatorAddresses;
    mapping(uint256 => address) public ethereumSeqChainBridges;
    mapping(uint256 => bytes32) public ethereumSeqChainStorageSlots;
    // TODO add admin functions for the stuff above

    /// @notice epoches and related chainIDs for which data is already validated and stored
    mapping(uint256 => mapping(uint256 => bool)) public archievedEpochData;

    /// @notice Validated epoch data
    mapping(uint256 => uint256) public epochTotalTokensUsed;
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainTokensUsed;
    mapping(uint256 => mapping(uint256 => address)) public epochAppchainEmissionsReceiver;
    // NOTE: if an appchain has different emissions receivers across different sequencing chains, the latest one to be validated will be used

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    event EpochDataValidated(uint256 indexed epoch, bytes32 dataHash, uint256 blockNumber);
    event GasAggregatorAddressUpdated(address indexed oldAddress, address indexed newAddress);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroAddress();
    error InvalidProof();
    error AccountDoesNotExist();
    error EmptySlot();
    error InvalidData();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    constructor() {
        _disableInitializers();
    }

    function initialize(address _gasAggregatorAddress, address admin) external initializer {
        if (_gasAggregatorAddress == address(0)) revert ZeroAddress();
        if (admin == address(0)) revert ZeroAddress();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // TODO rename me and write docs
    function _entryPoint(uint256 epoch, uint256 seqChainID, ProofData calldata proofs, EpochData calldata epochData)
        external
    {
        // verify the provided sequencing chain block header is valid according to ethereum
        bytes32 seqChainBlockHash = keccak256(proofs.appchainBlockHeader);
        _verifyEthereumProof({
            seqChainID: seqChainID,
            seqChainBlockHash: seqChainBlockHash,
            ethereumBlockHeader: proofs.ethereumBlockHeader,
            accountProof: proofs.ethereumAccountProof,
            storageProof: proofs.ethereumStorageProof
        });

        // verify that the provided epoch data is valid according to the sequencingchain proofs
        bytes32 verifiedEpochDataHash = _epochDataHashFromSeqChainStorageProof({
            epoch: epoch,
            seqChainID: seqChainID,
            blockHeader: proofs.appchainBlockHeader,
            accountProof: proofs.seqchainAccountProof,
            storageProof: proofs.seqchainStorageProof
        });
        bytes32 epochDataHash =
            keccak256(abi.encode(epochData.appchains, epochData.tokens, epochData.emissionsReceivers));
        if (verifiedEpochDataHash != epochDataHash) {
            revert InvalidData();
        }

        // data submitted is valid, store it
        uint256 totalTokensUsed = 0;
        for (uint256 i = 0; i < epochData.appchains.length; i++) {
            totalTokensUsed += epochData.tokens[i];
            epochAppchainTokensUsed[epoch][epochData.appchains[i]] += epochData.tokens[i];
            epochAppchainEmissionsReceiver[epoch][epochData.appchains[i]] = epochData.emissionsReceivers[i];
        }
        epochTotalTokensUsed[epoch] = totalTokensUsed;
        archievedEpochData[epoch][seqChainID] = true;
    }
    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // TODO docs
    function _verifyEthereumProof(
        uint256 seqChainID,
        bytes32 seqChainBlockHash,
        bytes calldata ethereumBlockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal view {
        bytes32 ethereumBlockHash = keccak256(ethereumBlockHeader);
        // TODO match the ethereum block hash with some trusted source

        address account = ethereumSeqChainBridges[seqChainID];
        bytes32 storageSlot = ethereumSeqChainStorageSlots[seqChainID];
        bytes32 stateRoot = _getStateRootFromHeader(ethereumBlockHeader);
        bytes32 verifiedSeqChainBlockHash = _getSlotValueFromProof({
            stateRoot: stateRoot,
            accountProof: accountProof,
            storageProof: storageProof,
            account: account,
            storageSlot: storageSlot
        });

        if (verifiedSeqChainBlockHash != seqChainBlockHash) {
            revert InvalidProof();
        }
    }

    /// @notice Validates gas usage data from an appchain using storage proof
    /// @param epoch The epoch to validate
    /// @param blockHeader the header of the block for which the proof was generated
    /// @param accountProof RLP-encoded proof of the GasAggregator account
    /// @param storageProof RLP-encoded proof of the storage slot
    function _epochDataHashFromSeqChainStorageProof(
        uint256 epoch,
        uint256 seqChainID,
        bytes calldata blockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal view returns (bytes32) {
        bytes32 stateRoot = _getStateRootFromHeader(blockHeader);

        // TODO we're considering the same gasAggretator address for all appchains
        bytes32 storageSlot = _getStorageSlot(epoch);
        return _getSlotValueFromProof({
            stateRoot: stateRoot,
            accountProof: accountProof,
            storageProof: storageProof,
            account: seqChainGasAggregatorAddresses[seqChainID],
            storageSlot: storageSlot
        });
    }

    // TODO docs
    function _storageRootFromAccountProof(address account, bytes32 stateRoot, bytes[] calldata accountProof)
        internal
        pure
        returns (bytes32)
    {
        bytes32 accountHash = keccak256(abi.encodePacked(account));
        RLPReader.RLPItem memory accountRlp = MerklePatriciaProofVerifier.extractProofValue({
            rootHash: stateRoot,
            path: abi.encodePacked(accountHash),
            stack: _parseRLPItems(accountProof)
        }).toRlpItem();

        // If the account does not exist, return the hash of an empty trie.
        if (accountRlp.len == 0) {
            revert AccountDoesNotExist();
        }

        RLPReader.RLPItem[] memory accountFields = accountRlp.toList();

        return bytes32(accountFields[STORAGE_ROOT_ACCOUNT_FIELDS_INDEX].toUintStrict());
    }

    // TODO docs
    function _getSlotValueFromProof(
        bytes32 stateRoot,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof,
        address account,
        bytes32 storageSlot
    ) internal pure returns (bytes32) {
        bytes32 storageRoot = _storageRootFromAccountProof(account, stateRoot, accountProof);

        RLPReader.RLPItem memory slotContents = MerklePatriciaProofVerifier.extractProofValue({
            rootHash: storageRoot,
            path: abi.encodePacked(storageSlot),
            stack: _parseRLPItems(storageProof)
        }).toRlpItem();

        if (slotContents.len == 0) {
            revert EmptySlot();
        }

        return bytes32(slotContents.toUintStrict());
    }

    function _getStateRootFromHeader(bytes calldata blockHeader) internal pure returns (bytes32) {
        RLPReader.RLPItem[] memory headerFields = blockHeader.toRlpItem().toList();
        return bytes32(headerFields[HEADER_STATE_ROOT_INDEX].toUintStrict());
    }

    /// @notice Calculates the storage slot for a given epoch in the aggregatedEpochDataHash mapping
    /// @param epoch The epoch to get the storage slot for
    /// @return The storage slot
    function _getStorageSlot(uint256 epoch) internal pure returns (bytes32) {
        return keccak256(abi.encode(epoch, AGGREGATED_EPOCH_DATA_HASH_SLOT));
    }

    /// @notice Parses RLP items from the given proof bytes.
    ///
    /// @param proof The proof bytes.
    ///
    /// @return The parsed RLP items.
    function _parseRLPItems(bytes[] memory proof) private pure returns (RLPReader.RLPItem[] memory) {
        RLPReader.RLPItem[] memory proofItems = new RLPReader.RLPItem[](proof.length);
        for (uint256 i; i < proof.length; i++) {
            proofItems[i] = proof[i].toRlpItem();
        }

        return proofItems;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/
}
