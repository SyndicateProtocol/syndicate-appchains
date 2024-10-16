package rpc

import (
	"context"
	"fmt"
	"math"
	"math/big"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
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
	var receipts []*ethtypes.Receipt
	for _, number := range numbers {
		numberInt, err := utils.HexToInt(number)
		if err != nil {
			return nil, fmt.Errorf("failed to convert block number: %w", err)
		}
		block, err := c.client.BlockByNumber(ctx, big.NewInt(int64(numberInt)))
		if err != nil {
			return nil, fmt.Errorf("failed to get block by number: %w", err)
		}
		var hashes []common.Hash
		for _, tx := range block.Transactions() {
			hashes = append(hashes, tx.Hash())
		}
		blockReceipts, err := c.FetchReceipts(ctx, eth.BlockToInfo(block), hashes)
		if err != nil {
			return nil, fmt.Errorf("failed to get receipts for block %d: %w", numberInt, err)
		}
		receipts = append(receipts, blockReceipts...)
	}
	return receipts, nil
}

func (c *RPCClient) FetchReceipts(ctx context.Context, blockInfo eth.BlockInfo, txHashes []common.Hash) (ethtypes.Receipts, error) {
	m := c.receiptsFetcher.PickReceiptsMethod(len(txHashes))
	block := eth.ToBlockID(blockInfo)

	var result ethtypes.Receipts
	var err error

	blockNumber := block.Number
	if blockNumber > math.MaxInt64 {
		return nil, fmt.Errorf("block number exceeds int64 range: %d", blockNumber)
	}

	switch m {
	case sources.EthGetTransactionReceiptBatch:
		result, err = c.client.BlockReceipts(ctx, rpc.BlockNumberOrHashWithNumber(rpc.BlockNumber(int64(block.Number)))) //nolint // overflow already checked
	case sources.AlchemyGetTransactionReceipts:
		var tmp receiptsWrapper
		err = c.rawClient.CallContext(ctx, &tmp, "alchemy_getTransactionReceipts", blockHashParameter{BlockHash: block.Hash})
		result = tmp.Receipts
	case sources.DebugGetRawReceipts:
		var rawReceipts []hexutil.Bytes
		err = c.rawClient.CallContext(ctx, &rawReceipts, "debug_getRawReceipts", block.Hash)
		if err == nil && len(rawReceipts) == len(txHashes) {
			result, err = eth.DecodeRawReceipts(block, rawReceipts, txHashes)
		} else {
			err = fmt.Errorf("got %d raw receipts, expected %d", len(rawReceipts), len(txHashes))
		}
	case sources.ParityGetBlockReceipts, sources.EthGetBlockReceipts, sources.ErigonGetBlockReceiptsByBlockHash:
		methodName := fmt.Sprintf("%d", m)
		err = c.rawClient.CallContext(ctx, &result, methodName, block.Hash)
	default:
		err = fmt.Errorf("unknown receipt fetching method: %d", uint64(m))
	}

	if err != nil {
		c.receiptsFetcher.OnReceiptsMethodErr(m, err)
		return nil, err
	}
	return result, nil
}
