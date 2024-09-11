package translator

import (
	"context"
	"testing"

	"github.com/SyndicateProtocol/op-translator/mocks"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

func TestInit(t *testing.T) {
	cfg := mocks.ConfigMock
	translator := Init(cfg)
	assert.NotNil(t, translator)
}

func TestGetBlockByNumber(t *testing.T) {
	mockClient := new(mocks.MockRPCClient)
	expectedBlock := &sources.RPCBlock{}
	ctx := context.Background()
	var number = "0xE730A8"
	mockClient.On("GetBlockByNumber", ctx, number, true).Return(expectedBlock, nil)

	translator := &OPTranslator{
		settlementChain: mockClient,
		sequencingChain: mockClient,
	}

	block, err := translator.GetBlockByNumber(ctx, number, true)

	assert.NoError(t, err)
	assert.Equal(t, expectedBlock, block)
	mockClient.AssertCalled(t, "GetBlockByNumber", ctx, number, true)
}

func TestGetBlockByHash(t *testing.T) {
	mockClient := new(mocks.MockRPCClient)
	expectedBlock := &sources.RPCBlock{}
	ctx := context.Background()
	var hash common.Hash
	copy(hash[:], "0xabc")
	mockClient.On("GetBlockByHash", ctx, hash, true).Return(expectedBlock, nil)

	translator := &OPTranslator{
		settlementChain: mockClient,
		sequencingChain: mockClient,
	}

	block, err := translator.GetBlockByHash(ctx, hash, true)

	assert.NoError(t, err)
	assert.Equal(t, expectedBlock, block)

	mockClient.AssertCalled(t, "GetBlockByHash", ctx, hash, true)
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
