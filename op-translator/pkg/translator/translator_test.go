package translator_test

import (
	"context"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestGetBlockByNumber(t *testing.T) {
	mockConfig := mocks.DefaultTestingConfig
	mockClient := new(mocks.MockRPCClient)
	mockMetrics := new(mocks.MockMetrics)
	mockMetrics.On("RecordRPCRequest", "eth_getBlockByNumber").Return()
	mockMetrics.On("RecordTranslationLatency", "eth_getBlockByNumber", mock.Anything).Return()

	number := "0x21"
	settlementBlock := types.Block{
		"number":       number,
		"hash":         "0xabc",
		"transactions": []any{},
	}
	ctx := context.Background()

	mockClient.On("GetBlockByNumber", ctx, number, true).Return(settlementBlock, nil)
	translatorMock := &translator.OPTranslator{
		SettlementChain:  mockClient,
		BatchProvider:    &mocks.MockBatchProvider{},
		Signer:           *translator.NewSigner(mockConfig),
		BackfillProvider: backfill.NewBackfillerProvider(mockConfig),
		Metrics:          mockMetrics,
	}

	block, err := translatorMock.GetBlockByNumber(ctx, number, true)

	assert.NoError(t, err)
	blockNumber, err := block.GetBlockNumberHex()
	assert.NoError(t, err)
	assert.Equal(t, "0x21", blockNumber)

	blockHash, err := block.GetBlockHash()
	assert.NoError(t, err)
	assert.Equal(t, "0xabc", blockHash)

	transactions, err := block.GetTransactions()
	assert.NoError(t, err)
	assert.Equal(t, 1, len(transactions))

	mockMetrics.AssertCalled(t, "RecordRPCRequest", "eth_getBlockByNumber")

	mockClient.AssertExpectations(t)
}

func TestGetBlockByHash(t *testing.T) {
	mockClient := new(mocks.MockRPCClient)
	mockMetrics := new(mocks.MockMetrics)
	mockMetrics.On("RecordRPCRequest", "eth_getBlockByHash").Return()
	mockMetrics.On("RecordTranslationLatency", "eth_getBlockByHash", mock.Anything).Return()
	mockConfig := mocks.DefaultTestingConfig

	number := "0x21"
	settlementBlock := types.Block{
		"number":       number,
		"hash":         "0xabc",
		"transactions": []any{},
	}
	ctx := context.Background()
	hash := common.HexToHash("0xabc")

	mockClient.On("GetBlockByHash", ctx, hash, true).Return(settlementBlock, nil)
	translatorMock := &translator.OPTranslator{
		SettlementChain:  mockClient,
		BatchProvider:    &mocks.MockBatchProvider{},
		Signer:           *translator.NewSigner(mockConfig),
		BackfillProvider: backfill.NewBackfillerProvider(mockConfig),
		Metrics:          mockMetrics,
	}

	block, err := translatorMock.GetBlockByHash(ctx, hash, true)

	assert.NoError(t, err)
	blockNumber, err := block.GetBlockNumberHex()
	assert.NoError(t, err)
	assert.Equal(t, "0x21", blockNumber)

	blockHash, err := block.GetBlockHash()
	assert.NoError(t, err)
	assert.Equal(t, "0xabc", blockHash)

	transactions, err := block.GetTransactions()
	assert.NoError(t, err)
	assert.Equal(t, 1, len(transactions))

	mockMetrics.AssertCalled(t, "RecordRPCRequest", "eth_getBlockByHash")

	mockClient.AssertExpectations(t)
}

func TestShouldTranslate(t *testing.T) {
	tests := []struct {
		method   string
		expected bool
	}{
		{"eth_getBlockByNumber", true},
		{"eth_getBlockByHash", true},
		{"eth_getTransactionByHash", false},
		{"eth_blockNumber", false},
	}

	for _, tt := range tests {
		t.Run(tt.method, func(t *testing.T) {
			result := translator.ShouldTranslate(tt.method)
			assert.Equal(t, tt.expected, result)
		})
	}
}
