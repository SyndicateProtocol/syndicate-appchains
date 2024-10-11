package types

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type BatcherTransaction struct {
	BlockHash            string `json:"blockHash"`
	BlockNumber          string `json:"blockNumber"`
	From                 string `json:"from"`
	To                   string `json:"to"`
	Input                string `json:"input"`
	Hash                 string `json:"hash"`
	Gas                  string `json:"gas"`
	GasPrice             string `json:"gasPrice"`
	Nonce                string `json:"nonce"`
	TransactionIndex     string `json:"transactionIndex"`
	Value                string `json:"value"`
	Type                 string `json:"type"`
	V                    string `json:"v"`
	R                    string `json:"r"`
	S                    string `json:"s"`
	YParity              string `json:"yParity"`
	ChainID              string `json:"chainId"`
	MaxPriorityFeePerGas string `json:"maxPriorityFeePerGas"`
	MaxFeePerGas         string `json:"maxFeePerGas"`
}

func NewBatcherTx(blockHash, blockNumber, from, to string, input []byte, chainID int64) ethtypes.Transaction {
	toAddr := common.HexToAddress(to)
	tx := ethtypes.DynamicFeeTx{
		ChainID:   big.NewInt(chainID),
		Nonce:     0,
		Gas:       0,
		GasTipCap: big.NewInt(0),
		GasFeeCap: big.NewInt(0),
		Value:     big.NewInt(0),
		Data:      input,
		To:        &toAddr,
	}

	return *ethtypes.NewTx(&tx)
}
