package rpc

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"sync"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type IReceiptsFetcher interface {
	FetchReceipts(ctx context.Context, block *types.Block, txHashes []common.Hash) (result ethtypes.Receipts, err error)
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
	return NewRPCClient(ethclient.NewClient(c), c, NewReceiptFetcher(c)), nil
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

func (c *RPCClient) GetReceiptsByBlocks(ctx context.Context, blocks []*types.Block) ([]*ethtypes.Receipt, error) {
	// WARNING: This function assumes that the block numbers are passed in order
	numbersLength := len(blocks)
	if numbersLength == 0 {
		return nil, nil
	}

	// Struct to hold a block and its receipts
	type BlockReceipts struct {
		block    *types.Block
		receipts []*ethtypes.Receipt
	}
	receiptsChan := make(chan BlockReceipts, numbersLength)
	errChan := make(chan error, numbersLength)

	// Fetch block and its receipts concurrently
	var wg sync.WaitGroup
	wg.Add(numbersLength)
	for _, block := range blocks {
		go func(block *types.Block) {
			defer wg.Done()

			transactions, err := block.GetTransactions()
			if err != nil {
				errChan <- fmt.Errorf("failed to get transactions: %w", err)
				return
			}

			hashes := make([]common.Hash, len(transactions))
			for i, tx := range transactions {
				hashes[i] = common.HexToHash(tx.(string))
			}

			blockNumber, err := block.GetBlockNumberHex()
			if err != nil {
				errChan <- fmt.Errorf("failed to get block number: %w", err)
				return
			}

			blockReceipts, err := c.receiptsFetcher.FetchReceipts(ctx, block, hashes)
			if err != nil {
				errChan <- fmt.Errorf("failed to get receipts for block %s: %w", blockNumber, err)
				return
			}

			receiptsChan <- BlockReceipts{
				block:    block,
				receipts: blockReceipts,
			}

		}(block)
	}

	wg.Wait()
	close(receiptsChan)
	close(errChan)

	if len(errChan) > 0 {
		var returnErr error
		for err := range errChan {
			returnErr = errors.Join(returnErr, err)
		}
		return nil, returnErr
	}

	blockReciptsMap := make(map[*types.Block][]*ethtypes.Receipt, numbersLength)
	for blockReceipts := range receiptsChan {
		blockReciptsMap[blockReceipts.block] = blockReceipts.receipts
	}

	var allReceipts []*ethtypes.Receipt
	for _, block := range blocks {
		allReceipts = append(allReceipts, blockReciptsMap[block]...)
	}
	return allReceipts, nil
}

type BlockStateCall struct {
	Calls []*ethtypes.Transaction `json:"calls"`
}

type SimulationRequest struct {
	Validation      bool             `json:"validation,omitempty"`
	BlockStateCalls []BlockStateCall `json:"blockStateCalls"`
}

func (c *RPCClient) SimulateTransactions(ctx context.Context, simulationRequest SimulationRequest, blockParameter string) error {
	var response any
	err := c.rawClient.CallContext(ctx, &response, "eth_simulateV1", simulationRequest, blockParameter)
	if err != nil {
		return err
	}
	return nil
}
