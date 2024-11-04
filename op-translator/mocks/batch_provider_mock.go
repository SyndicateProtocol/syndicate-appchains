package mocks

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
)

type MockBatchProvider struct{}

func (m *MockBatchProvider) Close() {}

func (m *MockBatchProvider) GetBatch(ctx context.Context, block types.Block) (*types.Batch, error) {
	batch := &types.Batch{
		ParentHash:      common.Hash{},
		EpochNumber:     0,
		EpochHash:       common.Hash{},
		Timestamp:       0,
		TransactionList: []hexutil.Bytes{},
	}

	return batch, nil
}

func (m *MockBatchProvider) FilterInvalidTransactions(ctx context.Context, batch *types.Batch) (*types.Batch, error) {
	return batch, nil
}
