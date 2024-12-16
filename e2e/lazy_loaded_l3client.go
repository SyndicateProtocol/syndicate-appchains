package e2e_test

import (
	"context"

	rpc "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
)

// TODO remove if possible, this only exists to resolve a dependency loop:
// op-translator needs an L3 client
// `initOpStack` needs op-translator

type lazyLoadedL3Client struct {
	c           translator.IRPCClient
	clientSetCh chan bool
}

var _ translator.IRPCClient = (*lazyLoadedL3Client)(nil)

func newLazyLoadedL3Client() *lazyLoadedL3Client {
	return &lazyLoadedL3Client{
		clientSetCh: make(chan bool),
	}
}

func (l *lazyLoadedL3Client) waitUntilClientIsSet() {
	// blocks until `setEthClient` is called
	if l.c != nil {
		return
	}
	<-l.clientSetCh
}

func (l *lazyLoadedL3Client) setEthClient(c *ethclient.Client) {
	l.c = rpc.NewRPCClient(c, c.Client(), rpc.NewReceiptFetcher(c.Client()))
	l.clientSetCh <- true
}

func (l *lazyLoadedL3Client) AsEthClient() rpc.IETHClient {
	l.waitUntilClientIsSet()
	return l.c.AsEthClient()
}

func (l *lazyLoadedL3Client) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error) {
	l.waitUntilClientIsSet()
	return l.c.GetBlockByHash(ctx, hash, withTransactions)
}

func (l *lazyLoadedL3Client) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error) {
	l.waitUntilClientIsSet()
	return l.c.GetBlockByNumber(ctx, number, withTransactions)
}

func (l *lazyLoadedL3Client) GetReceiptsByBlocks(ctx context.Context, blocks []*types.Block) ([]*ethtypes.Receipt, error) {
	l.waitUntilClientIsSet()
	return l.c.GetReceiptsByBlocks(ctx, blocks)
}

func (l *lazyLoadedL3Client) SimulateTransactions(ctx context.Context, transactions []*rpc.ParsedTransaction, blockParameter string) (any, error) {
	l.waitUntilClientIsSet()
	return l.c.SimulateTransactions(ctx, transactions, blockParameter)
}
