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
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

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

func TestValidateBlock(t *testing.T) {
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
