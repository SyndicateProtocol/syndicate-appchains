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

	"github.com/offchainlabs/nitro/arbutil"
)

type Wavm struct {
	seqMsgs            [][]byte
	posWithinMsg       uint64
	delayedMsgs        [][]byte
	delayedMsgFirstPos uint64
	preimages          map[common.Hash][]byte
	seqAdvanced        uint64
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

type ValidationInput struct {
	BlockHash    common.Hash
	PreimageData [][]byte
	Batches      [][]byte
	Messages     [][]byte
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

	w.delayedMsgs = make([][]byte, len(data.Messages))
	for i, msg := range data.Messages {
		requestId := common.Hash(msg[49 : 49+32]).Big()
		if !requestId.IsUint64() {
			return nil, errors.New("request id overflow")
		}
		if requestId.Uint64() != w.delayedMsgFirstPos+uint64(i) {
			return nil, fmt.Errorf("unexpected request id: got %d, expected %d", requestId.Uint64(), w.delayedMsgFirstPos+uint64(i))
		}
		w.delayedMsgs[i] = msg
	}

	w.seqMsgs = make([][]byte, len(data.Batches))
	delayedMsgCount := w.delayedMsgFirstPos
	for i, batch := range data.Batches {
		afterDelayedMessagesRead := binary.BigEndian.Uint64(batch[32:40])
		if afterDelayedMessagesRead < delayedMsgCount {
			return nil, errors.New("delayed message count decremented")
		}
		delayedMsgCount = afterDelayedMessagesRead
		w.preimages[crypto.Keccak256Hash(batch)] = batch
		w.seqMsgs[i] = batch
	}
	if delayedMsgCount != w.delayedMsgFirstPos+uint64(len(data.Messages)) {
		return nil, fmt.Errorf("batch delayed message count does not match delayed messages: %d != %d", delayedMsgCount, w.delayedMsgFirstPos+uint64(len(data.Messages)))
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
