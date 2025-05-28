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
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rlp"

	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbutil"
)

var (
	seqMsgs            [][]byte
	posWithinMsg       uint64
	delayedMsgs        [][]byte
	delayedMsgFirstPos uint64
	lastBlockHash      common.Hash
	preimages          map[common.Hash][]byte
	seqAdvanced        uint64
)

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

func GetBlockHeaderByHash(hash common.Hash) (*types.Header, error) {
	enc, err := ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
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
	MinTimestamp   uint64
	MaxTimestamp   uint64
	MinBlockNumber uint64
	MaxBlockNumber uint64
	Batch          []byte
	Messages       []arbostypes.L1IncomingMessage
}

type ValidationInput struct {
	BlockHash    common.Hash
	PreimageData [][]byte
	Batches      []Batch
}

func StubInit(data ValidationInput) error {
	lastBlockHash = data.BlockHash

	preimages = make(map[common.Hash][]byte)
	for _, preimage := range data.PreimageData {
		preimages[crypto.Keccak256Hash(preimage)] = preimage
	}

	header, err := GetBlockHeaderByHash(lastBlockHash)
	if err != nil {
		return err
	}
	delayedMsgFirstPos = types.NewBlockWithHeader(header).Nonce()

	delayedMsgPos := delayedMsgFirstPos
	for _, batch := range data.Batches {
		for _, msg := range batch.Messages {
			if msg.BatchGasCost != nil {
				return errors.New("batch gas cost is a computed property and must be nil")
			}
			if msg.Header.RequestId.Big().Uint64() != delayedMsgPos {
				return fmt.Errorf("unexpected request id: got %d, expected %d", msg.Header.RequestId.Big().Uint64(), delayedMsgPos)
			}
			data, err := msg.Serialize()
			if err != nil {
				return err
			}
			delayedMsgs = append(delayedMsgs, data)
			delayedMsgPos += 1
		}
		seqMsgs = append(seqMsgs, serialize([]uint64{batch.MinTimestamp, batch.MaxTimestamp, batch.MinBlockNumber, batch.MaxBlockNumber, delayedMsgPos}, batch.Batch))
	}
	return nil
}

func StubFinal() {
	log.Info("End state", "lastblockHash", lastBlockHash, "InboxPosition", seqAdvanced, "positionWithinMessage", posWithinMsg)
}

func GetLastBlockHash() (hash common.Hash) {
	return lastBlockHash
}

func ReadInboxMessage(msgNum uint64) ([]byte, error) {
	if msgNum >= uint64(len(seqMsgs)) {
		return nil, fmt.Errorf("trying to read bad msg %d", msgNum)
	}
	return seqMsgs[msgNum], nil
}

func ReadDelayedInboxMessage(seqNum uint64) ([]byte, error) {
	if seqNum < delayedMsgFirstPos || (seqNum-delayedMsgFirstPos >= uint64(len(delayedMsgs))) {
		return nil, fmt.Errorf("trying to read bad delayed msg %d", seqNum)
	}
	return delayedMsgs[seqNum-delayedMsgFirstPos], nil
}

func AdvanceInboxMessage() {
	seqAdvanced++
}

func ResolveTypedPreimage(ty arbutil.PreimageType, hash common.Hash) ([]byte, error) {
	val, ok := preimages[hash]
	if !ok {
		return []byte{}, errors.New("preimage not found")
	}
	return val, nil
}

func SetLastBlockHash(hash [32]byte) {
	lastBlockHash = hash
}

func SetSendRoot(hash [32]byte) {
}

func GetPositionWithinMessage() uint64 {
	return posWithinMsg
}

func SetPositionWithinMessage(pos uint64) {
	posWithinMsg = pos
}

func GetInboxPosition() uint64 {
	return seqAdvanced
}
