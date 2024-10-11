package translator

import (
	"context"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/types"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

func TestInit(t *testing.T) {
	mockConfig := mocks.MockConfig{}
	mockConfig.On("SettlementChainAddr").Return("http://localhost:8545")
	mockConfig.On("SequencingChainAddr").Return("http://localhost:8545")
	mockConfig.On("MetaBasedChainAddr").Return("http://localhost:8545")
	mockConfig.On("SequencingContractAddress").Return("0x0000000000000000000000000000000000000000")
	mockConfig.On("BatcherAddress").Return("0x0000000000000000000000000000000000000000")
	mockConfig.On("BatchInboxAddress").Return("0x0000000000000000000000000000000000000000")
	mockConfig.On("SettlementStartBlock").Return("1")
	mockConfig.On("SequencingStartBlock").Return("2")
	mockConfig.On("SequencePerSettlementBlock").Return("2")
	mockConfig.On("BatcherPrivateKey").Return("fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d")
	mockConfig.On("SettlementChainID").Return(84532)
	translator := Init(&mockConfig)
	assert.NotNil(t, translator)
}

func TestGetBlockByNumber(t *testing.T) {
	mockConfig := mocks.InitMockConfig()
	mockClient := new(mocks.MockRPCClient)
	number := "0x1"
	settlementBlock := types.Block{
		"number":       number,
		"hash":         "0xabc",
		"transactions": []any{},
	}
	ctx := context.Background()

	mockClient.On("GetBlockByNumber", ctx, number, true).Return(settlementBlock, nil)
	translator := &OPTranslator{
		SettlementChain: mockClient,
		BatchProvider:   &mocks.MockBatchProvider{},
		Signer:          *NewSigner(mockConfig),
	}

	block, err := translator.GetBlockByNumber(ctx, number, true)

	assert.NoError(t, err)
	blockNumber, err := block.GetBlockNumber()
	assert.NoError(t, err)
	assert.Equal(t, "0x1", blockNumber)

	blockHash, err := block.GetBlockHash()
	assert.NoError(t, err)
	assert.Equal(t, "0xabc", blockHash)

	transactions, err := block.GetTransactions()
	assert.NoError(t, err)
	assert.Equal(t, 1, len(transactions))

	mockClient.AssertExpectations(t)
}

func TestGetBlockByHash(t *testing.T) {
	mockClient := new(mocks.MockRPCClient)
	number := "0x1"
	settlementBlock := types.Block{
		"number":       number,
		"hash":         "0xabc",
		"transactions": []any{},
	}
	ctx := context.Background()
	hash := common.HexToHash("0xabc")

	mockClient.On("GetBlockByHash", ctx, hash, true).Return(settlementBlock, nil)
	mockConfig := mocks.InitMockConfig()
	translator := &OPTranslator{
		SettlementChain: mockClient,
		BatchProvider:   &mocks.MockBatchProvider{},
		Signer:          *NewSigner(mockConfig),
	}

	block, err := translator.GetBlockByHash(ctx, hash, true)

	assert.NoError(t, err)
	blockNumber, err := block.GetBlockNumber()
	assert.NoError(t, err)
	assert.Equal(t, "0x1", blockNumber)

	blockHash, err := block.GetBlockHash()
	assert.NoError(t, err)
	assert.Equal(t, "0xabc", blockHash)

	transactions, err := block.GetTransactions()
	assert.NoError(t, err)
	assert.Equal(t, 1, len(transactions))

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
			result := ShouldTranslate(tt.method)
			assert.Equal(t, tt.expected, result)
		})
	}
}
