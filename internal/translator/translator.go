package translator

import (
	"context"

	"github.com/SyndicateProtocol/op-translator/internal/config"
	"github.com/SyndicateProtocol/op-translator/internal/rpc-clients"
	"github.com/SyndicateProtocol/op-translator/internal/types"

	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
)

type OPTranslator struct {
	batchProvider       BatchProvider
	settlementChain     rpc.IRPCClient
	batcherInboxAddress common.Address
	sequencerAddress    common.Address
}

func Init(cfg config.IConfig) *OPTranslator {
	settlementChain, err := rpc.NewSettlementClient(cfg.SettlementChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}

	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg)

	return &OPTranslator{
		settlementChain:     settlementChain.Client,
		batcherInboxAddress: common.HexToAddress(cfg.SequencingContractAddress()),
		sequencerAddress:    common.HexToAddress(cfg.SequencingContractAddress()),
		batchProvider:       metaBasedBatchProvider,
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	batch, err := t.batchProvider.GetBatch(ctx, block)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Batch: %v", batch)

	frames, err := batch.ToFrames(config.MaxFrameSize)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Frames: %v", frames)

	err = block.AppendFrames(t.sequencerAddress, t.batcherInboxAddress, frames)
	if err != nil {
		return nil, err
	}

	return block, nil
}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByNumber")
	block, err := t.settlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by number")
		return nil, err
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByHash")
	block, err := t.settlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by hash")
		return nil, err
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) Close() {
	t.settlementChain.CloseConnection()
	t.batchProvider.Close()
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
