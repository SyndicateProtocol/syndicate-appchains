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
	gethlog "github.com/ethereum/go-ethereum/log"
)

type MetaBasedBatchProvider struct {
	MetaBasedChain         IRPCClient
	SequencingChain        IRPCClient
	Metrics                metrics.IMetrics
	log                    gethlog.Logger
	TransactionParser      *L3TransactionParser
	SequencingBlockFetcher *SequencingBlockFetcher
	SettlementStartBlock   uint64
}

func NewMetaBasedBatchProvider(
	settlementChainClient IRPCClient,
	sequencingChainClient IRPCClient,
	sequencingContractAddress common.Address,
	settlementStartBlock uint64,
	settlementChainBlockTime int,
	bpMetrics metrics.IMetrics,
	log gethlog.Logger,
) *MetaBasedBatchProvider {
	return &MetaBasedBatchProvider{
		MetaBasedChain:         settlementChainClient,
		SequencingChain:        sequencingChainClient,
		TransactionParser:      MustNewL3TransactionParser(sequencingContractAddress, log),
		SequencingBlockFetcher: NewSequencingBlockFetcher(sequencingChainClient, settlementChainBlockTime, log),
		Metrics:                bpMetrics,
		SettlementStartBlock:   settlementStartBlock,
		log:                    log,
	}
}

// NOTE [SEQ-144]: THIS ASSUMES THAT THE L3 HAS THE SAME BLOCK TIME AS THE SETTLEMENT L2
func (m *MetaBasedBatchProvider) getParentBlockHash(ctx context.Context, blockNumStr string) (string, error) {
	start := time.Now()
	defer func() {
		duration := time.Since(start).Seconds()
		m.Metrics.RecordBatchProviderBatchProcessingDuration("get_parent_block_hash", duration)
	}()

	m.log.Debug("getting parent block hash", "block_number", blockNumStr)
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
	m.log.Debug("settlement start block", "block_number", m.SettlementStartBlock)

	parentBlockNum := blockNum - m.SettlementStartBlock - 1

	m.log.Debug("getting block hash", "block_number", parentBlockNum)
	previousBlock, err := m.MetaBasedChain.AsEthClient().HeaderByNumber(ctx, new(big.Int).SetUint64(parentBlockNum))
	if err != nil {
		return "", err
	}
	m.log.Debug("previous block", "block", previousBlock)

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
				m.log.Warn("malformatted l2 receipt log in receipt", "receipt_index", i, "log_index", j, "error", parseErr)
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
	m.log.Debug("translating block", "block_number", blockNumber, "block_hash", blockHash, "receipts", receipts)

	txns := m.FilterReceipts(receipts)
	m.log.Debug("translating block", "block_number", blockNumber, "block_hash", blockHash, "filtered_transactions", txns)

	parentHash, err := m.getParentBlockHash(ctx, blockNumber)
	if err != nil {
		return nil, err
	}

	// Filter out invalid transactions
	txns, err = m.GetValidTransactions(txns)
	if err != nil {
		return nil, err
	}

	timestamp, err := block.GetBlockTimestampHex()
	if err != nil {
		return nil, err
	}

	batch, err := types.NewBatch(parentHash, blockNumber, blockHash, timestamp, txns)
	if err != nil {
		return nil, err
	}
	m.log.Debug("translating block", "block_number", blockNumber, "block_hash", blockHash, "batch", batch)

	m.Metrics.RecordBatchProviderBatchProcessed("get_batch") // success count metric

	return batch, nil
}

// GetValidTransactions do validation in two phases:
//   - Stateless (inexpensive): locally filter transactions
//   - Stateful (expensive): use simulate RPC to check if the block to-be produced is valid
func (m *MetaBasedBatchProvider) GetValidTransactions(rawTxs []hexutil.Bytes) ([]hexutil.Bytes, error) {
	start := time.Now()
	defer func() {
		duration := time.Since(start).Seconds()
		m.Metrics.RecordBatchProviderBatchProcessingDuration("get_valid_transactions", duration)
	}()

	// First phase validation: stateless
	rawFilteredTxStateless, parsedFilteredTxStateless := ParseRawTransactions(rawTxs, m.log)
	removedCountStateless := len(rawTxs) - len(rawFilteredTxStateless)
	m.Metrics.RecordBatchProviderInvalidTransactionsCount("stateless", removedCountStateless)
	if removedCountStateless > 0 {
		m.log.Debug("transactions got filtered by stateless validation", "count", removedCountStateless)
	}

	// Second phase validation: validate block
	rawFilteredTxStateful, err := m.ValidateBlock(rawFilteredTxStateless, parsedFilteredTxStateless)
	if err != nil {
		return nil, err
	}
	removedCountStateful := len(rawFilteredTxStateless) - len(rawFilteredTxStateful)
	m.Metrics.RecordBatchProviderInvalidTransactionsCount("stateful", removedCountStateful)
	if removedCountStateful > 0 {
		m.log.Debug("transactions got filtered by stateful validation", "count", removedCountStateful)
	}
	return rawFilteredTxStateful, nil
}
