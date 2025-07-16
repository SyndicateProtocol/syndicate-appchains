package enclave

import (
	"bytes"
	"compress/zlib"
	"crypto/ecdsa"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"strings"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/andybalholm/brotli"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/offchainlabs/nitro/arbos"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/daprovider"
)

// Wrapper around the teemodule.TeeTrustedInput to define the Hash method
type TrustedInput teemodule.TeeTrustedInput

// AccountResult struct for eth_getProof response
type AccountResult struct {
	Address      common.Address  `json:"address"`
	AccountProof []hexutil.Bytes `json:"accountProof"`
	Balance      hexutil.U256    `json:"balance"`
	CodeHash     common.Hash     `json:"codeHash"`
	Nonce        hexutil.Uint64  `json:"nonce"`
	StorageHash  common.Hash     `json:"storageHash"`
	StorageProof []StorageResult `json:"storageProof"`
}

type StorageResult struct {
	Key common.Hash `json:"key"`
	// TODO: use hexutil.U256 instead of hexutil.Big
	Value hexutil.Big     `json:"value"`
	Proof []hexutil.Bytes `json:"proof"`
}

func (input *TrustedInput) Hash() common.Hash {
	return crypto.Keccak256Hash(input.ConfigHash[:], input.AppStartBlockHash[:], input.SeqStartBlockHash[:], input.SetDelayedMessageAcc[:], input.L1StartBatchAcc[:], input.L1EndHash[:])
}

type VerifySequencingChainInput struct {
	TrustedInput    TrustedInput
	Config          Config
	DelayedMessages [][]byte
	// get this from the first delayed message event, based on SeqStartBlock.Nonce() or the corresponding batch
	// if there are no delayed messages, this must match the batch delayed message acc regardless
	StartDelayedMessagesAccumulator common.Hash
	Batches                         [][]byte
	IsL1Chain                       bool
	PreimageData                    map[arbutil.PreimageType][][]byte

	// Additional trustless input when the settlement chain is not a l1 chain
	EndBatchAccumulatorMerkleProof *AccountResult
	L1EndBlockHeader               *types.Header
}

type VerifyAppchainInput struct {
	TrustedInput    TrustedInput
	Config          Config
	DelayedMessages [][]byte
	// get this from the first delayed message event, based on AppStartBlock.Nonce()
	StartDelayedMessagesAccumulator common.Hash
	VerifySequencingChainOutput     VerifySequencingChainOutput
	// TODO: rm AppStartBlockHeader & fetch from preimage data instead
	AppStartBlockHeader types.Header
	PreimageData        map[arbutil.PreimageType][][]byte
}

type Config struct {
	SequencingContractAddress common.Address
	SequencingBridgeAddress   common.Address
	SettlementDelay           uint64
}

func (c *Config) Hash() common.Hash {
	var delay [8]byte
	binary.BigEndian.PutUint64(delay[:], c.SettlementDelay)
	return crypto.Keccak256Hash(c.SequencingContractAddress[:], c.SequencingBridgeAddress[:], delay[:])
}

type SyndicateBatch struct {
	Timestamp uint64
	Data      []byte
}

type VerifySequencingChainOutput struct {
	L1BatchAcc            common.Hash
	SequencingBlockHash   common.Hash
	SequencingBlockNumber uint64
	Batches               []SyndicateBatch
	Signature             []byte
}

func (output *VerifySequencingChainOutput) hash(input *TrustedInput) []byte {
	var data []byte
	var buffer [8]byte
	for _, batch := range output.Batches {
		binary.BigEndian.PutUint64(buffer[:], batch.Timestamp)
		data = append(data, buffer[:]...)
		binary.BigEndian.PutUint64(buffer[:], uint64(len(batch.Data)))
		data = append(data, buffer[:]...)
		data = append(data, batch.Data...)
	}
	teeHash := input.Hash()
	var startBlock [8]byte
	binary.BigEndian.PutUint64(startBlock[:], output.SequencingBlockNumber)
	return crypto.Keccak256(teeHash[:], output.L1BatchAcc[:], output.SequencingBlockHash[:], startBlock[:], data)
}

func (output *VerifySequencingChainOutput) sign(input *TrustedInput, key *ecdsa.PrivateKey) error {
	var err error
	output.Signature, err = crypto.Sign(output.hash(input), key)
	return err
}

func (output *VerifySequencingChainOutput) validate(input *TrustedInput, key *ecdsa.PublicKey) error {
	pubkey, err := crypto.SigToPub(output.hash(input), output.Signature)
	if err != nil {
		return err
	}
	if !pubkey.Equal(key) {
		return fmt.Errorf("recovered public key does not match expected value: got %s, expected %s", pubkey, key)
	}
	return nil
}

