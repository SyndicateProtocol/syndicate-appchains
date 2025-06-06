package enclave

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
)

// Appchain verifier inputs
type SettlementChainInput struct {
	// Trustless input
	DelayedMessages                 []arbostypes.L1IncomingMessage
	StartDelayedMessagesAccumulator common.Hash

	// Trusted input
	EndDelayedMessagesAccumulator common.Hash
}

type SequencingChainInput struct {
	// Trustless input
	StartSyndicateAccumulatorMerkleProof AccountProofResponse
	EndSyndicateAccumulatorMerkleProof   AccountProofResponse
	SyndicateTransactionEvents           []SyndicateTransactionEvent
	BlockHeaders                         []types.Header

	// Trusted input
	StartBlockHash common.Hash
	EndBlockHash   common.Hash
}

type AccountProofResponse struct {
	Address      common.Address  `json:"address"`
	Balance      *big.Int        `json:"balance"`
	CodeHash     common.Hash     `json:"codeHash"`
	Nonce        uint64          `json:"nonce"`
	StorageHash  common.Hash     `json:"storageHash"`
	AccountProof []string        `json:"accountProof"`
	StorageProof []StorageResult `json:"storageProof"`
}

type StorageResult struct {
	Key   common.Hash `json:"key"`
	Value *big.Int    `json:"value"`
	Proof []string    `json:"proof"`
}

type SyndicateTransactionEvent struct {
	BlockNumber uint64
	Timestamp   uint64
	Sender      common.Address
	Payload     []byte
}

// Sequencing chain verifier inputs
type L1ChainInput struct {
	// Trustless input
	StartBatchAccumulatorMerkleProof AccountProofResponse
	EndBatchAccumulatorMerkleProof   AccountProofResponse
	StartBlockHeader                 types.Header
	EndBlockHeader                   types.Header
	DelayedMessages                  []arbostypes.L1IncomingMessage
	Batches                          []ArbitrumBatch

	// Trusted input
	StartBlockHash common.Hash
	EndBlockHash   common.Hash
}

type ArbitrumBatch struct {
	DelayedAccumulator       common.Hash
	AfterDelayedMessagesRead uint64
	TimeBounds               TimeBounds
	Data                     []byte
}

type TimeBounds struct {
	MinTimestamp   uint64
	MaxTimestamp   uint64
	MinBlockNumber uint64
	MaxBlockNumber uint64
}

// Verify sequencing chain input & output (First call to the TEE synd-enclave)
type VerifySequencingChainConfig struct {
	ArbitrumBridgeAddress common.Address
}
type VerifySequencingChainInput struct {
	SeqConfigHash               common.Hash
	VerifySequencingChainConfig VerifySequencingChainConfig
	L1ChainInput                L1ChainInput
	SequencingStartBlockHash    common.Hash
	SequencingPreImageData      [][]byte
}

func (input *VerifySequencingChainInput) hash() common.Hash {
	return crypto.Keccak256Hash(input.SeqConfigHash[:], input.L1ChainInput.StartBlockHash[:], input.L1ChainInput.EndBlockHash[:], input.SequencingStartBlockHash[:])

}

type VerifySequencingChainOutput struct {
	SequencingBlockHash common.Hash
	Signature           []byte
}

func SanitizeVerifySequencingChainInput(input *VerifySequencingChainInput) {
	if input.L1ChainInput.Batches == nil {
		input.L1ChainInput.Batches = []ArbitrumBatch{}
	}
	if input.L1ChainInput.DelayedMessages == nil {
		input.L1ChainInput.DelayedMessages = []arbostypes.L1IncomingMessage{}
	}
	sanitizeAccountProof(&input.L1ChainInput.StartBatchAccumulatorMerkleProof)
	sanitizeAccountProof(&input.L1ChainInput.EndBatchAccumulatorMerkleProof)
}

// Verify appchain input & output (Second call to the TEE synd-enclave)

type VerifyAppchainConfig struct {
	SequencingContractAddress common.Address
	SettlementDelay           uint64
}

type VerifyAppchainInput struct {
	SeqConfigHash               common.Hash
	L1StartBlockHash            common.Hash
	L1EndBlockHash              common.Hash
	AppchainConfigHash          common.Hash
	VerifyAppchainConfig        VerifyAppchainConfig
	SettlementChainInput        SettlementChainInput
	SequencingChainInput        SequencingChainInput
	AppchainPreImageData        [][]byte
	VerifySequencingChainOutput VerifySequencingChainOutput
	AppchainStartBlockHash      common.Hash
}

func (input *VerifyAppchainInput) hash() common.Hash {
	return crypto.Keccak256Hash(input.AppchainConfigHash[:], input.AppchainStartBlockHash[:], input.SeqConfigHash[:], input.SequencingChainInput.StartBlockHash[:], input.SequencingChainInput.EndBlockHash[:], input.SequencingChainInput.StartBlockHash[:], input.SettlementChainInput.EndDelayedMessagesAccumulator[:], input.L1StartBlockHash[:], input.L1EndBlockHash[:])
}

func SanitizeVerifyAppchainInput(input *VerifyAppchainInput) {
	// Settlement
	if input.SettlementChainInput.DelayedMessages == nil {
		input.SettlementChainInput.DelayedMessages = []arbostypes.L1IncomingMessage{}
	}

	// Sequencing
	if input.SequencingChainInput.SyndicateTransactionEvents == nil {
		input.SequencingChainInput.SyndicateTransactionEvents = []SyndicateTransactionEvent{}
	}
	if input.SequencingChainInput.BlockHeaders == nil {
		input.SequencingChainInput.BlockHeaders = []types.Header{}
	}
	sanitizeAccountProof(&input.SequencingChainInput.StartSyndicateAccumulatorMerkleProof)
	sanitizeAccountProof(&input.SequencingChainInput.EndSyndicateAccumulatorMerkleProof)
}

func sanitizeAccountProof(proof *AccountProofResponse) {
	if proof.Balance == nil {
		proof.Balance = big.NewInt(0)
	}
	if proof.AccountProof == nil {
		proof.AccountProof = []string{}
	}
	if proof.StorageProof == nil {
		proof.StorageProof = []StorageResult{}
	}
	for i := range proof.StorageProof {
		if proof.StorageProof[i].Value == nil {
			proof.StorageProof[i].Value = big.NewInt(0)
		}
		if proof.StorageProof[i].Proof == nil {
			proof.StorageProof[i].Proof = []string{}
		}
	}
}

type VerifyAppchainOutput struct {
	SequencingBlockHash common.Hash
	AppchainBlockHash   common.Hash
	AppchainSendRoot    common.Hash
	Signature           []byte
}

// Block verifier inputs
type BlockVerifierInput struct {
	MinTimestamp   uint64
	MaxTimestamp   uint64
	MinBlockNumber uint64
	MaxBlockNumber uint64
	Messages       []arbostypes.L1IncomingMessage
	Batch          []byte
}
