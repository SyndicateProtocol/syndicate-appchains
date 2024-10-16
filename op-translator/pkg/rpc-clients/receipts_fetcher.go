package rpc

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/ethereum-optimism/optimism/op-service/sources"
)

type ReceiptsFetcher struct {
	*sources.RPCReceiptsFetcher
}

var _ interfaces.IReceiptsFetcher = (*ReceiptsFetcher)(nil)
