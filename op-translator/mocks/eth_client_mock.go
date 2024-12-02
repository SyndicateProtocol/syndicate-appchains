package mocks

import (
	"context"
	"math/big"

	optranslator_rpc "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/mock"
)

type MockEthClient struct {
	mock.Mock
}

var _ optranslator_rpc.IETHClient = (*MockEthClient)(nil)

func (m *MockEthClient) BlockReceipts(ctx context.Context, blockHashOrNumber rpc.BlockNumberOrHash) ([]*ethtypes.Receipt, error) {
	args := m.Called(ctx, blockHashOrNumber)
	return Args0[ethtypes.Receipts](args), args.Error(1)
}

func (m *MockEthClient) BlockByNumber(ctx context.Context, number *big.Int) (*ethtypes.Block, error) {
	args := m.Called(ctx, number)
	return Args0[*ethtypes.Block](args), args.Error(1)
}

func (m *MockEthClient) HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error) {
	args := m.Called(ctx, number)
	return Args0[*ethtypes.Header](args), args.Error(1)
}

func (m *MockEthClient) TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error) {
	args := m.Called(ctx, hash)
	return Args0[*ethtypes.Receipt](args), args.Error(1)
}

func (m *MockEthClient) BlockNumber(ctx context.Context) (uint64, error) {
	args := m.Called(ctx)
	return Args0[uint64](args), args.Error(1)
}

func (m *MockEthClient) ChainID(ctx context.Context) (*big.Int, error) {
	args := m.Called(ctx)
	return Args0[*big.Int](args), args.Error(1)
}

func (m *MockEthClient) Close() {
	m.Called()
}
