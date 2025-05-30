package enclave

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
)

type SettlementChainInput struct {
	// Trustless input
	DelayedMessages                 []arbostypes.L1IncomingMessage `json:"delayedMessages"`
	StartDelayedMessagesAccumulator common.Hash                    `json:"startDelayedMessagesAccumulator"`

	// Trusted input
	EndDelayedMessagesAccumulator common.Hash `json:"endDelayedMessagesAccumulator"`
}

type SequencingChainInput struct {
	// Trustless input
	StartSyndicateAccumulatorMerkleProof EIP1186AccountProofResponse `json:"startSyndicateAccumulatorMerkleProof"`
	EndSyndicateAccumulatorMerkleProof   EIP1186AccountProofResponse `json:"endSyndicateAccumulatorMerkleProof"`
	SyndicateTransactionEvents           []SyndicateTransactionEvent `json:"syndicateTransactionEvents"`
	BlockHeaders                         []Header                    `json:"blockHeaders"`

	// Trusted input
	StartBlockHash common.Hash `json:"startBlockHash"`
	EndBlockHash   common.Hash `json:"endBlockHash"`
}
