package enclave

import (
	"encoding/binary"
	"errors"
	"fmt"
	"strings"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teetypes"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/offchainlabs/nitro/arbcompress"
	"github.com/offchainlabs/nitro/arbos"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate"
	"github.com/offchainlabs/nitro/daprovider"
)

var allowedSeqMsgs = map[byte]struct{}{
	arbos.L2MessageKind_UnsignedUserTx: {},
	arbos.L2MessageKind_ContractTx:     {},
	arbos.L2MessageKind_Batch:          {},
	arbos.L2MessageKind_SignedTx:       {},
}

// TODO: make sure spurious errors eg out of memory are not returned by the brotli reader and rlp decoder
// These functions should panic if the compressed data is valid but decoding fails
func processEvent(data []byte) [][]byte {
	if len(data) == 0 {
		return nil
	}
	if _, ok := allowedSeqMsgs[data[0]]; !ok {
		panic(fmt.Errorf("unexpected event header byte: %d", data[0]))
	}
	if data[0] != arbos.L2MessageKind_Batch {
		if len(data) > arbostypes.MaxL2MessageSize {
			return nil
		}
		if data[0] == arbos.L2MessageKind_SignedTx && !isSignedTxValid(data[1:]) {
			return nil
		}
		// The sequencing contract ensures that unsigned transactions are valid
		return [][]byte{data}
	}
	raw, err := arbcompress.Decompress(data[1:], arbostypes.MaxL2MessageSize)
	if err != nil {
		return nil
	}
	// ignore data with a compression ratio > 10 to prevent spam
	if len(raw) > (len(data)-1)*10 {
		return nil
	}
	var txs [][]byte
	if err := rlp.DecodeBytes(raw, &txs); err != nil {
		return nil
	}
	var out [][]byte
	for _, tx := range txs {
		if len(tx) < arbostypes.MaxL2MessageSize && isSignedTxValid(tx) {
			out = append(out, append([]byte{arbos.L2MessageKind_SignedTx}, tx...))
		}
	}
	return out
}

func isSignedTxValid(data []byte) bool {
	var tx types.Transaction
	if err := tx.UnmarshalBinary(data); err != nil {
		return false
	}
	if tx.Type() >= types.ArbitrumDepositTxType || tx.Type() == types.BlobTxType {
		// Should be unreachable for Arbitrum types due to UnmarshalBinary not accepting Arbitrum internal txs
		// and we want to disallow BlobTxType since Arbitrum doesn't support EIP-4844 txs yet.
		return false
	}
	return true
}

func buildL2MessageSegment(txs [][]byte) ([]byte, error) {
	var l2Message []byte
	if len(txs) == 1 {
		l2Message = txs[0]
	} else {
		l2Message = []byte{arbos.L2MessageKind_Batch}
		var sizeBuf [8]byte
		for _, tx := range txs {
			binary.BigEndian.PutUint64(sizeBuf[:], uint64(len(tx)))
			l2Message = append(l2Message, sizeBuf[:]...)
			l2Message = append(l2Message, tx...)
		}
	}
	if len(l2Message) > arbostypes.MaxL2MessageSize {
		return nil, errors.New("l2message too long")
	}
	return rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindL2Message}, l2Message...))
}

const TX_PER_BLOCK = 100

func buildBatch(txs [][]byte, l1BlockNum uint64, l1BlockTimestamp uint64) ([]byte, error) {
	var data []byte

	if l1BlockTimestamp != 0 {
		segment, err := rlp.EncodeToBytes(l1BlockTimestamp)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceTimestamp}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	// TODO is this correct? why not apply this segment after the hardfork?
	// if l1BlockNum != 0 && l1BlockTimestamp < L1_BLOCK_NUM_HARDFORK_TS {
	if l1BlockNum != 0 {
		segment, err := rlp.EncodeToBytes(l1BlockNum)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceL1BlockNumber}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	var batchTxs [][]byte
	size := 1
	for _, tx := range txs {
		txSize := len(tx) + 8
		size += txSize
		if len(batchTxs) >= TX_PER_BLOCK || (len(batchTxs) > 0 && size > arbostypes.MaxL2MessageSize) {
			segment, err := buildL2MessageSegment(batchTxs)
			if err != nil {
				return nil, err
			}
			data = append(data, segment...)
			batchTxs = nil
			size = 1 + txSize
		}
		batchTxs = append(batchTxs, tx)
	}
	if len(batchTxs) > 0 {
		segment, err := buildL2MessageSegment(batchTxs)
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	// 0 is the fastest brotli compression level
	buffer, err := arbcompress.CompressLevel(data, 0)
	if err != nil {
		return nil, err
	}
	return append([]byte{daprovider.BrotliMessageHeaderByte}, buffer...), nil
}

type SyndicateAccumulator struct {
	Address     common.Address
	Batches     []teetypes.SyndicateBatch
	SeqBlockNum uint64
}

var TransactionProcessedEvent abi.Event

func init() {
	abi, err := abi.JSON(strings.NewReader(`[{"type":"event","name":"TransactionProcessed","inputs":[{"name":"sender","type":"address","indexed":true,"internalType":"address"},{"name":"data","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]`))
	if err != nil {
		panic(err)
	}
	TransactionProcessedEvent = abi.Events["TransactionProcessed"]
}

func (s *SyndicateAccumulator) ProcessBlock(block *types.Block, receipts types.Receipts, l1BlockNum uint64, timestamp uint64) error {
	if s.SeqBlockNum > 0 && s.SeqBlockNum+1 != block.NumberU64() {
		return errors.New("unexpected block number")
	}
	s.SeqBlockNum = block.NumberU64()
	var txs [][]byte
	for _, receipt := range receipts {
		for _, log := range receipt.Logs {
			if log.Address == s.Address && log.Topics[0] == TransactionProcessedEvent.ID {
				args, err := TransactionProcessedEvent.Inputs.Unpack(log.Data)
				if err != nil {
					return fmt.Errorf("failed to decode event: %w", err)
				}
				if len(args) != 1 {
					return errors.New("failed to decode event: unexpected number of args")
				}
				data, ok := args[0].([]byte)
				if !ok {
					return errors.New("failed to decode event: arg0 is not []byte")
				}
				txs = append(txs, processEvent(data)...)
			}
		}
	}
	var data []byte
	if len(txs) > 0 {
		var err error
		data, err = buildBatch(txs, l1BlockNum, timestamp)
		if err != nil {
			return err
		}
	}
	s.Batches = append(s.Batches, teetypes.SyndicateBatch{
		Timestamp:     block.Time(),
		Data:          data,
		L1BlockNumber: l1BlockNum,
	})
	return nil
}
