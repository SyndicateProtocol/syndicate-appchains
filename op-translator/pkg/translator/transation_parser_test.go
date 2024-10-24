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

func TestDecodeTransactionData_NoCompression(t *testing.T) {
	parser := &L3TransactionParser{}
	data := append([]byte{utils.NoCompression}, []byte("mock_data")...)
	decoded, err := parser.DecodeTransactionData(data)

	require.NoError(t, err)
	assert.Equal(t, []byte("mock_data"), decoded)
}

func TestDecodeTransactionData_ZlibCM8(t *testing.T) {
	parser := &L3TransactionParser{}
	originalData := []byte("original_data")
	compressedData := compressZlib(originalData, zlib.BestCompression) // compress using zlib
	decoded, err := parser.DecodeTransactionData(compressedData)

	require.NoError(t, err)
	assert.Equal(t, originalData, decoded)
}

func TestDecodeTransactionData_ZlibCM15(t *testing.T) {
	parser := &L3TransactionParser{}
	originalData := []byte("original_data")
	compressedData := compressZlib(originalData, zlib.BestCompression) // compress using zlib
	decoded, err := parser.DecodeTransactionData(compressedData)

	require.NoError(t, err)
	assert.Equal(t, originalData, decoded)
}

func TestDecodeTransactionData_Brotli(t *testing.T) {
	parser := &L3TransactionParser{}
	compressionType := utils.VersionBrotli
	originalData := []byte("original_data")
	compressedData := compressBrotli(originalData)
	data := append([]byte{compressionType}, compressedData...)
	decoded, err := parser.DecodeTransactionData(data)

	require.NoError(t, err)
	assert.Equal(t, originalData, decoded)
}

func TestDecodeTransactionData_UnknownCompressionType(t *testing.T) {
	parser := &L3TransactionParser{}
	data := []byte("mock_data")
	decoded, err := parser.DecodeTransactionData(data)

	require.Error(t, err)
	assert.Nil(t, decoded)
	assert.Contains(t, err.Error(), "cannot distinguish the compression algo used given type byte")
}
