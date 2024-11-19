package mocks

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/mock"
)

type MockReceiptsFetcher struct {
	mock.Mock
}

func (m *MockReceiptsFetcher) FetchReceipts(ctx context.Context, block *types.Block, txHashes []common.Hash) (result ethtypes.Receipts, err error) {
	args := m.Called(ctx, block, txHashes)
	return args.Get(0).(ethtypes.Receipts), args.Error(1) //nolint:errcheck // mock safe cast
}
