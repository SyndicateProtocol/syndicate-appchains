package translator

import (
	"context"
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/txpool"
	"github.com/ethereum/go-ethereum/params"
	"math/big"
	"slices"

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

const (
	// txSlotSize is used to calculate how many data slots a single transaction
	// takes up based on its size. The slots are used as DoS protection, ensuring
	// that validating a new transaction remains a constant operation (in reality
	// O(maxslots), where max slots are 4 currently).
	txSlotSize = 32 * 1024

	// txMaxSize is the maximum size a single transaction can have. This field has
	// non-trivial consequences: larger transactions are significantly harder and
	// more expensive to propagate; larger transactions also take more resources
	// to validate whether they fit into the pool or not.
	txMaxSize = 4 * txSlotSize // 128KB
)

type MetaBasedBatchProvider struct {
	MetaBasedChain    IRPCClient
	SequencingChain   IRPCClient
	TransactionParser *L3TransactionParser

	SettlementStartBlock       int
	SequencingStartBlock       int
	SequencePerSettlementBlock int
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
		MetaBasedChain:    metaBasedChain,
		SequencingChain:   sequencingChain,
		TransactionParser: InitL3TransactionParser(cfg),

		SettlementStartBlock:       cfg.SettlementStartBlock,
		SequencingStartBlock:       cfg.SequencingStartBlock,
		SequencePerSettlementBlock: cfg.SequencePerSettlementBlock,
	}
}

func NewMetaBasedBatchProvider(
	settlementChainClient IRPCClient,
	sequencingChainClient IRPCClient,
	sequencingContractAddress common.Address,
	settlementStartBlock int,
	sequencingStartBlock int,
	sequencePerSettlementBlock int,
) *MetaBasedBatchProvider {
	return &MetaBasedBatchProvider{
		MetaBasedChain:             settlementChainClient,
		SequencingChain:            sequencingChainClient,
		TransactionParser:          NewL3TransactionParser(sequencingContractAddress),
		SettlementStartBlock:       settlementStartBlock,
		SequencingStartBlock:       sequencingStartBlock,
		SequencePerSettlementBlock: sequencePerSettlementBlock,
	}
}

func (m *MetaBasedBatchProvider) Close() {
	log.Debug().Msg("Closing MetaBasedBatchProvider")
	m.SequencingChain.CloseConnection()
	m.MetaBasedChain.CloseConnection()
}

func (m *MetaBasedBatchProvider) GetLinkedBlocks(blockNumStr string) ([]string, error) {
	blockNum, err := utils.HexToInt(blockNumStr)
	if err != nil {
		return nil, err
	}

	if blockNum < m.SettlementStartBlock {
		return nil, errors.New("block number before start block")
	}

	end := m.SequencingStartBlock + (blockNum-m.SettlementStartBlock)*m.SequencePerSettlementBlock
	start := end - m.SequencePerSettlementBlock + 1

	ret := make([]string, m.SequencePerSettlementBlock)
	for i := range ret {
		ret[i] = utils.IntToHex(start + i)
	}

	return ret, nil
}

