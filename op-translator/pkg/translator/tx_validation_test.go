package translator_test

import (
	"errors"
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/params"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

var chainID = new(big.Int).SetInt64(1)

var txIntrinsicGasTooLow = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	tx.Gas = 0
	tx.ChainID = chainID
})
var rawTxIntrinsicGasTooLow = MustMarshalTransaction(txIntrinsicGasTooLow)

var txValid1 = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	to := common.HexToAddress("0x1234567890123456789012345678901234567001")
	tx.To = &to
	tx.Gas = 21000
	tx.ChainID = chainID
})
var rawTxValid1 = MustMarshalTransaction(txValid1)

// just to make sure we are respecting the ordering when filter returns
var txValid2 = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	to := common.HexToAddress("0x1234567890123456789012345678901234567002")
	tx.To = &to
	tx.Gas = 21000
	tx.ChainID = chainID
})
var rawTxValid2 = MustMarshalTransaction(txValid2)

func TestParseRawTransactions(t *testing.T) {
	type args struct {
		txs []hexutil.Bytes
	}
	tests := []struct { //nolint:govet // test struct
		name                          string
		args                          args
		wantFilteredTxsStateless      []hexutil.Bytes
		wantParsedFilteredTxStateless []*ethtypes.Transaction
	}{
		{"mix of valid and invalid, invalid at index 1",
			args{[]hexutil.Bytes{rawTxValid1, rawTxIntrinsicGasTooLow}},
			[]hexutil.Bytes{rawTxValid1},
			[]*ethtypes.Transaction{txValid1},
		},
		{"mix of valid and invalid, invalid at index 0",
			args{[]hexutil.Bytes{rawTxIntrinsicGasTooLow, rawTxValid1}},
			[]hexutil.Bytes{rawTxValid1},
			[]*ethtypes.Transaction{txValid1},
		},
		{"mix of valid and invalid, invalid at indexes 1, 2, 3",
			args{[]hexutil.Bytes{rawTxValid1, rawTxIntrinsicGasTooLow, rawTxIntrinsicGasTooLow, rawTxIntrinsicGasTooLow, rawTxValid2}},
			[]hexutil.Bytes{rawTxValid1, rawTxValid2},
			[]*ethtypes.Transaction{txValid1, txValid2},
		},
		{"unparseable",
			args{[]hexutil.Bytes{{0x00}}},
			[]hexutil.Bytes{},
			[]*ethtypes.Transaction{},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotFilteredTxsStateless, gotParsedFilteredTxStateless := translator.ParseRawTransactions(tt.args.txs)
			assert.Equalf(t, tt.wantFilteredTxsStateless, gotFilteredTxsStateless, "ParseRawTransactions(%v)", tt.args.txs)
			// parsed transactions are pointers, so we need to compare their contents one by one
			for i, parsedTx := range gotParsedFilteredTxStateless {
				assert.Equalf(t, tt.wantParsedFilteredTxStateless[i].Hash().String(), parsedTx.Hash, "ParseRawTransactions(%v)", tt.args.txs)
			}
		})
	}
}

func GenerateDummyTx(customFunc func(tx *ethtypes.DynamicFeeTx)) *ethtypes.Transaction {
	dynamicTx := &ethtypes.DynamicFeeTx{
		ChainID:   new(big.Int),
		Nonce:     0,
		Gas:       uint64(0),
		GasFeeCap: new(big.Int),
		GasTipCap: new(big.Int),
		To:        nil,
		Value:     new(big.Int),
		Data:      nil,
	}

	if customFunc != nil {
		customFunc(dynamicTx)
	}

	tx := ethtypes.NewTx(dynamicTx)

	// Generate a private key for the sender
	privateKey, _ := crypto.GenerateKey()
	// senderAddress := crypto.PubkeyToAddress(privateKey.PublicKey)

	// Sign the transaction
	signer := ethtypes.NewLondonSigner(dynamicTx.ChainID)
	signedTx, err := ethtypes.SignTx(tx, signer, privateKey)
	if err != nil {
		panic("failed to sign transaction")
	}

	return signedTx
}

func MustMarshalTransaction(tx *ethtypes.Transaction) hexutil.Bytes {
	bytes, err := tx.MarshalBinary()
	if err != nil {
		panic(err)
	}
	return bytes
}

