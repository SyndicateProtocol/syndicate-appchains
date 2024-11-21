package translator_test

import (
	"fmt"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestGetLastUsedBlockNumber(t *testing.T) {
	tests := []struct {
		setupMocks       func(mockClient *mocks.MockRPCClient)
		lastUsedBlock    *types.Block
		name             string
		expectedBlockNum uint64
		startTime        int
	}{
		{
			name:             "Returns 0 when LastUsedBlock is nil",
			setupMocks:       func(mockClient *mocks.MockRPCClient) {},
			lastUsedBlock:    nil,
			startTime:        100,
			expectedBlockNum: 0,
		},
		{
			name:             "Returns 0 when LastUsedBlock number cannot be parsed",
			setupMocks:       func(mockClient *mocks.MockRPCClient) {},
			lastUsedBlock:    &types.Block{"number": "invalid"},
			startTime:        100,
			expectedBlockNum: 0,
		},
		{
			name:             "Returns 0 when LastUsedBlock timestamp cannot be parsed",
			setupMocks:       func(mockClient *mocks.MockRPCClient) {},
			lastUsedBlock:    &types.Block{"number": "0xa", "timestamp": "invalid"},
			startTime:        100,
			expectedBlockNum: 0,
		},
		{
			name: "Returns 0 when error getting next block",
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0xb", false).
					Return(types.Block{}, fmt.Errorf("rpc error"))
			},
			lastUsedBlock:    &types.Block{"number": "0xa", "timestamp": "0x64"},
			startTime:        100,
			expectedBlockNum: 0,
		},
		{
			name: "Returns 0 when next block timestamp cannot be parsed",
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0xb", false).
					Return(types.Block{"timestamp": "invalid"}, nil)
			},
			lastUsedBlock:    &types.Block{"number": "0xa", "timestamp": "0x64"},
			startTime:        100,
			expectedBlockNum: 0,
		},
		{
			name: "Returns block number when it spans start time",
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0xb", false).
					Return(types.Block{"timestamp": "0x65"}, nil)
			},
			lastUsedBlock:    &types.Block{"number": "0xa", "timestamp": "0x64"},
			startTime:        100,
			expectedBlockNum: 10,
		},
		{
			name: "Returns 0 when block timestamps don't span start time",
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0xb", false).
					Return(types.Block{"timestamp": "0x63"}, nil)
			},
			lastUsedBlock:    &types.Block{"number": "0xa", "timestamp": "0x62"},
			startTime:        100,
			expectedBlockNum: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockClient := new(mocks.MockRPCClient)
			if tt.setupMocks != nil {
				tt.setupMocks(mockClient)
			}

			fetcher := translator.NewSequencingBlockFetcher(mockClient, 12)
			fetcher.LastUsedBlock = tt.lastUsedBlock

			result := fetcher.GetLastUsedBlockNumber(tt.startTime)
			assert.Equal(t, tt.expectedBlockNum, result)
		})
	}
}

