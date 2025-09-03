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
    /// @notice Storage slot of aggregatedEpochDataHash in GasAggregator (slot 7) (see `forge inspect GasAggregator storageLayout`)
    uint256 public constant AGGREGATED_EPOCH_DATA_HASH_SLOT = 7;
    uint256 public constant HEADER_STATE_ROOT_INDEX = 3;
    uint256 public constant STORAGE_ROOT_ACCOUNT_FIELDS_INDEX = 2;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice known block hashes for the ETH and SETTLEMENT chains are passed trustlessly via the bridge by a known source
    bytes32 public lastKnownEthereumBlockHash;
    bytes32 public lastKnownSettlementChainBlockHash;
    address public blockHashSender;

    /// @notice when using the settlement chain as the sequencing chain, the rollup hash proof is not required and `lastKnownSettlementChainBlockHash` will be used intead
    uint256 public settlementChainID;
    bool public useSettlementChainAsSequencingChain;

    /// @notice chainIDs of the sequencing chains to expect data from
    uint256[] public seqChainIDs;

    /// @notice mapping of sequencing chain IDs to the address of the gas aggregator contract
    mapping(uint256 => address) public seqChainGasAggregatorAddresses;
    /// @notice mapping of sequencing chain IDs to the address of the bridge contract for that sequencing chain (where the confirmed rollup hash can be found)
    mapping(uint256 => address) public ethereumSeqChainBridges;
    /// @notice mapping of sequencing chain IDs to the storage slot of the rollup hash in the bridge contract
    mapping(uint256 => bytes32) public ethereumSeqChainStorageSlots;

    mapping(uint256 => bytes32) public lastKnownSeqChainBlockHashes;

    /// @notice epoches and related chainIDs for which data is already validated and stored
    mapping(uint256 => bool) public archivedEpochData;

    /// @notice Validated epoch data
    mapping(uint256 => uint256) public epochTotalTokensUsed;
    mapping(uint256 => uint256[]) public epochAppchainIDs;
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
    error AccountDoesNotExistInProof();
    error EmptySlot();
    error InvalidData();
    error ChainIDNotFound();
    error CannotSubmitProofForSettlementChain();
    error NotBlockHashSender();
    error InvalidEthereumBlockHeader();
    error InvalidSeqChainBlockHeader();
    error sequencingChainAlreadyExists();
    error NotArchivedEpoch();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    constructor() {
        _disableInitializers();
    }

    function initialize(address _blockHashSender, uint256 _settlementChainID, address admin) external initializer {
        if (_blockHashSender == address(0)) revert ZeroAddress();
        if (admin == address(0)) revert ZeroAddress();
        blockHashSender = _blockHashSender;
        settlementChainID = _settlementChainID;

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Sets the last known block hashes for the ETH and SETTLEMENT chains
    /// @dev This function is called by the block hash sender on the settlement chain to share the last known block hashes
    /// @param ethBlockHash The last known block hash for the ETH chain
    /// @param seqBlockHash The last known block hash for the SETTLEMENT chain
    function setLastKnownBlockHashes(bytes32 ethBlockHash, bytes32 seqBlockHash) external {
        // TODO implement the contract to call this function
        if (msg.sender != blockHashSender) revert NotBlockHashSender();
        lastKnownEthereumBlockHash = ethBlockHash;
        lastKnownSettlementChainBlockHash = seqBlockHash;
    }

    /// @notice Confirms and stores a sequencing chain block hash using Ethereum storage proofs
    /// @dev Verifies that the provided block hash is confirmed in the respective Ethereum bridge contract storage
    /// @param seqChainID The ID of the sequencing chain
    /// @param seqChainBlockHash The sequencing chain block hash to confirm
    /// @param ethereumBlockHeader RLP-encoded Ethereum block header
    /// @param ethereumAccountProof Merkle proof of the bridge contract account
    /// @param ethereumStorageProof Merkle proof of the storage slot containing the block hash
    function confirmSequencingChainBlockHash(
        uint256 seqChainID,
        bytes32 seqChainBlockHash,
        bytes calldata ethereumBlockHeader,
        bytes[] calldata ethereumAccountProof,
        bytes[] calldata ethereumStorageProof
    ) external {
        _verifyEthereumProof({
            seqChainID: seqChainID,
            seqChainBlockHash: seqChainBlockHash,
            ethereumBlockHeader: ethereumBlockHeader,
            accountProof: ethereumAccountProof,
            storageProof: ethereumStorageProof
        });
        lastKnownSeqChainBlockHashes[seqChainID] = seqChainBlockHash;
    }

    /// @notice Validates and stores epoch gas usage data using sequencing chain storage proofs
    /// @dev Verifies epoch data against the sequencing chain's GasAggregator contract storage
    /// @param epoch The epoch number to validate
    /// @param seqChainID The sequencing chain ID
    /// @param seqChainBlockHeader RLP-encoded sequencing chain block header
    /// @param seqChainAccountProof Merkle proof of the GasAggregator account
    /// @param seqChainStorageProof Merkle proof of the epoch data storage slot
    /// @param appchains Array of appchain IDs
    /// @param tokens Array of token amounts used to pay for gas by each appchain on the sequencing chain
    /// @param emissionsReceivers Array of emissions receiver addresses for each appchain
    function confirmEpochDataHash(
        uint256 epoch,
        uint256 seqChainID,
        bytes calldata seqChainBlockHeader,
        bytes[] calldata seqChainAccountProof,
        bytes[] calldata seqChainStorageProof,
        uint256[] calldata appchains,
        uint256[] calldata tokens,
        address[] calldata emissionsReceivers
    ) external {
        // note: it's not necessary to validate the array lengths match because the GasAggregator already does that

        // seq chain header must matches the last confirmed block hash for this sequencing chain
        bytes32 seqChainBlockHash = keccak256(seqChainBlockHeader);
        bytes32 expectedBlockHash = lastKnownSeqChainBlockHashes[seqChainID];
        if (useSettlementChainAsSequencingChain) {
            expectedBlockHash = lastKnownSettlementChainBlockHash;
        }
        if (expectedBlockHash != seqChainBlockHash) {
            revert InvalidSeqChainBlockHeader();
        }
        // verify that the provided epoch data is valid according to the sequencingchain proofs
        bytes32 verifiedEpochDataHash = _epochDataHashFromSeqChainStorageProof({
            epoch: epoch,
            seqChainID: seqChainID,
            blockHeader: seqChainBlockHeader,
            accountProof: seqChainAccountProof,
            storageProof: seqChainStorageProof
        });
        bytes32 epochDataHash = keccak256(abi.encode(appchains, tokens, emissionsReceivers));
        if (verifiedEpochDataHash != epochDataHash) {
            revert InvalidData();
        }

        // data submitted is valid, store it
        uint256 totalTokensUsed = 0;
        epochAppchainIDs[epoch] = appchains;
        for (uint256 i = 0; i < appchains.length; i++) {
            totalTokensUsed += tokens[i];
            epochAppchainTokensUsed[epoch][appchains[i]] += tokens[i];
            epochAppchainEmissionsReceiver[epoch][appchains[i]] = emissionsReceivers[i];
        }
        epochTotalTokensUsed[epoch] = totalTokensUsed;
        archivedEpochData[epoch] = true;
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Verifies that a sequencing chain block hash is valid according to Ethereum storage proofs
    /// @dev Uses Merkle Patricia proofs to verify that the sequencing chain block hash exists in
    ///      the Ethereum bridge contract's storage at the specified slot
    /// @param seqChainID The sequencing chain ID to verify
    /// @param seqChainBlockHash The block hash from the sequencing chain to verify
    /// @param ethereumBlockHeader RLP-encoded Ethereum block header containing the state root
    /// @param accountProof Merkle proof of the bridge contract account in Ethereum state
    /// @param storageProof Merkle proof of the storage slot containing the sequencing chain block hash
    function _verifyEthereumProof(
        uint256 seqChainID,
        bytes32 seqChainBlockHash,
        bytes calldata ethereumBlockHeader,
        bytes[] calldata accountProof,
        bytes[] calldata storageProof
    ) internal view {
        if (keccak256(ethereumBlockHeader) != lastKnownEthereumBlockHash) {
            revert InvalidEthereumBlockHeader();
        }
        address account = ethereumSeqChainBridges[seqChainID];
        if (account == address(0)) {
            if (useSettlementChainAsSequencingChain && seqChainID == settlementChainID) {
                revert CannotSubmitProofForSettlementChain();
            }
            revert ChainIDNotFound();
        }
        bytes32 verifiedSeqChainBlockHash = _getSlotValueFromProof({
            stateRoot: _getStateRootFromHeader(ethereumBlockHeader),
            accountProof: accountProof,
            storageProof: storageProof,
            account: account,
            storageSlot: ethereumSeqChainStorageSlots[seqChainID]
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
        address account = seqChainGasAggregatorAddresses[seqChainID];
        if (account == address(0)) {
            revert ChainIDNotFound();
        }
        return _getSlotValueFromProof({
            stateRoot: _getStateRootFromHeader(blockHeader),
            accountProof: accountProof,
            storageProof: storageProof,
            account: account,
            storageSlot: _getStorageSlot(epoch)
        });
    }

    /// @notice Extracts the storage root from an account proof
    /// @dev Verifies the account proof against the state root and returns the account's storage root
    ///      from the RLP-decoded account fields
    /// @param account The account address to verify
    /// @param stateRoot The Ethereum state root to verify against
    /// @param accountProof Merkle proof of the account in the state trie
    /// @return The storage root hash of the verified account
    function _storageRootFromAccountProof(address account, bytes32 stateRoot, bytes[] calldata accountProof)
        internal
        pure
        returns (bytes32)
    {
        bytes32 accountHash = keccak256(abi.encodePacked(account));
        RLPReader.RLPItem memory accountRlp = MerklePatriciaProofVerifier.extractProofValue({
            rootHash: stateRoot,
            path: abi.encodePacked(accountHash),
            stack: _RLPItemsFromProofBytes(accountProof)
        }).toRlpItem();

        // If the account does not exist, return the hash of an empty trie.
        if (accountRlp.len == 0) {
            revert AccountDoesNotExistInProof();
        }

        RLPReader.RLPItem[] memory accountFields = accountRlp.toList();

        return bytes32(accountFields[STORAGE_ROOT_ACCOUNT_FIELDS_INDEX].toUintStrict());
    }

    /// @notice Retrieves a storage slot value using Merkle Patricia proofs (can be obtained from `eth_getProof`)
    /// @dev First verifies the account proof to get the storage root, then verifies the storage proof
    ///      to extract the value at the specified storage slot
    /// @param stateRoot The Ethereum state root to verify against
    /// @param accountProof Merkle proof of the account in the state trie
    /// @param storageProof Merkle proof of the storage slot in the account's storage trie
    /// @param account The account address containing the storage slot
    /// @param storageSlot The storage slot to retrieve the value from
    /// @return The value stored in the specified storage slot
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
            stack: _RLPItemsFromProofBytes(storageProof)
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

    /// @notice creates RLP items from the given proof bytes.
    ///
    /// @param proof The proof bytes.
    ///
    /// @return The RLP items.
    function _RLPItemsFromProofBytes(bytes[] memory proof) private pure returns (RLPReader.RLPItem[] memory) {
        RLPReader.RLPItem[] memory proofItems = new RLPReader.RLPItem[](proof.length);
        for (uint256 i; i < proof.length; i++) {
            proofItems[i] = proof[i].toRlpItem();
        }
        return proofItems;
    }

    /*//////////////////////////////////////////////////////////////
                             VIEWS
    //////////////////////////////////////////////////////////////*/

    modifier onlyArchivedEpoch(uint256 epochIndex) {
        if (!archivedEpochData[epochIndex]) revert NotArchivedEpoch();
        _;
    }

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
        uint256[] memory appchainIDs = epochAppchainIDs[epochIndex];
        return appchainIDs;
    }

    function getAppchainRewardsReceiver(uint256 epochIndex, uint256 appchainId)
        external
        view
        onlyArchivedEpoch(epochIndex)
        returns (address)
    {
        return epochAppchainEmissionsReceiver[epochIndex][appchainId];
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a new sequencing chain configuration
    /// @dev Only admin can add sequencing chains. Special handling for settlement chain as sequencing chain
    /// @param chainID The chain ID of the sequencing chain
    /// @param aggregatorAddress Address of the GasAggregator contract on the sequencing chain
    /// @param bridgeAddress Address of the bridge contract on Ethereum (not needed for settlement chain)
    /// @param storageSlot Storage slot in the bridge contract where the block hash is stored
    function addSequencingChain(uint256 chainID, address aggregatorAddress, address bridgeAddress, bytes32 storageSlot)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        if (seqChainGasAggregatorAddresses[chainID] != address(0)) {
            revert sequencingChainAlreadyExists();
        }
        if (aggregatorAddress == address(0)) {
            revert ZeroAddress();
        }

        if (chainID == settlementChainID) {
            useSettlementChainAsSequencingChain = true;
            seqChainGasAggregatorAddresses[chainID] = aggregatorAddress;
            return;
        }

        if (bridgeAddress == address(0)) {
            revert ZeroAddress();
        }

        seqChainIDs.push(chainID);
        seqChainGasAggregatorAddresses[chainID] = aggregatorAddress;
        ethereumSeqChainBridges[chainID] = bridgeAddress;
        ethereumSeqChainStorageSlots[chainID] = storageSlot;
    }

    /// @notice Removes a sequencing chain configuration
    /// @dev Only admin can remove sequencing chains. Special handling for settlement chain
    /// @param chainID The chain ID of the sequencing chain to remove
    function removeSeqChain(uint256 chainID) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (chainID == settlementChainID) {
            useSettlementChainAsSequencingChain = false;
            delete seqChainGasAggregatorAddresses[chainID];
            return;
        }

        uint256 index = seqChainIDs.length;
        for (uint256 i = 0; i < seqChainIDs.length; i++) {
            if (seqChainIDs[i] == chainID) {
                index = i;
                break;
            }
        }
        if (index == seqChainIDs.length) {
            revert ChainIDNotFound();
        }
        seqChainIDs[index] = seqChainIDs[seqChainIDs.length - 1];
        seqChainIDs.pop();
        delete seqChainGasAggregatorAddresses[chainID];
        delete ethereumSeqChainBridges[chainID];
        delete ethereumSeqChainStorageSlots[chainID];
    }

    /// @notice Updates the authorized block hash sender address
    /// @dev Only admin can change the block hash sender
    /// @param newBlockHashSender The new address authorized to send block hashes
    function setBlockHashSender(address newBlockHashSender) external onlyRole(DEFAULT_ADMIN_ROLE) {
        blockHashSender = newBlockHashSender;
    }
}
