package translator

import (
	"context"
	"errors"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	gethlog "github.com/ethereum/go-ethereum/log"
)

type IRPCClient interface {
	GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error)
	GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error)
	GetReceiptsByBlocks(ctx context.Context, blocks []*types.Block) ([]*ethtypes.Receipt, error)
	SimulateTransactions(ctx context.Context, transactions []*rpc.ParsedTransaction, blockParameter string) (any, error)
	AsEthClient() rpc.IETHClient
}

type IBatchProvider interface {
	GetBatch(ctx context.Context, block types.Block) (*types.Batch, error)
	ValidateBlock(rawTxns []hexutil.Bytes, txns []*rpc.ParsedTransaction) ([]hexutil.Bytes, error)
}

type IBackfillProvider interface {
	IsBlockInBackfillingWindow(block types.Block) bool
	GetBackfillFrames(ctx context.Context, block types.Block) ([]*types.Frame, error)
}

// guarantees that the IRPCClient interface is implemented by RPCClient
var _ IRPCClient = (*rpc.RPCClient)(nil)

type OPTranslator struct {
	settlementChain     IRPCClient
	batchProvider       IBatchProvider
	backfillProvider    IBackfillProvider
	metrics             metrics.IMetrics
	batcherSigner       *Signer
	batcherInboxAddress *common.Address
	log                 gethlog.Logger

	lastTranslatedBlock types.Block
}

func NewOPTranslator(
	settlementChain IRPCClient,
	batchProvider IBatchProvider,
	backfillProvider IBackfillProvider,
	signer *Signer,
	batcherInboxAddress *common.Address,
	metricsCollector metrics.IMetrics,
	log gethlog.Logger,
) *OPTranslator {
	return &OPTranslator{
		settlementChain:     settlementChain,
		batcherInboxAddress: batcherInboxAddress,
		batchProvider:       batchProvider,
		backfillProvider:    backfillProvider,
		batcherSigner:       signer,
		metrics:             metricsCollector,
		log:                 log,
	}
}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	t.log.Debug("-- HIT eth_getBlockByNumber")

	start := time.Now()
	defer func() {
		t.metrics.RecordOPTranslatorTranslationLatency("eth_getBlockByNumber", time.Since(start).Seconds())
	}()

	t.metrics.RecordOPTranslatorRPCRequest("eth_getBlockByNumber")

	settlementChainRPCStart := time.Now()
	block, err := t.settlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	t.metrics.RecordOPTranslatorRPCCallDuration("eth_getBlockByNumber", "settlement_chain", time.Since(settlementChainRPCStart).Seconds())
	if err != nil {
		t.log.Error("failed to get block by number", "error", err)
		t.metrics.RecordOPTranslatorError("eth_getBlockByNumber", "block_fetch_error")
		return nil, err
	}

	var translatedBlock types.Block
	if t.lastTranslatedBlock != nil {
		if bn, err2 := t.lastTranslatedBlock.GetBlockNumberHex(); err2 == nil && bn == blockNumber {
			translatedBlock = t.lastTranslatedBlock
		}
	}
	if translatedBlock.IsEmpty() {
		translatedBlock, err = t.translateBlock(ctx, block)
		if err != nil {
			return nil, err
		}
	}

	if transactionDetailFlag {
		return translatedBlock.WithoutReceipts(), nil
	}
	return translatedBlock.WithoutReceipts().WithoutTransactions(), nil
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	t.log.Debug("-- HIT eth_getBlockByHash")

	translatedBlock, err := t.getTranslatedBlockByHash(ctx, blockHash)
	if err != nil {
		return nil, err
	}
	if transactionDetailFlag {
		return translatedBlock.WithoutReceipts(), nil
	}
	return translatedBlock.WithoutReceipts().WithoutTransactions(), nil
}

func (t *OPTranslator) getTranslatedBlockByHash(ctx context.Context, blockHash common.Hash) (types.Block, error) {
	start := time.Now()
	defer func() {
		t.metrics.RecordOPTranslatorTranslationLatency("eth_getBlockByHash", time.Since(start).Seconds())
	}()

	t.metrics.RecordOPTranslatorRPCRequest("eth_getBlockByHash")

	if t.lastTranslatedBlock != nil {
		if hash, err := t.lastTranslatedBlock.GetBlockHash(); err == nil && common.HexToHash(hash) == blockHash {
			return t.lastTranslatedBlock, nil
		}
	}

	settlementChainRPCStart := time.Now()
	block, err := t.settlementChain.GetBlockByHash(ctx, blockHash, true)
	t.metrics.RecordOPTranslatorRPCCallDuration("eth_getBlockByHash", "settlement_chain", time.Since(settlementChainRPCStart).Seconds())
	if err != nil {
		t.log.Error("failed to get block by hash", "error", err)
		t.metrics.RecordOPTranslatorError("eth_getBlockByHash", "block_fetch_error")
		return nil, err
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockReceipts(ctx context.Context, hash common.Hash) (any, error) {
	t.log.Debug("-- HIT eth_getBlockReceipts")
	block, err := t.getTranslatedBlockByHash(ctx, hash)
	if err != nil {
		return nil, err
	}
	return block.GetReceipts()
}

func (t *OPTranslator) getFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	if t.backfillProvider.IsBlockInBackfillingWindow(block) {
		t.metrics.RecordBackfillProviderBackfillingWindow(true)
		return t.backfillProvider.GetBackfillFrames(ctx, block)
	} else {
		t.metrics.RecordBackfillProviderBackfillingWindow(false)
		batch, err := t.batchProvider.GetBatch(ctx, block)
		if err != nil {
			t.metrics.RecordOPTranslatorError("get_frames", "get_batch_error")
			return nil, err
		}
		return batch.GetFrames(flags.MaxFrameSize)
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	frames, err := t.getFrames(ctx, block)
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "get_frames_error")

		if errors.Is(err, ErrNoMetabasedChainBlock) {
			return nil, ethereum.NotFound // don't return a block until we have the L3 info
		}
		return nil, err
	}

	blockNumHex, err := block.GetBlockNumberHex()
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "block_number_error")
		return nil, err
	}

	if len(frames) == 0 {
		t.log.Debug("no frames to translate", "block_number_hex", blockNumHex)
		return block, nil
	}

	data, err := types.ToData(frames)
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "frames_to_data_error")
		return nil, err
	}

	blockHash, err := block.GetBlockHash()
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "get_block_hash_error")
		return nil, err
	}

	from := t.batcherSigner.Address().String()
	tx := types.NewBatcherTx(blockHash, blockNumHex, from, t.batcherInboxAddress.String(), data, t.batcherSigner.ChainID())

	signedTxn, err := t.batcherSigner.Sign(&tx)
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "transaction_signing_error")
		return nil, err
	}

	// receipts are necessary to re-calculate the correct receiptsRoot
	receipts, err := t.settlementChain.GetReceiptsByBlocks(ctx, []*types.Block{&block})
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "get_receipts_error")
		return nil, err
	}

	err = block.AppendTransaction(signedTxn, from, receipts)
	if err != nil {
		t.metrics.RecordOPTranslatorError("translate_block", "transaction_append_error")
		return nil, err
	}

	t.lastTranslatedBlock = block
	return block, nil
}
