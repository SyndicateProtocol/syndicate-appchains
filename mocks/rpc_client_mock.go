package mocks

import (
	"context"

	"github.com/SyndicateProtocol/op-translator/internal/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/mock"
)

type MockRPCClient struct {
	mock.Mock
}

func (m *MockRPCClient) CloseConnection() {
	m.Called()
}

func (m *MockRPCClient) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error) {
	args := m.Called(ctx, number, withTransactions)
	return args.Get(0).(types.Block), args.Error(1)
}

func (m *MockRPCClient) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error) {
	args := m.Called(ctx, hash, withTransactions)
	return args.Get(0).(types.Block), args.Error(1)
}

func (m *MockRPCClient) GetBlocksByNumbers(ctx context.Context, numbers []string, withTransactions bool) ([]types.Block, error) {
	args := m.Called(ctx, numbers, withTransactions)
	return args.Get(0).([]types.Block), args.Error(1)
}

func (m *MockRPCClient) GetReceiptsByHashes(ctx context.Context, hashes []common.Hash) ([]*ethtypes.Receipt, error) {
	args := m.Called(ctx, hashes)
	return args.Get(0).([]*ethtypes.Receipt), args.Error(1)
}

func (m *MockRPCClient) GetReceiptByHash(ctx context.Context, hash common.Hash) (ethtypes.Receipt, error) {
	args := m.Called(ctx, hash)
	return args.Get(0).(ethtypes.Receipt), args.Error(1)
}
