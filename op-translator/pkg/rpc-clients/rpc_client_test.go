package rpc

import (
	"context"
	"errors"
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	ethtypes "github.com/ethereum/go-ethereum/core/types"

	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestConnect(t *testing.T) {
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
			client, err := Connect(tt.address)

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
	client, err := Connect("http://localhost:8545")
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

func TestRPCClient_FetchReceipts(t *testing.T) {
	tests := []struct {
		method           sources.ReceiptsFetchingMethod
		block            *ethtypes.Block
		txHashes         []common.Hash
		setupMocks       func(*mocks.MockReceiptsFetcher, *mocks.MockEthClient, *mocks.MockRawRPCClient)
		expectedReceipts ethtypes.Receipts
		expectError      bool
		name             string
	}{
		{
			name:   "EthGetTransactionReceiptBatch - success",
			method: sources.EthGetTransactionReceiptBatch,
			block: ethtypes.NewBlock(
				&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
				&ethtypes.Body{},
				nil,
				nil,
			),
			txHashes: []common.Hash{common.HexToHash("0x456")},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				mockFetcher.On("PickReceiptsMethod", 1).Return(sources.EthGetTransactionReceiptBatch)
				// Mock the BlockReceipts method to return a valid result
				mockEthClient.On("BlockReceipts", mock.Anything, mock.Anything).Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)
			},
			expectedReceipts: ethtypes.Receipts{&ethtypes.Receipt{Status: 1}},
			expectError:      false,
		},
		{
			name:   "EthGetTransactionReceiptBatch - error fetching",
			method: sources.EthGetTransactionReceiptBatch,
			block: ethtypes.NewBlock(
				&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
				&ethtypes.Body{},
				nil,
				nil,
			),
			txHashes: []common.Hash{common.HexToHash("0x456")},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				mockFetcher.On("PickReceiptsMethod", 1).Return(sources.EthGetTransactionReceiptBatch)
				mockFetcher.On("OnReceiptsMethodErr", mock.Anything, mock.Anything).Return(errors.New("fetching error"))
				mockEthClient.On("BlockReceipts", mock.Anything, mock.Anything).Return(ethtypes.Receipts{}, errors.New("fetching error"))
			},
			expectedReceipts: nil,
			expectError:      true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockFetcher := new(mocks.MockReceiptsFetcher)
			mockEthClient := new(mocks.MockEthClient)
			mockRawClient := new(mocks.MockRawRPCClient)

			if tt.setupMocks != nil {
				tt.setupMocks(mockFetcher, mockEthClient, mockRawClient)
			}

			rpcClient := &RPCClient{
				receiptsFetcher: mockFetcher,
				client:          mockEthClient,
				rawClient:       mockRawClient,
			}

			result, err := rpcClient.FetchReceipts(context.Background(), eth.BlockToInfo(tt.block), tt.txHashes)

			if tt.expectError {
				assert.Error(t, err)
				assert.Nil(t, result)
			} else {
				require.NoError(t, err)
				assert.Equal(t, tt.expectedReceipts, result)
			}

			mockFetcher.AssertExpectations(t)
			mockEthClient.AssertExpectations(t)
		})
	}
}
