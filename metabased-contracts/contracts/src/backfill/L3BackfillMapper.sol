// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {AccessControl} from "openzeppelin-contracts/contracts/access/AccessControl.sol";

/// @title L3BackfillMapper
contract L3BackfillMapper is AccessControl {
    uint256 public immutable l3ChainId;

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
    mapping(uint256 => bytes32[]) public epochBlockNumberToBatchTxHashes;

    /// @notice Constructor that sets up the default admin and manager roles
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId_ The L3 chain ID
    constructor(address admin, address manager, uint256 l3ChainId_) {
        require(admin != address(0), "Admin address cannot be 0");
        require(manager != address(0), "Manager address cannot be 0");
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(l3ChainId_ != 0, "L3 chain ID cannot be 0");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, manager);
        l3ChainId = l3ChainId_;
    }

    /// @notice Indexes the data for a given L3 block number and transaction hash
    /// @param l1EpochBlockNumber The L1 epoch block number
    /// @param txHash The transaction hash corresponding to the transaction where the data was emitted
    function index(uint256 l1EpochBlockNumber, bytes32 txHash) external onlyRole(MANAGER_ROLE) {
        epochBlockNumberToBatchTxHashes[l1EpochBlockNumber].push(txHash);
    }

    /// @notice Indexes the data for a given L1 epoch block numbers and transaction hashes
    /// @param l1EpochBlockNumbers The L1 epoch block numbers
    /// @param txHashes The transaction hashes corresponding to the transactions where the data was emitted
    function indexForMany(uint256[] calldata l1EpochBlockNumbers, bytes32[] calldata txHashes)
        external
        onlyRole(MANAGER_ROLE)
    {
        require(
            l1EpochBlockNumbers.length == txHashes.length,
            "L1 epoch block numbers and transaction hashes must be the same length"
        );
        uint256 length = l1EpochBlockNumbers.length;
        for (uint256 i = 0; i < length; i++) {
            epochBlockNumberToBatchTxHashes[l1EpochBlockNumbers[i]].push(txHashes[i]);
        }
    }
}
