// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IPermissionModule} from "./interfaces/IPermissionModule.sol";
import {NotInitializedModule} from "./sequencing-modules/NotInitializedModule.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

interface ArbSys {
    function arbBlockNumber() external view returns (uint256);
}

ArbSys constant arbsys = ArbSys(0x0000000000000000000000000000000000000064);

/// @title SequencingModuleChecker
/// @notice A simplified contract that delegates permission checks to modules

abstract contract SequencingModuleChecker is Ownable, IPermissionModule {
    /// @custom:storage-location
    struct TxData {
        bytes32 acc;
        uint64 count;
    }

    // keccak256("syndicate.tx.data")
    bytes32 public constant TX_DATA_STORAGE_LOCATION =
        0xbcd134af035e52869741eb0221dfc8a26900a04521f5a2d44a59b675ea20a969;

    function _getTxData() private pure returns (TxData storage $) {
        assembly {
            $.slot := TX_DATA_STORAGE_LOCATION
        }
    }

    function txCount() public view returns (uint64) {
        return _getTxData().count;
    }

    function txAcc() public view returns (bytes32) {
        return _getTxData().acc;
    }

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice The requirement module that handles checks
    IPermissionModule public permissionRequirementModule;

    event RequirementModuleUpdated(address indexed newModule);

    error InvalidModuleAddress();
    error TransactionOrProposerNotAllowed();
    error AlreadyInitialized();

    bool internal hasBeenInitialized = false;

    /// @dev Constructor function
    // [Olympix Warning: no parameter validation in constructor] Admin validation handled by OpenZeppelin's Ownable
    constructor() Ownable(msg.sender) {
        permissionRequirementModule = new NotInitializedModule();
    }

    /// @notice Initializes the contract with a new admin and a requirement module
    /// @param admin The address of the new admin
    /// @param _permissionRequirementModule The address of the RequireAll or RequireAny module
    function initialize(address admin, address _permissionRequirementModule) external onlyOwner {
        if (hasBeenInitialized) revert AlreadyInitialized();
        if (_permissionRequirementModule == address(0)) revert InvalidModuleAddress();

        hasBeenInitialized = true;

        permissionRequirementModule = IPermissionModule(_permissionRequirementModule);

        transferOwnership(admin);
    }

    /// @notice Updates the requirement module
    /// @param _newModule The address of the new requirement module
    function updateRequirementModule(address _newModule) external onlyOwner {
        if (_newModule == address(0)) revert InvalidModuleAddress();
        permissionRequirementModule = IPermissionModule(_newModule);

        emit RequirementModuleUpdated(_newModule);
    }

    /// @notice Checks if both the proposer and calldata are allowed
    /// @param proposer The address to check
    /// @param originator The address of tx.origin.
    /// @param data The calldata to check
    /// @return bool indicating if both the proposer and calldata are allowed
    function isAllowed(address proposer, address originator, bytes calldata data) public view returns (bool) {
        return permissionRequirementModule.isAllowed(proposer, originator, data); //#olympix-ignore-calls-in-loop
    }

    function transactionProcessed(bytes calldata data) internal returns (bool) {
        if (!isAllowed(msg.sender, tx.origin, data)) {
            return false;
        }
        _transactionProcessed(data);
        return true;
    }

    function uncompressedTransactionProcessed(bytes calldata data) internal returns (bool) {
        if (!isAllowed(msg.sender, tx.origin, data)) {
            return false;
        }
        _transactionProcessed(prependZeroByte(data));
        return true;
    }

    function _transactionProcessed(bytes memory data) private {
        TxData storage txData = _getTxData();
        uint256 blockNumber = block.number;
        if (address(arbsys).code.length > 0) {
            try arbsys.arbBlockNumber() returns (uint256 number) {
                blockNumber = number;
            } catch {}
        }

        txData.acc = keccak256(
            abi.encodePacked(txData.acc, msg.sender, blockNumber, block.timestamp, txData.count, keccak256(data))
        );
        txData.count += 1;
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data with a zero byte prepended
    function prependZeroByte(bytes calldata _data) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
