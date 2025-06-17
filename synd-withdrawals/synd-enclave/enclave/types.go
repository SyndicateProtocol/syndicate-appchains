package enclave

import (
	"bytes"
	"compress/zlib"
	"crypto/ecdsa"
	"encoding/binary"
	"encoding/json"
	"errors"
	"io"

	"github.com/andybalholm/brotli"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/offchainlabs/nitro/arbos"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate"
	"github.com/offchainlabs/nitro/arbstate/daprovider"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
)

// AccountResult struct for eth_getProof response
type AccountResult struct {
	Address      common.Address  `json:"address"`
	AccountProof []string        `json:"accountProof"`
	Balance      *hexutil.Big    `json:"balance"`
	CodeHash     common.Hash     `json:"codeHash"`
	Nonce        hexutil.Uint64  `json:"nonce"`
	StorageHash  common.Hash     `json:"storageHash"`
	StorageProof []StorageResult `json:"storageProof"`
}

type StorageResult struct {
	Key   string      `json:"key"`
	Value common.Hash `json:"value"`
	Proof []string    `json:"proof"`
}

type TrustedInput struct {
	ConfigHash           common.Hash
	AppStartBlockHash    common.Hash
	SeqStartBlockHash    common.Hash
	SetDelayedMessageAcc common.Hash
	L1StartBatchAcc      common.Hash
	L1EndHash            common.Hash
}

func (input *TrustedInput) hash() common.Hash {
	return crypto.Keccak256Hash(input.ConfigHash[:], input.AppStartBlockHash[:], input.SeqStartBlockHash[:], input.SetDelayedMessageAcc[:], input.L1StartBatchAcc[:], input.L1EndHash[:])
}

type VerifySequencingChainInput struct {
	TrustedInput    TrustedInput
	Config          Config
	DelayedMessages []arbostypes.L1IncomingMessage
	// get this from the first delayed message event, based on SeqStartBlock.Nonce() or the corresponding batch
	// is ignored if there are no batches included
	StartDelayedMessagesAccumulator common.Hash
	Batches                         []ArbitrumBatch
	IsL1Chain                       bool
	PreimageData                    [][]byte

	// Additional trustless input when the settlement chain is not a l1 chain
	EndBatchAccumulatorMerkleProof *AccountResult
	L1EndBlockHeader               *types.Header
}

type VerifyAppchainInput struct {
	TrustedInput    TrustedInput
	Config          Config
	DelayedMessages []arbostypes.L1IncomingMessage
	// get this from the first delayed message event, based on AppStartBlock.Nonce()
	StartDelayedMessagesAccumulator common.Hash
	VerifySequencingChainOutput     VerifySequencingChainOutput
	// TODO: rm AppStartBlockHeader & fetch from preimage data instead
	AppStartBlockHeader types.Header
	PreimageData        [][]byte
}

type Config struct {
	SequencingContractAddress common.Address
	SequencingBridgeAddress   common.Address
	SettlementDelay           uint64
}

func (c *Config) Hash() common.Hash {
	delay := make([]byte, 8)
	binary.BigEndian.PutUint64(delay, c.SettlementDelay)
	return crypto.Keccak256Hash(c.SequencingContractAddress[:], c.SequencingBridgeAddress[:], delay)
}

type SyndicateBatch struct {
	Timestamp uint64
	Data      []byte
}

type ArbitrumBatch struct {
	DelayedAcc               common.Hash
	AfterDelayedMessagesRead uint64
	TimeBounds               bridgegen.IBridgeTimeBounds
	Data                     []byte
}

func (a *ArbitrumBatch) serialize() []byte {
	var msg []byte

	// Set header values
	for _, bound := range []uint64{
		a.TimeBounds.MinTimestamp, a.TimeBounds.MaxTimestamp, a.TimeBounds.MinBlockNumber, a.TimeBounds.MaxBlockNumber, a.AfterDelayedMessagesRead,
	} {
		var intData [8]byte
		binary.BigEndian.PutUint64(intData[:], bound)
		msg = append(msg, intData[:]...)
	}

	// Append the batch data
	msg = append(msg, a.Data...)

	return msg
}

func (a *ArbitrumBatch) accumulate(acc common.Hash) common.Hash {
	return crypto.Keccak256Hash(acc[:], crypto.Keccak256(a.serialize()), a.DelayedAcc[:])
}

type VerifySequencingChainOutput struct {
	L1BatchAcc            common.Hash
	SequencingBlockHash   common.Hash
	SequencingBlockNumber uint64
	Batches               []SyndicateBatch
	Signature             []byte
}

func (output *VerifySequencingChainOutput) hash(input *TrustedInput) ([]byte, error) {
	data, err := json.Marshal(output.Batches)
	if err != nil {
		return nil, err
	}
	teeHash := input.hash()
	startBlock := make([]byte, 8)
	binary.BigEndian.PutUint64(startBlock, output.SequencingBlockNumber)
	return crypto.Keccak256(teeHash[:], crypto.Keccak256(output.L1BatchAcc[:], output.SequencingBlockHash[:], startBlock, data)), nil
}

func (output *VerifySequencingChainOutput) sign(input *TrustedInput, key *ecdsa.PrivateKey) error {
	payload, err := output.hash(input)
	if err != nil {
		return err
	}
	output.Signature, err = crypto.Sign(payload, key)
	return err
}

