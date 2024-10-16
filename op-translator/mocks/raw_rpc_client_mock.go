package mocks

import (
	"context"

	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/mock"
)

type MockRawRPCClient struct {
	mock.Mock
}

func (m *MockRawRPCClient) CallContext(ctx context.Context, result any, method string, args ...any) error {
	arguments := m.Called(ctx, result, method, args)
	return arguments.Error(0)
}

func (m *MockRawRPCClient) BatchCallContext(ctx context.Context, b []rpc.BatchElem) error {
	arguments := m.Called(ctx, b)
	return arguments.Error(0)
}
