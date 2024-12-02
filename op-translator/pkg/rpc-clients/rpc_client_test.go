package rpc_test

import (
	"context"
	"errors"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	ethtypes "github.com/ethereum/go-ethereum/core/types"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestConnect(t *testing.T) {
	mockMetrics := mocks.NewMockMetrics()
	tests := []struct {
		name    string
		address string
		wantErr bool
	}{
		{"Valid address", "http://localhost:8545", false},
		{"Invalid address", "invalid", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client, err := rpc.Connect(tt.address, mockMetrics)

			if tt.wantErr {
				assert.Error(t, err, "expected an error but got none")
				assert.Nil(t, client, "client should be nil when an error occurs")
				return
			}

			assert.NoError(t, err, "expected no error but got one")
			assert.NotNil(t, client, "client should not be nil when no error occurs")
			require.NotNil(t, client.AsEthClient())
		})
	}
}

func TestCloseConnection(t *testing.T) {
	mockMetrics := mocks.NewMockMetrics()
	client, err := rpc.Connect("http://localhost:8545", mockMetrics)
	require.NoError(t, err)
	require.NotNil(t, client)

	client.CloseConnection()

	// Since Close() doesn't return an error in the original function,
	// we can't use assert.NoError here. Instead, we can just ensure the
	// method completes without panicking and include further checks.
	assert.NotNil(t, client.AsEthClient(), "Client should be non-nil before closing")

	var hash common.Hash
	copy(hash[:], "0xabc")

	_, err = client.GetBlockByHash(context.Background(), hash, true)
	assert.Error(t, err, "expected an error after closing the connection")
}

func TestGetReceiptsByBlocks(t *testing.T) {
	tests := []struct { //nolint:govet // test struct
		setupMocks  func(*mocks.MockReceiptsFetcher, *mocks.MockEthClient, *mocks.MockRawRPCClient)
		expected    []*ethtypes.Receipt
		blocks      []*types.Block
		name        string
		expectError bool
	}{
		{
			name:   "Successful receipt fetching",
			blocks: []*types.Block{{"number": "0xA", "transactions": []any{"0x123"}}},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txHash := common.HexToHash("0x123")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)
			},
			expected:    []*ethtypes.Receipt{{Status: 1}},
			expectError: false,
		},
		{
			name:        "Error getting block number",
			blocks:      []*types.Block{{"transactions": []any{"0x123"}}},
			setupMocks:  nil,
			expected:    nil,
			expectError: true,
		},
		{
			name:        "Error getting block transactions",
			blocks:      []*types.Block{{"number": "0xA"}},
			setupMocks:  nil,
			expected:    nil,
			expectError: true,
		},
		{
			name:   "Error in FetchReceipts",
			blocks: []*types.Block{{"number": "0xA", "transactions": []any{"0x123"}}},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txHash := common.HexToHash("0x123")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txHash}).
					Return(ethtypes.Receipts{}, errors.New("receipts fetching error"))
			},
			expected:    nil,
			expectError: true,
		},
		{
			name:        "No blocks provided",
			blocks:      []*types.Block{},
			setupMocks:  nil,
			expected:    []*ethtypes.Receipt(nil),
			expectError: false,
		},
		{
			name:   "Successful receipt fetching for multiple blocks",
			blocks: []*types.Block{{"number": "0xA", "transactions": []any{"0x123"}}, {"number": "0xB", "transactions": []any{"0x456"}}},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txAHash := common.HexToHash("0x123")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txAHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)

				txBHash := common.HexToHash("0x456")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txBHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)
			},
			expected:    []*ethtypes.Receipt{{Status: 1}, {Status: 1}},
			expectError: false,
		},
		{
			name:   "Partial success - one receipt fetch fails",
			blocks: []*types.Block{{"number": "0xA", "transactions": []any{"0x123"}}, {"number": "0xB", "transactions": []any{"0x456"}}},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txAHash := common.HexToHash("0x123")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txAHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)

				txBHash := common.HexToHash("0x456")
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txBHash}).
					Return(ethtypes.Receipts{}, errors.New("receipts fetching error"))
			},
			expected:    nil,
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockFetcher := new(mocks.MockReceiptsFetcher)
			mockEthClient := new(mocks.MockEthClient)
			mockRawClient := new(mocks.MockRawRPCClient)
			mockMetrics := mocks.NewMockMetrics()

			if tt.setupMocks != nil {
				tt.setupMocks(mockFetcher, mockEthClient, mockRawClient)
			}

			rpcClient := rpc.NewRPCClient(mockEthClient, mockRawClient, mockFetcher, mockMetrics)

			result, err := rpcClient.GetReceiptsByBlocks(context.Background(), tt.blocks)
			if tt.expectError {
				assert.Error(t, err)
				assert.Nil(t, result)
			} else {
				require.NoError(t, err)
				assert.Equal(t, tt.expected, result)
			}
			mockFetcher.AssertExpectations(t)
			mockEthClient.AssertExpectations(t)
		})
	}
}
