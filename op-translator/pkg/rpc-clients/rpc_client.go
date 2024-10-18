package rpc

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"sync"

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

type IReceiptsFetcher interface {
	FetchReceipts(ctx context.Context, blockInfo eth.BlockInfo, txHashes []common.Hash) (result ethtypes.Receipts, err error)
}

type IRawRPCClient interface {
	CallContext(ctx context.Context, result any, method string, args ...any) error
}

type IETHClient interface {
	BlockReceipts(ctx context.Context, blockNrOrHash rpc.BlockNumberOrHash) ([]*ethtypes.Receipt, error)
	BlockByNumber(ctx context.Context, number *big.Int) (*ethtypes.Block, error)
	HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error)
	TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error)
	Close()
}

type RPCClient struct {
	client          IETHClient
	rawClient       IRawRPCClient
	receiptsFetcher IReceiptsFetcher
}

func NewRPCClient(client IETHClient, rawClient IRawRPCClient, receiptsFetcher IReceiptsFetcher) *RPCClient {
	return &RPCClient{
		client:          client,
		rawClient:       rawClient,
		receiptsFetcher: receiptsFetcher,
	}
}

func Connect(address string) (*RPCClient, error) {
	c, err := rpc.Dial(address)
	if err != nil {
		return nil, fmt.Errorf("failed to dial address %s: %w", address, err)
	}
	log.Debug().Msgf("RPC connection established: %s", address)
	return NewRPCClient(ethclient.NewClient(c), c, sources.NewRPCReceiptsFetcher(c, nil, sources.RPCReceiptsConfig{})), nil
}

func (c *RPCClient) AsEthClient() IETHClient {
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

func (c *RPCClient) BlocksReceiptsByNumbers(ctx context.Context, blockNumbers []string) ([]*ethtypes.Receipt, error) {
	// WARNING: This function assumes that the block numbers are passed in order
	numbersLength := len(blockNumbers)
	type BlockReceipts struct {
		blockNumber string
		receipts    []*ethtypes.Receipt
	}
	receiptsChan := make(chan BlockReceipts, numbersLength)
	errChan := make(chan error, numbersLength)

	// Fetch block and its receipts concurrently
	var wg sync.WaitGroup
	wg.Add(numbersLength)
	for _, number := range blockNumbers {
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

			receiptsChan <- BlockReceipts{
				blockNumber: number,
				receipts:    blockReceipts}

		}(number)
	}

	go func() {
		wg.Wait()
		close(receiptsChan)
		close(errChan)
	}()

	blockReciptsMap := make(map[string][]*ethtypes.Receipt, numbersLength)
	for blockReceipts := range receiptsChan {
		blockReciptsMap[blockReceipts.blockNumber] = blockReceipts.receipts
	}

	var allReceipts []*ethtypes.Receipt
	for _, blockNumber := range blockNumbers {
		allReceipts = append(allReceipts, blockReciptsMap[blockNumber]...)
	}

	if len(errChan) > 0 {
		var returnErr error
		for err := range errChan {
			returnErr = errors.Join(returnErr, err)
		}
		return nil, returnErr
	}

	return allReceipts, nil
}
