package translator_test

import (
	"errors"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	SomeOtherABIHash = crypto.Keccak256Hash([]byte("someOtherABI"))
)

func getBatchProvider() *translator.MetaBasedBatchProvider {
	return &translator.MetaBasedBatchProvider{
		SettlementStartBlock:       1,
		SequencingStartBlock:       2,
		SequencePerSettlementBlock: 2,
		SequencingContractAddress:  common.HexToAddress("0x1111111111111111111111111111111111111111"),
		SequencingChain:            nil,
	}
}

func TestGetLinkedBlocks(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider()
	tests := []struct {
		err      error
		name     string
		block    string
		expected []string
	}{
		{nil, "Start block", "0x1", []string{"0x1", "0x2"}},
		{nil, "Block 10", "0xa", []string{"0x13", "0x14"}},
		{nil, "Large block", "0xd431", []string{"0x1a861", "0x1a862"}},
		{errors.New("block number before start block"), "Block before start block", "0x0", []string(nil)},
		{errors.New("invalid hex string, must start with 0x"), "Invalid block number", "foo", []string(nil)},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := metaBasedBatchProvider.GetLinkedBlocks(tt.block)

			assert.Equal(t, tt.err, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}

func TestGetLinkedBlocks_1to1(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider()
	metaBasedBatchProvider.SequencePerSettlementBlock = 1

	blocks, err := metaBasedBatchProvider.GetLinkedBlocks("0x2")

	require.NoError(t, err)
	assert.Equal(t, []string{"0x3"}, blocks)
}

func TestGetLinkedBlocks_4to1(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider()
	metaBasedBatchProvider.SequencePerSettlementBlock = 4
	metaBasedBatchProvider.SequencingStartBlock = 10

	blocks, err := metaBasedBatchProvider.GetLinkedBlocks("0x2")

	require.NoError(t, err)
	assert.Equal(t, []string{"0xb", "0xc", "0xd", "0xe"}, blocks)
}

func TestParseTransactionProcessed(t *testing.T) {
	senderAddr := common.HexToAddress("0x1234567890123456789012345678901234567890")
	encodedTxn := []byte{1, 2, 3, 4, 5}

	log := &types.Log{
		Address: common.HexToAddress("0x0000000000000000000000000000000000000000"),
		Topics: []common.Hash{
			translator.TransactionProcessedABIHash,
			common.BytesToHash(senderAddr.Bytes()),
		},
		Data: encodedTxn,
	}

	event, err := translator.ParseTransactionProcessed(log)

	require.NoError(t, err)
	assert.Equal(t, senderAddr, event.Sender)
	assert.Equal(t, encodedTxn, event.EncodedTxn)
}

func TestFilterReceipts(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider()
	sequencingContractAddress := metaBasedBatchProvider.SequencingContractAddress
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")
	encodedTxn1 := []byte{1, 2, 3}
	encodedTxn2 := []byte{4, 5, 6}

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn1,
				},
			},
		},
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn2,
				},
			},
		},
		{
			Status: types.ReceiptStatusFailed,
			Logs:   []*types.Log{},
		},
	}

	txns, err := metaBasedBatchProvider.FilterReceipts(receipts)

	require.NoError(t, err)
	assert.Len(t, txns, 2)
	assert.Equal(t, encodedTxn1, []byte(txns[0]))
	assert.Equal(t, encodedTxn2, []byte(txns[1]))
}

func TestFilterReceiptsWithExtraLog(t *testing.T) {
	metaBasedBatchProvider := getBatchProvider()
	sequencingContractAddress := metaBasedBatchProvider.SequencingContractAddress
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")
	encodedTxn := []byte{1, 2, 3}

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						translator.TransactionProcessedABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn,
				},
				{
					Address: sequencingContractAddress,
					Topics: []common.Hash{
						SomeOtherABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn,
				},
			},
		},
	}

	txns, err := metaBasedBatchProvider.FilterReceipts(receipts)

	assert.NoError(t, err)
	assert.Len(t, txns, 1)
	assert.Equal(t, encodedTxn, []byte(txns[0]))
}
