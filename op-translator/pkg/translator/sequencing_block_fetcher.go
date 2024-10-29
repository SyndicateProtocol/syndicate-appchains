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

func (s *SequencingBlockFetcher) GetSequencingBlocks(block types.Block) ([]string, error) {
	timeWindowEnd, err := block.GetBlockTimestamp()
	if err != nil {
		return nil, err
	}
	timeWindowStart := timeWindowEnd - s.SettlementChainBlockTime

	firstBlockBeforeTime, err := s.FindFirstBlockOnOrBeforeTime(timeWindowStart)
	if err != nil {
		return nil, err
	}

	firstBlockBeforeTimeNum, err := firstBlockBeforeTime.GetBlockNumber()
	if err != nil {
		return nil, err
	}

	return s.GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd, firstBlockBeforeTimeNum)
}

const BinarySearchDivisor = 2

// TODO []: Optimize this lookup
func (s *SequencingBlockFetcher) FindFirstBlockOnOrBeforeTime(time int) (types.Block, error) {
	// Get latest block from sequencing chain
	latestBlock, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), "latest", false)
	if err != nil {
		return nil, fmt.Errorf("error getting latest block: %w", err)
	}

	latestBlockNumber, err := latestBlock.GetBlockNumber()
	if err != nil {
		return nil, fmt.Errorf("error getting latest block timestamp: %w", err)
	}

	// Initialize binary search boundaries
	low := 1
	high := latestBlockNumber

	var result types.Block

	for low <= high {
		mid := (low + high) / BinarySearchDivisor

		block, err := s.SequencingChainClient.GetBlockByNumber(context.Background(), utils.IntToHex(mid), false)
		if err != nil {
			return nil, fmt.Errorf("error getting block %d: %w", mid, err)
		}

		timestamp, err := block.GetBlockTimestamp()
		if err != nil {
			return nil, fmt.Errorf("error getting timestamp for block %d: %w", mid, err)
		}

		if timestamp <= time {
			result = block
			low = mid + 1 // Look for a potentially closer block
		} else {
			high = mid - 1 // Block is too new, look earlier
		}
	}

	if result == nil {
		return nil, fmt.Errorf("no block found before timestamp %d", time)
	}

	return result, nil
}

func (s *SequencingBlockFetcher) GetSequencingBlocksByTimeWindow(timeWindowStart, timeWindowEnd, firstBlockBeforeStartTime int) ([]string, error) {
	var blocks []string
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

		// Get block timestamp
		timestamp, err := block.GetBlockTimestamp()
		if err != nil {
			return nil, fmt.Errorf("error getting timestamp for block %d: %w", currentBlockNum, err)
		}

		// If we've passed the end of the time window, break
		if timestamp > timeWindowEnd {
			break
		}

		// If the block is within our time window, add it to the result
		if timestamp > timeWindowStart {
			blockNum, err := block.GetBlockNumber()
			if err != nil {
				return nil, fmt.Errorf("error getting block number for block %d: %w", currentBlockNum, err)
			}
			blocks = append(blocks, utils.IntToHex(blockNum))
		} else {
			return nil, fmt.Errorf("invalid block timestamp before time window: %d", timestamp)
		}

		currentBlockNum++
	}

	return blocks, nil
}
