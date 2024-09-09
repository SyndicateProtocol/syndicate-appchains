package translator

import (
	"context"

	"github.com/SyndicateProtocol/op-translator/internal/config"
	rpcClient "github.com/SyndicateProtocol/op-translator/internal/rpc-clients"

	"github.com/rs/zerolog/log"
)

type OpTranslator struct {
	settlementChain *rpcClient.SettlementClient
	sequencingChain *rpcClient.SequencingClient
}

func Init(cfg *config.Config) *OpTranslator {
	settlementChain, err := rpcClient.NewSettlementClient(cfg.SettlementChainAddr)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}
	defer settlementChain.Client.CloseConnection() // Close the client connection when done

	sequencingChain, err := rpcClient.NewSequencingClient(cfg.SequencingChainAddr)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize sequencing chain")
	}

	defer sequencingChain.Client.CloseConnection() // Close the client connection when done

	return &OpTranslator{
		settlementChain: settlementChain,
		sequencingChain: sequencingChain,
	}
}

func (t *OpTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, fullTx bool) (map[string]any, error) {
	var result map[string]any
	log.Info().Msg("-- HIT eth_getBlockByNumber")
	// err := t.settlementChain.CallContext(ctx, &result, "eth_getBlockByNumber", blockNumber, fullTx)
	return result, nil
}

func (t *OpTranslator) GetBlockByHash(ctx context.Context, blockHash string, fullTx bool) (map[string]any, error) {
	var result map[string]any
	log.Info().Msg("-- HIT eth_getBlockByHash")
	// err := t.settlementChain.CallContext(ctx, &result, "eth_getBlockByHash", blockHash, fullTx)
	return result, nil
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
