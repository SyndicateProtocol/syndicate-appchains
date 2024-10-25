package translator

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/rs/zerolog/log"
)

type SequencingBlockFetcher struct {
	SequencingChainClient IRPCClient

	SettlementChainBlockTime int

	// TODO [SEQ-243]: Add cache to track previous requested sequencing blocks
}

func InitSequencingBlockFetcher(sequencingChainClient IRPCClient, cfg *config.Config) *SequencingBlockFetcher {
	return &SequencingBlockFetcher{
		SequencingChainClient: sequencingChainClient,

		SettlementChainBlockTime: cfg.SettlementChainBlockTime,
	}
}

func NewSequencingBlockFetcher(sequencingChainClient IRPCClient, settlementChainBlockTime int) *SequencingBlockFetcher {
	return &SequencingBlockFetcher{
		SequencingChainClient: sequencingChainClient,

		SettlementChainBlockTime: settlementChainBlockTime,
	}
}

func (s *SequencingBlockFetcher) GetSequencingBlocks(block types.Block) ([]string, error) {
	timestampHex, err := block.GetBlockTimestamp()
	if err != nil {
		return nil, err
	}

	timeWindowEnd, err := utils.HexToInt(timestampHex)
	if err != nil {
		return nil, err
	}

	timeWindowStart := timeWindowEnd - utils.SecondsToMilliseconds(s.SettlementChainBlockTime)
	log.Info().Msgf("Getting sequencing blocks for time window: %d to %d", timeWindowStart, timeWindowEnd)

	// TODO [SEQ-242]: Implement getting sequencing blocks by timewindow
	return []string{}, nil
}
