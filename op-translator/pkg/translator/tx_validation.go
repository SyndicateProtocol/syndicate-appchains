package translator

import (
	"context"
	"fmt"
	"regexp"
	"slices"
	"strconv"

	rpc "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/txpool"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
	"github.com/rs/zerolog/log"
)

const (
	// txMaxSize is the maximum size a single transaction can have
	// In go-ethereum this is set to 128KiB, which is 4 slots x 32KiB slot size
	// But since we need compatibility also with OP and Arb, plus support some
	// MetaBased overhead, we set this to 90KiB for now
	txMaxSize = 90 * 1024 // 90KiB

	// maxBigIntBitLen is the max length in bits for a BigInt
	maxBigIntBitLen = 256
)

// FilterTransactionsStateless perform an inexpensive local validation
// In the case we have an invalid transaction, it should not be included in the block
// This filters transactions using a local stateless validation, i.e. gas, nonces
// and chain-specific configs such as activated hardforks are *not* validated at this point
// State-dependent validations can only be performed by the MetaBased chain itself
func FilterTransactionsStateless(txs []hexutil.Bytes) (filteredTxsStateless []hexutil.Bytes, parsedFilteredTxStateless []*ethtypes.Transaction, removedCountStateless int) {
	filteredTxsStateless = make([]hexutil.Bytes, 0, len(txs))
	parsedFilteredTxStateless = make([]*ethtypes.Transaction, 0, len(txs))
	removedCountStateless = 0

	for _, rawTx := range txs {
		tx := new(ethtypes.Transaction)
		unmarshalErr := tx.UnmarshalBinary(rawTx)
		if unmarshalErr != nil {
			log.Warn().Err(unmarshalErr).Msgf("can't unmarshall transaction: %+v", tx)
			removedCountStateless++
			continue
		}
		validationErr := ValidateTransactionStateless(tx)
		if validationErr != nil {
			log.Warn().Err(validationErr).Msgf("skipping invalid transaction: %+v", tx)
			removedCountStateless++
			continue
		}
		filteredTxsStateless = append(filteredTxsStateless, rawTx)
		parsedFilteredTxStateless = append(parsedFilteredTxStateless, tx)
	}

	return filteredTxsStateless, parsedFilteredTxStateless, removedCountStateless
}

// ValidateTransactionStateless is a lightweight stateless MetaBased tx validation
// And should not be used a general validation for non-MB
func ValidateTransactionStateless(tx *ethtypes.Transaction) error {
	acceptedTypes := []uint8{
		ethtypes.LegacyTxType,     // supported since Berlin hardfork activation
		ethtypes.DynamicFeeTxType, // supported since London hardfork activation
		// currently not supported by off-the-shelf L2:
		// ethtypes.AccessListTxType
		// ethtypes.BlobTxType
	}
	if !slices.Contains(acceptedTypes, tx.Type()) {
		return fmt.Errorf("%w: tx type %v not supported by this pool", core.ErrTxTypeNotSupported, tx.Type())
	}

	// Before performing any expensive validations, sanity check that the tx is
	// smaller than the maximum limit the pool can meaningfully handle
	if tx.Size() > txMaxSize {
		return fmt.Errorf("%w: transaction size %v, limit %v", txpool.ErrOversizedData, tx.Size(), txMaxSize)
	}

	// Check whether the init code size has been exceeded
	if tx.To() == nil && len(tx.Data()) > params.MaxInitCodeSize {
		return fmt.Errorf("%w: code size %v, limit %v", core.ErrMaxInitCodeSizeExceeded, len(tx.Data()), params.MaxInitCodeSize)
	}

	// Transactions can't be negative. This may never happen using RLP decoded
	// transactions but may occur for transactions created using the RPC.
	if tx.Value().Sign() < 0 {
		return txpool.ErrNegativeValue
	}

	// Sanity check for extremely large numbers (supported by RLP or RPC)
	if tx.GasFeeCap().BitLen() > maxBigIntBitLen {
		return core.ErrFeeCapVeryHigh
	}
	if tx.GasTipCap().BitLen() > maxBigIntBitLen {
		return core.ErrTipVeryHigh
	}
	// Ensure gasFeeCap is greater than or equal to gasTipCap
	if tx.GasFeeCapIntCmp(tx.GasTipCap()) < 0 {
		return core.ErrTipAboveFeeCap
	}
	// Ensure the transaction has more gas than the bare minimum needed to cover
	// the transaction metadata
	intrGas, err := core.IntrinsicGas(tx.Data(), tx.AccessList(), tx.To() == nil, true, true, true)
	if err != nil {
		return err
	}
	if tx.Gas() < intrGas {
		return fmt.Errorf("%w: gas %v, minimum needed %v", core.ErrIntrinsicGas, tx.Gas(), intrGas)
	}
	return nil
}

