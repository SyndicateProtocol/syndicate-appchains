package types

import (
	"crypto/rand"

	"github.com/ethereum-optimism/optimism/op-node/rollup/derive"
)

type Frame struct {
	derive.Frame
}

// Boolean values for IsLast byte
const IsLastByte = 0x01
const IsNotLastByte = 0x00

func NewChannelID() (derive.ChannelID, error) {
	id := make([]byte, derive.ChannelIDLength)
	_, err := rand.Read(id)
	if err != nil {
		return derive.ChannelID{}, err
	}

	return derive.ChannelID(id), nil
}

func NewFrame(id derive.ChannelID, frameNumber uint16, data []byte, isLast bool) *Frame {
	return &Frame{
		Frame: derive.Frame{
			ID:          id,
			FrameNumber: frameNumber,
			Data:        data,
			IsLast:      isLast,
		},
	}
}
