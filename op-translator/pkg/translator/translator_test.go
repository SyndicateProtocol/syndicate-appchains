package translator_test

import (
	"context"
	"log/slog"
	"net/http"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum-optimism/optimism/op-service/testlog"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

func TestGetBlockByNumber(t *testing.T) {
	mockConfig := mocks.DefaultTestingConfig
	mockClient := new(mocks.MockRPCClient)
	mockMetrics := mocks.NewMockMetrics()

	number := "0x21"
	settlementBlock := types.Block{
		"number":       number,
		"hash":         "0xabc",
		"transactions": []any{},
	}
	ctx := context.Background()

	mockClient.On("GetBlockByNumber", ctx, number, true).Return(settlementBlock, nil)
	testlogger := testlog.Logger(t, slog.LevelDebug)
	translatorMock := translator.NewOPTranslator(
		mockClient,
		&mocks.MockBatchProvider{},
		backfill.NewBackfillerProvider(mockConfig.MetafillerURL, mockConfig.SettlementStartBlock, mockConfig.CutoverEpochBlock, &http.Client{}, mockMetrics, testlogger),
		mocks.TestSigner(t),
		&common.Address{},
		mockMetrics,
		testlogger,
	)

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

	mockMetrics.AssertCalled(t, "RecordOPTranslatorRPCRequest", "eth_getBlockByNumber")

	mockClient.AssertExpectations(t)
}

func TestGetBlockByHash(t *testing.T) {
	mockClient := new(mocks.MockRPCClient)
	mockMetrics := mocks.NewMockMetrics()
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
	testlogger := testlog.Logger(t, slog.LevelDebug)
	translatorMock := translator.NewOPTranslator(
		mockClient,
		&mocks.MockBatchProvider{},
		backfill.NewBackfillerProvider(mockConfig.MetafillerURL, mockConfig.SettlementStartBlock, mockConfig.CutoverEpochBlock, &http.Client{}, mockMetrics, testlogger),
		mocks.TestSigner(t),
		&common.Address{},
		mockMetrics,
		testlogger,
	)

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

	mockMetrics.AssertCalled(t, "RecordOPTranslatorRPCRequest", "eth_getBlockByHash")

	mockClient.AssertExpectations(t)
}
