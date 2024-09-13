package types

import (
	"testing"

	"github.com/ethereum-optimism/optimism/op-node/rollup/derive"
	"github.com/stretchr/testify/require"
)

func TestNewChannelID(t *testing.T) {
	channelID, err := NewChannelID()

	require.NoError(t, err)
	require.Equal(t, derive.ChannelIDLength, len(channelID))
	require.NotEqual(t, derive.ChannelID{}, channelID)
}

func TestNewFrame(t *testing.T) {
	channelID, err := NewChannelID()
	require.NoError(t, err, "Failed to generate ChannelID")

	frameNumber := uint16(1)
	data := []byte("test data")
	isLast := true

	frame := NewFrame(channelID, frameNumber, data, isLast)

	require.Equal(t, channelID, frame.ID)
	require.Equal(t, frameNumber, frame.FrameNumber)
	require.Equal(t, data, frame.Data)
	require.Equal(t, isLast, frame.IsLast)
}

func TestNewFrameIsNotLast(t *testing.T) {
	channelID, err := NewChannelID()
	require.NoError(t, err, "Failed to generate ChannelID")

	frameNumber := uint16(2)
	data := []byte("another test data")
	isLast := false

	frame := NewFrame(channelID, frameNumber, data, isLast)

	require.Equal(t, channelID, frame.ID)
	require.Equal(t, frameNumber, frame.FrameNumber)
	require.Equal(t, data, frame.Data)
	require.Equal(t, isLast, frame.IsLast)
}
