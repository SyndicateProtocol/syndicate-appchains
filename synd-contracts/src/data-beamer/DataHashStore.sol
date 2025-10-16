// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

contract DataHashStore is Ownable(msg.sender) {
    struct DataHashStorage {
        mapping(bytes32 key => bytes32 value) dataHash;
    }

    // Generated using: cast index-erc7201 syndicate.storage.DataHashStore
    bytes32 public constant DATA_HASH_STORAGE_LOCATION =
        0xdefd86032ebaaca80e3baa1b4281b4518ac2b830a870f52678dcf4c66353e100;

    /// @notice Internal function to access the ERC-7201 namespaced storage
    /// @dev Uses inline assembly to access the specific storage slot for this contract's data
    /// @return $ Storage pointer to the DataHashStorage struct
    function _getDataHashStorage() private pure returns (DataHashStorage storage $) {
        assembly {
            $.slot := DATA_HASH_STORAGE_LOCATION
        }
    }

    /// @notice Get the data hash for a given key
    /// @param key The key to get the data hash for
    /// @return The data hash for the given key
    function dataHash(bytes32 key) public view returns (bytes32) {
        DataHashStorage storage $ = _getDataHashStorage();
        return $.dataHash[key];
    }

    /// @notice Store a data hash for a given key
    /// @param key The key to store the data hash for
    /// @param value The data hash to store
    function storeDataHash(bytes32 key, bytes32 value) external onlyOwner {
        DataHashStorage storage $ = _getDataHashStorage();
        $.dataHash[key] = value;
    }
}