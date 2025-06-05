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
	DelayedMessages                 []arbostypes.L1IncomingMessage `json:"delayed_messages"`
	StartDelayedMessagesAccumulator common.Hash                    `json:"start_delayed_messages_accumulator"`

	// Trusted input
	EndDelayedMessagesAccumulator common.Hash `json:"end_delayed_messages_accumulator"`
}

type SequencingChainInput struct {
	// Trustless input
	StartSyndicateAccumulatorMerkleProof AccountProofResponse        `json:"start_syndicate_accumulator_merkle_proof"`
	EndSyndicateAccumulatorMerkleProof   AccountProofResponse        `json:"end_syndicate_accumulator_merkle_proof"`
	SyndicateTransactionEvents           []SyndicateTransactionEvent `json:"syndicate_transaction_events"`
	BlockHeaders                         []types.Header              `json:"block_headers"`

	// Trusted input
	StartBlockHash common.Hash `json:"start_block_hash"`
	EndBlockHash   common.Hash `json:"end_block_hash"`
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
	StartBatchCountMerkleProof      AccountProofResponse           `json:"startBatchCountMerkleProof"`
	EndBatchCountMerkleProof        AccountProofResponse           `json:"endBatchCountMerkleProof"`
	EndBatchAccumulatorMerkleProof  AccountProofResponse           `json:"endBatchAccumulatorMerkleProof"`
	StartDelayedMessagesAccumulator common.Hash                    `json:"startDelayedMessagesAccumulator"`
	StartBlockHeader                types.Header                   `json:"startBlockHeader"`
	EndBlockHeader                  types.Header                   `json:"endBlockHeader"`
	DelayedMessages                 []arbostypes.L1IncomingMessage `json:"delayedMessages"`
	Batches                         []ArbitrumBatch                `json:"batches"`

	// Trusted input
	StartBlockHash common.Hash `json:"startBlockHash"`
	EndBlockHash   common.Hash `json:"endBlockHash"`
}

type ArbitrumBatch struct {
	BatchNumber uint64         `json:"batchNumber"`
	Timestamp   uint64         `json:"timestamp"`
	Sender      common.Address `json:"sender"`
	Data        []byte         `json:"payload"`
}

// Verify sequencing chain input & output (First call to the TEE synd-enclave)
type VerifySequencingChainConfig struct {
	ArbitrumBridgeAddress common.Address `json:"arbitrumBridgeAddress"`
}
type VerifySequencingChainInput struct {
	VerifySequencingChainConfig VerifySequencingChainConfig `json:"verifySequencingChainConfig"`
	L1ChainInput                L1ChainInput                `json:"l1ChainInput"`
	SequencingPreImageData      []byte                      `json:"sequencingPreImageData"`
}
type VerifySequencingChainOutput struct {
	L1SequencingBlockHash common.Hash `json:"l1SequencingBlockHash"`
	L1EndBlockHash        common.Hash `json:"l1EndBlockHash"`
	Signature             []byte      `json:"signature"`
}

// Verify appchain input & output (Second call to the TEE synd-enclave)

type VerifyAppchainConfig struct {
	SequenceContractAddress common.Address `json:"sequencing_contract_address"`
	SettlementDelay         uint64         `json:"settlement_delay"`
}

type VerifyAppchainInput struct {
	AppchainConfigHash          common.Hash                 `json:"appchainConfigHash"`
	VerifyAppchainConfig        VerifyAppchainConfig        `json:"verifyAppchainConfig"`
	SettlementChainInput        SettlementChainInput        `json:"settlementChainInput"`
	SequencingChainInput        SequencingChainInput        `json:"sequencingChainInput"`
	AppchainPreImageData        [][]byte                    `json:"appchainPreImageData"`
	VerifySequencingChainOutput VerifySequencingChainOutput `json:"verifySequencingChainOutput"`
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
