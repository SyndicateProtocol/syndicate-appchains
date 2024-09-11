package translator

import (
	"bytes"
	"compress/zlib"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/rlp"
)

// TODO (SEQ-104): Move to config
// Max possible frame size is 1,000,000
// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#frame-format
const FrameSize = 1024

// Frame version byte is 0x00
// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batcher-transaction-format
const FrameVersionByte = 0x00

type Batch struct { //nolint:govet // order is necessary for correct RLP encoding
	ParentHash      common.Hash
	EpochNumber     uint64
	EpochHash       common.Hash
	Timestamp       uint64
	TransactionList []hexutil.Bytes
}

func NewBatch(parentHash common.Hash, epochNumber uint64, epochHash common.Hash, timestamp uint64, txs []hexutil.Bytes) *Batch {
	return &Batch{
		ParentHash:      parentHash,
		EpochNumber:     epochNumber,
		EpochHash:       epochHash,
		Timestamp:       timestamp,
		TransactionList: txs,
	}
}

func (b *Batch) encode() ([]byte, error) {
	buf := new(bytes.Buffer)

	err := buf.WriteByte(FrameVersionByte)
	if err != nil {
		return nil, err
	}

	// Write RLP encode (parent_hash, epoch_number, epoch_hash, timestamp, transaction_list)
	// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
	err = rlp.Encode(buf, b)
	if err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func (b *Batch) ToFrames() ([]*Frame, error) {
	encodedBatch, err := b.encode()
	if err != nil {
		return nil, err
	}

	channel, err := toChannel(encodedBatch)
	if err != nil {
		return nil, err
	}

	return toFrames(channel)
}

func toChannel(batch []byte) ([]byte, error) {
	var buf bytes.Buffer

	writer, err := zlib.NewWriterLevel(&buf, zlib.NoCompression)
	if err != nil {
		return nil, err
	}
	_, err = writer.Write(batch)
	if err != nil {
		return nil, err
	}
	err = writer.Close()
	if err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func toFrames(channel []byte) ([]*Frame, error) {
	numFrames := (len(channel) + FrameSize - 1) / FrameSize
	frames := make([]*Frame, numFrames)

	var frameNum uint16

	id, err := NewChannelID()
	if err != nil {
		return nil, err
	}

	frameNum = 0
	for i := 0; i < len(channel); i += FrameSize {
		end := i + FrameSize
		if end > len(channel) {
			end = len(channel)
		}

		frame := NewFrame(id, frameNum, channel[i:end], end == len(channel))

		frames[frameNum] = frame
		frameNum++
	}

	return frames, nil
}
