// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {ISyndicateSequencingChain} from "../../src/interfaces/ISyndicateSequencingChain.sol";
import {GasCounter} from "../../src/staking/GasCounter.sol";
import "./SequencingModuleCheckerV2.sol";

/// @title SyndicateSequencingChainV2
/// @notice Upgraded version with additional storage fields - tests upgrade safety
/// @dev This tests both traditional storage (appended to end) and namespaced storage safety
contract SyndicateSequencingChainV2 is
    Initializable,
    SequencingModuleCheckerV2,
    ISyndicateSequencingChain,
    GasCounter,
    UUPSUpgradeable
{
    error NoTxData();
    error TransactionOrSenderNotAllowed();

    /*//////////////////////////////////////////////////////////////
                STATE VARIABLES - DO NOT REORDER
    //////////////////////////////////////////////////////////////*/

    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    /// Storage slot 0
    uint256 public appchainId;

    /// @notice The address that receives emissions for this sequencing chain
    /// Storage slot 1
    address public emissionsReceiver;

    /// @notice The factory contract that deployed this chain
    /// Storage slot 2
    address public factory;

    /// @notice Whether to allow gas tracking ban on upgrade (defaults to true for backwards compatibility)
    /// Storage slot 3
    bool public allowGasTrackingBanOnUpgrade;

    /*//////////////////////////////////////////////////////////////
                NEW STORAGE VARIABLES - APPENDED SAFELY
    //////////////////////////////////////////////////////////////*/

    /// @notice NEW FIELD: Maximum gas limit per transaction
    /// Storage slot 4
    uint256 public maxGasPerTransaction;

    /// @notice NEW FIELD: Enable/disable transaction replay protection
    /// Storage slot 5
    bool public replayProtectionEnabled;

    /// @notice NEW FIELD: Minimum time between transactions from same address
    /// Storage slot 6
    uint256 public minTimeBetweenTxs;

    /// @notice NEW FIELD: Mapping of address to last transaction timestamp
    /// Storage slot 7
    mapping(address => uint256) public lastTransactionTime;

    /*//////////////////////////////////////////////////////////////
                            EVENTS
    //////////////////////////////////////////////////////////////*/

    event TransactionProcessed(address indexed sender, bytes data);
    event MaxGasPerTransactionUpdated(uint256 newMax);
    event ReplayProtectionToggled(bool enabled);
    event MinTimeBetweenTxsUpdated(uint256 newMin);

    /*//////////////////////////////////////////////////////////////
                        INITIALIZATION
    //////////////////////////////////////////////////////////////*/

    function initialize(
        address admin,
        address _permissionRequirementModule,
        uint256 _appchainId,
        address _factory,
        address _emissionsReceiver
    ) external initializer {
        // Initialize parent contracts
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
        __UUPSUpgradeable_init();
        
        // Initialize storage variables
        appchainId = _appchainId;
        factory = _factory;
        emissionsReceiver = _emissionsReceiver;
        
        // Enable gas tracking
        _enableGasTracking();
        // Set default to false for new deployments
        allowGasTrackingBanOnUpgrade = false;

        // Initialize new V2 fields with default values
        maxGasPerTransaction = 1000000; // 1M gas default
        replayProtectionEnabled = true;
        minTimeBetweenTxs = 0; // No minimum by default
    }

    /*//////////////////////////////////////////////////////////////
                        NEW V2 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice NEW FUNCTION: Set maximum gas per transaction
    function setMaxGasPerTransaction(uint256 _maxGas) external onlyOwner {
        require(_maxGas > 0, "Max gas must be greater than 0");
        maxGasPerTransaction = _maxGas;
        emit MaxGasPerTransactionUpdated(_maxGas);
    }

    /// @notice NEW FUNCTION: Toggle replay protection
    function toggleReplayProtection() external onlyOwner {
        replayProtectionEnabled = !replayProtectionEnabled;
        emit ReplayProtectionToggled(replayProtectionEnabled);
    }

    /// @notice NEW FUNCTION: Set minimum time between transactions
    function setMinTimeBetweenTxs(uint256 _minTime) external onlyOwner {
        minTimeBetweenTxs = _minTime;
        emit MinTimeBetweenTxsUpdated(_minTime);
    }

    /*//////////////////////////////////////////////////////////////
                    TRANSACTION PROCESSING
    //////////////////////////////////////////////////////////////*/

    modifier onlyWhenAllowed(bytes calldata data) {
        if (!isAllowed(msg.sender, tx.origin, data)) {
            revert TransactionOrSenderNotAllowed();
        }
        _;
    }

    modifier checkReplayProtection() {
        if (replayProtectionEnabled && minTimeBetweenTxs > 0) {
            require(
                block.timestamp >= lastTransactionTime[msg.sender] + minTimeBetweenTxs,
                "Transaction too soon after previous"
            );
            lastTransactionTime[msg.sender] = block.timestamp;
        }
        _;
    }

    function processTransaction(bytes calldata data) 
        external 
        onlyWhenAllowed(data) 
        checkReplayProtection // NEW: Check replay protection
    {
        if (data.length == 0) {
            revert NoTxData();
        }

        // NEW: Check gas limit if set
        if (maxGasPerTransaction > 0) {
            require(gasleft() <= maxGasPerTransaction, "Transaction exceeds gas limit");
        }

        emit TransactionProcessed(msg.sender, data);
    }

    function processTransactionsBulk(bytes[] calldata data) 
        external 
        checkReplayProtection // NEW: Check replay protection
    {
        // Check permission for bulk transaction
        bytes memory bulkData = abi.encode(data);
        if (!isAllowed(msg.sender, tx.origin, bulkData)) {
            revert TransactionOrSenderNotAllowed();
        }

        require(batchProcessingEnabled(), "Batch processing is disabled"); // NEW: Check if batch processing enabled

        uint256 length = data.length;
        require(length > 0, "No transactions provided");
        require(length <= maxTransactionsPerBatch(), "Too many transactions in batch"); // NEW: Check batch limit

        for (uint256 i = 0; i < length; i++) {
            if (data[i].length > 0) {
                emit TransactionProcessed(msg.sender, data[i]);
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                        EMISSIONS & OWNERSHIP
    //////////////////////////////////////////////////////////////*/

    function getEmissionsReceiver() public view returns (address) {
        return emissionsReceiver == address(0) ? owner() : emissionsReceiver;
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        if (!allowGasTrackingBanOnUpgrade) {
            require(allowGasTrackingBanOnUpgrade, "Upgrade would result in gas tracking ban");
        }
    }

    /*//////////////////////////////////////////////////////////////
                         GAS TRACKING ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function disableGasTracking() external onlyOwner {
        _disableGasTracking();
    }

    function enableGasTracking() external onlyOwner {
        _enableGasTracking();
    }

    function setAllowGasTrackingBanOnUpgrade(bool _allowGasTrackingBanOnUpgrade) external onlyOwner {
        allowGasTrackingBanOnUpgrade = _allowGasTrackingBanOnUpgrade;
    }

    /*//////////////////////////////////////////////////////////////
                            UTILITY FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function encodeTransaction(bytes calldata data) public pure returns (bytes memory) {
        return data;
    }
}