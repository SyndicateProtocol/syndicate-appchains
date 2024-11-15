package translator

import (
	"context"
	"fmt"
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

type TransactionsWrapper struct {
	RawTxs    []hexutil.Bytes
	ParsedTxs []*rpc.ParsedTransaction
}

// ParseRawTransactions perform an inexpensive local validation
// In the case we have an invalid transaction, it should not be included in the block
// This filters transactions using a local stateless validation, i.e. gas, nonces
// and chain-specific configs such as activated hardforks are *not* validated at this point
// State-dependent validations can only be performed by the MetaBased chain itself
func ParseRawTransactions(txs []hexutil.Bytes) (rawTxns []hexutil.Bytes, parsedTxns []*rpc.ParsedTransaction, removedCountStateless int) {
	rawTxns = make([]hexutil.Bytes, 0, len(txs))
	parsedTxns = make([]*rpc.ParsedTransaction, 0, len(txs))
	removedCountStateless = 0

	for _, rawTx := range txs {
		tx := new(ethtypes.Transaction)
		unmarshalErr := tx.UnmarshalBinary(rawTx)
		if unmarshalErr != nil {
			log.Warn().Err(unmarshalErr).Msgf("can't unmarshall transaction: %+v", tx)
			removedCountStateless++
			continue
		}

		// Validate transaction locally
		validationErr := ValidateTransactionInternal(tx)
		if validationErr != nil {
			log.Warn().Err(validationErr).Msgf("skipping invalid transaction: %+v", tx)
			removedCountStateless++
			continue
		}

		// Derive from address from the sender
		from, err := ethtypes.Sender(ethtypes.NewEIP155Signer(tx.ChainId()), tx)
		if err != nil {
			log.Warn().Err(err).Msgf("can't derive from address from the sender: %+v", tx)
			removedCountStateless++
			continue
		}

		simTx := &rpc.ParsedTransaction{
			From:                 from.Hex(),
			To:                   tx.To().Hex(),
			Value:                tx.Value().String(),
			Data:                 hexutil.Encode(tx.Data()),
			Nonce:                strconv.FormatUint(tx.Nonce(), 10),
			Gas:                  strconv.FormatUint(tx.Gas(), 10),
			MaxFeePerGas:         tx.GasPrice().String(),
			MaxPriorityFeePerGas: tx.GasTipCap().String(),
		}

		rawTxns = append(rawTxns, rawTx)
		parsedTxns = append(parsedTxns, simTx)
	}
	return rawTxns, parsedTxns, removedCountStateless
}

// ValidateTransactionInternal is a lightweight stateless MetaBased tx validation
// And should not be used a general validation for non-MB
func ValidateTransactionInternal(tx *ethtypes.Transaction) error {
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

func (m *MetaBasedBatchProvider) ValidateTransactionBlock(rawTxns []hexutil.Bytes, txns []rpc.ParsedTransaction) []hexutil.Bytes {
	// Step 1: Create empty state
	// var validationState = rpc.ValidationState{
	// 	TransactionStateValidation: make(map[string]rpc.TransactionStateValidation),
	// 	BlockStateValidation:       rpc.BlockStateValidation{},
	// }

	validTxs := make([]hexutil.Bytes, len(rawTxns))

	for {
		// Step 2: Call internal ValidateTransactionBlock
		// validTxs = rpc.ValidateTransactionState(txns, validationState)

		// Step 3: Call external ValidateTransactionBlock
		log.Debug().Msgf("ValidateTransactionStateful txs: %v", txns)
		request := rpc.SimulationRequest{
			BlockStateCalls: []rpc.BlockStateCall{
				{
					Calls: txns,
				},
			},
			Validation: true,
		}
		log.Debug().Interface("request", request).Msg("Simulation request")
		result, err := m.MetaBasedChain.SimulateTransactions(context.Background(), request, "latest")
		log.Debug().Interface("result", result).Msg("Simulation result")

		// If error: Pasre info into state loop
		if err != nil {
			// validationState.BlockStateValidation.Error = err.Error()
			log.Debug().Err(err).Msg("Error in simulation")
			continue
		} else {
			// If success: return
			return validTxs
		}
	}
}