func (m *MetaBasedBatchProvider) ValidateTransactionStateful(txs []rpc.Transaction) error {
	log.Debug().Msgf("ValidateTransactionStateful txs: %v", txs)
	request := rpc.SimulationRequest{
		BlockStateCalls: []rpc.BlockStateCall{
			{
				Calls: txs,
			},
		},
		Validation: true,
	}
	log.Debug().Interface("request", request).Msg("Simulation request")
	return m.MetaBasedChain.SimulateTransactions(context.Background(), request, "latest")
}

func ExtractTxIndexFromError(errorMessage string) (int, error) {
	// Define a regex pattern to find the "tx: <index>" substring
	pattern := `tx: (\d+)`
	re := regexp.MustCompile(pattern)

	// Find the first match
	match := re.FindStringSubmatch(errorMessage)
	if len(match) < 2 {
		return 0, fmt.Errorf("transaction index not found in error message")
	}

	txIndex, err := strconv.Atoi(match[1])
	if err != nil {
		return 0, fmt.Errorf("failed to parse transaction index: %w", err)
	}

	return txIndex, nil
}

// FilterTransactionsStateful depends on the state of the chain to get access to
// gas parameters, limits of the mempool, account state for nonces
// so we delegate this to the MetaBased chain for now until op-translator
// is not aware of the chain state
func (m *MetaBasedBatchProvider) FilterTransactionsStateful(rawTxs []hexutil.Bytes, parsedTxs []rpc.Transaction) (rawFilteredTxStateful []hexutil.Bytes, removedCountStateful int) {
	rawFilteredTxStateful = make([]hexutil.Bytes, 0, len(rawTxs))
	removedCountStateful = 0

	for {
		log.Debug().Msgf("rawTxs: %v, parsedTxs: %v", rawTxs, parsedTxs)
		// Run validation on the current set of parsed transactions
		validationErr := m.ValidateTransactionStateful(parsedTxs)
		if validationErr == nil {
			log.Debug().Msgf("no validation errors, breaking")
			// No validation errors, break out of the loop
			rawFilteredTxStateful = append(rawFilteredTxStateful, rawTxs...)
			break
		}

		// Extract the transaction index from the error
		log.Error().Msgf("validation error: %v", validationErr)
		log.Error().Msgf("validation error.Error(): %v", validationErr.Error())
		txIndex, err := ExtractTxIndexFromError(validationErr.Error())
		if err != nil || txIndex < 0 || txIndex >= len(parsedTxs) {
			// If the extraction fails or the index is out of range, break the loop
			fmt.Println("Failed to extract valid transaction index from error:", err)
			break
		}

		// Remove the invalid transaction from both slices
		rawTxs = append(rawTxs[:txIndex], rawTxs[txIndex+1:]...)
		parsedTxs = append(parsedTxs[:txIndex], parsedTxs[txIndex+1:]...)
		removedCountStateful++

		// Reset rawFilteredTxStateful for the next iteration, as we're modifying rawTxs
		rawFilteredTxStateful = rawFilteredTxStateful[:0]
	}

	return rawFilteredTxStateful, removedCountStateful
}