func TestValidateTransactionInternal(t *testing.T) {
	// supported legacy transaction
	txLegacyType := ethtypes.NewTx(&ethtypes.LegacyTx{
		Gas: 53000,
	})

	// generate some malformed transactions
	txDepositType := ethtypes.NewTx(&ethtypes.DepositTx{})
	txAccessListType := ethtypes.NewTx(&ethtypes.AccessListTx{})
	txBlobType := ethtypes.NewTx(&ethtypes.BlobTx{})

	const limit = 90 * 1024              // 90KiB
	dataWithLimit := make([]byte, limit) // just an array filled with a bunch of zeroes
	txTooBig := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		to := common.HexToAddress("0x1234567890123456789012345678901234567")
		tx.To = &to
		tx.Gas = 21000
		tx.Data = dataWithLimit
		tx.ChainID = chainID
	})

	contractCreationWithLimit := make([]byte, params.MaxInitCodeSize+1)
	txContractCreationTooBig := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.Data = contractCreationWithLimit
		tx.ChainID = chainID
	})

	txNegativeValue := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.Value = new(big.Int).SetInt64(-1)
		tx.ChainID = chainID
	})

	bigIntTooLarge := new(big.Int).SetBytes([]byte{
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 64
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 128
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 192
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 256
		0xff, // this breaches the limit at 264 bits
	})

	txTooBigGasFeeCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasFeeCap = bigIntTooLarge
		tx.ChainID = chainID
	})

	txTooBigGasTipCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasTipCap = bigIntTooLarge
		tx.ChainID = chainID
	})

	txGasTipCapHigherThanGasFeeCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasTipCap = new(big.Int).SetInt64(10)
		tx.GasFeeCap = new(big.Int).SetInt64(9)
		tx.ChainID = chainID
	})

	type args struct {
		tx *ethtypes.Transaction
	}
	tests := []struct { //nolint:govet // test struct
		name      string
		args      args
		wantErr   bool
		errString string
	}{
		{"valid - tx type dynamic",
			args{txValid1},
			false,
			"",
		},
		{"valid - tx type legacy",
			args{txLegacyType},
			false,
			"",
		},
		{"invalid - non supported tx type deposit",
			args{txDepositType},
			true,
			"transaction type not supported: tx type 126 not supported by this pool",
		},
		{"invalid - non supported tx type access list",
			args{txAccessListType},
			true,
			"transaction type not supported: tx type 1 not supported by this pool",
		},
		{"invalid - non supported tx type blob",
			args{txBlobType},
			true,
			"transaction type not supported: tx type 3 not supported by this pool",
		},
		{"invalid - contract creation too big",
			args{txContractCreationTooBig},
			true,
			"max initcode size exceeded: code size 49153, limit 49152",
		},
		{"invalid - too big",
			args{txTooBig},
			true,
			"oversized data: transaction size 92266, limit 92160",
		},
		{"invalid - intrinsic gas too low",
			args{txIntrinsicGasTooLow},
			true,
			"intrinsic gas too low: gas 0, minimum needed 53000",
		},
		{"invalid - negative value",
			args{txNegativeValue},
			true,
			"negative value",
		},
		{"invalid - too big gas fee cap",
			args{txTooBigGasFeeCap},
			true,
			"max fee per gas higher than 2^256-1",
		},
		{"invalid - too big gas tip cap",
			args{txTooBigGasTipCap},
			true,
			"max priority fee per gas higher than 2^256-1",
		},
		{"invalid - gas tip cap higher than gas fee cap",
			args{txGasTipCapHigherThanGasFeeCap},
			true,
			"max priority fee per gas higher than max fee per gas",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := translator.ValidateTransactionInternal(tt.args.tx)
			if tt.wantErr && err == nil {
				t.Errorf("an error is expected but got nil.")
			}
			if !tt.wantErr && err != nil {
				t.Errorf("an error is not expected but got: %v", err)
			}
			if tt.wantErr && err != nil && err.Error() != tt.errString {
				t.Errorf("expected error: %v, but got %v", tt.errString, err)
			}
		})
	}
}

