package types

import (
	"fmt"

	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type Block map[string]any

func (b Block) IsEmpty() bool {
	return len(b) == 0
}

func (b Block) GetBlockHash() (string, error) {
	blockHash, ok := b["hash"].(string)
	if !ok {
		return "", fmt.Errorf("parsing error: block hash")
	}

	return blockHash, nil
}

func (b Block) GetBlockNumber() (string, error) {
	blockNum, ok := b["number"].(string)
	if !ok {
		return "", fmt.Errorf("parsing error: block number")
	}

	return blockNum, nil
}

func (b Block) GetBlockTimestamp() (string, error) {
	timestamp, ok := b["timestamp"].(string)
	if !ok {
		return "", fmt.Errorf("parsing error: block number")
	}

	return timestamp, nil
}

func (b Block) GetTransactions() ([]any, error) {
	transactions, ok := b["transactions"].([]any)
	if !ok {
		return nil, fmt.Errorf("parsing error: transactions")
	}

	return transactions, nil
}

func (b Block) AppendTransaction(txn *ethtypes.Transaction) error {
	transactions, err := b.GetTransactions()
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}
	transactions = append(transactions, txn)
	b["transactions"] = transactions

	return nil
}