type VerifyAppchainOutput struct {
	PendingAssertion teemodule.PendingAssertion
	Signature        []byte
}

func (output *VerifyAppchainOutput) hash(input *TrustedInput) []byte {
	teeHash := input.Hash()
	return crypto.Keccak256(teeHash[:], crypto.Keccak256(output.PendingAssertion.AppBlockHash[:], output.PendingAssertion.AppSendRoot[:], output.PendingAssertion.SeqBlockHash[:], output.PendingAssertion.L1BatchAcc[:]))
}

func (output *VerifyAppchainOutput) sign(input *TrustedInput, priv *ecdsa.PrivateKey) (err error) {
	output.Signature, err = crypto.Sign(output.hash(input), priv)
	// We need to add 27 to the v value to get the correct signature
	// OpenZeppelin's ECDSA.recover expects the v-byte of a 65-byte signature to be 27 or 28,
	// but Go-Ethereum's crypto.Sign spits out 0 or 1 for that final byte.
	// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/bc8f775df2132ea5f8f7d6db9c456f46adaa957f/contracts/utils/cryptography/ECDSA.sol#L174-L175
	if len(output.Signature) != 65 {
		return fmt.Errorf("signature must be 65 bytes")
	}
	output.Signature[64] += 27
	return
}

func (output *VerifyAppchainOutput) validate(input *TrustedInput, key *ecdsa.PublicKey) error {
	if len(output.Signature) != 65 {
		return fmt.Errorf("signature must be 65 bytes")
	}
	outputSignatureCopy := make([]byte, len(output.Signature))
	copy(outputSignatureCopy, output.Signature)
	outputSignatureCopy[64] -= 27
	pubkey, err := crypto.SigToPub(output.hash(input), outputSignatureCopy)
	if err != nil {
		return err
	}
	if !pubkey.Equal(key) {
		return fmt.Errorf("recovered public key does not match expected value: got %s, expected %s", pubkey, key)
	}
	return nil
}

// TODO: make sure spurious errors eg out of memory are not returned by the zlib reader and rlp decoder
// These functions should panic if the compressed data is valid but decoding fails
func processEvent(data []byte) [][]byte {
	if len(data) == 0 {
		return nil
	}
	if data[0] == 0 {
		return [][]byte{data[1:]}
	}
	version := data[0] & 15
	if version != 8 && version != 15 {
		return nil
	}
	r, err := zlib.NewReader(bytes.NewReader(data))
	if err != nil {
		return nil
	}
	defer r.Close()
	data, err = io.ReadAll(r)
	if err != nil {
		return nil
	}
	var txs [][]byte
	if err := rlp.DecodeBytes(data, &txs); err != nil {
		return nil
	}
	return txs
}

func buildBatch(txs [][]byte, ts uint64, blockNum uint64) ([]byte, error) {
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
	if len(txs) == 1 {
		l2Message = append(l2Message, arbos.L2MessageKind_SignedTx)
		l2Message = append(l2Message, txs[0]...)
	} else {
		l2Message = append(l2Message, arbos.L2MessageKind_Batch)
		var sizeBuf [8]byte
		for _, tx := range txs {
			binary.BigEndian.PutUint64(sizeBuf[:], uint64(len(tx)+1))
			l2Message = append(l2Message, sizeBuf[:]...)
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

var TransactionProcessedEvent abi.Event

func init() {
	abi, err := abi.JSON(strings.NewReader(`[{"type":"event","name":"TransactionProcessed","inputs":[{"name":"sender","type":"address","indexed":true,"internalType":"address"},{"name":"data","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]`))
	if err != nil {
		panic(err)
	}
	TransactionProcessedEvent = abi.Events["TransactionProcessed"]
}

func (s *SyndicateAccumulator) ProcessBlock(block *types.Block, receipts types.Receipts) error {
	if s.BlockNum > 0 && s.BlockNum+1 != block.NumberU64() {
		return errors.New("unexpected block number")
	}
	s.BlockNum = block.NumberU64()
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
	// the first delayed message between the accs is only necessary if the end delayed message accs differ
	// it is used to make sure the timestamp does not fit in the previous batch
	SetFirstDelayedMessage []byte
	// this is necessary because we only track the end set delayed message acc and not the start one
	SetRemainingDelayedMessageHashes []common.Hash
	// same as the second sequencing start block header. only required if the set delayed message accs differ
	SeqFirstEndBlockHeader *types.Header
}

type CombineAppchainOutput struct {
	Input  TrustedInput
	Output VerifyAppchainOutput
}
