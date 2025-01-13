package publisher

import (
	"context"
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
)

// RPCAPI defines the interface for RPC operations
// This interface is kept minimal to match current usage while allowing future expansion
type RPCAPI interface {
	BlockNumber(ctx context.Context) (uint64, error)
	BlockByNumber(ctx context.Context, number *big.Int) (*types.Block, error)
	ChainID(ctx context.Context) (*big.Int, error)
}

// GethRPCAPI implements the RPCAPI interface using ethclient.Client
type GethRPCAPI struct {
	client *ethclient.Client
}

// NewGethRPCAPI creates a new GethRPCAPI instance
// Supports both HTTP and WebSocket connections based on the URL scheme
func NewGethRPCAPI(url string) (*GethRPCAPI, error) {
	var rawClient *rpc.Client
	var err error

	switch {
	case len(url) >= 2 && url[:2] == "ws":
		rawClient, err = rpc.DialWebsocket(context.Background(), url, "")
		if err != nil {
			return nil, fmt.Errorf("failed to dial WebSocket address %s: %w", url, err)
		}
	case len(url) >= 4 && url[:4] == "http":
		rawClient, err = rpc.Dial(url)
		if err != nil {
			return nil, fmt.Errorf("failed to dial HTTP address %s: %w", url, err)
		}
	default:
		return nil, fmt.Errorf("invalid address format: %s (must start with ws or http)", url)
	}

	client := ethclient.NewClient(rawClient)
	return &GethRPCAPI{
		client: client,
	}, nil
}

// BlockNumber implements RPCAPI.BlockNumber
func (g *GethRPCAPI) BlockNumber(ctx context.Context) (uint64, error) {
	return g.client.BlockNumber(ctx)
}

// BlockByNumber implements RPCAPI.BlockByNumber
func (g *GethRPCAPI) BlockByNumber(ctx context.Context, number *big.Int) (*types.Block, error) {
	return g.client.BlockByNumber(ctx, number)
}

// ChainID implements RPCAPI.ChainID
func (g *GethRPCAPI) ChainID(ctx context.Context) (*big.Int, error) {
	return g.client.ChainID(ctx)
}

// Close closes the underlying RPC connection
func (g *GethRPCAPI) Close() {
	if g.client != nil {
		g.client.Close()
	}
}
