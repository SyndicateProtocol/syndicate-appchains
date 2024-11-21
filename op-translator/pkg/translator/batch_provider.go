package translator

import (
	"context"
	"errors"
	"math/big"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/rs/zerolog/log"
)

type MetaBasedBatchProvider struct {
	MetaBasedChain         IRPCClient
	SequencingChain        IRPCClient
	TransactionParser      *L3TransactionParser
	SequencingBlockFetcher *SequencingBlockFetcher
	Metrics                metrics.IMetrics
	SettlementStartBlock   uint64
}

func NewMetaBasedBatchProvider(
	settlementChainClient IRPCClient,
	sequencingChainClient IRPCClient,
	sequencingContractAddress common.Address,
	settlementStartBlock uint64,
	settlementChainBlockTime int,
	bpMetrics metrics.IMetrics,
) *MetaBasedBatchProvider {
	return &MetaBasedBatchProvider{
		MetaBasedChain:         settlementChainClient,
		SequencingChain:        sequencingChainClient,
		TransactionParser:      NewL3TransactionParser(sequencingContractAddress),
		SequencingBlockFetcher: NewSequencingBlockFetcher(sequencingChainClient, settlementChainBlockTime),
		Metrics:                bpMetrics,
		SettlementStartBlock:   settlementStartBlock,
	}
}

// NOTE [SEQ-144]: THIS ASSUMES THAT THE L3 HAS THE SAME BLOCK TIME AS THE SETTLEMENT L2
func (m *MetaBasedBatchProvider) getParentBlockHash(ctx context.Context, blockNumStr string) (string, error) {
	start := time.Now()
	defer func() {
		duration := time.Since(start).Seconds()
		m.Metrics.RecordBatchProviderBatchProcessingDuration("get_parent_block_hash", duration)
	}()

	log.Debug().Msgf("Getting parent block hash for block number %s", blockNumStr)
	blockNum, err := utils.HexToUInt64(blockNumStr)
	if err != nil {
		return "", err
	}

	if blockNum < m.SettlementStartBlock {
		return "", errors.New("block number before start block")
	}

	if blockNum == m.SettlementStartBlock {
		return constants.ZeroHash, nil
	}
	log.Debug().Msgf("Settlement start block: %d", m.SettlementStartBlock)

	parentBlockNum := blockNum - m.SettlementStartBlock - 1

	log.Debug().Msgf("Getting block hash for block number %d", parentBlockNum)
	previousBlock, err := m.MetaBasedChain.AsEthClient().HeaderByNumber(ctx, new(big.Int).SetUint64(parentBlockNum))
	if err != nil {
		return "", err
	}
	log.Debug().Msgf("Previous block: %v", previousBlock)

	return previousBlock.Hash().Hex(), nil
}

func (m *MetaBasedBatchProvider) FilterReceipts(receipts []*ethtypes.Receipt) []hexutil.Bytes {
	var transactions []hexutil.Bytes
	for i, rec := range receipts {
		if rec.Status != ethtypes.ReceiptStatusSuccessful {
			continue
		}

		for j, txLog := range rec.Logs {
			if !m.TransactionParser.IsLogTransactionProcessed(txLog) {
				continue
			}
			transactionsInEvent, parseErr := m.TransactionParser.GetEventTransactions(txLog)
			if parseErr != nil {
				log.Warn().Err(parseErr).Msgf("malformatted l2 receipt log in receipt %d, log %d", i, j)
				continue
			}
			transactions = append(transactions, transactionsInEvent...)
		}
	}
	return transactions
}

func (m *MetaBasedBatchProvider) GetBatch(ctx context.Context, block types.Block) (*types.Batch, error) {
	start := time.Now()

	defer func() {
		duration := time.Since(start).Seconds()
		m.Metrics.RecordBatchProviderBatchProcessingDuration("get_batch", duration)
	}()

	blockNumber, err := block.GetBlockNumberHex()
	if err != nil {
		return nil, err
	}
	blockHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	seqBlocks, err := m.SequencingBlockFetcher.GetSequencingBlocks(block)
	if err != nil {
		return nil, err
	}

	receipts, err := m.SequencingChain.GetReceiptsByBlocks(ctx, seqBlocks)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: receipts: %v", blockNumber, blockHash, receipts)

	txns := m.FilterReceipts(receipts)
	log.Debug().Msgf("Translating block number %s and hash %s: filtered transactions: %v", blockNumber, blockHash, txns)

	parentHash, err := m.getParentBlockHash(ctx, blockNumber)
	if err != nil {
		return nil, err
	}

	// Filter out invalid transactions
	txns = m.GetValidTransactions(txns)

	timestamp, err := block.GetBlockTimestampHex()
	if err != nil {
		return nil, err
	}

	batch, err := types.NewBatch(parentHash, blockNumber, blockHash, timestamp, txns)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: batch: %v", blockNumber, blockHash, batch)

	m.Metrics.RecordBatchProviderBatchProcessed("get_batch") // success count metric

	return batch, nil
}

// GetValidTransactions do validation in two phases:
//   - Stateless (inexpensive): locally filter transactions
//   - Stateful (expensive): use simulate RPC to check if the block to-be produced is valid
func (m *MetaBasedBatchProvider) GetValidTransactions(rawTxs []hexutil.Bytes) []hexutil.Bytes {
	start := time.Now()
	defer func() {
		duration := time.Since(start).Seconds()
		m.Metrics.RecordBatchProviderBatchProcessingDuration("get_valid_transactions", duration)
	}()

	// First phase validation: stateless
	rawFilteredTxStateless, parsedFilteredTxStateless, removedCountStateless := FilterTransactionsStateless(rawTxs)
	if removedCountStateless > 0 {
		log.Debug().Msgf("Transactions got filtered by stateless validation: %d", removedCountStateless)
		m.Metrics.RecordBatchProviderInvalidTransactionsCount("stateless", removedCountStateless)
	}

	// Second phase validation: stateful
	rawFilteredTxsStateful, _, removedCountStateful := m.FilterTransactionsStateful(rawFilteredTxStateless, parsedFilteredTxStateless)
	if removedCountStateful > 0 {
		log.Debug().Msgf("Transactions got filtered by stateful validation: %d", removedCountStateful)
		m.Metrics.RecordBatchProviderInvalidTransactionsCount("stateful", removedCountStateful)
	}

	return rawFilteredTxsStateful
}
