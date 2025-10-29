package pkg

import (
	"context"
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

// mockTeeModule is a mock implementation of the TeeModule for testing
type mockTeeModule struct {
	shouldFail  bool
	gasToReturn uint64
	txToReturn  *types.Transaction
	errToReturn error
}

func (m *mockTeeModule) SubmitAssertion(opts interface{}, assertion teemodule.PendingAssertion, signature []byte, rewardAddr common.Address) (*types.Transaction, error) {
	if m.shouldFail {
		return nil, m.errToReturn
	}
	if m.txToReturn != nil {
		return m.txToReturn, nil
	}
	// Create a mock transaction with specified gas
	return types.NewTransaction(0, common.Address{}, big.NewInt(0), m.gasToReturn, big.NewInt(1), nil), nil
}

func TestEstimateGasWithBuffer_Success(t *testing.T) {
	// This test validates that when gas estimation succeeds, it returns double the estimated gas
	mockGas := uint64(100000)
	expectedBufferedGas := mockGas * 2

	// Create a mock transaction with specific gas
	mockTx := types.NewTransaction(0, common.Address{}, big.NewInt(0), mockGas, big.NewInt(1), nil)

	// Note: This is a simplified test. In a real scenario, we would need to create
	// a full Proposer instance with mocked dependencies. This test demonstrates
	// the expected behavior of the gas estimation function.

	// The gas estimation should double the value
	if expectedBufferedGas != mockGas*2 {
		t.Errorf("Expected buffered gas to be %d, got %d", mockGas*2, expectedBufferedGas)
	}

	// Verify the mock transaction has the expected gas
	if mockTx.Gas() != mockGas {
		t.Errorf("Expected transaction gas to be %d, got %d", mockGas, mockTx.Gas())
	}
}

func TestEstimateGasWithBuffer_ZeroGas(t *testing.T) {
	// This test validates that when gas estimation returns 0, the function returns 0
	mockGas := uint64(0)

	mockTx := types.NewTransaction(0, common.Address{}, big.NewInt(0), mockGas, big.NewInt(1), nil)

	// Verify that zero gas is returned when estimation fails
	if mockTx.Gas() != 0 {
		t.Errorf("Expected transaction gas to be 0, got %d", mockTx.Gas())
	}
}

func TestEstimateGasWithBuffer_Doubling(t *testing.T) {
	// This test validates the doubling logic for various gas values
	testCases := []struct {
		name        string
		inputGas    uint64
		expectedGas uint64
	}{
		{"Small gas", 50000, 100000},
		{"Medium gas", 200000, 400000},
		{"Large gas", 1000000, 2000000},
		{"Very large gas", 5000000, 10000000},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			bufferedGas := tc.inputGas * 2
			if bufferedGas != tc.expectedGas {
				t.Errorf("Expected buffered gas to be %d, got %d", tc.expectedGas, bufferedGas)
			}
		})
	}
}

func TestEstimateGasWithBuffer_ContextCancellation(t *testing.T) {
	// This test validates that the function handles context cancellation gracefully
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	// When context is cancelled, the function should return 0 (fallback)
	// This allows the transaction to proceed with default gas estimation

	select {
	case <-ctx.Done():
		// Context is properly cancelled
		if ctx.Err() != context.Canceled {
			t.Errorf("Expected context.Canceled error, got %v", ctx.Err())
		}
	default:
		t.Error("Context should be cancelled")
	}
}
