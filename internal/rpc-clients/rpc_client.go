package rpc

import (
	"context"
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/op-translator/internal/types"
	"github.com/SyndicateProtocol/op-translator/internal/utils"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type IRPCClient interface {
	CloseConnection()
	HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error)
	GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error)
	GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error)
	BlocksReceiptsByNumbers(ctx context.Context, numbers []string) ([]*ethtypes.Receipt, error)
	TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error)
}

type RPCClient struct {
	*ethclient.Client             // by embedding we get all the methods of the ethclient
	rawClient         *rpc.Client // allows direct access to `CallContext`
}

// guarantees that the IRPCClient interface is implemented by RPCClient
var _ IRPCClient = (*RPCClient)(nil)

func Connect(address string) (*RPCClient, error) {
	c, err := rpc.Dial(address)
	if err != nil {
		return nil, fmt.Errorf("failed to dial address %s: %w", address, err)
	}
	log.Debug().Msgf("RPC connection established: %s", address)
	ethClient := ethclient.NewClient(c)
	return &RPCClient{Client: ethClient, rawClient: c}, nil
}

func (c *RPCClient) CloseConnection() {
	c.Close()
	log.Debug().Msgf("RPC connection closed")
}

func (c *RPCClient) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error) {
	// TODO (SEQ-137): Revisit block interface. Keeping it as flexible and simple as possible for now
	var block types.Block
	err := c.rawClient.CallContext(ctx, &block, "eth_getBlockByNumber", number, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}

func (c *RPCClient) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error) {
	// TODO (SEQ-137): Revisit block interface. Keeping it as flexible and simple as possible for now
	var block types.Block
	err := c.rawClient.CallContext(ctx, &block, "eth_getBlockByHash", hash, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}

func (c *RPCClient) BlocksReceiptsByNumbers(ctx context.Context, numbers []string) ([]*ethtypes.Receipt, error) {
	receipts := make([]*ethtypes.Receipt, 0)
	for _, number := range numbers {
		numberInt, err := utils.HexToInt(number)
		if err != nil {
			return nil, fmt.Errorf("failed to convert block number to int, err: %w", err)
		}
		blockReceipts, err := c.BlockReceipts(ctx, rpc.BlockNumberOrHashWithNumber(rpc.BlockNumber(numberInt)))
		if err != nil {
			return nil, fmt.Errorf("failed to get receipts for block number=%d, err: %w", numberInt, err)
		}
		receipts = append(receipts, blockReceipts...)
	}
	return receipts, nil
}
