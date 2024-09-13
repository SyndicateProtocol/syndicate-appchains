package types

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getTestFrames() []*Frame {
	id, _ := NewChannelID()
	frame1 := NewFrame(id, 1, []byte("Hello World"), false)
	frame2 := NewFrame(id, 2, []byte("Hello World"), true)

	return []*Frame{frame1, frame2}
}

func TestGetBlockNumber_ParsingError(t *testing.T) {
	block := Block{
		"number":       123, // Invalid type, should be string
		"hash":         "0xabc",
		"transactions": []any(nil),
	}

	number, err := block.GetBlockNumber()

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: block number")
	assert.Equal(t, number, "")
}

func TestGetBlockHash_ParsingError(t *testing.T) {
	block := Block{
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
	block := Block{
		"number":       "0x1",
		"hash":         "0xabc",
		"transactions": "invalid", // Invalid type, should be []any
	}

	err := block.appendTransaction(any(nil))

	assert.Error(t, err)
	assert.EqualError(t, err, "parsing error: transactions")
}

func TestAppendFramesToBlock_Success(t *testing.T) {
	frames := getTestFrames()

	block := Block{
		"number":       "0x1",
		"hash":         "0xabc",
		"transactions": []any(nil),
	}

	from := "0x123"
	to := "0x456"

	err := block.AppendFrames(from, to, frames)

	assert.NoError(t, err)
	assert.Equal(t, "0x1", block["number"])
	assert.Equal(t, "0xabc", block["hash"])

	transactions, ok := block["transactions"].([]any)

	assert.True(t, ok)
	assert.Len(t, transactions, 1)
}

func TestAppendFramesToBlock_EmptyFrames(t *testing.T) {
	block := Block{
		"number":       "0x1",
		"hash":         "0xabc",
		"transactions": []any(nil),
	}

	from := "0x123"
	to := "0x456"
	frames := []*Frame{}

	err := block.AppendFrames(from, to, frames)

	assert.Error(t, err)
	assert.EqualError(t, err, "no frames to append")
}
