package translator

import (
	"log/slog"
	"math/big"
	"testing"

	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
	"github.com/stretchr/testify/assert"
)

var txIntrinsicGasTooLow = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	tx.Gas = 0
})
var rawTxIntrinsicGasTooLow = MustMarshalTransaction(txIntrinsicGasTooLow)

var txValid1 = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	to := common.HexToAddress("0x1234567890123456789012345678901234567001")
	tx.To = &to
	tx.Gas = 21000
})
var rawTxValid1 = MustMarshalTransaction(txValid1)

// just to make sure we are respecting the ordering when filter returns
var txValid2 = GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
	to := common.HexToAddress("0x1234567890123456789012345678901234567002")
	tx.To = &to
	tx.Gas = 21000
})
var rawTxValid2 = MustMarshalTransaction(txValid2)

func TestFilterTransactionsStateless(t *testing.T) {
	type args struct {
		txs []hexutil.Bytes
	}
	tests := []struct { //nolint:govet // test struct
		name                          string
		args                          args
		wantFilteredTxsStateless      []hexutil.Bytes
		wantParsedFilteredTxStateless []*ethtypes.Transaction
		wantRemovedCountStateless     int
	}{
		{
			"mix of valid and invalid, invalid at index 1",
			args{[]hexutil.Bytes{rawTxValid1, rawTxIntrinsicGasTooLow}},
			[]hexutil.Bytes{rawTxValid1},
			[]*ethtypes.Transaction{txValid1},
			1,
		},
		{
			"mix of valid and invalid, invalid at index 0",
			args{[]hexutil.Bytes{rawTxIntrinsicGasTooLow, rawTxValid1}},
			[]hexutil.Bytes{rawTxValid1},
			[]*ethtypes.Transaction{txValid1},
			1,
		},
		{
			"mix of valid and invalid, invalid at indexes 1, 2, 3",
			args{[]hexutil.Bytes{rawTxValid1, rawTxIntrinsicGasTooLow, rawTxIntrinsicGasTooLow, rawTxIntrinsicGasTooLow, rawTxValid2}},
			[]hexutil.Bytes{rawTxValid1, rawTxValid2},
			[]*ethtypes.Transaction{txValid1, txValid2},
			3,
		},
		{
			"unparseable",
			args{[]hexutil.Bytes{{0x00}}},
			[]hexutil.Bytes{},
			[]*ethtypes.Transaction{},
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotFilteredTxsStateless, gotParsedFilteredTxStateless, gotRemovedCountStateless := FilterTransactionsStateless(tt.args.txs, testlog.Logger(t, slog.LevelDebug))
			assert.Equalf(t, tt.wantFilteredTxsStateless, gotFilteredTxsStateless, "FilterTransactionsStateless(%v)", tt.args.txs)
			assert.Equalf(t, tt.wantRemovedCountStateless, gotRemovedCountStateless, "FilterTransactionsStateless(%v)", tt.args.txs)
			// parsed transactions are pointers, so we need to compare their contents one by one
			for i, parsedTx := range gotParsedFilteredTxStateless {
				assert.Equalf(t, tt.wantParsedFilteredTxStateless[i].Hash(), parsedTx.Hash(), "FilterTransactionsStateless(%v)", tt.args.txs)
			}
		})
	}
}

func GenerateDummyTx(customFunc func(tx *ethtypes.DynamicFeeTx)) *ethtypes.Transaction {
	dynamicTx := &ethtypes.DynamicFeeTx{
		ChainID:   new(big.Int),
		Nonce:     0,
		Gas:       uint64(0),
		GasFeeCap: new(big.Int),
		GasTipCap: new(big.Int),
		To:        nil,
		Value:     new(big.Int),
		Data:      nil,
	}

	if customFunc != nil {
		customFunc(dynamicTx)
	}

	tx := ethtypes.NewTx(dynamicTx)

	return tx
}

func MustMarshalTransaction(tx *ethtypes.Transaction) hexutil.Bytes {
	bytes, err := tx.MarshalBinary()
	if err != nil {
		panic(err)
	}
	return bytes
}

