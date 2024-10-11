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
	SettlementChain     rpc.IRPCClient
	BatchProvider       BatchProvider
	Signer              Signer
	BatcherInboxAddress common.Address
	BatcherAddress      common.Address
}

func Init(cfg config.IConfig) *OPTranslator {
	settlementChain, err := rpc.Connect(cfg.SettlementChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}

	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg)
	signer := NewSigner(cfg)

	return &OPTranslator{
		SettlementChain:     settlementChain,
		BatcherInboxAddress: common.HexToAddress(cfg.BatchInboxAddress()),
		BatcherAddress:      common.HexToAddress(cfg.BatcherAddress()),
		BatchProvider:       metaBasedBatchProvider,
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

	frames, err := batch.ToFrames(config.MaxFrameSize)
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
