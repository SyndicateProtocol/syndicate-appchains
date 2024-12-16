package types

import (
	"bytes"
	"compress/zlib"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/rlp"
)

// Batch version byte is 0x00
// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
const BatchVersionByte = 0x00

type Batch struct { //nolint:govet // order is necessary for correct RLP encoding
	ParentHash      common.Hash
	EpochNumber     uint64
	EpochHash       common.Hash
	Timestamp       uint64
	TransactionList []hexutil.Bytes
}

func NewBatch(parentHashStr, epochNumberStr, epochHashStr, timestampStr string, txs []hexutil.Bytes) (*Batch, error) {
	parentHash := common.HexToHash(parentHashStr)
	epochHash := common.HexToHash(epochHashStr)

	epochNumber, err := utils.HexToUInt64(epochNumberStr)
	if err != nil {
		return nil, err
	}

	timestamp, err := utils.HexToUInt64(timestampStr)
	if err != nil {
		return nil, err
	}

	timestamp += 2 // TODO remove this

	return &Batch{
		ParentHash:      parentHash,
		EpochNumber:     epochNumber,
		EpochHash:       epochHash,
		Timestamp:       timestamp,
		TransactionList: txs,
	}, nil
}

func (b *Batch) Encode() ([]byte, error) {
	buf := new(bytes.Buffer)

	err := buf.WriteByte(BatchVersionByte)
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

func (b *Batch) GetFrames(frameSize int) ([]*Frame, error) {
	encodedBatch, err := b.Encode()
	if err != nil {
		return nil, err
	}

	buff := bytes.NewBuffer(nil)
	err = rlp.Encode(buff, encodedBatch)
	if err != nil {
		return nil, err
	}

	channel, err := ToChannel(buff.Bytes())
	if err != nil {
		return nil, err
	}

	return ToFrames(channel, frameSize, b.EpochHash)
}

func ToChannel(batch []byte) ([]byte, error) {
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

func ToFrames(channel []byte, frameSize int, blockHash common.Hash) ([]*Frame, error) {
	numFrames := (len(channel) + frameSize - 1) / frameSize
	frames := make([]*Frame, numFrames)

	var frameNum uint16

	id, err := NewChannelID(blockHash)
	if err != nil {
		return nil, err
	}

	frameNum = 0
	for i := 0; i < len(channel); i += frameSize {
		end := i + frameSize
		if end > len(channel) {
			end = len(channel)
		}

		frame := NewFrame(id, frameNum, channel[i:end], end == len(channel))
		frames[frameNum] = frame
		frameNum++
	}

	return frames, nil
}
