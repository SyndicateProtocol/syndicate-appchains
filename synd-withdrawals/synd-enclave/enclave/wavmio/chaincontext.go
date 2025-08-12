package wavmio

import (
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/consensus"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
	"github.com/offchainlabs/nitro/arbos"
)

type WavmChainContext struct {
	ChainConfig *params.ChainConfig
	Wavm        *Wavm
}

func (c WavmChainContext) Config() *params.ChainConfig {
	return c.ChainConfig
}

func (c WavmChainContext) Engine() consensus.Engine {
	return arbos.Engine{}
}

func (c WavmChainContext) GetHeader(hash common.Hash, num uint64) *types.Header {
	header, err := c.Wavm.GetBlockHeaderByHash(hash)
	if err != nil {
		panic(fmt.Sprintf("missing preimage data for block header hash %v", hash))
	}
	if !header.Number.IsUint64() || header.Number.Uint64() != num {
		panic(fmt.Sprintf("retrieved wrong block number for header hash %v -- requested %v but got %v", hash, num, header.Number.String()))
	}
	return header
}
