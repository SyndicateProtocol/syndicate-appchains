package translator_test

import (
	"log/slog"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum-optimism/optimism/op-service/testlog"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/stretchr/testify/assert"
)

var (
	SomeOtherABIHash          = crypto.Keccak256Hash([]byte("someOtherABI"))
	SequencingContractAddress = common.HexToAddress("0x1111111111111111111111111111111111111111")
)

func getBatchProvider(t *testing.T) *translator.MetaBasedBatchProvider {
	t.Helper()
	mockMetrics := mocks.NewMockMetrics()
	return translator.NewMetaBasedBatchProvider(
		nil,
		nil,
		SequencingContractAddress,
		1,
		2,
		mockMetrics,
		testlog.Logger(t, slog.LevelDebug),
	)
}

func TestFilterReceipts(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider(t)
	sequencingContractAddress := SequencingContractAddress
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedSigHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: translator.DummyEncodedData,
				},
			},
		},
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedSigHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: translator.DummyEncodedData,
				},
			},
		},
		{
			Status: types.ReceiptStatusFailed,
			Logs:   []*types.Log{},
		},
	}

	txns := metaBasedBatchProvider.FilterReceipts(receipts)
	assert.Len(t, txns, 2)
	assert.Equal(t, hexutil.Bytes{translator.DummyTxn[1]}, txns[0])
	assert.Equal(t, hexutil.Bytes{translator.DummyTxn[1]}, txns[1])
}

func TestFilterReceiptsWithExtraLog(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider(t)
	sequencingContractAddress := SequencingContractAddress
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedSigHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: translator.DummyEncodedData,
				},
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						SomeOtherABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: translator.DummyEncodedData,
				},
			},
		},
	}

	txns := metaBasedBatchProvider.FilterReceipts(receipts)
	assert.Len(t, txns, 1)
	assert.Equal(t, hexutil.Bytes{translator.DummyTxn[1]}, txns[0])
}
