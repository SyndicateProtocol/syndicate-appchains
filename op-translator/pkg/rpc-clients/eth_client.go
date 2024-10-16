package rpc

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/ethereum/go-ethereum/ethclient"
)

type ETHClient struct {
	*ethclient.Client
}

var _ interfaces.IETHClient = (*ETHClient)(nil)
