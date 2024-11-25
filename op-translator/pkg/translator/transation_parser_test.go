package translator

import (
	"bytes"
	"compress/zlib"
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"log/slog"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/andybalholm/brotli"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	DummyEncodedData = common.Hex2Bytes("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020002000000000000000000000000000000000000000000000000000000000000")
	DummyTxn         = common.Hex2Bytes("0002") // Compression byte (00) + transaction data (02)
)

func TestIsLogTransactionProcessed(t *testing.T) {
	parser := MustNewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"), testlog.Logger(t, slog.LevelDebug))

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

func TestGetEventTransactions(t *testing.T) {
	parser := MustNewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"), testlog.Logger(t, slog.LevelDebug))

	senderAddr := common.HexToAddress("0xabcdef0123456789abcdef0123456789abcdef01")

	log := &types.Log{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Topics: []common.Hash{
			TransactionProcessedSigHash,
			common.BytesToHash(senderAddr.Bytes()),
		},
		Data: DummyEncodedData,
	}

	result, err := parser.GetEventTransactions(log)

	require.NoError(t, err)
	assert.Equal(t, hexutil.Bytes{DummyTxn[1]}, result[0])
}

func TestGetEventTransactions_Error(t *testing.T) {
	parser := MustNewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"), testlog.Logger(t, slog.LevelDebug))

	log := &types.Log{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Topics: []common.Hash{
			TransactionProcessedSigHash,
			{},
		},
		Data: []byte{},
	}

	result, err := parser.GetEventTransactions(log)

	assert.Error(t, err)
	assert.Nil(t, result)
}

func TestDecodeEventData(t *testing.T) {
	uncompressedData := generateTransactionData()
	zlibData, _ := compressZlib(uncompressedData)
	brotliData, _ := compressBrotli(uncompressedData)

	// TODO (SEQ-250): Use shared input folder
	// Real data from the metabased-sequencer
	// Link: https://github.com/SyndicateProtocol/metabased-rollup/pull/38/files#diff-899764f1a0141b8b7d697a7267fca84a7ad749984a49050d44be069bbfe6130bR338
	realZlibData, _ := base64.StdEncoding.DecodeString("eJylzTsIAXEAgPE/CYVEkpUykEdJTOQVJQlJrpBBSnIpNoWcFIOFTJLyuEGZPAflMRgky6UbrFK3WhRXjCaL3/71AQCoAACY+kgiFKeWgXAtJoRrM5Z8zKbbl61A9hYaLdz1HXE2Xu2RKUFxXex6VJubfIBf1VLH7rLMnPmNWKk+cJFCGLV6hmysq5yz0PZT1A8nzZHgGG/mZvCLF9jS8xBGhBKnw+dYPk79BPiyWNL2BvUE32RkNe+2MEpLFBrwS6Cq+6uHVpw3gaRL/g==")
	sampleTx1, _ := hex.DecodeString("02f86b83014a3407830f4240830f443e825208944e527486594696a7607ff3379e21746689a3fd6d1480c080a0502ec1e72aa5d8e52f2547c3dcb973d6129364828ea54cfd166ea74350a60cd4a02db70ba79cfb18a45d6b415e58aed8947bb66efc1156c2067e59d4ea5c69cfcb")
	sampleTx2, _ := hex.DecodeString("cdb554ea000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c320000000000000000000000000000000000000000000000000000000000568936000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c32000000000000000000000000000000000000000000000000000000000059bd0d")

	tests := []struct {
		name     string
		input    []byte
		expected []hexutil.Bytes
		hasError bool
	}{
		{
			name:     "Uncompressed Data",
			input:    append([]byte{0x0}, []byte("mock_data")...),
			expected: []hexutil.Bytes{hexutil.Bytes([]byte("mock_data"))},
			hasError: false,
		},
		{
			name:     "Zlib-Compressed Data",
			input:    zlibData,
			expected: []hexutil.Bytes{{0x12, 0x34}},
			hasError: false,
		},
		{
			name:     "Brotli-Compressed Data",
			input:    brotliData,
			expected: []hexutil.Bytes{{0x12, 0x34}},
			hasError: false,
		},
		{
			name:     "Truncated Data",
			input:    []byte{0x10},
			expected: nil,
			hasError: true,
		},
		{
			name:     "Invalid Compression Type",
			input:    []byte{0xF9, 0x01, 0x02, 0x03},
			expected: nil,
			hasError: true,
		},
		{
			name:     "Real Zlib compressed data with 2 transactions",
			input:    realZlibData,
			expected: []hexutil.Bytes{sampleTx1, sampleTx2},
			hasError: false,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			result, err := DecodeEventData(tc.input)
			if tc.hasError {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tc.expected, result)
			}
		})
	}
}

