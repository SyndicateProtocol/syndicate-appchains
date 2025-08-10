package wavmio

import (
	"bytes"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/log"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
)

type WavmInbox struct {
	Wavm *Wavm
}

func (i WavmInbox) PeekSequencerInbox() ([]byte, common.Hash, error) {
	pos := i.Wavm.GetInboxPosition()
	res, err := i.Wavm.ReadInboxMessage(pos)
	if err != nil {
		return nil, common.Hash{}, err
	}
	log.Info("PeekSequencerInbox", "pos", pos, "res[:8]", res[:8])
	// Our BlobPreimageReader doesn't need the block hash
	return res, common.Hash{}, nil
}

func (i WavmInbox) GetSequencerInboxPosition() uint64 {
	pos := i.Wavm.GetInboxPosition()
	log.Info("GetSequencerInboxPosition", "pos", pos)
	return pos
}

func (i WavmInbox) AdvanceSequencerInbox() {
	log.Info("AdvanceSequencerInbox")
	i.Wavm.AdvanceInboxMessage()
}

func (i WavmInbox) GetPositionWithinMessage() uint64 {
	pos := i.Wavm.GetPositionWithinMessage()
	log.Info("GetPositionWithinMessage", "pos", pos)
	return pos
}

func (i WavmInbox) SetPositionWithinMessage(pos uint64) {
	log.Info("SetPositionWithinMessage", "pos", pos)
	i.Wavm.SetPositionWithinMessage(pos)
}

func (i WavmInbox) ReadDelayedInbox(seqNum uint64) (*arbostypes.L1IncomingMessage, error) {
	log.Info("ReadDelayedMsg", "seqNum", seqNum)
	data, err := i.Wavm.ReadDelayedInboxMessage(seqNum)
	if err != nil {
		return nil, err
	}
	return arbostypes.ParseIncomingL1Message(bytes.NewReader(data), nil)
}
