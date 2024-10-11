package mocks

import (
	"context"
	"math/big"

	"github.com/SyndicateProtocol/op-translator/internal/rpc-clients"
	"github.com/SyndicateProtocol/op-translator/internal/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/stretchr/testify/mock"
)

type MockRPCClient struct {
	mock.Mock
}

// guarantees that the IRPCClient interface is implemented by MockRPCClient
var _ rpc.IRPCClient = (*MockRPCClient)(nil)

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

func (m *MockRPCClient) BlocksReceiptsByNumbers(ctx context.Context, numbers []string) ([]*ethtypes.Receipt, error) {
	args := m.Called(ctx, numbers)
	return args.Get(0).([]*ethtypes.Receipt), args.Error(1)
}

func (m *MockRPCClient) HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error) {
	args := m.Called(ctx, number)
	return args.Get(0).(*ethtypes.Header), args.Error(1)
}

func (m *MockRPCClient) TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error) {
	args := m.Called(ctx, hash)
	return args.Get(0).(*ethtypes.Receipt), args.Error(1)
}

func (m *MockRPCClient) AsEthClient() *ethclient.Client {
	return nil
}
