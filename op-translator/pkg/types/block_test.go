package types_test

import (
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
)

func TestGetBlockTimestamp_ParsingError(t *testing.T) {
	block := types.Block{
		"number":    "0x1",
		"hash":      "0xabc",
		"timestamp": 123, // Invalid type, should be string
	}

	timestamp, err := block.GetBlockTimestamp()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block timestamp")
	assert.Equal(t, timestamp, 0)
}

func TestGetBlockTimestampHex_ParsingError(t *testing.T) {
	block := types.Block{
		"number":    "0x1",
		"hash":      "0xabc",
		"timestamp": 123, // Invalid type, should be string
	}

	timestamp, err := block.GetBlockTimestampHex()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block timestamp")
	assert.Equal(t, timestamp, "")
}

func TestGetBlockNumber_ParsingError(t *testing.T) {
	block := types.Block{
		"number":       123, // Invalid type, should be string
		"hash":         "0xabc",
		"transactions": []any(nil),
	}

	number, err := block.GetBlockNumber()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block number")
	assert.Equal(t, number, uint64(0))
}

func TestGetBlockNumberHex_ParsingError(t *testing.T) {
	block := types.Block{
		"number":       123, // Invalid type, should be string
		"hash":         "0xabc",
		"transactions": []any(nil),
	}

	number, err := block.GetBlockNumberHex()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block number")
	assert.Equal(t, number, "")
}

func TestGetBlockHash_ParsingError(t *testing.T) {
	block := types.Block{
		"number":       "0x1",
		"hash":         123, // Invalid type, should be string
		"transactions": []any(nil),
	}

	hash, err := block.GetBlockHash()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block hash")
	assert.Equal(t, hash, "")
}

func TestAppendTransactions_ParsingError(t *testing.T) {
	block := types.Block{
		"number":       "0x1",
		"hash":         "0xabc",
		"transactions": "invalid", // Invalid type, should be []any
	}

	err := block.AppendTransaction(&ethtypes.Transaction{})

	assert.Error(t, err)
	assert.EqualError(t, err, "error appending txn to batch: parsing error: transactions")
}
