package types_test

import (
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum-optimism/optimism/op-node/rollup/derive"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/require"
)

func TestNewChannelID(t *testing.T) {
	blockHash := common.HexToHash("0xabc")
	channelID, err := types.NewChannelID(blockHash)

	require.NoError(t, err)
	require.Equal(t, derive.ChannelIDLength, len(channelID))
	require.NotEqual(t, derive.ChannelID{}, channelID)

	channelID2, err := types.NewChannelID(blockHash)
	require.NoError(t, err)
	// ChannelID is deterministic based on the block hash
	require.Equal(t, channelID, channelID2)
}

func TestNewFrame(t *testing.T) {
	blockHash := common.HexToHash("0xabc")
	channelID, err := types.NewChannelID(blockHash)
	require.NoError(t, err, "Failed to generate ChannelID")

	frameNumber := uint16(1)
	data := []byte("test data")
	isLast := true

	frame := types.NewFrame(channelID, frameNumber, data, isLast)

	require.Equal(t, channelID, frame.ID)
	require.Equal(t, frameNumber, frame.FrameNumber)
	require.Equal(t, data, frame.Data)
	require.Equal(t, isLast, frame.IsLast)
}

func TestNewFrameIsNotLast(t *testing.T) {
	blockHash := common.HexToHash("0xabc")
	channelID, err := types.NewChannelID(blockHash)
	require.NoError(t, err, "Failed to generate ChannelID")

	frameNumber := uint16(2)
	data := []byte("another test data")
	isLast := false

	frame := types.NewFrame(channelID, frameNumber, data, isLast)

	require.Equal(t, channelID, frame.ID)
	require.Equal(t, frameNumber, frame.FrameNumber)
	require.Equal(t, data, frame.Data)
	require.Equal(t, isLast, frame.IsLast)
}
