package translator_test

import (
	"errors"
	"log/slog"
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestParseValidationErrorAndUpdateState(t *testing.T) {
	validationState := translator.ValidationState{
		WalletStateValidation: make(map[string]translator.WalletStateValidation),
		BlockStateValidation:  translator.BlockStateValidation{},
	}

	tests := []struct {
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

	tests := []struct {
		name          string
		errorMessage  string
		expectError   bool
		expectedNonce uint64
	}{
		{"Valid nonce error", "nonce too high: address 0x123456, tx: 5 state: 3", false, 3},
		{"Malformed nonce error", "nonce too high: address missing fields", true, 0},
		{"Invalid nonce format", "nonce too high: address 0x123456, tx: 5 state: abc", true, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleNonceError(validationState, tt.errorMessage)
			if tt.expectError {
				assert.NotNil(t, err)
			} else {
				assert.Nil(t, err)
				address := "0x123456"
				assert.Equal(t, tt.expectedNonce, updatedState.WalletStateValidation[address].Nonce)
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

	tests := []struct {
		name             string
		errorMessage     string
		expectError      bool
		expectedGasLimit uint64
	}{
		{"Valid gas limit error", "gas limit reached: 30000000 >= 15000000", false, 15000000},
		{"Malformed gas limit error", "gas limit error missing fields", true, 0},
		{"Invalid gas limit format", "gas limit reached: 30000000 >= abc", true, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			updatedState, err := translator.HandleBlockGasLimitError(validationState, tt.errorMessage)
			if tt.expectError {
				assert.NotNil(t, err)
			} else {
				assert.Nil(t, err)
				assert.Equal(t, tt.expectedGasLimit, updatedState.BlockStateValidation.GasLimit)
			}
		})
	}
}

func TestCloneValidationState(t *testing.T) {
	originalState := translator.ValidationState{
		WalletStateValidation: map[string]translator.WalletStateValidation{
			"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
				Nonce:   3,
				Balance: big.NewInt(100000000000),
			},
		},
		BlockStateValidation: translator.BlockStateValidation{
			BaseFeePerGas: big.NewInt(10000000000),
			GasLimit:      30000000,
		},
	}

	clonedState := originalState.Clone()

	// Modify the cloned state and ensure original is not affected
	walletState := clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"]
	walletState.Nonce = 4
	clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"] = walletState

	assert.Equal(t, uint64(3), originalState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"].Nonce)
	assert.Equal(t, uint64(4), clonedState.WalletStateValidation["0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef"].Nonce)
}

func TestValidateBlockState(t *testing.T) {
	rawTxns := []hexutil.Bytes{
		hexutil.MustDecode("0xf86a0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000001"),
		hexutil.MustDecode("0xf86b0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000002"),
	}

	nonce1 := hexutil.MustDecodeUint64("0x1")
	gas1 := hexutil.MustDecodeUint64("0x5208")
	maxFeePerGas1 := hexutil.MustDecodeBig("0xb84470600")
	from1 := common.HexToAddress("0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef")
	to1 := common.HexToAddress("0x868c2f4324ddddf07ebeb3605b5a0dc3bfc918a8")

	nonce2 := hexutil.MustDecodeUint64("0x2")
	gas2 := hexutil.MustDecodeUint64("0x5208")
	maxFeePerGas2 := hexutil.MustDecodeBig("0xb84470600")
	from2 := common.HexToAddress("0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef")
	to2 := common.HexToAddress("0x868c2f4324ddddf07ebeb3605b5a0dc3bfc918a8")

	txns := []*rpc.ParsedTransaction{
		{
			Nonce:        (*hexutil.Uint64)(&nonce1),
			Value:        (*hexutil.Big)(big.NewInt(0)),
			Gas:          (*hexutil.Uint64)(&gas1),
			MaxFeePerGas: (*hexutil.Big)(maxFeePerGas1),
			From:         &from1,
			To:           &to1,
		},
		{
			Nonce:        (*hexutil.Uint64)(&nonce2),
			Value:        (*hexutil.Big)(big.NewInt(0)),
			Gas:          (*hexutil.Uint64)(&gas2),
			MaxFeePerGas: (*hexutil.Big)(maxFeePerGas2),
			From:         &from2,
			To:           &to2,
		},
	}

	tests := []struct {
		name          string
		state         translator.ValidationState
		expectedValid int
	}{
		{
			name: "All valid transactions",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   1,
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      30000000,
				},
			},
			expectedValid: 2,
		},
		{
			name: "Invalid nonce",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   3,
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      30000000,
				},
			},
			expectedValid: 0,
		},
		{
			name: "Exceeds block gas limit",
			state: translator.ValidationState{
				WalletStateValidation: map[string]translator.WalletStateValidation{
					"0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef": {
						Nonce:   1,
						Balance: big.NewInt(2000000000000000),
					},
				},
				BlockStateValidation: translator.BlockStateValidation{
					BaseFeePerGas: big.NewInt(10000000000),
					GasLimit:      10000,
				},
			},
			expectedValid: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			validRawTxns, validTxns := translator.ValidateBlockState(rawTxns, txns, tt.state, testlog.Logger(t, slog.LevelDebug))
			assert.Equal(t, tt.expectedValid, len(validRawTxns))
			assert.Equal(t, tt.expectedValid, len(validTxns))
		})
	}
}

func TestValidateBlock(t *testing.T) {
	mockChain := &mocks.MockRPCClient{}
	provider := translator.NewMetaBasedBatchProvider(mockChain, mockChain, common.Address{}, 0, 0, nil, testlog.Logger(t, slog.LevelDebug))

	validRawTxns := []hexutil.Bytes{
		hexutil.MustDecode("0xf86a0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000001"),
		hexutil.MustDecode("0xf86b0180850b8447060082520894868c2f4324ddddf07ebeb3605b5a0dc3bfc918a80b844a9059cbb00000000000000000000000033e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef00000000000000000000000000000000000000000000000000000000000000002"),
	}
	validParsedTxns := []*rpc.ParsedTransaction{
		{
			Nonce:        (*hexutil.Uint64)(new(uint64)),
			Value:        (*hexutil.Big)(big.NewInt(0)),
			Gas:          (*hexutil.Uint64)(new(uint64)),
			MaxFeePerGas: (*hexutil.Big)(big.NewInt(50000000000)),
			From: func() *common.Address {
				addr := common.HexToAddress("0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef")
				return &addr
			}(),
			To: func() *common.Address {
				addr := common.HexToAddress("0x868c2f4324ddddf07ebeb3605b5a0dc3bfc918a8")
				return &addr
			}(),
		},
		{
			Nonce:        (*hexutil.Uint64)(new(uint64)),
			Value:        (*hexutil.Big)(big.NewInt(0)),
			Gas:          (*hexutil.Uint64)(new(uint64)),
			MaxFeePerGas: (*hexutil.Big)(big.NewInt(50000000000)),
			From: func() *common.Address {
				addr := common.HexToAddress("0x33e244b5c8b54cd1f0e7b2a7b2e75e2204acb2ef")
				return &addr
			}(),
			To: func() *common.Address {
				addr := common.HexToAddress("0x868c2f4324ddddf07ebeb3605b5a0dc3bfc918a8")
				return &addr
			}(),
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
			rawTxns, err := provider.ValidateBlock(tc.inputRawTxns, tc.inputTxns)

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
