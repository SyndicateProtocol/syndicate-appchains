// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

/// @title SyndicateSequencingChainUpgradeV2
/// @notice V2 upgrade for testing - adds new functionality safely
/// @dev This demonstrates safe upgrade patterns:
///      - Traditional storage: New variables appended at end
///      - Namespaced storage: New fields in ERC-7201 namespace
contract SyndicateSequencingChainUpgradeV2 is SyndicateSequencingChain {
    /*//////////////////////////////////////////////////////////////
                    NEW V2 STORAGE - APPENDED SAFELY
    //////////////////////////////////////////////////////////////*/

    /// @notice V2: Transaction processing fee in wei
    uint256 public processingFee;

    /// @notice V2: Enable/disable fee collection
    bool public feeCollectionEnabled;

    /// @notice V2: Total fees collected
    uint256 public totalFeesCollected;

    /*//////////////////////////////////////////////////////////////
                            NEW V2 EVENTS
    //////////////////////////////////////////////////////////////*/

    event ProcessingFeeUpdated(uint256 newFee);
    event FeeCollectionToggled(bool enabled);
    event FeeCollected(address indexed payer, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                        NEW V2 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the processing fee for transactions
    /// @param _fee New fee amount in wei
    function setProcessingFee(uint256 _fee) external onlyOwner {
        processingFee = _fee;
        emit ProcessingFeeUpdated(_fee);
    }

    /// @notice Toggle fee collection on/off
    function toggleFeeCollection() external onlyOwner {
        feeCollectionEnabled = !feeCollectionEnabled;
        emit FeeCollectionToggled(feeCollectionEnabled);
    }

    /// @notice Process transaction with fee (new V2 functionality)
    /// @dev Adds fee collection to existing functionality
    function processTransactionWithFee(bytes calldata data) external payable trackGasUsage {
        require(data.length > 0, NoTxData());

        // Encode transaction
        bytes memory transaction = encodeTransaction(data);

        // Check authorization
        require(isAllowed(msg.sender, tx.origin, transaction), TransactionOrSenderNotAllowed());

        // Collect fee if enabled
        if (feeCollectionEnabled && processingFee > 0) {
            require(msg.value >= processingFee, "Insufficient fee");
            totalFeesCollected += msg.value;
            emit FeeCollected(msg.sender, msg.value);
        }

        emit TransactionProcessed(msg.sender, transaction);
    }

    /// @notice Withdraw collected fees
    /// @param recipient Address to receive fees
    function withdrawFees(address payable recipient) external onlyOwner {
        require(recipient != address(0), "Invalid recipient");
        uint256 amount = address(this).balance;
        require(amount > 0, "No fees to withdraw");

        (bool success,) = recipient.call{value: amount}("");
        require(success, "Transfer failed");
    }

    /// @notice Get contract version
    /// @return Version string
    function contractVersion() external pure returns (string memory) {
        return "2.0.0";
    }
}
