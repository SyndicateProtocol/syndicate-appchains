package rpcclient

import (
	"fmt"

	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

// RPCClient is a wrapper around the go-ethereum rpc.Client to add any additional methods we need.
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

func (c *RPCClient) CloseConnection() {
	c.Close()
	log.Debug().Msgf("RPC connection closed")
}
