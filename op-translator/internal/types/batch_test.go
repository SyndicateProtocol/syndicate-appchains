package types

import (
	"crypto/rand"
	"io"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

const frameSizeTest = 1024

func mockBatch() *Batch {
	return &Batch{
		ParentHash:      common.HexToHash("0x123"),
		EpochNumber:     1,
		EpochHash:       common.HexToHash("0x456"),
		Timestamp:       2,
		TransactionList: nil,
	}
}

func TestBatchEncode(t *testing.T) {
	batch := mockBatch()

	encoded, err := batch.encode()
	assert.Nil(t, err)

	expected := []byte{0x0, 0xf8, 0x45, 0xa0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x23, 0x1, 0xa0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x56, 0x2, 0xc0}

	// First byte is batch_version and should be 0x00
	assert.Equal(t, byte(0x00), encoded[0])
	assert.Equal(t, expected, encoded)
}

func TestToChannel(t *testing.T) {
	t.Run("Empty Input", func(t *testing.T) {
		input := []byte{}
		expected := []byte{0x78, 0x1, 0x1, 0x0, 0x0, 0xff, 0xff, 0x0, 0x0, 0x0, 0x1}

		output, err := toChannel(input)

		assert.NoError(t, err)
		assert.Equal(t, expected, output)
	})

	t.Run("Valid Input", func(t *testing.T) {
		input := []byte{0x00, 0x01, 0x02, 0x03, 0x04}

		output, err := toChannel(input)

		assert.NoError(t, err)
		// First bytes equals NO_COMPRESSION flag for zlib
		assert.Equal(t, byte(0x78), output[0])
		assert.Equal(t, byte(0x01), output[1])
	})
}

func randData(size int) []byte {
	bytes := make([]byte, size)
	_, _ = io.ReadFull(rand.Reader, bytes)
	return bytes
}

func TestToFrames_Public(t *testing.T) {
	batch := mockBatch()
	_, err := batch.ToFrames(frameSizeTest)
	assert.NoError(t, err)
}

func TestToFrames_Private(t *testing.T) {
	t.Run("1 frame data size", func(t *testing.T) {
		data := []byte("data")
		blockHash := common.HexToHash("0xabc")
		frames, err := toFrames(data, frameSizeTest, blockHash)
		assert.NoError(t, err)

		assert.Len(t, frames, 1)
		frame := frames[0]

		assert.NotNil(t, frame.ID)
		assert.Equal(t, uint16(0), frame.FrameNumber)
		assert.Equal(t, data, frame.Data)
		assert.Equal(t, true, frame.IsLast)
	})

	t.Run("2 frame data size", func(t *testing.T) {
		data := randData(frameSizeTest + 1)
		blockHash := common.HexToHash("0xabc")
		frames, err := toFrames(data, frameSizeTest, blockHash)
		assert.NoError(t, err)

		assert.Len(t, frames, 2)

		assert.Equal(t, frames[0].ID, frames[1].ID)

		assert.Equal(t, uint16(0), frames[0].FrameNumber)
		assert.Equal(t, uint16(1), frames[1].FrameNumber)

		assert.Equal(t, data[:frameSizeTest], frames[0].Data)
		assert.Equal(t, data[frameSizeTest:], frames[1].Data)

		assert.Equal(t, false, frames[0].IsLast)
		assert.Equal(t, true, frames[1].IsLast)
	})
}
