package translator

import (
	"bytes"
	"compress/zlib"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/andybalholm/brotli"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	DummyEncodedData = common.Hex2Bytes("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020002000000000000000000000000000000000000000000000000000000000000")
	DummyTxn         = common.Hex2Bytes("0002") // Compression byte (00) + transaction data (02)
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

func compressZlib(data []byte, level int) []byte {
	var buf bytes.Buffer
	writer, _ := zlib.NewWriterLevel(&buf, level)
	writer.Write(data) //nolint:errcheck //just used for testing
	writer.Close()
	return buf.Bytes()
}

func compressBrotli(data []byte) []byte {
	var buf bytes.Buffer
	writer := brotli.NewWriter(&buf)
	writer.Write(data) //nolint:errcheck //just used for testing
	writer.Close()
	return buf.Bytes()
}

func TestDecodeTransactionData(t *testing.T) {
	tests := []struct {
		name           string
		expectedError  string
		input          []byte
		expectedOutput []byte
	}{
		{
			name:           "No Compression",
			input:          append([]byte{utils.NoCompression}, []byte("mock_data")...),
			expectedOutput: []byte("mock_data"),
		},
		{
			name:           "ZlibCM8",
			input:          compressZlib([]byte("original_data"), zlib.BestCompression),
			expectedOutput: []byte("original_data"),
		},
		{
			name:           "ZlibCM15",
			input:          compressZlib([]byte("original_data"), zlib.BestCompression),
			expectedOutput: []byte("original_data"),
		},
		{
			name:           "Brotli",
			input:          append([]byte{utils.VersionBrotli}, compressBrotli([]byte("original_data"))...),
			expectedOutput: []byte("original_data"),
		},
		{
			name:          "Unknown Compression Type",
			input:         []byte("mock_data"),
			expectedError: "cannot distinguish the compression algo used given type byte",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := &L3TransactionParser{}
			decoded, err := parser.DecodeTransactionData(tt.input)

			if tt.expectedError != "" {
				require.Error(t, err)
				assert.Contains(t, err.Error(), tt.expectedError)
				assert.Nil(t, decoded)
			} else {
				require.NoError(t, err)
				assert.Equal(t, tt.expectedOutput, decoded)
			}
		})
	}
}
