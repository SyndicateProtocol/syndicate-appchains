package publisher

import (
	"context"
	"testing"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestBasic(t *testing.T) {
	log := testlog.Logger(t, gethlog.LvlDebug)

	mockL3 := &mockL3{}
	mockedBatchProvider := &mockBatchProvider{}
	mockAltDAProvider := &mockAltDAProvider{}
	publisher := NewPublisher(
		mockL3,
		mockedBatchProvider,
		mockAltDAProvider,
		10*time.Millisecond,
		10*time.Millisecond,
		log,
		metrics.NoopMetrics,
	)

	// no L3 blocks
	mockedL3Call := mockL3.On("BlockNumber", mock.Anything).
		Return(uint64(0), nil)
	mockedBatchProvider.On("GetBatch", mock.Anything, mock.Anything).
		Return(&types.Batch{}, nil)
	mockAltDAProvider.On("SetInput", mock.Anything, mock.Anything).
		Return(altda.GenericCommitment{}, nil)

	publisher.Start(context.Background())

	time.Sleep(20 * time.Millisecond) // wait for the publisher to process the batch (more time than the poll interval)
	require.Len(t, mockAltDAProvider.Calls, 0)

	// progress the L3 block number to 1
	mockedL3Call.Unset()
	mockL3.On("BlockNumber", mock.Anything).
		Return(uint64(1), nil)

	time.Sleep(20 * time.Millisecond) // wait for the publisher to process the batch (more time than the poll interval)
	// assert the altDA provider was called once
	require.Len(t, mockAltDAProvider.Calls, 1)
}

////////////////////////////////////////////////////////////
// Mocks
////////////////////////////////////////////////////////////

// Mocked L3 RPC API
type mockL3 struct{ mock.Mock }

var _ L3RPCAPI = (*mockL3)(nil)

func (m *mockL3) BlockNumber(ctx context.Context) (uint64, error) {
	args := m.Called(ctx)
	return args.Get(0).(uint64), args.Error(1) //nolint:errcheck // mock safe cast
}

// Mocked Batch Provider
type mockBatchProvider struct{ mock.Mock }

var _ translator.IBatchProvider = (*mockBatchProvider)(nil)

func (m *mockBatchProvider) GetBatch(ctx context.Context, block types.Block) (*types.Batch, error) {
	args := m.Called(ctx, block)
	return args.Get(0).(*types.Batch), args.Error(1) //nolint:errcheck // mock safe cast
}

func (m *mockBatchProvider) Close() {
}

// Mocked AltDA Provider
type mockAltDAProvider struct{ mock.Mock }

var _ AltDAProvider = (*mockAltDAProvider)(nil)

func (m *mockAltDAProvider) GetInput(ctx context.Context, comm altda.CommitmentData) ([]byte, error) {
	args := m.Called(ctx, comm)
	return args.Get(0).([]byte), args.Error(1) //nolint:errcheck // mock safe cast
}

func (m *mockAltDAProvider) SetInput(ctx context.Context, img []byte) (altda.CommitmentData, error) {
	args := m.Called(ctx, img)
	return args.Get(0).(altda.CommitmentData), args.Error(1) //nolint:errcheck // mock safe cast
}
