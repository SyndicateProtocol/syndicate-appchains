package rpc

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type ReceiptsFetcher struct {
	*sources.RPCReceiptsFetcher
}

var _ interfaces.IReceiptsFetcher = (*ReceiptsFetcher)(nil)

// Some Nethermind and Alchemy RPC endpoints require an object to identify a block, instead of a string.
type blockHashParameter struct {
	BlockHash common.Hash `json:"blockHash"`
}
type receiptsWrapper struct {
	Receipts []*ethtypes.Receipt `json:"receipts"`
}
