package mocks

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/stretchr/testify/mock"
)

type MockBatchProvider struct {
	mock.Mock
}

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

func (m *MockBatchProvider) ValidateBlock(rawTxns []hexutil.Bytes, txns []*rpc.ParsedTransaction) ([]hexutil.Bytes, error) {
	args := m.Called(rawTxns, txns)
	return Args0[[]hexutil.Bytes](args), args.Error(1)
}
