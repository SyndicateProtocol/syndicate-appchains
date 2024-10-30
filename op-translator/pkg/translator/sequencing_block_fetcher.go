package translator

import (
	"context"
	"fmt"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
)

type SequencingBlockFetcher struct {
	SequencingChainClient IRPCClient

	SettlementChainBlockTime int

	// TODO [SEQ-243]: Add cache to track previous requested sequencing blocks
}

func InitSequencingBlockFetcher(sequencingChainClient IRPCClient, cfg *config.Config) *SequencingBlockFetcher {
	return NewSequencingBlockFetcher(sequencingChainClient, cfg.SettlementChainBlockTime)
}

func NewSequencingBlockFetcher(sequencingChainClient IRPCClient, settlementChainBlockTime int) *SequencingBlockFetcher {
	return &SequencingBlockFetcher{
		SequencingChainClient: sequencingChainClient,

		SettlementChainBlockTime: settlementChainBlockTime,
	}
}

func (s *SequencingBlockFetcher) GetSequencingBlocks(block types.Block) ([]*types.Block, error) {
	timeWindowEnd, err := block.GetBlockTimestamp()
	if err != nil {
		return nil, err
	}
	timeWindowStart := timeWindowEnd - s.SettlementChainBlockTime

	firstBlockNumberBeforeTime, err := s.FindFirstBlockOnOrBeforeTime(timeWindowStart)
	if err != nil {
		return nil, err
	}

	return s.GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd, firstBlockNumberBeforeTime)
}

const BinarySearchDivisor = 2

// TODO [SEQ-254]: Optimize this lookup
func (s *SequencingBlockFetcher) FindFirstBlockOnOrBeforeTime(time int) (int, error) {
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
	low := 1
	high := latestBlockNumber

	var result int

	for low <= high {
		mid := (low + high) / BinarySearchDivisor

		block, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), utils.IntToHex(mid), false)
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

func (s *SequencingBlockFetcher) GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd, firstBlockBeforeStartTime int) ([]*types.Block, error) {
	var blocks []*types.Block
	currentBlockNum := firstBlockBeforeStartTime + 1

	for {
		// Get the current block
		block, err := s.SequencingChainClient.GetBlockByNumber(
			context.Background(),
			utils.IntToHex(currentBlockNum),
			false,
		)
		if err != nil {
			return nil, fmt.Errorf("error fetching block %d: %w", currentBlockNum, err)
		}

		if block == nil {
			return nil, fmt.Errorf("block is nil")
		}

		timestamp, err := block.GetBlockTimestamp()
		if err != nil {
			return nil, fmt.Errorf("error getting block timestamp: %w", err)
		}

		// If we've passed the end of the time window, break
		if timestamp > timeWindowEnd {
			break
		}

		// If the block is within our time window, add it to the result
		if timestamp > timeWindowStart {
			blocks = append(blocks, &block)
		} else {
			return nil, fmt.Errorf("invalid block timestamp before time window: %d", timestamp)
		}

		currentBlockNum++
	}

	return blocks, nil
}
