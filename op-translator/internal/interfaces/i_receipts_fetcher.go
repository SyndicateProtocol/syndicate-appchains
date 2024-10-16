package interfaces

import (
	"context"

	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type IReceiptsFetcher interface {
	PickReceiptsMethod(txCount int) sources.ReceiptsFetchingMethod
	OnReceiptsMethodErr(method sources.ReceiptsFetchingMethod, err error)
	FetchReceipts(ctx context.Context, blockInfo eth.BlockInfo, txHashes []common.Hash) (result ethtypes.Receipts, err error)
}
