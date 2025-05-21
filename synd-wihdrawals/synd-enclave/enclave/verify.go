package enclave

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

type Address string
type Bytes string
type U256 string
type FixedBytes32 string

type DelayedMessage struct {
	Kind      uint8   `json:"kind"`
	Sender    Address `json:"sender"`
	Data      Bytes   `json:"data"`
	BaseFeeL1 U256    `json:"base_fee_l1"`
}

type Slot struct {
	SeqBlockNumber uint64       `json:"seq_block_number"`
	SeqBlockHash   FixedBytes32 `json:"seq_block_hash"`
	SetBlockNumber uint64       `json:"set_block_number"`
	SetBlockHash   FixedBytes32 `json:"set_block_hash"`
}

type MBlock struct {
	Timestamp uint64           `json:"timestamp"`
	Slot      Slot             `json:"slot"`
	Payload   *PayloadWithMsgs `json:"payload,omitempty"` // optional
}

type PayloadWithMsgs struct {
	Batch    Bytes            `json:"0"` // Tuple field index
	Messages []DelayedMessage `json:"1"`
}

type TEEInput struct {
	TrustlessInput TrustlessInput
	TrustedInput   TrustedInput
	Config         TEEConfig
}

type TrustlessInput struct {
	L1Blocks         []types.Block
	SettlementBlocks []types.Block
	SequenceBlocks   []types.Block
	PreImageData     [][]byte
}

type TrustedInput struct {
	AppchainConfigHash          common.Hash
	AppchainStartBlockHash      common.Hash
	AppchainDelayedMessagesHash common.Hash
	SeqConfigHash               common.Hash
	SeqStartBlockHash           common.Hash
	SeqDelayedMessagesHash      common.Hash
	SetStartBlockNumber         uint64
	SetEndBlockNumber           uint64
	SetEndBlockHash             common.Hash
	L1StartBlockNumber          uint64
	L1EndBlockNumber            uint64
	L1EndBlockHash              common.Hash
}

type PendingAssertion struct {
	// appchain
	LastAppchainBlockHash             common.Hash
	LastAppchainSendRoot              common.Hash
	UnusedAppchainDelayedMessagesHash common.Hash
	// sequencing chain
	LastSequencingBlockHash      common.Hash
	UnusedSeqDelayedMessagesHash common.Hash
	// settlement
	LastSettlementBlockHash common.Hash
	// l1 chain
	LastL1BlockHash common.Hash
}

type TEEOutput struct {
	PendingAssertions PendingAssertion
	Signature         []byte
}

type TEEConfig struct {
	AppchainVerifierConfig AppchainVerifierConfig
}

type AppchainVerifierConfig struct {
	ArbitrumBridgeAddress         common.Address   `json:"arbitrum_bridge_address"`
	ArbitrumInboxAddress          common.Address   `json:"arbitrum_inbox_address"`
	ArbitrumIgnoreDelayedMessages bool             `json:"arbitrum_ignore_delayed_messages"`
	AllowedSettlementAddresses    []common.Address `json:"allowed_settlement_addresses"`
	SequencingContractAddress     common.Address   `json:"sequencing_contract_address"`
	SettlementDelay               uint64           `json:"settlement_delay"`
}

type VerifierOutput struct {
	BlockHash      common.Hash
	MinTimestamp   uint64
	MaxTimestamp   uint64
	MinBlockNumber uint64
	MaxBlockNumber uint64
	Messages       []DelayedMessage
	Batch          Bytes
}

type VerifyAppchainOutput struct {
	VerifierOutput                    []VerifierOutput
	UnusedAppchainDelayedMessagesHash common.Hash
}

type SequencingChainInput struct {
	Blocks           []types.Block
	Receipts         [][]types.Receipt
	StartBlockNumber uint64
	EndBlockHash     common.Hash
}

type SettlementChainInput struct {
	Blocks           []types.Block
	Receipts         [][]types.Receipt
	StartBlockNumber uint64
	EndBlockHash     common.Hash
}
