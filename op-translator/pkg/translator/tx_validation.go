package translator

import (
	"context"
	"errors"
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
	// Create empty state
	validationState := ValidationState{
		WalletStateValidation: make(map[string]WalletStateValidation),
		BlockStateValidation:  BlockStateValidation{},
	}

	validParsedTxns := txns
	validRawTxns := rawTxns

	for {
		// Call SimulateTransactions to either pass or fail and get the state
		_, err := m.MetaBasedChain.SimulateTransactions(context.Background(), validParsedTxns, "latest")
		// If simulation error, parse error and update state
		if err != nil {
			validationState, err = ParseValidationError(validationState, err)
			if err != nil {
				log.Error().Err(err).Msg("Error in ParseValidationError")
				return []hexutil.Bytes{}, err
			}
			validRawTxns, validParsedTxns = ValidateBlockState(validRawTxns, validParsedTxns, validationState)
			continue
		}

		// Finalize the state after all transactions are validated
		log.Debug().Interface("validParsedTxns", validParsedTxns).Msg("ValidateTransactionsBlock end")
		return validRawTxns, nil
	}
}

func ValidateBlockState(rawTransactions []hexutil.Bytes, parsedTransactions []*rpc.ParsedTransaction, state ValidationState) ([]hexutil.Bytes, []*rpc.ParsedTransaction) {
	validTransactions := []*rpc.ParsedTransaction{}
	validRawTransactions := []hexutil.Bytes{}
	tempState := cloneValidationState(state) // Create a temporary copy of the state

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
		if blockBaseFeePerGas != nil && blockBaseFeePerGas.Cmp(txMaxFeePerGas) > 0 {
			log.Debug().Msgf("maxFeePerGas mismatch: %v < %v", txMaxFeePerGas, blockBaseFeePerGas)
			continue
		}
		newGasUsed := new(big.Int).Add(gasUsedInBlock, txGas)
		if blockGasLimit != nil && blockGasLimit.Cmp(newGasUsed) < 0 {
			log.Debug().Msgf("gas limit mismatch: %v < %v", newGasUsed, blockGasLimit)
			continue
		}

		from := parsedTransaction.From
		walletState, exists := tempState.WalletStateValidation[from]
		if exists {
			// Validate nonce: tx.Nonce == tempState.WalletStateValidation[from].Nonce
			if txNonce.Cmp(walletState.Nonce) != 0 {
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
			tempState.WalletStateValidation[from] = WalletStateValidation{
				Nonce: new(big.Int).Add(txNonce, big.NewInt(1)),
			}
		}

		// Add valid transaction to the list
		validTransactions = append(validTransactions, parsedTransaction)
		validRawTransactions = append(validRawTransactions, rawTransactions[i])
	}
	return validRawTransactions, validTransactions
}

func cloneValidationState(state ValidationState) ValidationState {
	clonedState := ValidationState{
		WalletStateValidation: make(map[string]WalletStateValidation),
		BlockStateValidation: BlockStateValidation{
			BaseFeePerGas: nil,
			GasLimit:      nil,
		},
	}

	// Clone BlockStateValidation
	if state.BlockStateValidation.BaseFeePerGas != nil {
		clonedState.BlockStateValidation.BaseFeePerGas = new(big.Int).Set(state.BlockStateValidation.BaseFeePerGas)
	}
	if state.BlockStateValidation.GasLimit != nil {
		clonedState.BlockStateValidation.GasLimit = new(big.Int).Set(state.BlockStateValidation.GasLimit)
	}

	// Clone WalletStateValidation
	for addr, walletState := range state.WalletStateValidation {
		clonedWalletState := WalletStateValidation{
			Nonce:   nil,
			Balance: nil,
		}
		if walletState.Nonce != nil {
			clonedWalletState.Nonce = new(big.Int).Set(walletState.Nonce)
		}
		if walletState.Balance != nil {
			clonedWalletState.Balance = new(big.Int).Set(walletState.Balance)
		}
		clonedState.WalletStateValidation[addr] = clonedWalletState
	}

	return clonedState
}

const (
	NonceError   = "NonceError"
	BalanceError = "BalanceError"
	BaseFeeError = "BaseFeeError"
	UnknownError = "UnknownError"
)

func ParseValidationError(validationState ValidationState, err error) (ValidationState, error) {
	switch getErrorType(err) {
	case NonceError:
		return handleNonceError(validationState, err.Error())
	case BalanceError:
		return handleBalanceError(validationState, err.Error())
	case BaseFeeError:
		return handleBaseFeeError(validationState, err.Error())
	case UnknownError:
		return validationState, errors.New("unknown error to parse")
	}
	return validationState, nil
}

func getErrorType(err error) string {
	switch {
	case regexp.MustCompile(`nonce too (low|high)`).MatchString(err.Error()):
		return NonceError
	case regexp.MustCompile(`insufficient funds for`).MatchString(err.Error()):
		return BalanceError
	case regexp.MustCompile(`max fee per gas less than block base fee`).MatchString(err.Error()):
		return BaseFeeError
	default:
		return UnknownError
	}
}

func handleNonceError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: nonce too high: address 0xBA401CdaC1A3b6AEeDe21c9C4a483be6C29F88C5, tx: 153 state: 89 (supplied gas 50000)
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

func handleBalanceError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: insufficient funds for gas * price + value: address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 have 16940 want 1001771318794850000 (supplied gas 50000)
	// Regular expression to extract address, current balance, and required balance
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+) have (\d+)`)
	matches := re.FindStringSubmatch(errorMessage)

	// Validate matches
	if len(matches) != 3 {
		return validationState, fmt.Errorf("failed to parse balance error: %s", errorMessage)
	}

	address := matches[1]
	currentBalance, ok := new(big.Int).SetString(matches[2], 10)
	if !ok {
		return validationState, fmt.Errorf("invalid current balance value: %s", matches[2])
	}

	// Update the balance in the validation state
	walletState, exists := validationState.WalletStateValidation[address]
	if !exists {
		walletState = WalletStateValidation{}
	}

	// Set the wallet's balance to the current balance
	walletState.Balance = currentBalance
	validationState.WalletStateValidation[address] = walletState

	return validationState, nil
}

func handleBaseFeeError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: max fee per gas less than block base fee: address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266, maxFeePerGas: 10000000000, baseFee: 19810336964 (supplied gas 50000)
	// Regular expression to extract address, maxFeePerGas, and baseFee
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+), maxFeePerGas: (\d+), baseFee: (\d+)`)
	matches := re.FindStringSubmatch(errorMessage)

	// Validate matches
	if len(matches) != 4 {
		return validationState, fmt.Errorf("failed to parse fee error: %s", errorMessage)
	}

	baseFee, ok := new(big.Int).SetString(matches[3], 10)
	if !ok {
		return validationState, fmt.Errorf("invalid baseFee value: %s", matches[3])
	}

	// Update the block base fee in the validation state
	validationState.BlockStateValidation.BaseFeePerGas = baseFee

	return validationState, nil
}
