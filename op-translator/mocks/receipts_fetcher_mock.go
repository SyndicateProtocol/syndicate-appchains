package mocks

import (
	"context"

	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/mock"
)

type MockReceiptsFetcher struct {
	mock.Mock
}

func (m *MockReceiptsFetcher) FetchReceipts(ctx context.Context, blockInfo eth.BlockInfo, txHashes []common.Hash) (result ethtypes.Receipts, err error) {
	args := m.Called(ctx, blockInfo, txHashes)
	return args.Get(0).(ethtypes.Receipts), args.Error(1)
}
