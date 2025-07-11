// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package wavmio

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"errors"
	"fmt"

	"github.com/Layr-Labs/eigenda/api/clients/codecs"
	"github.com/Layr-Labs/eigenda/encoding/rs"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/kzg4844"
	"github.com/ethereum/go-ethereum/rlp"

	"github.com/offchainlabs/nitro/arbutil"
)

var g1 []bn254.G1Affine

func Init(g1Data []byte) error {
	if len(g1Data)%32 != 0 {
		return fmt.Errorf("unexpected g1 data length: %d", len(g1Data))
	}
	g1 = make([]bn254.G1Affine, len(g1Data)/32)
	for i := range g1 {
		j := i * 32
		if read, err := g1[i].SetBytes(g1Data[j : j+32]); read != 32 || err != nil {
			return fmt.Errorf("failed to parse g1 point: read=%d, err=%w", read, err)
		}
	}
	return nil
}

type Wavm struct {
	seqMsgs            [][]byte
	posWithinMsg       uint64
	delayedMsgs        [][]byte
	delayedMsgFirstPos uint64
	Preimages          map[arbutil.PreimageType]map[common.Hash][]byte
	seqAdvanced        uint64
}

func (w *Wavm) GetBlockHeaderByHash(hash common.Hash) (*types.Header, error) {
	enc, err := w.ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
	if err != nil {
		return nil, fmt.Errorf("error resolving preimage for block %s: %w", hash, err)
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
	PreimageData map[arbutil.PreimageType][][]byte
	Batches      [][]byte
	Messages     [][]byte
}

func New(data ValidationInput) (*Wavm, error) {
	w := Wavm{Preimages: map[arbutil.PreimageType]map[common.Hash][]byte{
		arbutil.Keccak256PreimageType:        make(map[common.Hash][]byte),
		arbutil.Sha2_256PreimageType:         make(map[common.Hash][]byte),
		arbutil.EthVersionedHashPreimageType: make(map[common.Hash][]byte),
		arbutil.EigenDaPreimageType:          make(map[common.Hash][]byte),
	}}

	// matches the solidity code in executeReadPreImage() from src/osp/OneStepProverHostIo.sol
	for ty, preimages := range data.PreimageData {
		for _, preimage := range preimages {
			var key common.Hash
			switch ty {
			case arbutil.Keccak256PreimageType:
				key = crypto.Keccak256Hash(preimage)
			case arbutil.Sha2_256PreimageType:
				// This is unused currently
				key = sha256.Sum256(preimage)
			case arbutil.EthVersionedHashPreimageType:
				// kgz4844.BlobToCommitment uses modified SRS (structured reference string) G1 points that are used to
				// calculate the commitment for data encoded in lagrange (point-value) form.
				//
				// This allows individual words of the preimage to be verified against the blob hash via a point evaluation
				// proof without requiring an ifft transform to convert the preimage data from lagrange to standard coefficient
				// form prior to calculating the commitment hash.
				c, err := kzg4844.BlobToCommitment((*kzg4844.Blob)(preimage))
				if err != nil {
					return nil, err
				}
				key = kzg4844.CalcBlobHashV1(sha256.New(), &c)
			case arbutil.EigenDaPreimageType:
				// without padding, there can be collisions
				n := len(preimage)
				if n < 32 || n&(n-1) != 0 {
					return nil, fmt.Errorf("eigenda preimage missing padding: len=%d", n)
				}

				// Convert from lagrange (point-value) representation to standard polynomial coefficients via an ifft transform.
				//
				// This allows individual words of the preimage to be verified against the commitment hash via a point
				// evaluation proof at a power of the primitive root of unity, no SRS G1 points (powers of tau) necessary.
				//
				// The basic idea is: evaluating a polynomial at powers of a primitive root of unity is a fourier transform,
				// and fft(iff(preimage data)) = preimage data.
				//
				// For our use case, point evaluation is not useful because we want to verify the entire preimage data
				// for the commitment hash instead of a 32-byte word.
				data, err := codecs.IFFT(preimage)
				if err != nil {
					return nil, err
				}
				p, err := rs.ToFrArray(data)
				if err != nil {
					return nil, err
				}
				if len(g1) < len(p) {
					return nil, fmt.Errorf("failed to generate eigenda blob commitment: too few srs g1 points: have %d, need %d", len(g1), len(p))
				}
				c, err := new(bn254.G1Affine).MultiExp(g1[:len(p)], p, ecc.MultiExpConfig{})
				if err != nil {
					return nil, err
				}
				var dataLength [4]byte
				binary.BigEndian.PutUint32(dataLength[:], uint32(len(p)))
				key = crypto.Keccak256Hash(bytes.TrimLeft(c.X.Marshal(), "\x00"), bytes.TrimLeft(c.Y.Marshal(), "\x00"), dataLength[:])
			default:
				return nil, fmt.Errorf("unknown preimage type: %d", ty)
			}
			w.Preimages[ty][key] = preimage
		}
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
		w.Preimages[arbutil.Keccak256PreimageType][crypto.Keccak256Hash(batch)] = batch
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
	preimages := w.Preimages[ty]
	if preimages == nil {
		return nil, fmt.Errorf("preimage type %d not found", ty)
	}
	val, ok := preimages[hash]
	if !ok {
		return []byte{}, fmt.Errorf("preimage not found: %s", hash)
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
