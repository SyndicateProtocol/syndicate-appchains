package rpc

import (
	"context"
	"fmt"
	"math/big"
	"sync"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type RPCClient struct {
	client          interfaces.IETHClient
	rawClient       interfaces.IRawRPCClient
	receiptsFetcher interfaces.IReceiptsFetcher
}

// guarantees that the IRPCClient interface is implemented by RPCClient
var _ interfaces.IRPCClient = (*RPCClient)(nil)

func Connect(address string) (*RPCClient, error) {
	c, err := rpc.Dial(address)
	if err != nil {
		return nil, fmt.Errorf("failed to dial address %s: %w", address, err)
	}
	log.Debug().Msgf("RPC connection established: %s", address)

	return &RPCClient{
		client:          ethclient.NewClient(c),
		rawClient:       c,
		receiptsFetcher: sources.NewRPCReceiptsFetcher(c, nil, sources.RPCReceiptsConfig{}),
	}, nil
}

func (c *RPCClient) AsEthClient() interfaces.IETHClient {
	return c.client
}

func (c *RPCClient) CloseConnection() {
	c.client.Close()
	log.Debug().Msg("RPC connection closed")
}

func (c *RPCClient) GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error) {
	var block types.Block
	err := c.rawClient.CallContext(ctx, &block, "eth_getBlockByNumber", number, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}

func (c *RPCClient) GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error) {
	var block types.Block
	err := c.rawClient.CallContext(ctx, &block, "eth_getBlockByHash", hash, withTransactions)
	if err != nil {
		return nil, err
	}
	return block, nil
}

func (c *RPCClient) BlocksReceiptsByNumbers(ctx context.Context, numbers []string) ([]*ethtypes.Receipt, error) {
	numbersLength := len(numbers)
	receiptsChan := make(chan []*ethtypes.Receipt, numbersLength)
	errChan := make(chan error, numbersLength)

	// Fetch block and its receipts concurrently
	var wg sync.WaitGroup
	wg.Add(numbersLength)
	for _, number := range numbers {
		go func(number string) {
			defer wg.Done()

			numberInt, err := utils.HexToInt(number)
			if err != nil {
				errChan <- fmt.Errorf("failed to convert block number: %w", err)
				return
			}

			block, err := c.client.BlockByNumber(ctx, big.NewInt(int64(numberInt)))
			if err != nil {
				errChan <- fmt.Errorf("failed to get block by number: %w", err)
				return
			}

			hashes := make([]common.Hash, len(block.Transactions()))
			for i, tx := range block.Transactions() {
				hashes[i] = tx.Hash()
			}

			blockReceipts, err := c.receiptsFetcher.FetchReceipts(ctx, eth.BlockToInfo(block), hashes)
			if err != nil {
				errChan <- fmt.Errorf("failed to get receipts for block %d: %w", numberInt, err)
				return
			}

			receiptsChan <- blockReceipts
		}(number)
	}

	go func() {
		wg.Wait()
		close(receiptsChan)
		close(errChan)
	}()

	var allReceipts []*ethtypes.Receipt
	for blockReceipts := range receiptsChan {
		allReceipts = append(allReceipts, blockReceipts...)
	}

	if len(errChan) > 0 {
		return nil, <-errChan
	}

	return allReceipts, nil
}
