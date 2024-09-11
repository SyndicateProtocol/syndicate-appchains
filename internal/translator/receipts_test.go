package translator

import (
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	SomeOtherABIHash = crypto.Keccak256Hash([]byte("someOtherABI"))
)

func TestParseTransactionProcessed(t *testing.T) {
	// Setup
	senderAddr := common.HexToAddress("0x1234567890123456789012345678901234567890")
	encodedTxn := []byte{1, 2, 3, 4, 5}

	log := &types.Log{
		Address: common.HexToAddress("0x0000000000000000000000000000000000000000"),
		Topics: []common.Hash{
			TransactionProcessedABIHash,
			common.BytesToHash(senderAddr.Bytes()),
		},
		Data: encodedTxn,
	}

	// Test
	event, err := ParseTransactionProcessed(log)

	// Assert
	require.NoError(t, err)
	assert.Equal(t, senderAddr, event.Sender)
	assert.Equal(t, encodedTxn, event.EncodedTxn)
}

func TestFilterReceipts(t *testing.T) {
	// Setup
	sequencerAddr := common.HexToAddress("0x1111111111111111111111111111111111111111")
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")
	encodedTxn1 := []byte{1, 2, 3}
	encodedTxn2 := []byte{4, 5, 6}

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencerAddr,
					Topics: []common.Hash{
						TransactionProcessedABIHash,
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
					Address: sequencerAddr,
					Topics: []common.Hash{
						TransactionProcessedABIHash,
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

	// Test
	txns, err := FilterReceipts(receipts, sequencerAddr)

	// Assert
	require.NoError(t, err)
	assert.Len(t, txns, 2)
	assert.Equal(t, encodedTxn1, []byte(txns[0]))
	assert.Equal(t, encodedTxn2, []byte(txns[1]))
}

func TestFilterReceiptsWithExtraLog(t *testing.T) {
	// Setup
	sequencerAddr := common.HexToAddress("0x1111111111111111111111111111111111111111")
	senderAddr := common.HexToAddress("0x2222222222222222222222222222222222222222")
	encodedTxn := []byte{1, 2, 3}

	receipts := []*types.Receipt{
		{
			Status: types.ReceiptStatusSuccessful,
			Logs: []*types.Log{
				{
					Address: sequencerAddr,
					Topics: []common.Hash{
						TransactionProcessedABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn,
				},
				{
					Address: sequencerAddr,
					Topics: []common.Hash{
						SomeOtherABIHash,
						common.BytesToHash(senderAddr.Bytes()),
					},
					Data: encodedTxn,
				},
			},
		},
	}

	// Test
	txns, err := FilterReceipts(receipts, sequencerAddr)

	// Assert
	assert.NoError(t, err)
	assert.Len(t, txns, 1)
	assert.Equal(t, encodedTxn, []byte(txns[0]))
}
