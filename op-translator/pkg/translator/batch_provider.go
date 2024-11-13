package translator

import (
	"context"
	"errors"
	"math/big"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
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

	SettlementStartBlock int
}

func InitMetaBasedBatchProvider(cfg *config.Config) *MetaBasedBatchProvider {
	sequencingChain, err := rpc.Connect(cfg.SequencingChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize sequencing chain")
	}

	metaBasedChain, err := rpc.Connect(cfg.MetaBasedChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize metabased chain")
	}

	return &MetaBasedBatchProvider{
		MetaBasedChain:         metaBasedChain,
		SequencingChain:        sequencingChain,
		TransactionParser:      InitL3TransactionParser(cfg),
		SequencingBlockFetcher: InitSequencingBlockFetcher(sequencingChain, cfg),

		SettlementStartBlock: cfg.SettlementStartBlock,
	}
}

func NewMetaBasedBatchProvider(
	settlementChainClient IRPCClient,
	sequencingChainClient IRPCClient,
	sequencingContractAddress common.Address,
	settlementStartBlock int,
	settlementChainBlockTime int,
) *MetaBasedBatchProvider {
	return &MetaBasedBatchProvider{
		MetaBasedChain:         settlementChainClient,
		SequencingChain:        sequencingChainClient,
		TransactionParser:      NewL3TransactionParser(sequencingContractAddress),
		SequencingBlockFetcher: NewSequencingBlockFetcher(sequencingChainClient, settlementChainBlockTime),

		SettlementStartBlock: settlementStartBlock,
	}
}

func (m *MetaBasedBatchProvider) Close() {
	log.Debug().Msg("Closing Arbitrum MetaBasedBatchProvider")
	m.SequencingChain.CloseConnection()
	m.MetaBasedChain.CloseConnection()
}

// NOTE [SEQ-144]: THIS ASSUMES THAT THE L3 HAS THE SAME BLOCK TIME AS THE SETTLEMENT L2
func (m *MetaBasedBatchProvider) getParentBlockHash(ctx context.Context, blockNumStr string) (string, error) {
	log.Debug().Msgf("Getting parent block hash for block number %s", blockNumStr)
	blockNum, err := utils.HexToInt(blockNumStr)
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

	parentBlockNum := int64(blockNum - m.SettlementStartBlock - 1)

	log.Debug().Msgf("Getting block hash for block number %d", parentBlockNum)
	previousBlock, err := m.MetaBasedChain.AsEthClient().HeaderByNumber(ctx, big.NewInt(parentBlockNum))
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

	return batch, nil
}

// GetValidTransactions do validation in two phases:
//   - Stateless (inexpensive): locally filter transactions
//   - Stateful (expensive): use simulate RPC to check if the block to-be produced is valid
func (m *MetaBasedBatchProvider) GetValidTransactions(rawTxs []hexutil.Bytes) []hexutil.Bytes {
	// First phase validation: stateless
	rawFilteredTxStateless, parsedFilteredTxStateless, removedCountStateless := FilterTransactionsStateless(rawTxs)
	if removedCountStateless > 0 {
		log.Debug().Msgf("Transactions got filtered by stateless validation: %d", removedCountStateless)
	}

	// Second phase validation: stateful
	rawFilteredTxsStateful, removedCountStateful := m.FilterTransactionsStateful(rawFilteredTxStateless, parsedFilteredTxStateless)
	if removedCountStateful > 0 {
		log.Debug().Msgf("Transactions got filtered by stateful validation: %d", removedCountStateful)
	}

	return rawFilteredTxsStateful
}
