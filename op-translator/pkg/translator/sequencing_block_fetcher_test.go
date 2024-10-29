package translator_test

import (
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestFindFirstBlockOnOrBeforeTime(t *testing.T) {
	tests := []struct {
		setupMocks    func(mockClient *mocks.MockRPCClient)
		expectedBlock types.Block
		name          string
		targetTime    int
		expectError   bool
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
			expectedBlock: types.Block{"number": "0xc", "timestamp": "0x63"},
			expectError:   false,
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
			expectedBlock: types.Block{"number": "0xf", "timestamp": "0x64"},
			expectError:   false,
		},
		{
			name:       "Error getting latest block",
			targetTime: 100,
			setupMocks: func(mockClient *mocks.MockRPCClient) {
				mockClient.On("GetBlockByNumber", mock.Anything, "latest", false).
					Return(types.Block{}, assert.AnError)
			},
			expectedBlock: nil,
			expectError:   true,
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
			expectedBlock: nil,
			expectError:   true,
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
			expectedBlock: nil,
			expectError:   true,
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
				assert.Nil(t, block)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expectedBlock, block)
			}

			mockClient.AssertExpectations(t)
		})
	}
}

func TestGetSequencingBlocksByTimeWindow(t *testing.T) {
	tests := []struct {
		setupMocks            func(mockClient *mocks.MockRPCClient)
		name                  string
		expectedBlocks        []string
		timeWindowStart       int
		timeWindowEnd         int
		firstBlockBeforeStart int
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
			expectedBlocks: []string{"0x6", "0x7", "0x8", "0x9"},
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
			expectedBlocks: []string(nil),
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
