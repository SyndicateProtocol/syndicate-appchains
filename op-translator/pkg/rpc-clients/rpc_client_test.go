package rpc_test

import (
	"context"
	"errors"
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/trie"

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
			client, err := rpc.Connect(tt.address)

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
	client, err := rpc.Connect("http://localhost:8545")
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

func TestBlocksReceiptsByNumbers(t *testing.T) {
	tests := []struct { //nolint:govet // test struct
		setupMocks  func(*mocks.MockReceiptsFetcher, *mocks.MockEthClient, *mocks.MockRawRPCClient)
		expected    []*ethtypes.Receipt
		blockNumber []string
		name        string
		expectError bool
	}{
		{
			name:        "Successful receipt fetching",
			blockNumber: []string{"0xA"},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				tx := ethtypes.NewTransaction(0, common.Address{}, big.NewInt(0), 0, big.NewInt(0), nil)
				block := ethtypes.NewBlock(
					&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
					&ethtypes.Body{Transactions: []*ethtypes.Transaction{tx}, Uncles: []*ethtypes.Header{}, Withdrawals: []*ethtypes.Withdrawal{}},
					[]*ethtypes.Receipt{},
					trie.NewStackTrie(nil),
				)
				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(10)).Return(block, nil)
				txHash := tx.Hash()
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)
			},
			expected:    []*ethtypes.Receipt{{Status: 1}},
			expectError: false,
		},
		{
			name:        "Error in BlockByNumber",
			blockNumber: []string{"0xA"},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				// Mock an error in BlockByNumber
				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(10)).
					Return(&ethtypes.Block{}, errors.New("block fetching error"))
			},
			expected:    nil,
			expectError: true,
		},
		{
			name:        "Error in FetchReceipts",
			blockNumber: []string{"0xA"},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				tx := ethtypes.NewTransaction(0, common.Address{}, big.NewInt(0), 0, big.NewInt(0), nil)
				block := ethtypes.NewBlock(
					&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
					&ethtypes.Body{Transactions: []*ethtypes.Transaction{tx}, Uncles: []*ethtypes.Header{}, Withdrawals: []*ethtypes.Withdrawal{}},
					[]*ethtypes.Receipt{},
					trie.NewStackTrie(nil),
				)
				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(10)).Return(block, nil)
				txHash := tx.Hash()
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txHash}).
					Return(ethtypes.Receipts{}, errors.New("receipts fetching error"))
			},
			expected:    nil,
			expectError: true,
		},
		{
			name:        "No block numbers provided",
			blockNumber: []string{},
			setupMocks:  nil,
			expected:    []*ethtypes.Receipt(nil),
			expectError: false,
		},
		{
			name:        "Successful receipt fetching for multiple blocks",
			blockNumber: []string{"0xA", "0xB"},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txA := ethtypes.NewTransaction(0, common.Address{}, big.NewInt(0), 0, big.NewInt(0), nil)
				blockA := ethtypes.NewBlock(
					&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
					&ethtypes.Body{Transactions: []*ethtypes.Transaction{txA}, Uncles: []*ethtypes.Header{}, Withdrawals: []*ethtypes.Withdrawal{}},
					[]*ethtypes.Receipt{},
					trie.NewStackTrie(nil),
				)
				txB := ethtypes.NewTransaction(0, common.Address{}, big.NewInt(0), 0, big.NewInt(0), nil)
				blockB := ethtypes.NewBlock(
					&ethtypes.Header{Number: big.NewInt(11), ParentHash: common.HexToHash("0x456")},
					&ethtypes.Body{Transactions: []*ethtypes.Transaction{txB}, Uncles: []*ethtypes.Header{}, Withdrawals: []*ethtypes.Withdrawal{}},
					[]*ethtypes.Receipt{},
					trie.NewStackTrie(nil),
				)

				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(10)).Return(blockA, nil)
				txAHash := txA.Hash()
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txAHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)

				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(11)).Return(blockB, nil)
				txBHash := txB.Hash()
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txBHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)
			},
			expected:    []*ethtypes.Receipt{{Status: 1}, {Status: 1}},
			expectError: false,
		},
		{
			name:        "Partial success - one block fetch fails",
			blockNumber: []string{"0xA", "0xB"},
			setupMocks: func(mockFetcher *mocks.MockReceiptsFetcher, mockEthClient *mocks.MockEthClient, mockRawClient *mocks.MockRawRPCClient) {
				txA := ethtypes.NewTransaction(0, common.Address{}, big.NewInt(0), 0, big.NewInt(0), nil)
				blockA := ethtypes.NewBlock(
					&ethtypes.Header{Number: big.NewInt(10), ParentHash: common.HexToHash("0x123")},
					&ethtypes.Body{Transactions: []*ethtypes.Transaction{txA}, Uncles: []*ethtypes.Header{}, Withdrawals: []*ethtypes.Withdrawal{}},
					[]*ethtypes.Receipt{},
					trie.NewStackTrie(nil),
				)

				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(10)).Return(blockA, nil)
				txAHash := txA.Hash()
				mockFetcher.On("FetchReceipts", mock.Anything, mock.Anything, []common.Hash{txAHash}).
					Return(ethtypes.Receipts{&ethtypes.Receipt{Status: 1}}, nil)

				mockEthClient.On("BlockByNumber", mock.Anything, big.NewInt(11)).
					Return(&ethtypes.Block{}, errors.New("block fetching error"))
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

			if tt.setupMocks != nil {
				tt.setupMocks(mockFetcher, mockEthClient, mockRawClient)
			}

			rpcClient := rpc.NewRPCClient(mockEthClient, mockRawClient, mockFetcher)

			result, err := rpcClient.BlocksReceiptsByNumbers(context.Background(), tt.blockNumber)
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
