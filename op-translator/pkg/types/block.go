package types

import (
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/trie"
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

func (b Block) GetBlockNumberHex() (string, error) {
	blockNum, ok := b["number"].(string)
	if !ok {
		return "", fmt.Errorf("parsing error: block number")
	}

	return blockNum, nil
}

func (b Block) GetBlockNumber() (uint64, error) {
	blockNumHex, err := b.GetBlockNumberHex()
	if err != nil {
		return 0, err
	}
	return utils.HexToUInt64(blockNumHex)
}

func (b Block) GetBlockTimestampHex() (string, error) {
	timestamp, ok := b["timestamp"].(string)
	if !ok {
		return "", fmt.Errorf("parsing error: block timestamp")
	}

	return timestamp, nil
}

func (b Block) GetBlockTimestamp() (int, error) {
	timestampHex, err := b.GetBlockTimestampHex()
	if err != nil {
		return 0, err
	}
	return utils.HexToInt(timestampHex)
}

func (b Block) GetTransactions() ([]any, error) {
	transactions, ok := b["transactions"].([]any)
	if !ok {
		return nil, fmt.Errorf("parsing error: transactions")
	}

	return transactions, nil
}

func (b Block) AppendTransaction(txn *ethtypes.Transaction, from string, receipts []*ethtypes.Receipt) error {
	transactions, err := b.GetTransactions()
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}
	transactions = append(transactions, txn)
	b["transactions"] = transactions

	hasher := trie.NewStackTrie(nil)
	// TODO update "transactionsRoot" ?
	// b["transactionsRoot"] = types.DeriveSha(types.Transactions(transactions), hasher).Hex()

	// TODO it would probably be easier to unmarshal the block into the proper type
	blockNumber, err := b.GetBlockNumber()
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}
	blockHash, err := b.GetBlockHash()
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}

	// also append a receipt
	receipt := &types.Receipt{
		Type:        types.DynamicFeeTxType,
		Logs:        []*types.Log{},
		Status:      types.ReceiptStatusSuccessful,
		TxHash:      txn.Hash(),
		BlockNumber: big.NewInt(int64(blockNumber)),
		BlockHash:   common.HexToHash(blockHash),
	}
	receipt.Bloom = types.CreateBloom(types.Receipts{receipt})

	receipts = append(receipts, receipt)
	// receiptsJSON, err := json.Marshal(receipts)
	// if err != nil {
	// 	return err
	// }
	b["receipts"] = receipts

	// recalculate receiptsRoot
	b["receiptsRoot"] = types.DeriveSha(types.Receipts(receipts), hasher).Hex()
	b["logsBloom"] = types.CreateBloom(receipts)

	// TODO update block hash ?

	return nil
}

func (b Block) GetReceipts() (any, error) {
	receipts, ok := b["receipts"].(any)
	if !ok {
		return nil, fmt.Errorf("parsing error: receipts")
	}
	return receipts, nil
}

func (b Block) Clone() Block {
	cloned := Block{}
	for k, v := range b {
		cloned[k] = v
	}
	return cloned
}

func (b Block) WithoutReceipts() Block {
	cloned := b.Clone()
	delete(cloned, "receipts")
	return cloned
}

func (b Block) WithoutTransactions() Block {
	cloned := b.Clone()
	delete(cloned, "transactions")
	return cloned
}
