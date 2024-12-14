package translator

import (
	"fmt"
	"slices"

	rpc "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/txpool"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/params"
)

const (
	// txMaxSize is the maximum size a single transaction can have
	// In go-ethereum this is set to 128KiB, which is 4 slots x 32KiB slot size
	// But since we need compatibility also with OP and Arb, plus support some
	// MetaBased overhead, we set this to 90KiB for now
	txMaxSize = 90 * 1024 // 90KiB

	// maxBigIntBitLen is the max length in bits for a BigInt
	maxBigIntBitLen = 256

	decimalBase = 10
)

// ParseRawTransactions perform an inexpensive local validation
// In the case we have an invalid transaction, it should not be included in the block
// This filters transactions using a local stateless validation, i.e. gas, nonces
// and chain-specific configs such as activated hardforks are *not* validated at this point
// State-dependent validations can only be performed by the MetaBased chain itself
func ParseRawTransactions(txs []hexutil.Bytes, log gethlog.Logger) (rawTxns []hexutil.Bytes, parsedTxns []*rpc.ParsedTransaction) {
	rawTxns = make([]hexutil.Bytes, 0, len(txs))
	parsedTxns = make([]*rpc.ParsedTransaction, 0, len(txs))

	for _, rawTx := range txs {
		tx := new(ethtypes.Transaction)
		unmarshalErr := tx.UnmarshalBinary(rawTx)
		if unmarshalErr != nil {
			log.Warn("can't unmarshall transaction", "error", unmarshalErr, "transaction", tx)
			continue
		}

		// Validate transaction locally
		validationErr := ValidateTransactionInternal(tx)
		if validationErr != nil {
			log.Warn("skipping invalid transaction", "error", validationErr, "transaction", tx)
			continue
		}

		// Derive from address from the sender
		from, err := ethtypes.Sender(ethtypes.NewLondonSigner(tx.ChainId()), tx)
		if err != nil {
			log.Warn("can't derive from address from the sender", "tx", tx, "error", err)
			continue
		}

		gas := tx.Gas()
		nonce := tx.Nonce()
		data := tx.Data()

		simTx := &rpc.ParsedTransaction{
			From:                 &from,
			To:                   tx.To(),
			Gas:                  (*hexutil.Uint64)(&gas),
			Value:                (*hexutil.Big)(tx.Value()),
			Data:                 (*hexutil.Bytes)(&data),
			Nonce:                (*hexutil.Uint64)(&nonce),
			MaxFeePerGas:         (*hexutil.Big)(tx.GasFeeCap()),
			MaxPriorityFeePerGas: (*hexutil.Big)(tx.GasTipCap()),
		}

		rawTxns = append(rawTxns, rawTx)
		parsedTxns = append(parsedTxns, simTx)
	}
	return rawTxns, parsedTxns
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
