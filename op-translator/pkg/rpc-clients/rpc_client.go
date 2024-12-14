package rpc

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"sync"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"

	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
)

type IReceiptsFetcher interface {
	FetchReceipts(ctx context.Context, block *types.Block, txHashes []common.Hash) (result ethtypes.Receipts, err error)
}

type IRawRPCClient interface {
	CallContext(ctx context.Context, result any, method string, args ...any) error
}

type IETHClient interface {
	BlockNumber(ctx context.Context) (uint64, error)
	BlockReceipts(ctx context.Context, blockNrOrHash rpc.BlockNumberOrHash) ([]*ethtypes.Receipt, error)
	BlockByNumber(ctx context.Context, number *big.Int) (*ethtypes.Block, error)
	HeaderByNumber(ctx context.Context, number *big.Int) (*ethtypes.Header, error)
	TransactionReceipt(ctx context.Context, hash common.Hash) (*ethtypes.Receipt, error)
	ChainID(ctx context.Context) (*big.Int, error)
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
	var c *rpc.Client
	var err error
	switch {
	case address[:2] == "ws":
		// Use DialWebsocket for WebSocket connections
		c, err = rpc.DialWebsocket(context.Background(), address, "")
		if err != nil {
			return nil, fmt.Errorf("failed to dial WebSocket address %s: %w", address, err)
		}
	case address[:4] == "http":
		// Use Dial for HTTP connections
		c, err = rpc.Dial(address)
		if err != nil {
			return nil, fmt.Errorf("failed to dial HTTP address %s: %w", address, err)
		}
	default:
		return nil, fmt.Errorf("invalid address format: %s (must start with ws or http)", address)
	}

	return NewRPCClient(ethclient.NewClient(c), c, NewReceiptFetcher(c)), nil
}

func (c *RPCClient) AsEthClient() IETHClient {
	return c.client
}

func (c *RPCClient) CloseConnection() {
	c.client.Close()
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
				hashes[i] = common.HexToHash(tx.(string)) //nolint:errcheck // safe to cast to string
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

type ParsedTransaction struct {
	From                 *common.Address `json:"from"`
	To                   *common.Address `json:"to"`
	Gas                  *hexutil.Uint64 `json:"gas"`
	GasPrice             *hexutil.Big    `json:"gasPrice"`
	MaxFeePerGas         *hexutil.Big    `json:"maxFeePerGas"`
	MaxPriorityFeePerGas *hexutil.Big    `json:"maxPriorityFeePerGas"`
	Value                *hexutil.Big    `json:"value"`
	Nonce                *hexutil.Uint64 `json:"nonce"`
	Data                 *hexutil.Bytes  `json:"data"`
	Hash                 *common.Hash    `json:"hash"`
}

type BlockStateCall struct {
	Calls []*ParsedTransaction `json:"calls"`
}

type SimulationRequest struct {
	BlockStateCalls []BlockStateCall `json:"blockStateCalls"`
	Validation      bool             `json:"validation,omitempty"`
}

func (c *RPCClient) SimulateTransactions(ctx context.Context, transactions []*ParsedTransaction, blockParameter string) (any, error) {
	var response any
	request := SimulationRequest{
		BlockStateCalls: []BlockStateCall{
			{
				Calls: transactions,
			},
		},
		Validation: true,
	}
	err := c.rawClient.CallContext(ctx, &response, "eth_simulateV1", request, blockParameter)
	if err != nil {
		return nil, err
	}
	return response, nil
}