func TestValidateTransactionStateless(t *testing.T) {
	// supported legacy transaction
	txLegacyType := ethtypes.NewTx(&ethtypes.LegacyTx{
		Gas: 53000,
	})

	// generate some malformed transactions
	txDepositType := ethtypes.NewTx(&ethtypes.DepositTx{})
	txAccessListType := ethtypes.NewTx(&ethtypes.AccessListTx{})
	txBlobType := ethtypes.NewTx(&ethtypes.BlobTx{})

	const limit = 90 * 1024              // 90KiB
	dataWithLimit := make([]byte, limit) // just an array filled with a bunch of zeroes
	txTooBig := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		to := common.HexToAddress("0x1234567890123456789012345678901234567")
		tx.To = &to
		tx.Gas = 21000
		tx.Data = dataWithLimit
	})

	contractCreationWithLimit := make([]byte, params.MaxInitCodeSize+1)
	txContractCreationTooBig := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.Data = contractCreationWithLimit
	})

	txNegativeValue := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.Value = new(big.Int).SetInt64(-1)
	})

	bigIntTooLarge := new(big.Int).SetBytes([]byte{
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 64
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 128
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 192
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 256
		0xff, // this breaches the limit at 264 bits
	})

	txTooBigGasFeeCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasFeeCap = bigIntTooLarge
	})

	txTooBigGasTipCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasTipCap = bigIntTooLarge
	})

	txGasTipCapHigherThanGasFeeCap := GenerateDummyTx(func(tx *ethtypes.DynamicFeeTx) {
		tx.Gas = 21000
		tx.GasTipCap = new(big.Int).SetInt64(10)
		tx.GasFeeCap = new(big.Int).SetInt64(9)
	})

	type args struct {
		tx *ethtypes.Transaction
	}
	tests := []struct {
		args      args
		name      string
		errString string
		wantErr   bool
	}{
		{
			args{txValid1},
			"valid - tx type dynamic",
			"",
			false,
		},
		{
			args{txLegacyType},
			"valid - tx type legacy",
			"",
			false,
		},
		{
			args{txDepositType},
			"invalid - non supported tx type deposit",
			"transaction type not supported: tx type 126 not supported by this pool",
			true,
		},
		{
			args{txAccessListType},
			"invalid - non supported tx type access list",
			"transaction type not supported: tx type 1 not supported by this pool",
			true,
		},
		{
			args{txBlobType},
			"invalid - non supported tx type blob",
			"transaction type not supported: tx type 3 not supported by this pool",
			true,
		},
		{
			args{txContractCreationTooBig},
			"invalid - contract creation too big",
			"max initcode size exceeded: code size 49153, limit 49152",
			true,
		},
		{
			args{txTooBig},
			"invalid - too big",
			"oversized data: transaction size 92202, limit 92160",
			true,
		},
		{
			args{txIntrinsicGasTooLow},
			"invalid - intrinsic gas too low",
			"intrinsic gas too low: gas 0, minimum needed 53000",
			true,
		},
		{
			args{txNegativeValue},
			"invalid - negative value",
			"negative value",
			true,
		},
		{
			args{txTooBigGasFeeCap},
			"invalid - too big gas fee cap",
			"max fee per gas higher than 2^256-1",
			true,
		},
		{
			args{txTooBigGasTipCap},
			"invalid - too big gas tip cap",
			"max priority fee per gas higher than 2^256-1",
			true,
		},
		{
			args{txGasTipCapHigherThanGasFeeCap},
			"invalid - gas tip cap higher than gas fee cap",
			"max priority fee per gas higher than max fee per gas",
			true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := ValidateTransactionStateless(tt.args.tx)
			if tt.wantErr && err == nil {
				t.Errorf("an error is expected but got nil.")
			}
			if !tt.wantErr && err != nil {
				t.Errorf("an error is not expected but got: %v", err)
			}
			if tt.wantErr && err != nil && err.Error() != tt.errString {
				t.Errorf("expected error: %v, but got %v", tt.errString, err)
			}
		})
	}
}
