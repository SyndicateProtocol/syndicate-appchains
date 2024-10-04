package types

import (
	"math/big"

	"github.com/SyndicateProtocol/op-translator/internal/constants"
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

func NewBatcherTx(blockHash, blockNumber, from, to string, input []byte) ethtypes.Transaction {
	tx := ethtypes.DynamicFeeTx{}
	tx.ChainID = big.NewInt(constants.ConfigChainID)
	tx.Nonce = 0
	tx.Gas = 0
	tx.GasTipCap = big.NewInt(0)
	tx.GasFeeCap = big.NewInt(0)
	tx.Value = big.NewInt(0)
	tx.Data = input
	toAddr := common.HexToAddress(to)
	tx.To = &toAddr

	return *ethtypes.NewTx(&tx)
}
