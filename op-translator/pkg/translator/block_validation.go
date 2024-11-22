package translator

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"regexp"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/rs/zerolog/log"
)

const (
	NonceError         = "NonceError"         // nonce too low|high
	BalanceError       = "BalanceError"       // insufficient funds for gas * price + value
	BaseFeeError       = "BaseFeeError"       // max fee per gas less than block base fee
	BlockGasLimitError = "BlockGasLimitError" // gas limit reached
	UnknownError       = "UnknownError"
)

var (
	nonceErrorRegex         = regexp.MustCompile(`nonce too (low|high)`)
	balanceErrorRegex       = regexp.MustCompile(`insufficient funds for`)
	baseFeeErrorRegex       = regexp.MustCompile(`max fee per gas less than block base fee`)
	blockGasLimitErrorRegex = regexp.MustCompile(`gas limit reached`)
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

func (m *MetaBasedBatchProvider) ValidateBlock(rawTxns []hexutil.Bytes, txns []*rpc.ParsedTransaction) ([]hexutil.Bytes, error) {
	if len(rawTxns) != len(txns) {
		log.Error().Msgf("rawTxns and txns have different lengths: %v != %v", len(rawTxns), len(txns))
		return []hexutil.Bytes{}, fmt.Errorf("rawTxns and txns have different lengths")
	}
	if len(rawTxns) == 0 {
		log.Debug().Msg("No transactions provided")
		return []hexutil.Bytes{}, nil
	}
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
			log.Warn().Err(err).Msg("Error in SimulateTransactions")
			validationState, err = ParseValidationErrorAndUpdateState(validationState, err)
			if err != nil {
				log.Error().Err(err).Msg("Error in ParseValidationError")
				return []hexutil.Bytes{}, err
			}
			validRawTxns, validParsedTxns = ValidateBlockState(validRawTxns, validParsedTxns, validationState)
			continue
		}

		// Finalize the state after all transactions are validated
		log.Debug().Interface("validParsedTxns", validParsedTxns).Msg("ValidateBlock end")
		return validRawTxns, nil
	}
}

func ValidateBlockState(rawTransactions []hexutil.Bytes, parsedTransactions []*rpc.ParsedTransaction, state ValidationState) ([]hexutil.Bytes, []*rpc.ParsedTransaction) {
	validTransactions := []*rpc.ParsedTransaction{}
	validRawTransactions := []hexutil.Bytes{}

	tempState := state.Clone() // Create a temporary copy of the state
	gasUsedInBlock := big.NewInt(0)

	for i, tx := range parsedTransactions {
		if isValid, err := validateTransaction(tx, tempState, gasUsedInBlock); !isValid {
			if err != nil {
				log.Debug().Interface("parsedTransaction", tx).Err(err).Msg("Transaction validation failed")
			}
			continue
		}

		// Update gas usage and state after successful validation
		txGas, err := hexutil.DecodeBig(tx.Gas)
		if err != nil {
			log.Error().Err(err).Msg("can't convert gas to big int")
			continue
		}
		gasUsedInBlock.Add(gasUsedInBlock, txGas)
		updateStateForTransaction(tx, tempState)

		// Add the valid transaction to the results
		validTransactions = append(validTransactions, tx)
		validRawTransactions = append(validRawTransactions, rawTransactions[i])
	}
	return validRawTransactions, validTransactions
}

// validateTransaction checks if a transaction is valid according to the block and wallet state
func validateTransaction(tx *rpc.ParsedTransaction, state ValidationState, gasUsedInBlock *big.Int) (bool, error) {
	txNonce := hexutil.MustDecodeBig(tx.Nonce)
	txMaxFeePerGas := hexutil.MustDecodeBig(tx.MaxFeePerGas)
	txGas := hexutil.MustDecodeBig(tx.Gas)
	txValue := hexutil.MustDecodeBig(tx.Value)

	// Validate Base Fee
	if state.BlockStateValidation.BaseFeePerGas != nil && state.BlockStateValidation.BaseFeePerGas.Cmp(txMaxFeePerGas) > 0 {
		return false, fmt.Errorf("maxFeePerGas mismatch: %v < %v", txMaxFeePerGas, state.BlockStateValidation.BaseFeePerGas)
	}

	// Validate Gas Limit
	newGasUsed := new(big.Int).Add(gasUsedInBlock, txGas)
	if state.BlockStateValidation.GasLimit != nil && state.BlockStateValidation.GasLimit.Cmp(newGasUsed) < 0 {
		return false, fmt.Errorf("gas limit mismatch: %v < %v", newGasUsed, state.BlockStateValidation.GasLimit)
	}

	// Validate Wallet State
	walletState, exists := state.WalletStateValidation[tx.From]
	if exists {
		// Validate Nonce
		if walletState.Nonce != nil && txNonce.Cmp(walletState.Nonce) != 0 {
			return false, fmt.Errorf("nonce mismatch: %v != %v", txNonce, walletState.Nonce)
		}

		// Validate Balance
		totalCost := calculateTransactionCost(txMaxFeePerGas, txGas, txValue)
		if walletState.Balance != nil && walletState.Balance.Cmp(totalCost) < 0 {
			return false, fmt.Errorf("balance mismatch: %v < %v", walletState.Balance, totalCost)
		}
	}

	return true, nil
}

func updateStateForTransaction(tx *rpc.ParsedTransaction, state ValidationState) {
	txNonce := hexutil.MustDecodeBig(tx.Nonce)
	txMaxFeePerGas := hexutil.MustDecodeBig(tx.MaxFeePerGas)
	txGas := hexutil.MustDecodeBig(tx.Gas)
	txValue := hexutil.MustDecodeBig(tx.Value)

	from := tx.From
	state.WalletStateValidation[from] = WalletStateValidation{
		Nonce: new(big.Int).Add(txNonce, big.NewInt(1)),
	}
	walletState := state.WalletStateValidation[from]

	// Update Balance
	if walletState.Balance != nil {
		totalCost := calculateTransactionCost(txMaxFeePerGas, txGas, txValue)
		walletState.Balance.Sub(walletState.Balance, totalCost)
	}
}