func TestParseEventData(t *testing.T) {
	tests := []struct {
		name           string
		errorMessage   string
		data           []byte
		expectedResult []hexutil.Bytes
		expectError    bool
	}{
		{
			name:           "valid data with one transaction",
			data:           createTestEventData([][]byte{[]byte("abcd")}),
			expectedResult: []hexutil.Bytes{hexutil.Bytes([]byte("abcd"))},
			expectError:    false,
		},
		{
			name: "valid data with multiple transactions",
			data: createTestEventData([][]byte{[]byte("abcd"), []byte("1234")}),
			expectedResult: []hexutil.Bytes{
				hexutil.Bytes([]byte("abcd")),
				hexutil.Bytes([]byte("1234")),
			},
			expectError: false,
		},
		{
			name:         "insufficient data length",
			data:         make([]byte, NumTransactionsBytes+LengthTransactionBytes-1),
			expectError:  true,
			errorMessage: "insufficient data length to contain transaction details",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ParseEventData(tt.data)

			if tt.expectError {
				assert.Error(t, err)
				assert.Nil(t, result)
				assert.Contains(t, err.Error(), tt.errorMessage)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expectedResult, result)
			}
		})
	}
}

// Helper functions for testing

func compressZlib(data []byte) ([]byte, error) {
	var buf bytes.Buffer
	writer := zlib.NewWriter(&buf)
	_, err := writer.Write(data)
	writer.Close()
	if err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func compressBrotli(data []byte) ([]byte, error) {
	var buf bytes.Buffer
	writer := brotli.NewWriter(&buf)
	_, err := writer.Write(data)
	writer.Close()
	if err != nil {
		return nil, err
	}
	return append([]byte{utils.VersionBrotli}, buf.Bytes()...), nil
}

func createTestEventData(transactions [][]byte) []byte {
	data := make([]byte, NumTransactionsBytes)
	binary.BigEndian.PutUint32(data[:NumTransactionsBytes], uint32(len(transactions))) //nolint:all //just used for testing
	for _, tx := range transactions {
		length := make([]byte, LengthTransactionBytes)
		binary.BigEndian.PutUint32(length, uint32(len(tx))) //nolint:all //just used for testing
		data = append(data, length...)
		data = append(data, tx...)
	}

	return data
}

func generateTransactionData() []byte {
	numTransactions := 1
	txLength := 2
	txData := []byte{0x12, 0x34}

	buf := make([]byte, 0, NumTransactionsBytes+LengthTransactionBytes+txLength)
	numTransactionsBytes := make([]byte, LengthTransactionBytes)
	binary.BigEndian.PutUint32(numTransactionsBytes, uint32(numTransactions)) //nolint:all //just used for testing
	buf = append(buf, numTransactionsBytes...)
	lengthBytes := make([]byte, LengthTransactionBytes)
	binary.BigEndian.PutUint32(lengthBytes, uint32(txLength)) //nolint:all //just used for testing
	buf = append(buf, lengthBytes...)
	buf = append(buf, txData...)
	return buf
}
