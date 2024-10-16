package mocks

import (
	"context"

	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/mock"
)

// MockRPCClient is a mock of *rpc.Client for testing
type MockRawRPCClient struct {
	mock.Mock
}

// Mocking CallContext method of *rpc.Client
func (m *MockRawRPCClient) CallContext(ctx context.Context, result interface{}, method string, args ...interface{}) error {
	arguments := m.Called(ctx, result, method, args)
	return arguments.Error(0)
}

// Mock other methods like BatchCallContext as needed
func (m *MockRawRPCClient) BatchCallContext(ctx context.Context, b []rpc.BatchElem) error {
	arguments := m.Called(ctx, b)
	return arguments.Error(0)
}
