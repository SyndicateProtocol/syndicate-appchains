package publisher

import (
	"context"
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
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

	mockL3 := &mockOpTranslatorRPC{}
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

	err := publisher.Start(context.Background())
	require.NoError(t, err, "Publisher should start without error")

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
	err = publisher.Stop()
	require.NoError(t, err, "Publisher should stop without error")
}

////////////////////////////////////////////////////////////
// Mocks
////////////////////////////////////////////////////////////

// Mocked L3 RPC API
type mockOpTranslatorRPC struct{ mock.Mock }

var _ RPCAPI = (*mockOpTranslatorRPC)(nil)

func (m *mockOpTranslatorRPC) BlockNumber(ctx context.Context) (uint64, error) {
	args := m.Called(ctx)
	val, ok := args.Get(0).(uint64)
	if !ok {
		return 0, fmt.Errorf("unexpected type for BlockNumber return value")
	}
	return val, args.Error(1)
}

func (m *mockOpTranslatorRPC) BlockByNumber(ctx context.Context, number *big.Int) (*types.Block, error) {
	args := m.Called(ctx)
	val, ok := args.Get(0).(*types.Block)
	if !ok {
		return nil, fmt.Errorf("unexpected type for BlockByNumber return value")
	}
	return val, args.Error(1)
}

func (m *mockOpTranslatorRPC) ChainID(ctx context.Context) (*big.Int, error) {
	args := m.Called(ctx)
	val, ok := args.Get(0).(*big.Int)
	if !ok {
		return nil, fmt.Errorf("unexpected type for ChainID return value")
	}
	return val, args.Error(1)
}

// Mocked AltDA Provider
type mockAltDAProvider struct{ mock.Mock }

var _ AltDAProvider = (*mockAltDAProvider)(nil)

func (m *mockAltDAProvider) GetInput(ctx context.Context, comm altda.CommitmentData) ([]byte, error) {
	args := m.Called(ctx, comm)
	val, ok := args.Get(0).([]byte)
	if !ok {
		return nil, fmt.Errorf("unexpected type for GetInput return value")
	}
	return val, args.Error(1)
}

func (m *mockAltDAProvider) SetInput(ctx context.Context, img []byte) (altda.CommitmentData, error) {
	args := m.Called(ctx, img)
	val, ok := args.Get(0).(altda.CommitmentData)
	if !ok {
		return altda.NewGenericCommitment([]byte{}), fmt.Errorf("unexpected type for SetInput return value")
	}
	return val, args.Error(1)
}