func TestFindFirstBlockOnOrBeforeTime(t *testing.T) {
	tests := []struct {
		setupMocks          func(mockClient *mocks.MockRPCClient)
		name                string
		expectedBlockNumber uint64
		targetTime          int
		expectError         bool
	}{
		{
			name:       "Successfully finds block before target time",
			targetTime: 100,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x14", "timestamp": "0x96"}, nil)

				mockClient.On("GetBlockByNumber", mock.Anything, "0xa", false).
					Return(types.Block{"number": "0xa", "timestamp": "0x50"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xf", false).
					Return(types.Block{"number": "0xf", "timestamp": "0x78"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xc", false).
					Return(types.Block{"number": "0xc", "timestamp": "0x63"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xd", false).
					Return(types.Block{"number": "0xd", "timestamp": "0x65"}, nil)
			},
			expectedBlockNumber: 0xc,
			expectError:         false,
		},
		{
			name:       "Successfully finds block on target timee",
			targetTime: 100,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x14", "timestamp": "0x96"}, nil)

				mockClient.On("GetBlockByNumber", mock.Anything, "0xa", false).
					Return(types.Block{"number": "0xa", "timestamp": "0x50"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xf", false).
					Return(types.Block{"number": "0xf", "timestamp": "0x64"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x12", false).
					Return(types.Block{"number": "0xc", "timestamp": "0x65"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x10", false).
					Return(types.Block{"number": "0xc", "timestamp": "0x65"}, nil)
			},
			expectedBlockNumber: 0xf,
			expectError:         false,
		},
		{
			name:       "Error getting latest block",
			targetTime: 100,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{}, assert.AnError)
			},
			expectedBlockNumber: 0,
			expectError:         true,
		},
		{
			name:       "Error during binary search",
			targetTime: 100,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x14", "timestamp": "0x96"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xa", false).
					Return(types.Block{}, assert.AnError)
			},
			expectedBlockNumber: 0,
			expectError:         true,
		},
		{
			name:       "No block found before target time",
			targetTime: 1,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x14", "timestamp": "0x96"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xa", false).
					Return(types.Block{"number": "0xa", "timestamp": "0x50"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x5", false).
					Return(types.Block{"number": "0x5", "timestamp": "0x28"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x2", false).
					Return(types.Block{"number": "0x2", "timestamp": "0x14"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x1", false).
					Return(types.Block{"number": "0x1", "timestamp": "0xa"}, nil)
			},
			expectedBlockNumber: 0,
			expectError:         true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockClient := new(mocks.MockRPCClient)
			if tt.setupMocks != nil {
				tt.setupMocks(mockClient)
			}

			fetcher := translator.NewSequencingBlockFetcher(mockClient, 2)
			block, err := fetcher.FindFirstBlockOnOrBeforeTime(tt.targetTime)

			if tt.expectError {
				assert.Error(t, err)
				assert.Zero(t, block)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expectedBlockNumber, block)
			}

			mockClient.AssertExpectations(t)
		})
	}
}

func TestGetSequencingBlocksByTimeWindow(t *testing.T) {
	tests := []struct {
		setupMocks            func(mockClient *mocks.MockRPCClient)
		name                  string
		expectedBlocks        []*types.Block
		timeWindowStart       int
		timeWindowEnd         int
		firstBlockBeforeStart uint64
		expectError           bool
	}{
		{
			name:                  "Blocks within time window",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x96,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x60"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x7", false).
					Return(types.Block{"number": "0x7", "timestamp": "0x70"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x8", false).
					Return(types.Block{"number": "0x8", "timestamp": "0x80"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x9", false).
					Return(types.Block{"number": "0x9", "timestamp": "0x90"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0xa", false).
					Return(types.Block{"number": "0xa", "timestamp": "0xa0"}, nil)
			},
			expectedBlocks: []*types.Block{{"number": "0x6", "timestamp": "0x60"}, {"number": "0x7", "timestamp": "0x70"}, {"number": "0x8", "timestamp": "0x80"}, {"number": "0x9", "timestamp": "0x90"}},
			expectError:    false,
		},
		{
			name:                  "No block beyond end of time window",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x90,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x60"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x7", false).
					Return(types.Block{"number": "0x7", "timestamp": "0x70"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x8", false).
					Return(types.Block{"number": "0x8", "timestamp": "0x80"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x9", false).
					Return(types.Block(nil), nil)
			},
			expectedBlocks: nil,
			expectError:    true,
		},
		{
			name:                  "No blocks within time window",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x60,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x70"}, nil)
			},
			expectedBlocks: nil,
			expectError:    false,
		},
		{
			name:                  "First block timestamp is before start of time window",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x60,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x40"}, nil)
			},
			expectedBlocks: nil,
			expectError:    true,
		},
		{
			name:                  "First block timestamp is equal to start of time window",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x60,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x50"}, nil)
			},
			expectedBlocks: nil,
			expectError:    true,
		},
		{
			name:                  "Error fetching block",
			timeWindowStart:       0x50,
			timeWindowEnd:         0x96,
			firstBlockBeforeStart: 0x5,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{}, assert.AnError)
			},
			expectedBlocks: nil,
			expectError:    true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockClient := new(mocks.MockRPCClient)
			if tt.setupMocks != nil {
				tt.setupMocks(mockClient)
			}

			fetcher := translator.NewSequencingBlockFetcher(mockClient, 2)
			blocks, err := fetcher.GetSequencingBlocksByTimeWindow(tt.timeWindowStart, tt.timeWindowEnd, tt.firstBlockBeforeStart)

			if tt.expectError {
				assert.Error(t, err)
				assert.Nil(t, blocks)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expectedBlocks, blocks)
			}

			mockClient.AssertExpectations(t)
		})
	}
}

func TestGetSequencingBlocks(t *testing.T) {
	tests := []struct {
		block                 types.Block
		setupMocks            func(mockClient *mocks.MockRPCClient)
		name                  string
		expectedBlocks        []*types.Block
		timeWindowStart       int
		timeWindowEnd         int
		firstBlockBeforeStart uint64
		expectError           bool
	}{
		{
			name: "Successful fetch with blocks in time window",
			block: types.Block{
				"timestamp": "0x6",
			},
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x4", false).
					Return(types.Block{"number": "0x4", "timestamp": "0x4"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x5", false).
					Return(types.Block{"number": "0x5", "timestamp": "0x5"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{"number": "0x6", "timestamp": "0x6"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x7", false).
					Return(types.Block{"number": "0x7", "timestamp": "0x7"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x8", "timestamp": "0x8"}, nil)
			},
			expectedBlocks: []*types.Block{
				{"number": "0x5", "timestamp": "0x5"},
				{"number": "0x6", "timestamp": "0x6"},
			},
			expectError: false,
		},
		{
			name: "Error fetching block timestamp",
			block: types.Block{
				"timestamp": nil,
			},
			setupMocks:     func(mockClient *mocks.MockRPCClient) {},
			expectedBlocks: nil,
			expectError:    true,
		},
		{
			name: "No blocks found within time window",
			block: types.Block{
				"timestamp": "0x6",
			},
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x2", false).
					Return(types.Block{"number": "0x3", "timestamp": "0x2"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x3", false).
					Return(types.Block{"number": "0x3", "timestamp": "0x3"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x4", false).
					Return(types.Block{"number": "0x4", "timestamp": "0x7"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x7", "timestamp": "0x7"}, nil)
			},
			expectedBlocks: nil,
			expectError:    false,
		},
		{
			name: "Error during block fetch",
			block: types.Block{
				"timestamp": "0x6",
			},
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "0x3", false).
					Return(types.Block{"number": "0x3", "timestamp": "0x3"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x4", false).
					Return(types.Block{"number": "0x4", "timestamp": "0x4"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x5", false).
					Return(types.Block{"number": "0x5", "timestamp": "0x5"}, nil)
				mockClient.On("GetBlockByNumber", mock.Anything, "0x6", false).
					Return(types.Block{}, assert.AnError)
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{"number": "0x5", "timestamp": "0x1"}, nil)
			},
			expectedBlocks: nil,
			expectError:    true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockClient := new(mocks.MockRPCClient)
			if tt.setupMocks != nil {
				tt.setupMocks(mockClient)
			}

			fetcher := translator.NewSequencingBlockFetcher(mockClient, 2)
			blocks, err := fetcher.GetSequencingBlocks(tt.block)

			if tt.expectError {
				assert.Error(t, err)
				assert.Nil(t, blocks)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expectedBlocks, blocks)
			}
			mockClient.AssertExpectations(t)
		})
	}
}
