package rpc

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type ReceiptFetcher struct {
	client IRawRPCClient
}

func NewReceiptFetcher(client IRawRPCClient) *ReceiptFetcher {
	return &ReceiptFetcher{client: client}
}

// TODO []: Implement fallbacks / alternatives for receipt fetching
func (f *ReceiptFetcher) FetchReceipts(ctx context.Context, block *types.Block, txHashes []common.Hash) (result ethtypes.Receipts, err error) {
	hash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	err = f.client.CallContext(ctx, &result, "eth_getBlockReceipts", hash)
	if err != nil {
		return nil, err
	}
	return result, nil
}
