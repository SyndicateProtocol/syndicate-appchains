package types

import (
	"bytes"

	"github.com/ethereum-optimism/optimism/op-node/rollup/derive"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
)

type Frame struct {
	derive.Frame
}

// Boolean values for IsLast byte
const IsLastByte = 0x01
const IsNotLastByte = 0x00

func NewChannelID(blockHash common.Hash) (derive.ChannelID, error) {
	id := make([]byte, derive.ChannelIDLength)
	// Make Channel id deterministic based on block hash
	hashed := crypto.Keccak256(blockHash.Bytes())
	copy(id, hashed[:derive.ChannelIDLength])

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

func ToData(frames []*Frame) ([]byte, error) {
	buf := bytes.NewBuffer([]byte{BatcherTransactionVersionByte})
	for _, frame := range frames {
		err := frame.MarshalBinary(buf)
		if err != nil {
			return nil, err
		}
	}

	return buf.Bytes(), nil
}