func (output *VerifySequencingChainOutput) validate(input *TrustedInput, key *ecdsa.PublicKey) bool {
	payload, err := output.hash(input)
	if err != nil {
		return false
	}
	return crypto.VerifySignature(crypto.FromECDSAPub(key), payload, output.Signature)
}

type VerifyAppchainOutput struct {
	AppchainBlockHash   common.Hash
	AppchainSendRoot    common.Hash
	SequencingBlockHash common.Hash
	L1BatchAcc          common.Hash
	// can include SetStartDelayedMessageAcc as well
	Signature []byte
}

func (output *VerifyAppchainOutput) hash(input *TrustedInput) []byte {
	teeHash := input.hash()
	return crypto.Keccak256(teeHash[:], crypto.Keccak256(output.AppchainBlockHash[:], output.AppchainSendRoot[:], output.SequencingBlockHash[:], output.L1BatchAcc[:]))
}

func (output *VerifyAppchainOutput) sign(input *TrustedInput, priv *ecdsa.PrivateKey) (err error) {
	output.Signature, err = crypto.Sign(output.hash(input), priv)
	return
}

func (output *VerifyAppchainOutput) validate(input *TrustedInput, key *ecdsa.PublicKey) bool {
	return crypto.VerifySignature(crypto.FromECDSAPub(key), output.hash(input), output.Signature)
}

func processEvent(data []byte) ([][]byte, error) {
	if len(data) == 0 {
		return nil, nil
	}
	if data[0] == 0 {
		return [][]byte{data[1:]}, nil
	}
	version := data[0] & 15
	if version != 8 && version != 15 {
		return nil, nil
	}
	r, err := zlib.NewReader(bytes.NewReader(data[1:]))
	if err != nil {
		return nil, err
	}
	defer r.Close()
	data, err = io.ReadAll(r)
	if err != nil {
		// compression errors are ignored
		return nil, nil
	}
	var txs [][]byte
	// rlp decoding errors are ignored
	if err := rlp.DecodeBytes(data, &txs); err != nil {
		return nil, nil
	}
	return txs, nil
}

func buildBatch(txes [][]byte, ts uint64, blockNum uint64) ([]byte, error) {
	var data []byte

	if ts != 0 {
		segment, err := rlp.EncodeToBytes(ts)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceTimestamp}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	if blockNum != 0 {
		segment, err := rlp.EncodeToBytes(blockNum)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceL1BlockNumber}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	var l2Message []byte
	if len(txes) == 1 {
		l2Message = append(l2Message, arbos.L2MessageKind_SignedTx)
		l2Message = append(l2Message, txes[0]...)
	} else {
		l2Message = append(l2Message, arbos.L2MessageKind_Batch)
		sizeBuf := make([]byte, 8)
		for _, tx := range txes {
			binary.BigEndian.PutUint64(sizeBuf, uint64(len(tx)+1))
			l2Message = append(l2Message, sizeBuf...)
			l2Message = append(l2Message, arbos.L2MessageKind_SignedTx)
			l2Message = append(l2Message, tx...)
		}
	}
	if len(l2Message) > arbostypes.MaxL2MessageSize {
		return nil, errors.New("l2message too long")
	}
	segment, err := rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindL2Message}, l2Message...))
	if err != nil {
		return nil, err
	}
	data = append(data, segment...)

	var buffer bytes.Buffer
	writer := brotli.NewWriter(&buffer)
	lenWritten, err := writer.Write(data)
	if err != nil {
		return nil, err
	}
	if lenWritten != len(data) {
		return nil, errors.New("write failed")
	}
	if err := writer.Close(); err != nil {
		return nil, err
	}
	return append([]byte{daprovider.BrotliMessageHeaderByte}, buffer.Bytes()...), nil
}

type SyndicateAccumulator struct {
	Address  common.Address
	Batches  []SyndicateBatch
	BlockNum uint64
}

var TransactionProcessedEvent = crypto.Keccak256Hash([]byte("TransactionProcessed(address,bytes)"))

func (s *SyndicateAccumulator) ProcessBlock(block *types.Block, receipts types.Receipts) error {
	if s.BlockNum > 0 && s.BlockNum+1 != block.NumberU64() {
		return errors.New("unexpected block number")
	}
	s.BlockNum = block.NumberU64()
	var txs [][]byte
	for _, receipt := range receipts {
		for _, log := range receipt.Logs {
			if log.Address == s.Address && log.Topics[0] == TransactionProcessedEvent {
				eventTxs, err := processEvent(log.Data)
				if err != nil {
					return err
				}
				txs = append(txs, eventTxs...)
			}
		}
	}
	var data []byte
	if len(txs) > 0 {
		var err error
		data, err = buildBatch(txs, block.Time(), block.NumberU64())
		if err != nil {
			return err
		}
	}
	s.Batches = append(s.Batches, SyndicateBatch{
		Timestamp: block.Time(),
		Data:      data,
	})
	return nil
}

type CombineAppchainInput struct {
	Inputs  [2]TrustedInput
	Outputs [2]VerifyAppchainOutput
	Config  Config
	// this is necessary because we do not track the start set delayed message acc
	SetDelayedMessageAccs []common.Hash
	// the first delayed message between the accs is only necessary if the end delayed message accs differ
	// it is used to make sure the timestamp does not fit in the previous batch
	SetFirstDelayedMessage *arbostypes.L1IncomingMessage
}
