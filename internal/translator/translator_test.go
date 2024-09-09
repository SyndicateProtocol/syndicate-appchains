package translator

import (
	"context"
	"testing"

	"github.com/SyndicateProtocol/op-translator/mocks"
	"github.com/stretchr/testify/assert"
)

func TestInit(t *testing.T) {
	cfg := mocks.ConfigMock
	translator := Init(cfg)
	assert.NotNil(t, translator)
}

func TestGetBlockByNumber(t *testing.T) {
	translator := &OpTranslator{}

	ctx := context.Background()
	blockNumber := "0x1"
	fullTx := false

	result, err := translator.GetBlockByNumber(ctx, blockNumber, fullTx)

	assert.NoError(t, err)
	assert.Nil(t, result)
}

func TestGetBlockByHash(t *testing.T) {
	translator := &OpTranslator{}

	ctx := context.Background()
	blockHash := "0xabc123"
	fullTx := false

	result, err := translator.GetBlockByHash(ctx, blockHash, fullTx)

	assert.NoError(t, err)
	assert.Nil(t, result)
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