// NOTE [SEQ-144]: THIS ASSUMES THAT THE L3 HAS THE SAME BLOCK TIME AS THE SETTLEMENT L2
func (m *MetaBasedBatchProvider) getParentBlockHash(ctx context.Context, blockNumStr string) (*ethtypes.Header, string, error) {
	log.Debug().Msgf("Getting parent block hash for block number %s", blockNumStr)
	blockNum, err := utils.HexToInt(blockNumStr)
	if err != nil {
		return nil, "", err
	}

	if blockNum < m.SettlementStartBlock {
		return nil, "", errors.New("block number before start block")
	}

	if blockNum == m.SettlementStartBlock {
		return nil, constants.ZeroHash, nil
	}
	log.Debug().Msgf("Settlement start block: %d", m.SettlementStartBlock)

	parentBlockNum := int64(blockNum - m.SettlementStartBlock - 1)

	log.Debug().Msgf("Getting block hash for block number %d", parentBlockNum)
	previousBlock, err := m.MetaBasedChain.AsEthClient().HeaderByNumber(ctx, big.NewInt(parentBlockNum))
	if err != nil {
		return nil, "", err
	}
	log.Debug().Msgf("Previous block: %v", previousBlock)

	return previousBlock, previousBlock.Hash().Hex(), nil
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

// SEQ-205: make sure the blocks being produced are valid
// In the case we have an invalid transaction, it should not be included in the block
// This filters transactions using a local stateless validation, i.e. gas, nonces
// and chain-specific configs such as activated hardforks are *not* validated at this point
// State-dependent validations can only be performed by the NetaBased chain itself
func (m *MetaBasedBatchProvider) FilterTransactions(txs []hexutil.Bytes, header *ethtypes.Header) ([]hexutil.Bytes, int) {
	filtered := make([]hexutil.Bytes, len(txs))
	removedCount := 0
	for _, rawTx := range txs {
		var tx *ethtypes.Transaction
		unmarshalErr := tx.UnmarshalBinary(rawTx)
		if unmarshalErr != nil {
			log.Warn().Err(unmarshalErr).Msgf("can't unmarshall transaction: %q", tx)
			removedCount++
			continue
		}
		validationErr := ValidateTransaction(tx, header)
		if validationErr != nil {
			log.Warn().Err(validationErr).Msgf("skipping invalid transaction: %q", tx)
			removedCount++
			continue
		}
		filtered = append(filtered, rawTx)
	}

	return filtered, removedCount
}

// This is a lightweight stateless L3 tx validation
// And should not be used a general validation for non-L3
func ValidateTransaction(tx *ethtypes.Transaction, head *ethtypes.Header) error {
	acceptedTypes := []uint8{
		ethtypes.LegacyTxType,     // Berlin
		ethtypes.DynamicFeeTxType, // London
	}
	if !slices.Contains(acceptedTypes, tx.Type()) {
		return fmt.Errorf("%w: tx type %v not supported by this pool", core.ErrTxTypeNotSupported, tx.Type())
	}

	// Before performing any expensive validations, sanity check that the tx is
	// smaller than the maximum limit the pool can meaningfully handle
	if tx.Size() > txMaxSize {
		return fmt.Errorf("%w: transaction size %v, limit %v", txpool.ErrOversizedData, tx.Size(), txMaxSize)
	}

	// Check whether the init code size has been exceeded
	if tx.To() == nil && len(tx.Data()) > params.MaxInitCodeSize {
		return fmt.Errorf("%w: code size %v, limit %v", core.ErrMaxInitCodeSizeExceeded, len(tx.Data()), params.MaxInitCodeSize)
	}

	// Transactions can't be negative. This may never happen using RLP decoded
	// transactions but may occur for transactions created using the RPC.
	if tx.Value().Sign() < 0 {
		return txpool.ErrNegativeValue
	}

	// Sanity check for extremely large numbers (supported by RLP or RPC)
	if tx.GasFeeCap().BitLen() > 256 {
		return core.ErrFeeCapVeryHigh
	}
	if tx.GasTipCap().BitLen() > 256 {
		return core.ErrTipVeryHigh
	}
	// Ensure gasFeeCap is greater than or equal to gasTipCap
	if tx.GasFeeCapIntCmp(tx.GasTipCap()) < 0 {
		return core.ErrTipAboveFeeCap
	}
	// Ensure the transaction has more gas than the bare minimum needed to cover
	// the transaction metadata
	intrGas, err := core.IntrinsicGas(tx.Data(), tx.AccessList(), tx.To() == nil, true, true, true)
	if err != nil {
		return err
	}
	if tx.Gas() < intrGas {
		return fmt.Errorf("%w: gas %v, minimum needed %v", core.ErrIntrinsicGas, tx.Gas(), intrGas)
	}
	return nil
}

func (m *MetaBasedBatchProvider) GetBatch(ctx context.Context, block types.Block) (*types.Batch, error) {
	blockNumber, err := block.GetBlockNumber()
	if err != nil {
		return nil, err
	}
	blockHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	seqBlockNumbers, err := m.GetLinkedBlocks(blockNumber)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: linked block numbers: %s", blockNumber, blockHash, seqBlockNumbers)

	receipts, err := m.SequencingChain.BlocksReceiptsByNumbers(ctx, seqBlockNumbers)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: receipts: %v", blockNumber, blockHash, receipts)

	header, parentHash, err := m.getParentBlockHash(ctx, blockNumber)
	if err != nil {
		return nil, err
	}

	txns := m.FilterReceipts(receipts)
	log.Debug().Msgf("Translating block number %s and hash %s: filtered transactions: %v", blockNumber, blockHash, txns)

	txns, removedCount := m.FilterTransactions(txns, header)
	if removedCount > 0 {
		log.Debug().Msgf("Transactions got filtered by stateless validation: %d", removedCount)
	}

	timestamp, err := block.GetBlockTimestamp()
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
