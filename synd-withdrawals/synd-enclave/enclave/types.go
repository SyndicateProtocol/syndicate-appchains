package enclave

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
)

// Appchain verifier inputs
type SettlementChainInput struct {
	// Trustless input
	DelayedMessages                 []arbostypes.L1IncomingMessage `json:"delayedMessages"`
	StartDelayedMessagesAccumulator common.Hash                    `json:"startDelayedMessagesAccumulator"`

	// Trusted input
	EndDelayedMessagesAccumulator common.Hash `json:"endDelayedMessagesAccumulator"`
}

type SequencingChainInput struct {
	// Trustless input
	StartSyndicateAccumulatorMerkleProof AccountProofResponse        `json:"startSyndicateAccumulatorMerkleProof"`
	EndSyndicateAccumulatorMerkleProof   AccountProofResponse        `json:"endSyndicateAccumulatorMerkleProof"`
	SyndicateTransactionEvents           []SyndicateTransactionEvent `json:"syndicateTransactionEvents"`
	BlockHeaders                         []types.Header              `json:"blockHeaders"`

	// Trusted input
	StartBlockHash common.Hash `json:"startBlockHash"`
	EndBlockHash   common.Hash `json:"endBlockHash"`
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
	BlockNumber uint64         `json:"blockNumber"`
	Timestamp   uint64         `json:"timestamp"`
	Sender      common.Address `json:"sender"`
	Payload     []byte         `json:"payload"`
}

// Sequencing chain verifier inputs
type L1ChainInput struct {
	// Trustless input
	// TODO: add types

	// Trusted input
	// TODO: add types
}

// Verify sequencing chain input & output (First call to the TEE synd-enclave)
type VerifySequencingChainInput struct {
	L1ChainInput           L1ChainInput `json:"l1ChainInput"`
	SequencingPreImageData []byte       `json:"sequencingPreImageData"`
}
type VerifySequencingChainOutput struct {
	L1SequencingBlockHash common.Hash `json:"l1SequencingBlockHash"`
	L1EndBlockHash        common.Hash `json:"l1EndBlockHash"`
	Signature             []byte      `json:"signature"`
}

// Verify appchain output (Second call to the TEE synd-enclave)
type VerifyAppchainInput struct {
	SettlementChainInput        SettlementChainInput        `json:"settlementChainInput"`
	SequencingChainInput        SequencingChainInput        `json:"sequencingChainInput"`
	AppchainPreImageData        []byte                      `json:"appchainPreImageData"`
	VerifySequencingChainOutput VerifySequencingChainOutput `json:"verifySequencingChainOutput"`
}

type VerifyAppchainOutput struct {
	L1SequencingBlockHash common.Hash `json:"l1SequencingBlockHash"`
	L1EndBlockHash        common.Hash `json:"l1EndBlockHash"`
	LastAppchainBlockHash common.Hash `json:"lastAppchainBlockHash"`
	LastAppchainSendRoot  common.Hash `json:"lastAppchainSendRoot"`
	BatchCount            uint64      `json:"batchCount"`
	Signature             []byte      `json:"signature"`
}

// Block verifier inputs
type BlockVerifierInput struct {
	MinTimestamp   uint64                         `json:"minTimestamp"`
	MaxTimestamp   uint64                         `json:"maxTimestamp"`
	MinBlockNumber uint64                         `json:"minBlockNumber"`
	MaxBlockNumber uint64                         `json:"maxBlockNumber"`
	Messages       []arbostypes.L1IncomingMessage `json:"messages"`
	Batch          []byte                         `json:"batch"`
}
