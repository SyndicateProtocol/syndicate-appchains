package mocks

import (
	"context"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/mock"
)

type MockEthClient struct {
	mock.Mock
}

func (m *MockEthClient) BlockReceipts(ctx context.Context, blockHashOrNumber rpc.BlockNumberOrHash) ([]*ethtypes.Receipt, error) {
	args := m.Called(ctx, blockHashOrNumber)
	return args.Get(0).(ethtypes.Receipts), args.Error(1)
}

func (m *MockEthClient) BlockByNumber(ctx context.Context, number *big.Int) (*ethtypes.Block, error) {
	args := m.Called(ctx, number)
	return args.Get(0).(*ethtypes.Block), args.Error(1)
}

func (m *MockEthClient) HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error) {
	args := m.Called(ctx, number)
	return args.Get(0).(*ethtypes.Header), args.Error(1)
}

func (m *MockEthClient) TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error) {
	args := m.Called(ctx, hash)
	return args.Get(0).(*ethtypes.Receipt), args.Error(1)
}

func (m *MockEthClient) Close() {
	m.Called()
}
