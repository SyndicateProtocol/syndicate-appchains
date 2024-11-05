package translator

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/rs/zerolog/log"
)

type IRPCClient interface {
	CloseConnection()
	GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error)
	GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error)
	BlocksReceiptsByNumbers(ctx context.Context, numbers []string) ([]*ethtypes.Receipt, error)
	AsEthClient() rpc.IETHClient
}

type IBatchProvider interface {
	GetBatch(ctx context.Context, block types.Block) (*types.Batch, error)
	Close()
}

// guarantees that the IRPCClient interface is implemented by RPCClient
var _ IRPCClient = (*rpc.RPCClient)(nil)

type OPTranslator struct {
	SettlementChain     IRPCClient
	MetaBasedChain      IRPCClient
	BatchProvider       IBatchProvider
	BackfillProvider    *backfill.BackfillProvider
	Signer              Signer
	BatcherInboxAddress common.Address
	BatcherAddress      common.Address
}

func Init(cfg *config.Config) *OPTranslator {
	settlementChain, err := rpc.Connect(cfg.SettlementChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain client")
	}

	metaBasedChain, err := rpc.Connect(cfg.MetaBasedChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize metabased chain client")
	}

	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg)
	signer := NewSigner(cfg)
	backfillProvider := backfill.NewBackfillerProvider(cfg)

	return &OPTranslator{
		SettlementChain:     settlementChain,
		MetaBasedChain:      metaBasedChain,
		BatcherInboxAddress: common.HexToAddress(cfg.BatchInboxAddress),
		BatcherAddress:      common.HexToAddress(cfg.BatcherAddress),
		BatchProvider:       metaBasedBatchProvider,
		BackfillProvider:    backfillProvider,
		Signer:              *signer,
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	batch, err := t.BatchProvider.GetBatch(ctx, block)
	if err != nil {
		return nil, err
	}

	// TODO SEQ-209: Write logic for switching between backfill and regular data fetching
	frames, err := batch.GetFrames(config.MaxFrameSize)

	if err != nil {
		return nil, err
	}

	data, err := types.ToData(frames)
	if err != nil {
		return nil, err
	}

	blockNum, err := block.GetBlockNumber()
	if err != nil {
		return nil, err
	}

	blockHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}
	tx := types.NewBatcherTx(blockHash, blockNum, t.BatcherAddress.String(), t.BatcherInboxAddress.String(), data, t.Signer.ChainID())

	signedTxn, err := t.Signer.Sign(&tx)
	if err != nil {
		return nil, err
	}

	err = block.AppendTransaction(signedTxn)
	if err != nil {
		return nil, err
	}

	return block, nil
}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByNumber")
	block, err := t.SettlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by number")
		return nil, err
	}
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByHash")
	block, err := t.SettlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by hash")
		return nil, err
	}
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) Close() {
	t.SettlementChain.CloseConnection()
	t.BatchProvider.Close()
}

func ShouldTranslate(method string) bool {
	switch method {
	case "eth_getBlockByNumber":
		return true
	case "eth_getBlockByHash":
		return true
	}

	return false
}
