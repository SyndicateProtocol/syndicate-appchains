package enclave

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

type BlockVerifierInput struct {
	MinTimestamp   uint64           `json:"min_timestamp"`
	MaxTimestamp   uint64           `json:"max_timestamp"`
	MinBlockNumber uint64           `json:"min_block_number"`
	MaxBlockNumber uint64           `json:"max_block_number"`
	Messages       []DelayedMessage `json:"messages"`
	Batch          Bytes            `json:"batch"`
}

type VerifyAppchainOutput struct {
	BlockVerifierInputs [][]BlockVerifierInput
}
