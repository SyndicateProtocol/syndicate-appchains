package translator

import (
	"bytes"
	"compress/zlib"
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/andybalholm/brotli"
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

func TestGetEventTransactions(t *testing.T) {
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

	result, err := parser.GetEventTransactions(log)

	require.NoError(t, err)
	assert.Equal(t, hexutil.Bytes{DummyTxn[1]}, result[0])
}

func TestGetEventTransactions_Error(t *testing.T) {
	parser := NewL3TransactionParser(common.HexToAddress("0x1234567890123456789012345678901234567890"))

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
			// expected: []hexutil.Bytes{{0x2, 0xf8, 0x6b, 0x83, 0x1, 0x4a, 0x34, 0x7, 0x83, 0xf, 0x42, 0x40, 0x83, 0xf, 0x44, 0x3e, 0x82, 0x52, 0x8, 0x94, 0x4e, 0x52, 0x74, 0x86, 0x59, 0x46, 0x96, 0xa7, 0x60, 0x7f, 0xf3, 0x37, 0x9e, 0x21, 0x74, 0x66, 0x89, 0xa3, 0xfd, 0x6d, 0x14, 0x80, 0xc0, 0x80, 0xa0, 0x50, 0x2e, 0xc1, 0xe7, 0x2a, 0xa5, 0xd8, 0xe5, 0x2f, 0x25, 0x47, 0xc3, 0xdc, 0xb9, 0x73, 0xd6, 0x12, 0x93, 0x64, 0x82, 0x8e, 0xa5, 0x4c, 0xfd, 0x16, 0x6e, 0xa7, 0x43, 0x50, 0xa6, 0xc, 0xd4, 0xa0, 0x2d, 0xb7, 0xb, 0xa7, 0x9c, 0xfb, 0x18, 0xa4, 0x5d, 0x6b, 0x41, 0x5e, 0x58, 0xae, 0xd8, 0x94, 0x7b, 0xb6, 0x6e, 0xfc, 0x11, 0x56, 0xc2, 0x6, 0x7e, 0x59, 0xd4, 0xea, 0x5c, 0x69, 0xcf, 0xcb}, {0xcd, 0xb5, 0x54, 0xea, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb8, 0xb9, 0x4, 0xc7, 0x3d, 0x2f, 0xb4, 0xd8, 0xc1, 0x73, 0x29, 0x8a, 0x51, 0xc2, 0x7f, 0xab, 0x70, 0x22, 0x2c, 0x32, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x56, 0x89, 0x36, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb8, 0xb9, 0x4, 0xc7, 0x3d, 0x2f, 0xb4, 0xd8, 0xc1, 0x73, 0x29, 0x8a, 0x51, 0xc2, 0x7f, 0xab, 0x70, 0x22, 0x2c, 0x32, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x59, 0xbd, 0xd}},
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
	tests := []struct { //nolint:govet //just used for testing
		expectError    bool
		name           string
		errorMessage   string
		data           []byte
		expectedResult []hexutil.Bytes
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