func calculateTransactionCost(maxFeePerGas, gas, value *big.Int) *big.Int {
	totalCost := new(big.Int).Mul(maxFeePerGas, gas)
	return totalCost.Add(totalCost, value)
}

func (state ValidationState) Clone() ValidationState {
	walletValidations := make(map[string]WalletStateValidation, len(state.WalletStateValidation))

	blockValidation := BlockStateValidation{
		BaseFeePerGas: utils.CloneBigIntPtr(state.BlockStateValidation.BaseFeePerGas),
		GasLimit:      utils.CloneBigIntPtr(state.BlockStateValidation.GasLimit),
	}

	for addr, walletState := range state.WalletStateValidation {
		walletValidations[addr] = WalletStateValidation{
			Nonce:   utils.CloneBigIntPtr(walletState.Nonce),
			Balance: utils.CloneBigIntPtr(walletState.Balance),
		}
	}

	return ValidationState{
		WalletStateValidation: walletValidations,
		BlockStateValidation:  blockValidation,
	}
}

func ParseValidationErrorAndUpdateState(validationState ValidationState, err error) (ValidationState, error) {
	switch GetErrorType(err) {
	case NonceError:
		return HandleNonceError(validationState, err.Error())
	case BalanceError:
		return HandleBalanceError(validationState, err.Error())
	case BaseFeeError:
		return HandleBaseFeeError(validationState, err.Error())
	case BlockGasLimitError:
		return HandleBlockGasLimitError(validationState, err.Error())
	case UnknownError:
		return validationState, errors.New("unknown error to parse")
	}
	return validationState, nil
}

func GetErrorType(err error) string {
	switch {
	case nonceErrorRegex.MatchString(err.Error()):
		return NonceError
	case balanceErrorRegex.MatchString(err.Error()):
		return BalanceError
	case baseFeeErrorRegex.MatchString(err.Error()):
		return BaseFeeError
	case blockGasLimitErrorRegex.MatchString(err.Error()):
		return BlockGasLimitError
	default:
		return UnknownError
	}
}

func HandleNonceError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: nonce too high: address 0xBA401CdaC1A3b6AEeDe21c9C4a483be6C29F88C5, tx: 153 state: 89 (supplied gas 50000)
	// Regular expression to match address and state value
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+), tx: \d+ state: (\d+)`)
	expectedMatches := 3
	matches := re.FindStringSubmatch(errorMessage)

	// Validate matches
	if len(matches) != expectedMatches {
		return validationState, fmt.Errorf("failed to parse nonce error: %s", errorMessage)
	}

	address := matches[1]
	currentNonce, ok := new(big.Int).SetString(matches[2], decimalBase)
	if !ok {
		return validationState, fmt.Errorf("invalid nonce number: %s", currentNonce)
	}

	validationState.WalletStateValidation[address] = WalletStateValidation{
		Nonce: currentNonce,
	}

	return validationState, nil
}

func HandleBalanceError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: insufficient funds for gas * price + value: address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 have 16940 want 1001771318794850000 (supplied gas 50000)
	// Regular expression to extract address, current balance, and required balance
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+) have (\d+)`)
	matches := re.FindStringSubmatch(errorMessage)
	expectedMatches := 3
	// Validate matches
	if len(matches) != expectedMatches {
		return validationState, fmt.Errorf("failed to parse balance error: %s", errorMessage)
	}

	address := matches[1]
	currentBalance, ok := new(big.Int).SetString(matches[2], decimalBase)
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

func HandleBaseFeeError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: max fee per gas less than block base fee: address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266, maxFeePerGas: 10000000000, baseFee: 19810336964 (supplied gas 50000)
	// Regular expression to extract address, maxFeePerGas, and baseFee
	re := regexp.MustCompile(`address (0x[a-fA-F0-9]+), maxFeePerGas: (\d+), baseFee: (\d+)`)
	matches := re.FindStringSubmatch(errorMessage)
	expectedMatches := 4

	// Validate matches
	if len(matches) != expectedMatches {
		return validationState, fmt.Errorf("failed to parse fee error: %s", errorMessage)
	}

	baseFee, ok := new(big.Int).SetString(matches[3], decimalBase)
	if !ok {
		return validationState, fmt.Errorf("invalid baseFee value: %s", matches[3])
	}

	// Update the block base fee in the validation state
	validationState.BlockStateValidation.BaseFeePerGas = baseFee

	return validationState, nil
}

func HandleBlockGasLimitError(validationState ValidationState, errorMessage string) (ValidationState, error) {
	// err: block gas limit reached: 826486073456 >= 30000000
	// Regular expression to extract the current gas used and the block gas limit
	re := regexp.MustCompile(`gas limit reached: (\d+) >= (\d+)`)
	expectedMatches := 3
	matches := re.FindStringSubmatch(errorMessage)

	// Validate matches
	if len(matches) != expectedMatches {
		return validationState, fmt.Errorf("failed to parse block gas limit error: %s", errorMessage)
	}

	blockGasLimit, ok := new(big.Int).SetString(matches[2], decimalBase)
	if !ok {
		return validationState, fmt.Errorf("invalid block gas limit value: %s", matches[2])
	}

	// Update the block gas limit in the validation state
	validationState.BlockStateValidation.GasLimit = blockGasLimit

	return validationState, nil
}