func TestParseValidationErrorAndUpdateState(t *testing.T) {
	validationState := translator.ValidationState{
		WalletStateValidation: make(map[string]translator.WalletStateValidation),
		BlockStateValidation:  translator.BlockStateValidation{},
	}

	tests := []struct { //nolint:govet // test struct
		name         string
		err          error
		expectedType string
		shouldError  bool
	}{
		{"Nonce too low", errors.New("nonce too low: address 0x123456, tx: 5 state: 3"), translator.NonceError, false},
		{"Nonce too high", errors.New("nonce too high: address 0x123456, tx: 10 state: 7"), translator.NonceError, false},
		{"Malformed nonce error", errors.New("nonce error without proper format"), translator.NonceError, true},
		{"Insufficient funds", errors.New("insufficient funds for gas: address 0x123456 have 5000"), translator.BalanceError, false},
		{"Malformed balance error", errors.New("balance error without address"), translator.BalanceError, true},
		{"Max fee less than base fee", errors.New("max fee per gas less than block base fee: address 0x123456, maxFeePerGas: 100, baseFee: 200"), translator.BaseFeeError, false},
		{"Malformed base fee error", errors.New("max fee error with missing values"), translator.BaseFeeError, true},
		{"Gas limit reached", errors.New("gas limit reached: 30000000 >= 15000000"), translator.BlockGasLimitError, false},
		{"Malformed gas limit error", errors.New("gas limit error with missing values"), translator.BlockGasLimitError, true},
		{"Unknown error", errors.New("random unexpected error"), translator.UnknownError, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := translator.ParseValidationErrorAndUpdateState(validationState, tt.err)
			if (err != nil) != tt.shouldError {
				t.Errorf("expected error: %v, got: %v", tt.shouldError, err)
			}
		})
	}
}

func TestHandleNonceError(t *testing.T) {
	validationState := translator.ValidationState{
		WalletStateValidation: make(map[string]translator.WalletStateValidation),
	}

	tests := []struct { //nolint:govet // test struct
		name          string
		errorMessage  string
		expectError   bool
		expectedNonce *big.Int
	}{
		{"Valid nonce error", "nonce too high: address 0x123456, tx: 5 state: 3", false, big.NewInt(3)},
		{"Malformed nonce error", "nonce too high: address missing fields", true, nil},
		{"Invalid nonce format", "nonce too high: address 0x123456, tx: 5 state: abc", true, nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleNonceError(validationState, tt.errorMessage)
			if tt.expectError {
				if err == nil {
					t.Errorf("expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("expected no error, got: %v", err)
				}

				address := "0x123456"
				if updatedState.WalletStateValidation[address].Nonce.Cmp(tt.expectedNonce) != 0 {
					t.Errorf("expected nonce: %v, got: %v", tt.expectedNonce, updatedState.WalletStateValidation[address].Nonce)
				}
			}
		})
	}
}

func TestHandleBalanceError(t *testing.T) {
	validationState := translator.ValidationState{
		WalletStateValidation: make(map[string]translator.WalletStateValidation),
	}

	tests := []struct { //nolint:govet // test struct
		name            string
		errorMessage    string
		expectError     bool
		expectedBalance *big.Int
	}{
		{"Valid balance error", "insufficient funds for gas: address 0x123456 have 5000", false, big.NewInt(5000)},
		{"Malformed balance error", "insufficient funds error missing fields", true, nil},
		{"Invalid balance format", "insufficient funds for gas: address 0x123456 have abc", true, nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleBalanceError(validationState, tt.errorMessage)
			if tt.expectError {
				if err == nil {
					t.Errorf("expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("expected no error, got: %v", err)
				}

				address := "0x123456"
				if updatedState.WalletStateValidation[address].Balance.Cmp(tt.expectedBalance) != 0 {
					t.Errorf("expected balance: %v, got: %v", tt.expectedBalance, updatedState.WalletStateValidation[address].Balance)
				}
			}
		})
	}
}

