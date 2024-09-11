package mocks

import (
	"context"

	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/mock"
)

type MockRPCClient struct {
	mock.Mock
}

func (m *MockRPCClient) CloseConnection() {
	m.Called()
}

func (m *MockRPCClient) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (any, error) {
	args := m.Called(ctx, number, withTransactions)
	return args.Get(0).(*sources.RPCBlock), args.Error(1)
}

func (m *MockRPCClient) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (any, error) {
	args := m.Called(ctx, hash, withTransactions)
	return args.Get(0).(*sources.RPCBlock), args.Error(1)
}
