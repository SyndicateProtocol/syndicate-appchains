package translator

import (
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	DummyEncodedData = common.Hex2Bytes("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010200000000000000000000000000000000000000000000000000000000000000")
	DummyTxn         = common.Hex2Bytes("000002")
)

func TestIsLogTransactionProcessed(t *testing.T) {
	parser := NewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"))

	testCases := []struct {
		log      *types.Log
		name     string
		expected bool
	}{
		{
			name: "Valid TransactionProcessed log",
			log: &types.Log{
				Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
				Topics:  []common.Hash{TransactionProcessedSigHash},
			},
			expected: true,
		},
		{
			name: "Invalid address",
			log: &types.Log{
				Address: common.HexToAddress("0x0000000000000000000000000000000000000000"),
				Topics:  []common.Hash{TransactionProcessedSigHash},
			},
			expected: false,
		},
		{
			name: "Invalid topic",
			log: &types.Log{
				Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
				Topics:  []common.Hash{{}},
			},
			expected: false,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			result := parser.IsLogTransactionProcessed(tc.log)
			assert.Equal(t, tc.expected, result)
		})
	}
}

func TestParseTransactionProcessed(t *testing.T) {
	parser := NewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"))

	senderAddr := common.HexToAddress("0xabcdef0123456789abcdef0123456789abcdef01")

	log := &types.Log{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Topics: []common.Hash{
			TransactionProcessedSigHash,
			common.BytesToHash(senderAddr.Bytes()),
		},
		Data: DummyEncodedData,
	}

	result, err := parser.ParseTransactionProcessed(log)

	require.NoError(t, err)
	assert.Equal(t, senderAddr, result.Sender)
	assert.Equal(t, DummyTxn, result.EncodedData)
}

func TestParseTransactionProcessed_Error(t *testing.T) {
	parser := NewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"))

	log := &types.Log{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Topics: []common.Hash{
			TransactionProcessedSigHash,
			{},
		},
		Data: []byte{},
	}

	result, err := parser.ParseTransactionProcessed(log)

	assert.Error(t, err)
	assert.Nil(t, result)
}
