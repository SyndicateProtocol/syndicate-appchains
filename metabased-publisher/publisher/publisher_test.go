package publisher

import (
	"context"
	"math/big"
	"testing"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"

	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestBasic(t *testing.T) {
	log := testlog.Logger(t, gethlog.LvlDebug)

	mockL3 := &mockEthClient{}
	mockAltDAProvider := &mockAltDAProvider{}
	publisher := NewPublisher(
		mockL3,
		mockAltDAProvider,
		common.Address{},
		common.Address{},
		10*time.Millisecond,
		10*time.Millisecond,
		10*time.Millisecond,
		log,
		metrics.NoopMetrics,
	)

	// no L3 blocks
	mockedL3Call := mockL3.On("BlockNumber", mock.Anything).
		Return(uint64(0), nil)
	mockAltDAProvider.On("SetInput", mock.Anything, mock.Anything).
		Return(altda.NewGenericCommitment([]byte{}), nil)

	publisher.Start(context.Background())

	time.Sleep(20 * time.Millisecond) // wait for the publisher to process the batch (more time than the poll interval)

	require.Len(t, mockAltDAProvider.Calls, 0)

	// progress the L3 block number to 1
	mockedL3Call.Unset()
	mockL3.On("BlockNumber", mock.Anything).
		Return(uint64(1), nil)
	mockL3.On("BlockByNumber", mock.Anything, mock.Anything).
		Return(types.NewBlock(&types.Header{Number: big.NewInt(1)}, nil, nil, nil), nil)

	time.Sleep(20 * time.Millisecond) // wait for the publisher to process the batch (more time than the poll interval)
	// assert the altDA provider was called once
	require.Len(t, mockAltDAProvider.Calls, 1)

	// Clean up
	publisher.Stop()
}

////////////////////////////////////////////////////////////
// Mocks
////////////////////////////////////////////////////////////

// Mocked L3 RPC API
type mockEthClient struct{ mock.Mock }

// Verify interface compliance
var _ RPCClient = (*mockEthClient)(nil)

func (m *mockEthClient) BlockNumber(ctx context.Context) (uint64, error) {
	args := m.Called(ctx)
	return mocks.Args0[uint64](args), args.Error(1)
}

func (m *mockEthClient) BlockByNumber(ctx context.Context, number *big.Int) (*types.Block, error) {
	args := m.Called(ctx, number)
	return mocks.Args0[*types.Block](args), args.Error(1)
}


func (m *mockEthClient) ChainID(ctx context.Context) (*big.Int, error) {
	args := m.Called(ctx)
	return mocks.Args0[*big.Int](args), args.Error(1)
}

func (m *mockEthClient) Close() {
	m.Called()
}

// Mocked AltDA Provider
type mockAltDAProvider struct{ mock.Mock }

var _ AltDAProvider = (*mockAltDAProvider)(nil)

func (m *mockAltDAProvider) GetInput(ctx context.Context, comm altda.CommitmentData) ([]byte, error) {
	args := m.Called(ctx, comm)
	return mocks.Args0[[]byte](args), args.Error(1)
}

func (m *mockAltDAProvider) SetInput(ctx context.Context, img []byte) (altda.CommitmentData, error) {
	args := m.Called(ctx, img)
	return mocks.Args0[altda.CommitmentData](args), args.Error(1)
}
