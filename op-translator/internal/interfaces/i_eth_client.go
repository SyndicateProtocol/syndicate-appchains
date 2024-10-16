package interfaces

import (
	"context"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rpc"
)

type IETHClient interface {
	BlockReceipts(ctx context.Context, blockNrOrHash rpc.BlockNumberOrHash) ([]*ethtypes.Receipt, error)
	BlockByNumber(ctx context.Context, number *big.Int) (*ethtypes.Block, error)
	HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error)
	TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error)
	Close()
}
