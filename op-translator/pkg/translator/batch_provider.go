package translator

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/contracts/l2"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
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
	TransactionProcessedABI     = `[{"anonymous":false,"inputs":[{"indexed":true,"name":"Sender","type":"address"},{"indexed":false,"name":"EncodedTxn","type":"bytes"}],"name":"TransactionProcessed","type":"event"}]`
	TransactionProcessedSig     = "TransactionProcessed(address,bytes)"
	TransactionProcessedSigHash = crypto.Keccak256Hash([]byte(TransactionProcessedSig))
)

type MetaBasedBatchProvider struct {
	MetaBasedChain            IRPCClient
	SequencingChain           IRPCClient
	SequencingContractAddress common.Address

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
		MetaBasedChain:            metaBasedChain,
		SequencingChain:           sequencingChain,
		SequencingContractAddress: common.HexToAddress(cfg.SequencingContractAddress),

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
		SequencingContractAddress:  sequencingContractAddress,
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

func parseTransactionProcessed(txLog *ethtypes.Log) (*l2.TransactionProcessed, error) {
	contractABI, err := abi.JSON(strings.NewReader(TransactionProcessedABI))
	if err != nil {
		return nil, fmt.Errorf("failed to parse ABI: %v", err)
	}

	event := l2.TransactionProcessed{}
	event.Sender = common.HexToAddress(txLog.Topics[1].Hex())

	err = contractABI.UnpackIntoInterface(&event, "TransactionProcessed", txLog.Data)
	if err != nil {
		return nil, fmt.Errorf("failed to unpack log data: %v", err)
	}

	return &event, nil
}

func (m *MetaBasedBatchProvider) FilterReceipts(receipts []*ethtypes.Receipt) (txns []hexutil.Bytes, result error) {
	for i, rec := range receipts {
		if rec.Status != ethtypes.ReceiptStatusSuccessful {
			continue
		}

		for j, txLog := range rec.Logs {
			if txLog.Address == m.sequencingContractAddress && txLog.Topics[0] == TransactionProcessedSigHash {
				proc, err := parseTransactionProcessed(txLog)
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

	txns, err := m.FilterReceipts(receipts)
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