func TestHandleBaseFeeError(t *testing.T) {
	validationState := translator.ValidationState{
		BlockStateValidation: translator.BlockStateValidation{},
	}

	tests := []struct { //nolint:govet // test struct
		name            string
		errorMessage    string
		expectError     bool
		expectedBaseFee *big.Int
	}{
		{"Valid base fee error", "max fee per gas less than block base fee: address 0x123456, maxFeePerGas: 100, baseFee: 200", false, big.NewInt(200)},
		{"Malformed base fee error", "base fee error missing fields", true, nil},
		{"Invalid base fee format", "max fee per gas less than block base fee: address 0x123456, maxFeePerGas: 100, baseFee: abc", true, nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleBaseFeeError(validationState, tt.errorMessage)
			if tt.expectError {
				if err == nil {
					t.Errorf("expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("expected no error, got: %v", err)
				}

				if updatedState.BlockStateValidation.BaseFeePerGas.Cmp(tt.expectedBaseFee) != 0 {
					t.Errorf("expected base fee: %v, got: %v", tt.expectedBaseFee, updatedState.BlockStateValidation.BaseFeePerGas)
				}
			}
		})
	}
}

func TestHandleBlockGasLimitError(t *testing.T) {
	validationState := translator.ValidationState{
		BlockStateValidation: translator.BlockStateValidation{},
	}

	tests := []struct { //nolint:govet // test struct
		name             string
		errorMessage     string
		expectError      bool
		expectedGasLimit *big.Int
	}{
		{"Valid gas limit error", "gas limit reached: 30000000 >= 15000000", false, big.NewInt(15000000)},
		{"Malformed gas limit error", "gas limit error missing fields", true, nil},
		{"Invalid gas limit format", "gas limit reached: 30000000 >= abc", true, nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleBlockGasLimitError(validationState, tt.errorMessage)
			if tt.expectError {
				if err == nil {
					t.Errorf("expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("expected no error, got: %v", err)
				}

				if updatedState.BlockStateValidation.GasLimit.Cmp(tt.expectedGasLimit) != 0 {
					t.Errorf("expected gas limit: %v, got: %v", tt.expectedGasLimit, updatedState.BlockStateValidation.GasLimit)
				}
			}
		})
	}
}

func TestCloneValidationState(t *testing.T) {
	originalState := translator.ValidationState{
		WalletStateValidation: map[string]translator.WalletStateValidation{
			"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
				Nonce:   big.NewInt(1),
				Balance: big.NewInt(100000000000),
			},
		},
		BlockStateValidation: translator.BlockStateValidation{
			BaseFeePerGas: big.NewInt(10000000000),
			GasLimit:      big.NewInt(30000000),
		},
	}

	clonedState := originalState.Clone()

	// Modify the cloned state and ensure original is not affected
	walletState := clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"]
	walletState.Nonce = big.NewInt(2)
	clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"] = walletState

	assert.Equal(t, big.NewInt(1), originalState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"].Nonce)
	assert.Equal(t, big.NewInt(2), clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"].Nonce)
}

func TestValidateBlockState(t *testing.T) {
	rawTxns := []hexutil.Bytes{
		hexutil.MustDecode("0xf86a0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000001"),
		hexutil.MustDecode("0xf86b0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000002"),
	}
	txns := []*rpc.ParsedTransaction{
		{
			Nonce:        "0x1",
			Value:        "0x0",
			Gas:          "0x5208",
			MaxFeePerGas: "0xb84470600",
			From:         "0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef",
			To:           "0x868c2f4324ddddd07ebeb3605b5a0dc3bfc918a8",
		},
		{
			Nonce:        "0x2",
			Value:        "0x0",
			Gas:          "0x5208",
			MaxFeePerGas: "0xb84470600",
			From:         "0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef",
			To:           "0x868c2f4324ddddd07ebeb3605b5a0dc3bfc918a8",
		},
	}

	tests := []struct { //nolint:govet // test struct
		name          string
		state         translator.ValidationState
		expectedValid int
	}{
		{
			name: "All valid transactions",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   big.NewInt(1),
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      big.NewInt(30000000),
				},
			},
			expectedValid: 2,
		},
		{
			name: "Invalid nonce",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   big.NewInt(3),
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      big.NewInt(30000000),
				},
			},
			expectedValid: 0,
		},
		{
			name: "Insufficient balance",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   big.NewInt(1),
						Balance: big.NewInt(5000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      big.NewInt(30000000),
				},
			},
			expectedValid: 0,
		},
		{
			name: "Exceeds block gas limit",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   big.NewInt(1),
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      big.NewInt(10000),
				},
			},
			expectedValid: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			validRawTxns, validTxns := translator.ValidateBlockState(rawTxns, txns, tt.state)
			if len(validRawTxns) != tt.expectedValid || len(validTxns) != tt.expectedValid {
				t.Errorf("expected %d valid transactions, got %d valid raw txns and %d valid txns", tt.expectedValid, len(validRawTxns), len(validTxns))
			}
		})
	}
}

