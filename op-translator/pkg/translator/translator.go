package translator

import (
	"context"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
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
	GetReceiptsByBlocks(ctx context.Context, blocks []*types.Block) ([]*ethtypes.Receipt, error)
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
	BatchProvider       IBatchProvider
	BackfillProvider    *backfill.BackfillProvider
	Metrics             metrics.IMetrics
	Signer              Signer
	BatcherInboxAddress common.Address
	BatcherAddress      common.Address
}

func Init(cfg *config.Config) *OPTranslator {
	settlementChain, err := rpc.Connect(cfg.SettlementChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}

	metricsCollector := metrics.NewMetrics()
	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg, metricsCollector)
	signer := NewSigner(cfg)
	backfillProvider := backfill.NewBackfillerProvider(cfg, metricsCollector)

	return &OPTranslator{
		SettlementChain:     settlementChain,
		BatcherInboxAddress: common.HexToAddress(cfg.BatchInboxAddress),
		BatcherAddress:      common.HexToAddress(cfg.BatcherAddress),
		BatchProvider:       metaBasedBatchProvider,
		BackfillProvider:    backfillProvider,
		Signer:              *signer,
		Metrics:             metricsCollector,
	}
}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByNumber")

	start := time.Now()
	defer func() {
		t.Metrics.RecordOPTranslatorTranslationLatency("eth_getBlockByNumber", time.Since(start).Seconds())
	}()

	t.Metrics.RecordOPTranslatorRPCRequest("eth_getBlockByNumber")

	block, err := t.SettlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by number")
		t.Metrics.RecordOPTranslatorError("eth_getBlockByNumber", "block_fetch_error")
		return nil, err
	}
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByHash")

	start := time.Now()
	defer func() {
		t.Metrics.RecordOPTranslatorTranslationLatency("eth_getBlockByHash", time.Since(start).Seconds())
	}()

	t.Metrics.RecordOPTranslatorRPCRequest("eth_getBlockByHash")

	block, err := t.SettlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by hash")
		t.Metrics.RecordOPTranslatorError("eth_getBlockByHash", "block_fetch_error")
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

func (t *OPTranslator) getFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	if t.BackfillProvider.IsBlockInBackfillingWindow(block) {
		t.Metrics.RecordBackfillProviderBackfillingWindow(true)
		return t.BackfillProvider.GetBackfillFrames(ctx, block)
	} else {
		t.Metrics.RecordBackfillProviderBackfillingWindow(false)
		batch, err := t.BatchProvider.GetBatch(ctx, block)
		if err != nil {
			t.Metrics.RecordOPTranslatorError("get_frames", "get_batch_error")
			return nil, err
		}
		return batch.GetFrames(config.MaxFrameSize)
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	frames, err := t.getFrames(ctx, block)
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "get_frames_error")
		return nil, err
	}

	blockNumHex, err := block.GetBlockNumberHex()
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "block_number_error")
		return nil, err
	}

	if len(frames) == 0 {
		log.Debug().Msgf("No frames to translate, block number (hex): %s", blockNumHex)
		return block, nil
	}

	data, err := types.ToData(frames)
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "frames_to_data_error")
		return nil, err
	}

	blockHash, err := block.GetBlockHash()
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "get_block_hash_error")
		return nil, err
	}

	tx := types.NewBatcherTx(blockHash, blockNumHex, t.BatcherAddress.String(), t.BatcherInboxAddress.String(), data, t.Signer.ChainID())

	signedTxn, err := t.Signer.Sign(&tx)
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "transaction_signing_error")
		return nil, err
	}

	err = block.AppendTransaction(signedTxn)
	if err != nil {
		t.Metrics.RecordOPTranslatorError("translate_block", "transaction_append_error")
		return nil, err
	}

	return block, nil
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
