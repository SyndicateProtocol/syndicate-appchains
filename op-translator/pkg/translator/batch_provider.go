package translator

import (
	"context"
	"errors"
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/contracts/l2"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/hashicorp/go-multierror"
	"github.com/rs/zerolog/log"
)

var (
	// ref: OP `crossdomain` package
	AddressType, _              = abi.NewType("address", "", nil)
	indexedAddressTypeArgs      = abi.Arguments{{Name: "Sender", Type: AddressType, Indexed: true}}
	TransactionProcessedABI     = l2.TransactionProcessedMetaData.ABI
	TransactionProcessedABIHash = crypto.Keccak256Hash([]byte(TransactionProcessedABI))
)

type MetaBasedBatchProvider struct {
	metaBasedChain            interfaces.IRPCClient
	sequencingChain           interfaces.IRPCClient
	sequencingContractAddress common.Address

	settlementStartBlock       int
	sequencingStartBlock       int
	sequencePerSettlementBlock int
}

func InitMetaBasedBatchProvider(cfg interfaces.IConfig) *MetaBasedBatchProvider {
	sequencingChain, err := rpc.Connect(cfg.SequencingChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize sequencing chain")
	}

	metaBasedChain, err := rpc.Connect(cfg.MetaBasedChainAddr())
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize metabased chain")
	}

	return &MetaBasedBatchProvider{
		metaBasedChain:            metaBasedChain,
		sequencingChain:           sequencingChain,
		sequencingContractAddress: common.HexToAddress(cfg.SequencingContractAddress()),

		settlementStartBlock:       cfg.SettlementStartBlock(),
		sequencingStartBlock:       cfg.SequencingStartBlock(),
		sequencePerSettlementBlock: cfg.SequencePerSettlementBlock(),
	}
}

func NewMetaBasedBatchProvider(
	settlementChainClient interfaces.IRPCClient,
	sequencingChainClient interfaces.IRPCClient,
	sequencingContractAddress common.Address,
	settlementStartBlock int,
	sequencingStartBlock int,
	sequencePerSettlementBlock int,
) *MetaBasedBatchProvider {
	return &MetaBasedBatchProvider{
		metaBasedChain:             settlementChainClient,
		sequencingChain:            sequencingChainClient,
		sequencingContractAddress:  sequencingContractAddress,
		settlementStartBlock:       settlementStartBlock,
		sequencingStartBlock:       sequencingStartBlock,
		sequencePerSettlementBlock: sequencePerSettlementBlock,
	}
}

func (m *MetaBasedBatchProvider) Close() {
	log.Debug().Msg("Closing MetaBasedBatchProvider")
	m.sequencingChain.CloseConnection()
	m.metaBasedChain.CloseConnection()
}

func (m *MetaBasedBatchProvider) getLinkedBlocks(blockNumStr string) ([]string, error) {
	blockNum, err := utils.HexToInt(blockNumStr)
	if err != nil {
		return nil, err
	}

	if blockNum < m.settlementStartBlock {
		return nil, errors.New("block number before start block")
	}

	end := m.sequencingStartBlock + (blockNum-m.settlementStartBlock)*m.sequencePerSettlementBlock
	start := end - m.sequencePerSettlementBlock + 1

	ret := make([]string, m.sequencePerSettlementBlock)
	for i := range ret {
		ret[i] = utils.IntToHex(start + i)
	}

	return ret, nil
}

// NOTE [SEQ-144]: THIS ASSUMES THAT THE L3 HAS THE SAME BLOCK TIME AS THE SETTLEMENT L2
func (m *MetaBasedBatchProvider) getParentBlockHash(ctx context.Context, blockNumStr string) (string, error) {
	log.Debug().Msgf("Getting parent block hash for block number %s", blockNumStr)
	blockNum, err := utils.HexToInt(blockNumStr)
	if err != nil {
		return "", err
	}

	if blockNum < m.settlementStartBlock {
		return "", errors.New("block number before start block")
	}

	if blockNum == m.settlementStartBlock {
		return constants.ZeroHash, nil
	}
	log.Debug().Msgf("Settlement start block: %d", m.settlementStartBlock)

	parentBlockNum := int64(blockNum - m.settlementStartBlock - 1)

	log.Debug().Msgf("Getting block hash for block number %d", parentBlockNum)
	previousBlock, err := m.metaBasedChain.AsEthClient().HeaderByNumber(ctx, big.NewInt(parentBlockNum))
	if err != nil {
		return "", err
	}
	log.Debug().Msgf("Previous block: %v", previousBlock)

	// TODO [SEQ-163]: MAYBE REMOVE??
	if previousBlock.EmptyBody() {
		return constants.ZeroHash, nil
	}

	return previousBlock.Hash().Hex(), nil
}

func parseTransactionProcessed(txLog *ethtypes.Log) (*l2.TransactionProcessed, error) {
	event := new(l2.TransactionProcessed)
	if err := abi.ParseTopics(event, indexedAddressTypeArgs, txLog.Topics[1:]); err != nil {
		return nil, err
	}
	event.EncodedTxn = txLog.Data
	return event, nil
}

func (m *MetaBasedBatchProvider) filterReceipts(receipts []*ethtypes.Receipt) (txns []hexutil.Bytes, result error) {
	for i, rec := range receipts {
		if rec.Status != ethtypes.ReceiptStatusSuccessful {
			continue
		}
		for j, log := range rec.Logs {
			if log.Address == m.sequencingContractAddress && len(log.Topics) > 0 && log.Topics[0] == TransactionProcessedABIHash {
				proc, err := parseTransactionProcessed(log)
				if err != nil {
					result = multierror.Append(result, fmt.Errorf("malformatted l2 receipt log in receipt %d, log %d: %w", i, j, err))
				} else {
					txns = append(txns, proc.EncodedTxn)
				}
			}
		}
	}
	return txns, result
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

	seqBlockNumbers, err := m.getLinkedBlocks(blockNumber)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: linked block numbers: %s", blockNumber, blockHash, seqBlockNumbers)

	receipts, err := m.sequencingChain.BlocksReceiptsByNumbers(ctx, seqBlockNumbers)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: receipts: %v", blockNumber, blockHash, receipts)

	txns, err := m.filterReceipts(receipts)
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Translating block number %s and hash %s: filtered transactions: %v", blockNumber, blockHash, txns)

	parentHash, err := m.getParentBlockHash(ctx, blockNumber)
	if err != nil {
		return nil, err
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
