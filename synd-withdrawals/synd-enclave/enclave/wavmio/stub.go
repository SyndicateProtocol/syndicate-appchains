// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package wavmio

import (
	"encoding/binary"
	"errors"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rlp"

	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
)

type Wavm struct {
	seqMsgs            [][]byte
	posWithinMsg       uint64
	delayedMsgs        [][]byte
	delayedMsgFirstPos uint64
	preimages          map[common.Hash][]byte
	seqAdvanced        uint64
}

func serialize(headerVals []uint64, batch []byte) []byte {
	var msg []byte

	// Set header values
	for _, bound := range headerVals {
		var intData [8]byte
		binary.BigEndian.PutUint64(intData[:], bound)
		msg = append(msg, intData[:]...)
	}

	// Append the batch data
	msg = append(msg, batch...)

	return msg
}

func (w *Wavm) GetBlockHeaderByHash(hash common.Hash) (*types.Header, error) {
	enc, err := w.ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
	if err != nil {
		return nil, fmt.Errorf("error resolving preimage: %w", err)
	}
	header := &types.Header{}
	err = rlp.DecodeBytes(enc, &header)
	if err != nil {
		return nil, fmt.Errorf("error parsing resolved block header: %w", err)
	}
	return header, nil
}

type Batch struct {
	TimeBounds bridgegen.IBridgeTimeBounds
	Data       []byte
	Messages   []arbostypes.L1IncomingMessage
}

type ValidationInput struct {
	BlockHash    common.Hash
	PreimageData [][]byte
	Batches      []Batch
}

func Init(data ValidationInput) (*Wavm, error) {
	w := Wavm{}
	w.preimages = make(map[common.Hash][]byte)
	for _, preimage := range data.PreimageData {
		w.preimages[crypto.Keccak256Hash(preimage)] = preimage
	}

	header, err := w.GetBlockHeaderByHash(data.BlockHash)
	if err != nil {
		return nil, err
	}
	w.delayedMsgFirstPos = types.NewBlockWithHeader(header).Nonce()

	delayedMsgPos := w.delayedMsgFirstPos
	for _, batch := range data.Batches {
		for _, msg := range batch.Messages {
			if msg.BatchGasCost != nil {
				return nil, errors.New("batch gas cost is a computed property and must be nil")
			}
			if msg.Header.RequestId.Big().Uint64() != delayedMsgPos {
				return nil, fmt.Errorf("unexpected request id: got %d, expected %d", msg.Header.RequestId.Big().Uint64(), delayedMsgPos)
			}
			data, err := msg.Serialize()
			if err != nil {
				return nil, err
			}
			w.delayedMsgs = append(w.delayedMsgs, data)
			delayedMsgPos += 1
		}
		w.seqMsgs = append(w.seqMsgs, serialize([]uint64{batch.TimeBounds.MinTimestamp, batch.TimeBounds.MaxTimestamp, batch.TimeBounds.MinBlockNumber, batch.TimeBounds.MaxBlockNumber, delayedMsgPos}, batch.Data))
	}
	return &w, nil
}

func (w *Wavm) ReadInboxMessage(msgNum uint64) ([]byte, error) {
	if msgNum >= uint64(len(w.seqMsgs)) {
		return nil, fmt.Errorf("trying to read bad msg %d", msgNum)
	}
	return w.seqMsgs[msgNum], nil
}

func (w *Wavm) ReadDelayedInboxMessage(seqNum uint64) ([]byte, error) {
	if seqNum < w.delayedMsgFirstPos || (seqNum-w.delayedMsgFirstPos >= uint64(len(w.delayedMsgs))) {
		return nil, fmt.Errorf("trying to read bad delayed msg %d", seqNum)
	}
	return w.delayedMsgs[seqNum-w.delayedMsgFirstPos], nil
}

func (w *Wavm) AdvanceInboxMessage() {
	w.seqAdvanced++
}

func (w *Wavm) ResolveTypedPreimage(ty arbutil.PreimageType, hash common.Hash) ([]byte, error) {
	val, ok := w.preimages[hash]
	if !ok {
		return []byte{}, errors.New("preimage not found")
	}
	return val, nil
}

func (w *Wavm) GetPositionWithinMessage() uint64 {
	return w.posWithinMsg
}

func (w *Wavm) SetPositionWithinMessage(pos uint64) {
	w.posWithinMsg = pos
}

func (w *Wavm) GetInboxPosition() uint64 {
	return w.seqAdvanced
}