func TestValidateTransactionsBlock(t *testing.T) {
	mockChain := &mocks.MockRPCClient{}
	provider := translator.NewMetaBasedBatchProvider(mockChain, mockChain, common.Address{}, 0, 0, nil)

	validRawTxns := []hexutil.Bytes{
		hexutil.MustDecode("0xf86a0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000001"),
		hexutil.MustDecode("0xf86b0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000002"),
	}
	validParsedTxns := []*rpc.ParsedTransaction{
		{
			Nonce:        "0x1",
			Value:        "0x0",
			Gas:          "0x5208",
			MaxFeePerGas: "0xb84470600",
			From:         "0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef",
			To:           "0x868c2f4324ddddd07ebeb3605b5a0dc3bfc918a8",
		},
		{
			Nonce:        "0x2",
			Value:        "0x0",
			Gas:          "0x5208",
			MaxFeePerGas: "0xb84470600",
			From:         "0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef",
			To:           "0x868c2f4324ddddd07ebeb3605b5a0dc3bfc918a8",
		},
	}

	testCases := []struct { //nolint:govet // test struct
		name          string
		mockResponses []error
		expectedValid int
		expectedError bool
		inputTxns     []*rpc.ParsedTransaction
		inputRawTxns  []hexutil.Bytes
	}{
		{
			name:          "All valid transactions",
			mockResponses: []error{nil},
			expectedValid: len(validRawTxns),
			expectedError: false,
			inputTxns:     validParsedTxns,
			inputRawTxns:  validRawTxns,
		},
		{
			name:          "Empty input",
			mockResponses: []error{},
			expectedValid: 0,
			expectedError: false,
			inputTxns:     []*rpc.ParsedTransaction{},
			inputRawTxns:  []hexutil.Bytes{},
		},
		{
			name: "Multiple iterations before success",
			mockResponses: []error{
				errors.New("nonce too low: address 0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef, tx: 1 state: 0"),
				errors.New("nonce too low: address 0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef, tx: 1 state: 1"),
				nil,
			},
			expectedValid: 0,
			expectedError: false,
			inputTxns:     validParsedTxns,
			inputRawTxns:  validRawTxns,
		},
		{
			name: "Partial success after filtering",
			mockResponses: []error{
				errors.New("nonce too low: address 0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef, tx: 1 state: 2"),
				nil,
			},
			expectedValid: 1, // One transaction is valid after filtering
			expectedError: false,
			inputTxns:     validParsedTxns,
			inputRawTxns:  validRawTxns,
		},
		{
			name: "Unhandled validation error",
			mockResponses: []error{
				errors.New("unknown error"),
			},
			expectedValid: 0,
			expectedError: true,
			inputTxns:     validParsedTxns,
			inputRawTxns:  validRawTxns,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Set up mock responses for SimulateTransactions
			for _, resp := range tc.mockResponses {
				mockChain.On("SimulateTransactions", mock.Anything, mock.Anything, "latest").Return(nil, resp).Once()
			}

			// Call the function under test
			rawTxns, err := provider.ValidateTransactionsBlock(tc.inputRawTxns, tc.inputTxns)

			// Validate error expectations
			if tc.expectedError {
				if err == nil {
					t.Fatalf("expected an error but got none")
				}
			} else {
				if err != nil {
					t.Fatalf("expected no error, got: %v", err)
				}
			}

			// Validate valid transactions count
			if len(rawTxns) != tc.expectedValid {
				t.Fatalf("expected %d valid transactions, got: %d", tc.expectedValid, len(rawTxns))
			}

			// Assert mock expectations
			mockChain.AssertExpectations(t)
		})
	}
}
