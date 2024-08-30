package translator

import (
	"context"

	config "github.com/SyndicateProtocol/op-translator/internal/config"

	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type OpTranslator struct {
	settlementChain *rpc.Client
	sequencingChain *rpc.Client
}

func Init(cfg *config.Config) *OpTranslator {
	return &OpTranslator{}
}

func (t *OpTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, fullTx bool) (map[string]interface{}, error) {
	var result map[string]interface{}
	log.Info().Msg("-- HIT eth_getBlockByNumber")
	// err := t.settlementChain.CallContext(ctx, &result, "eth_getBlockByNumber", blockNumber, fullTx)
	return result, nil
}

func (t *OpTranslator) GetBlockByHash(ctx context.Context, blockHash string, fullTx bool) (map[string]interface{}, error) {
	var result map[string]interface{}
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
