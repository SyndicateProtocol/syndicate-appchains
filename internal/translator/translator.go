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
	settlementChain rpc.IRPCClient
	sequencingChain rpc.IRPCClient
}

func Init(cfg config.IConfig) *OPTranslator {
	settlementChain, err := rpc.NewSettlementClient(cfg.SettlementChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}
	defer settlementChain.Client.CloseConnection() // Close the client connection when done

	sequencingChain, err := rpc.NewSequencingClient(cfg.SequencingChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize sequencing chain")
	}

	defer sequencingChain.Client.CloseConnection() // Close the client connection when done

	return &OPTranslator{
		settlementChain: settlementChain.Client,
		sequencingChain: sequencingChain.Client,
	}
}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	log.Info().Msg("-- HIT eth_getBlockByNumber")
	block, err := t.settlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by number")
		return nil, err
	}
	return block, nil
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Info().Msg("-- HIT eth_getBlockByHash")
	block, err := t.settlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by hash")
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
