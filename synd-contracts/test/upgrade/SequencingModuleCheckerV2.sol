// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "../../src/interfaces/IPermissionModule.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {DataTooLarge} from "@arbitrum/nitro-contracts/src/libraries/Error.sol";

/// @custom:storage-location erc7201:syndicate.storage.SequencingModule
struct SequencingModuleStorage {
    /// @notice The requirement module that handles checks
    IPermissionModule permissionRequirementModule;
    /// @notice NEW FIELD: Maximum number of transactions per batch
    uint256 maxTransactionsPerBatch;
    /// @notice NEW FIELD: Enable/disable batch processing
    bool batchProcessingEnabled;
    /// @notice NEW FIELD: Timestamp of last permission update
    uint256 lastPermissionUpdate;
}

/// @title SequencingModuleCheckerV2
/// @notice Upgraded version with additional storage fields - tests namespaced storage safety
abstract contract SequencingModuleCheckerV2 is Initializable, OwnableUpgradeable, IPermissionModule {
    event RequirementModuleUpdated(address indexed newModule);
    event MaxTransactionsPerBatchUpdated(uint256 newMax);
    event BatchProcessingToggled(bool enabled);

    // Just in case, limit the amount of tx data sent to the isAllowed function.
    uint256 public constant maxDataSize = 200000;

    // cast index-erc7201 syndicate.storage.SequencingModule
    bytes32 public constant SEQUENCING_MODULE_STORAGE_LOCATION =
        0x5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500;

    function _getSequencingModuleStorage() private pure returns (SequencingModuleStorage storage $) {
        assembly {
            $.slot := SEQUENCING_MODULE_STORAGE_LOCATION
        }
    }

    function permissionRequirementModule() public view returns (IPermissionModule) {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        return $.permissionRequirementModule;
    }

    /// @notice NEW GETTER: Get max transactions per batch
    function maxTransactionsPerBatch() public view returns (uint256) {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        return $.maxTransactionsPerBatch;
    }

    /// @notice NEW GETTER: Check if batch processing is enabled
    function batchProcessingEnabled() public view returns (bool) {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        return $.batchProcessingEnabled;
    }

    /// @notice NEW GETTER: Get last permission update timestamp
    function lastPermissionUpdate() public view returns (uint256) {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        return $.lastPermissionUpdate;
    }

    function __SequencingModuleChecker_init(address admin, address _permissionRequirementModule)
        internal
        onlyInitializing
    {
        __Ownable_init(admin);
        _getSequencingModuleStorage().permissionRequirementModule = IPermissionModule(_permissionRequirementModule);
        // Initialize new fields with default values
        _getSequencingModuleStorage().maxTransactionsPerBatch = 100;
        _getSequencingModuleStorage().batchProcessingEnabled = true;
        _getSequencingModuleStorage().lastPermissionUpdate = block.timestamp;
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        _getSequencingModuleStorage().permissionRequirementModule = IPermissionModule(_newModule);
        _getSequencingModuleStorage().lastPermissionUpdate = block.timestamp; // NEW: Track update time

        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice NEW FUNCTION: Set max transactions per batch
    function setMaxTransactionsPerBatch(uint256 _max) external onlyOwner {
        require(_max > 0, "Max must be greater than 0");
        _getSequencingModuleStorage().maxTransactionsPerBatch = _max;
        emit MaxTransactionsPerBatchUpdated(_max);
    }

    /// @notice NEW FUNCTION: Toggle batch processing
    function toggleBatchProcessing() external onlyOwner {
        SequencingModuleStorage storage $ = _getSequencingModuleStorage();
        $.batchProcessingEnabled = !$.batchProcessingEnabled;
        emit BatchProcessingToggled($.batchProcessingEnabled);
    }

    /// @notice Checks if both the proposer and calldata are allowed
    function isAllowed(address proposer, address originator, bytes memory data) public view returns (bool) {
        require(data.length <= maxDataSize, DataTooLarge(data.length, maxDataSize));
        return address(permissionRequirementModule()) == address(1)
            || permissionRequirementModule().isAllowed(proposer, originator, data);
    }
}
