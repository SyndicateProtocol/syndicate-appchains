package translator

import (
	"context"

	"github.com/SyndicateProtocol/op-translator/internal/config"
	"github.com/SyndicateProtocol/op-translator/internal/rpc-clients"
	"github.com/SyndicateProtocol/op-translator/internal/types"
	"github.com/SyndicateProtocol/op-translator/internal/utils"

	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
)

type OPTranslator struct {
	batchProvider       BatchProvider
	settlementChain     rpc.IRPCClient
	batcherInboxAddress common.Address
	batcherAddress      common.Address
}

func Init(cfg config.IConfig) *OPTranslator {
	settlementChain, err := rpc.NewSettlementClient(cfg.SettlementChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}

	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg)

	return &OPTranslator{
		settlementChain:     settlementChain.Client,
		batcherInboxAddress: common.HexToAddress(cfg.BatchInboxAddress()),
		batcherAddress:      common.HexToAddress(cfg.BatcherAddress()),
		batchProvider:       metaBasedBatchProvider,
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	num, _ := block.GetBlockNumber()
	bn, err := utils.HexToUInt64(num)
	if err != nil {
		return nil, err
	}
	log.Info().Msgf("Block number: %d", bn)

	batch, err := t.batchProvider.GetBatch(ctx, block)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Batch: %v", batch)

	log.Info().Msgf("Adding batch: %v", batch)

	frames, err := batch.ToFrames(config.MaxFrameSize)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Frames: %v", frames)

	data, err := types.ToData(frames)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Data: %v", data)

	blockNum, err := block.GetBlockNumber()
	if err != nil {
		return nil, err
	}
	blockHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	tx := types.NewBatcherTx(blockHash, blockNum, t.batcherAddress.String(), t.batcherInboxAddress.String(), data)

	signer := NewSigner()
	signedTxn, err := signer.Sign(&tx)
	if err != nil {
		return nil, err
	}
	log.Info().Msgf("Signed transaction: %v", signedTxn)

	err = block.AppendTransaction(signedTxn)
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
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByHash")
	log.Info().Msgf("Block number: %s", blockHash.String())
	block, err := t.settlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
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
