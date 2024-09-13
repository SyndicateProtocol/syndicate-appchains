package rpc

import (
	"fmt"

	"context"

	"github.com/SyndicateProtocol/op-translator/internal/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type RPCClient struct {
	*rpc.Client
}

func Connect(address string) (*RPCClient, error) {
	c, err := rpc.Dial(address)
	if err != nil {
		return nil, fmt.Errorf("failed to dial address %s: %w", address, err)
	}
	log.Debug().Msgf("RPC connection established: %s", address)
	return &RPCClient{Client: c}, nil
}

type IRPCClient interface {
	CloseConnection()
	GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error)
	GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error)
}

func (c *RPCClient) CloseConnection() {
	c.Close()
	log.Debug().Msgf("RPC connection closed")
}

func (c *RPCClient) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error) {
	// TODO (SEQ-137): Revisit block interface. Keeping it as flexible and simple as possible for now
	var block types.Block
	err := c.CallContext(ctx, &block, "eth_getBlockByNumber", number, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}

func (c *RPCClient) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error) {
	// TODO (SEQ-137): Revisit block interface. Keeping it as flexible and simple as possible for now
	var block types.Block
	err := c.CallContext(ctx, &block, "eth_getBlockByHash", hash, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}
