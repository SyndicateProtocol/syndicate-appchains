package teetypes

import (
	"crypto/ecdsa"
	"encoding/binary"
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/offchainlabs/nitro/arbutil"
)

// Storage slot of the batch accumulator
// <https://github.com/SyndicateProtocol/nitro-contracts/blob/9a100a86242176b633a1d907e5efd41296922144/src/bridge/AbsBridge.sol#L51>
// Since the batch accumulator is a dynamic array, this slot contains the length of the array
var BATCH_ACCUMULATOR_STORAGE_SLOT = common.BigToHash(big.NewInt(7))

// Storage slot of the first element in the batch accumulator array
// Dynamic types are stored starting at the keccak256 of the original storage slot plus an offset
// This value is Keccak256("0x7")
var (
	BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT           = crypto.Keccak256Hash(BATCH_ACCUMULATOR_STORAGE_SLOT[:]).Big()
	BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE = new(big.Int).Sub(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, common.Big1)
)

// field offsets into the serialized arbostypes.L1IncomingMessage struct
const (
	DelayedMessageSenderOffset    = 13
	DelayedMessageTimestampOffset = 41
	DelayedMessageRequestIdOffset = 49
	DelayedMessageDataOffset      = 113
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

func (output *VerifySequencingChainOutput) Sign(input *TrustedInput, key *ecdsa.PrivateKey) error {
	var err error
	output.Signature, err = crypto.Sign(output.hash(input), key)
	return err
}

func (output *VerifySequencingChainOutput) Validate(input *TrustedInput, key *ecdsa.PublicKey) error {
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

func (output *VerifyAppchainOutput) Sign(input *TrustedInput, priv *ecdsa.PrivateKey) error {
	var err error
	output.Signature, err = crypto.Sign(output.hash(input), priv)
	if err != nil {
		return err
	}
	// We need to add 27 to the v value to get the correct signature
	// OpenZeppelin's ECDSA.recover expects the v-byte of a 65-byte signature to be 27 or 28,
	// but Go-Ethereum's crypto.Sign spits out 0 or 1 for that final byte.
	// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/bc8f775df2132ea5f8f7d6db9c456f46adaa957f/contracts/utils/cryptography/ECDSA.sol#L174-L175
	if len(output.Signature) != 65 {
		return fmt.Errorf("signature must be 65 bytes, is %d bytes instead", len(output.Signature))
	}
	output.Signature[64] += 27
	return nil
}

func (output *VerifyAppchainOutput) Validate(input *TrustedInput, key *ecdsa.PublicKey) error {
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
