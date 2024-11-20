package translator

import (
	"context"
	"fmt"
	"math/big"
	"regexp"
	"slices"
	"strconv"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
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

type BlockStateValidation struct {
	BaseFeePerGas *big.Int // All MaxFeePerGas values in the block have to be at least this value
	GasLimit      *big.Int // Sum of all gas values in the block have to be at most this value
}

type WalletStateValidation struct {
	Nonce   *big.Int // Always greater than the nonce of the previous transaction with the same from address
	Balance *big.Int // Higher than SUM(MaxFeePerGas * Gas) for all transactions with the same from address
}

type ValidationState struct {
	WalletStateValidation map[string]WalletStateValidation // Maps from address to transaction state validation
	BlockStateValidation  BlockStateValidation
}

// ParseRawTransactions perform an inexpensive local validation
// In the case we have an invalid transaction, it should not be included in the block
// This filters transactions using a local stateless validation, i.e. gas, nonces
// and chain-specific configs such as activated hardforks are *not* validated at this point
// State-dependent validations can only be performed by the MetaBased chain itself
func ParseRawTransactions(txs []hexutil.Bytes) (rawTxns []hexutil.Bytes, parsedTxns []*rpc.ParsedTransaction) {
	rawTxns = make([]hexutil.Bytes, 0, len(txs))
	parsedTxns = make([]*rpc.ParsedTransaction, 0, len(txs))

	for _, rawTx := range txs {
		tx := new(ethtypes.Transaction)
		unmarshalErr := tx.UnmarshalBinary(rawTx)
		if unmarshalErr != nil {
			log.Warn().Err(unmarshalErr).Msgf("can't unmarshall transaction: %+v", tx)
			continue
		}

		// Validate transaction locally
		validationErr := ValidateTransactionInternal(tx)
		if validationErr != nil {
			log.Warn().Err(validationErr).Msgf("skipping invalid transaction: %+v", tx)
			continue
		}

		// Derive from address from the sender
		from, err := ethtypes.Sender(ethtypes.NewEIP155Signer(tx.ChainId()), tx)
		if err != nil {
			log.Warn().Err(err).Msgf("can't derive from address from the sender: %+v", tx)
			continue
		}

		simTx := &rpc.ParsedTransaction{
			Hash:                 tx.Hash().Hex(),
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

func (m *MetaBasedBatchProvider) ValidateTransactionsBlock(rawTxns []hexutil.Bytes, txns []*rpc.ParsedTransaction) ([]hexutil.Bytes, error) {
	// Step 1: Create empty state
	validationState := ValidationState{
		WalletStateValidation: make(map[string]WalletStateValidation),
		BlockStateValidation:  BlockStateValidation{},
	}

	validParsedTxns := txns
	validRawTxns := rawTxns

	for {
		// Step 2: Call internal ValidateBlockState
		log.Debug().Interface("validParsedTxns", validParsedTxns).Msg("valid parsed txns 1")
		validRawTxns, validParsedTxns = ValidateBlockState(validRawTxns, validParsedTxns, validationState)
		log.Debug().Interface("validParsedTxns", validParsedTxns).Msg("valid parsed txns 2")

		// Step 3: Call external SimulateTransactions
		_, err := m.MetaBasedChain.SimulateTransactions(context.Background(), validParsedTxns, "latest")
		log.Debug().Msgf("error type: %v", getErrorType(err))
		// If simulation error, parse error and update state
		if err != nil {
			log.Debug().Err(err).Msg("Error in SimulateTransactions")
			validationState, err = ParseValidationError(validationState, err)
			log.Debug().Interface("validationState", validationState).Msg("state now")
			if err != nil {
				log.Error().Err(err).Msg("Error in ParseValidationError")
				return []hexutil.Bytes{}, err
			}
			continue
		} else {
			log.Debug().Interface("validParsedTxns", validParsedTxns).Msg("valid parsed txns end")
			return validRawTxns, nil
		}
	}
}

func ValidateBlockState(rawTransactions []hexutil.Bytes, parsedTransactions []*rpc.ParsedTransaction, state ValidationState) ([]hexutil.Bytes, []*rpc.ParsedTransaction) {
	validTransactions := []*rpc.ParsedTransaction{}
	validRawTransactions := []hexutil.Bytes{}
	blockBaseFeePerGas := state.BlockStateValidation.BaseFeePerGas
	blockGasLimit := state.BlockStateValidation.GasLimit
	gasUsedInBlock := big.NewInt(0)

	for i, parsedTransaction := range parsedTransactions {
		txNonce, err := utils.HexToBigInt(parsedTransaction.Nonce)
		if err != nil {
			log.Warn().Err(err).Msgf("can't convert nonce to big int: %v", parsedTransaction.Nonce)
			continue
		}

		txMaxFeePerGas, err := utils.HexToBigInt(parsedTransaction.MaxFeePerGas)
		if err != nil {
			log.Warn().Err(err).Msgf("can't convert maxFeePerGas to big int: %v", parsedTransaction.MaxFeePerGas)
			continue
		}

		txGas, err := utils.HexToBigInt(parsedTransaction.Gas)
		if err != nil {
			log.Warn().Err(err).Msgf("can't convert gas to big int: %v", parsedTransaction.Gas)
			continue
		}

		txValue, err := utils.HexToBigInt(parsedTransaction.Value)
		if err != nil {
			log.Warn().Err(err).Msgf("can't convert value to big int: %v", parsedTransaction.Value)
			continue
		}

		// Check tx.MaxFeePerGas < block BaseFeePerGas if available
		if blockBaseFeePerGas != nil && blockBaseFeePerGas.Cmp(txMaxFeePerGas) < 1 {
			log.Debug().Msgf("maxFeePerGas mismatch: %v < %v", txMaxFeePerGas, blockBaseFeePerGas)
			continue
		}
		newGasUsed := new(big.Int).Add(gasUsedInBlock, txGas)
		if blockGasLimit != nil && blockGasLimit.Cmp(newGasUsed) < 1 {
			log.Debug().Msgf("gas limit mismatch: %v < %v", newGasUsed, blockGasLimit)
			continue
		}

		from := parsedTransaction.From
		walletState, exists := state.WalletStateValidation[from]
		if exists {
			// Validate nonce: tx.Nonce == state.WalletStateValidation[from].Nonce
			if txNonce != walletState.Nonce {
				log.Debug().Msgf("nonce mismatch: %v != %v", txNonce, walletState.Nonce)
				continue
			}

			// Validate balance: txState.Balance >= tx.MaxFeePerGas * tx.Gas + tx.Value
			if walletState.Balance != nil {
				totalCost := new(big.Int).Mul(txMaxFeePerGas, txGas)
				totalCost.Add(totalCost, txValue)
				if walletState.Balance.Cmp(totalCost) < 0 {
					log.Debug().Msgf("balance mismatch: %v < %v", walletState.Balance, totalCost)
					continue
				}
				// Update balance
				walletState.Balance.Sub(walletState.Balance, totalCost)
			}

			// Update state
			walletState.Nonce = new(big.Int).Add(walletState.Nonce, big.NewInt(1))
			gasUsedInBlock = newGasUsed

		} else {
			// Update state with new transaction
			state.WalletStateValidation[from] = WalletStateValidation{
				Nonce: new(big.Int).Add(txNonce, big.NewInt(1)),
			}
		}

		// Add valid transaction to the list
		validTransactions = append(validTransactions, parsedTransaction)
		validRawTransactions = append(validRawTransactions, rawTransactions[i])
	}
	return validRawTransactions, validTransactions
}

const (
	NonceError   = "NonceError"
	UnknownError = "UnknownError"
)

func ParseValidationError(validationState ValidationState, err error) (ValidationState, error) {
	switch getErrorType(err) {
	case NonceError:
		return handleNonceError(validationState, err.Error())
	case "insufficient funds for gas * price + value":
		validationState = handleInsufficientFundsError(validationState, err)
	case "transaction underpriced":
		validationState = handleUnderpricedError(validationState, err)
	default:
		return validationState, fmt.Errorf("unknown validation error: %w", err)
	}

	return validationState, nil
}

func getErrorType(err error) string {
	switch {
	case regexp.MustCompile(`nonce too (low|high)`).MatchString(err.Error()):
		return NonceError
	default:
		return UnknownError
	}
}

func handleNonceError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// error="err: nonce too high: address 0xBA401CdaC1A3b6AEeDe21c9C4a483be6C29F88C5, tx: 153 state: 89 (supplied gas 50000)"
	// Regular expression to match address and state value
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+), tx: \d+ state: (\d+)`)
	matches := re.FindStringSubmatch(errorMessage)

	// Validate matches
	if len(matches) != 3 {
		return validationState, fmt.Errorf("failed to parse nonce error: %s", errorMessage)
	}

	address := matches[1]
	currentNonce, ok := new(big.Int).SetString(matches[2], 10)
	if !ok {
		return validationState, fmt.Errorf("invalid nonce number: %s", currentNonce)
	}

	validationState.WalletStateValidation[address] = WalletStateValidation{
		Nonce: currentNonce,
	}

	return validationState, nil
}

func handleInsufficientFundsError(state ValidationState, err error) ValidationState {
	// TODO SEQ-316: Implement insufficient funds error handling
	return state
}

func handleUnderpricedError(state ValidationState, err error) ValidationState {
	// TODO SEQ-316: Implement underpriced error handling
	return state
}
