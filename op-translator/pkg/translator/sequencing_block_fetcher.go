package translator

import (
	"context"
	"fmt"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/rs/zerolog/log"
)

type SequencingBlockFetcher struct {
	LastUsedBlock            *types.Block
	SequencingChainClient    IRPCClient
	SettlementChainBlockTime int
}

func InitSequencingBlockFetcher(sequencingChainClient IRPCClient, cfg *config.Config) *SequencingBlockFetcher {
	return NewSequencingBlockFetcher(sequencingChainClient, cfg.SettlementChainBlockTime)
}

func NewSequencingBlockFetcher(sequencingChainClient IRPCClient, settlementChainBlockTime int) *SequencingBlockFetcher {
	return &SequencingBlockFetcher{
		SequencingChainClient:    sequencingChainClient,
		SettlementChainBlockTime: settlementChainBlockTime,
	}
}

func (s *SequencingBlockFetcher) GetLastUsedBlockNumber(startTime int) uint64 {
	if s.LastUsedBlock == nil {
		log.Debug().Msg("No last used block found")
		return 0
	}

	blockNumber, err := s.LastUsedBlock.GetBlockNumber()
	if err != nil {
		log.Error().Err(err).Msg("Error getting last used block number")
		return 0
	}

	blockTimestamp, err := s.LastUsedBlock.GetBlockTimestamp()
	if err != nil {
		log.Error().Err(err).Msg("Error getting last used block timestamp")
		return 0
	}

	nextBlock, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), utils.UInt64ToHex(blockNumber+1), false)
	if err != nil {
		log.Error().Err(err).Msg("Error getting next block")
		return 0
	}

	nextBlockTimestamp, err := nextBlock.GetBlockTimestamp()
	if err != nil {
		log.Error().Err(err).Msg("Error getting next block timestamp")
		return 0
	}

	// Return cached block if it's timestamp is equal to or greater than the timewindow start time
	// and the next block's timestamp is strictly greater than the start time
	if nextBlockTimestamp > startTime && blockTimestamp <= startTime {
		return blockNumber
	}

	log.Debug().Msg("Last used block is not valid, returning 0")
	return 0
}

func (s *SequencingBlockFetcher) GetSequencingBlocks(block types.Block) ([]*types.Block, error) {
	timeWindowEnd, err := block.GetBlockTimestamp()
	if err != nil {
		return nil, err
	}
	timeWindowStart := timeWindowEnd - s.SettlementChainBlockTime

	var firstBlockNumberBeforeTime uint64

	firstBlockNumberBeforeTime = s.GetLastUsedBlockNumber(timeWindowStart)
	if firstBlockNumberBeforeTime == 0 {
		firstBlockNumberBeforeTime, err = s.FindFirstBlockOnOrBeforeTime(timeWindowStart)
		if err != nil {
			return nil, err
		}
	}

	blocks, err := s.GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd, firstBlockNumberBeforeTime)
	if err != nil {
		return nil, err
	}

	// Save the last used block if any blocks were found
	if len(blocks) > 0 {
		s.LastUsedBlock = blocks[len(blocks)-1]
	}

	// Returns empty array if no blocks were found
	return blocks, nil
}

const BinarySearchDivisor = uint64(2)

// TODO [SEQ-254]: Optimize this lookup
func (s *SequencingBlockFetcher) FindFirstBlockOnOrBeforeTime(time int) (uint64, error) {
	// Get latest block from sequencing chain
	latestBlock, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), "latest", false)
	if err != nil {
		return 0, fmt.Errorf("error getting latest block: %w", err)
	}

	latestBlockNumber, err := latestBlock.GetBlockNumber()
	if err != nil {
		return 0, fmt.Errorf("error getting latest block number: %w", err)
	}

	// Initialize binary search boundaries
	low := uint64(1)
	high := latestBlockNumber

	var result uint64

	for low <= high {
		mid := (low + high) / BinarySearchDivisor

		block, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), utils.UInt64ToHex(mid), false)

		if err != nil {
			return 0, fmt.Errorf("error getting block %d: %w", mid, err)
		}

		blockTimestamp, err := block.GetBlockTimestamp()
		if err != nil {
			return 0, fmt.Errorf("error getting block timestamp: %w", err)
		}

		if blockTimestamp <= time {
			result = mid
			low = mid + 1 // Look for a potentially closer block
		} else {
			high = mid - 1 // Block is too new, look earlier
		}
	}

	if result == 0 {
		return 0, fmt.Errorf("no block found before timestamp %d", time)
	}

	return result, nil
}

func (s *SequencingBlockFetcher) GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd int, firstBlockBeforeStartTime uint64) ([]*types.Block, error) {
	var blocks []*types.Block
	currentBlockNum := firstBlockBeforeStartTime + 1

	for {
		// Get the current block
		block, err := s.SequencingChainClient.GetBlockByNumber(
			context.Background(),
			utils.UInt64ToHex(currentBlockNum),
			false,
		)
		if err != nil {
			return nil, fmt.Errorf("error fetching block %d: %w", currentBlockNum, err)
		}

		if block == nil {
			return nil, fmt.Errorf("block %d is nil", currentBlockNum)
		}

		blockTimestamp, err := block.GetBlockTimestamp()
		if err != nil {
			return nil, fmt.Errorf("error getting block timestamp: %w", err)
		}

		// If we've passed the end of the time window, break
		if blockTimestamp > timeWindowEnd {
			break
		}

		// If the block is within our time window, add it to the result
		if blockTimestamp > timeWindowStart {
			blocks = append(blocks, &block)
		} else {
			return nil, fmt.Errorf("invalid block timestamp before time window: %d", blockTimestamp)
		}

		currentBlockNum++
	}

	return blocks, nil
}
