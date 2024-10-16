package mocks

import (
	"context"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/mock"
)

type MockEthClient struct {
	mock.Mock
}

func (m *MockEthClient) BlockReceipts(ctx context.Context, blockHashOrNumber rpc.BlockNumberOrHash) (types.Receipts, error) {
	args := m.Called(ctx, blockHashOrNumber)
	return args.Get(0).(types.Receipts), args.Error(1)
}
