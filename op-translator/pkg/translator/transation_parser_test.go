package translator

import (
	"bytes"
	"compress/zlib"
	"encoding/binary"
	"fmt"
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
	assert.Equal(t, DummyTxn, result[0])
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
	fmt.Println("UNCOMPRESSED DATA", uncompressedData)
	zlibData, _ := compressZlib(uncompressedData)
	brotliData, _ := compressBrotli(uncompressedData)

	t.Run("Uncompressed Data", func(t *testing.T) {
		result, err := DecodeEventData(append([]byte{0x0}, []byte("mock_data")...))
		assert.NoError(t, err)
		expected := []hexutil.Bytes{hexutil.Bytes([]byte("mock_data"))}
		assert.Equal(t, expected, result)
	})
	t.Run("Zlib-Compressed Data", func(t *testing.T) {
		result, err := DecodeEventData(zlibData)
		assert.NoError(t, err)
		expected := []hexutil.Bytes{{0x12, 0x34}}
		assert.Equal(t, expected, result)
	})

	t.Run("Brotli-Compressed Data", func(t *testing.T) {
		result, err := DecodeEventData(brotliData)
		assert.NoError(t, err)
		expected := []hexutil.Bytes{{0x12, 0x34}}
		assert.Equal(t, expected, result)
	})

	t.Run("Truncated Data", func(t *testing.T) {
		data := []byte{0x10} // Too short
		_, err := DecodeEventData(data)
		assert.Error(t, err)
	})
	t.Run("Invalid Compression Type", func(t *testing.T) {
		data := []byte{0xF9, 0x01, 0x02, 0x03}
		_, err := DecodeEventData(data)
		assert.Error(t, err)
	})
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
		{
			name: "data truncated before expected transaction length",
			data: func() []byte {
				data := make([]byte, NumTransactionsBytes+LengthTransactionBytes+1)
				binary.BigEndian.PutUint32(data[NumTransactionsBytes:], 10) // Large length value
				return data
			}(),
			expectError:  true,
			errorMessage: "data truncated before expected transaction length",
		},
		{
			name: "transaction data length exceeds data boundary",
			data: func() []byte {
				data := make([]byte, NumTransactionsBytes+LengthTransactionBytes+4)
				binary.BigEndian.PutUint32(data[NumTransactionsBytes:], 5) // Exceeding boundary
				return data
			}(),
			expectError:  true,
			errorMessage: "transaction data length exceeds data boundary",
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
	return buf.Bytes(), nil // prepend compression type
}

func compressBrotli(data []byte) ([]byte, error) {
	var buf bytes.Buffer
	writer := brotli.NewWriter(&buf)
	_, err := writer.Write(data)
	writer.Close()
	if err != nil {
		return nil, err
	}
	return append([]byte{utils.VersionBrotli}, buf.Bytes()...), nil // prepend compression type
}
func createTestEventData(transactions [][]byte) []byte {
	var data []byte
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
	binary.BigEndian.PutUint32(numTransactionsBytes, uint32(numTransactions))
	buf = append(buf, numTransactionsBytes...)
	lengthBytes := make([]byte, LengthTransactionBytes)
	binary.BigEndian.PutUint32(lengthBytes, uint32(txLength))
	buf = append(buf, lengthBytes...)
	buf = append(buf, txData...)
	return buf
}
