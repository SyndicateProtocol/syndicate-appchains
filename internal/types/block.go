package types

import (
	"bytes"
	"encoding/hex"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
)

// BatcherTransactionVersionByte 0x00 is the version for frames
// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batcher-transaction-format
const BatcherTransactionVersionByte = 0x00

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

func (b Block) appendTransaction(txn any) error {
	transactions, err := b.GetTransactions()
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}

	transactions = append(transactions, txn)
	b["transactions"] = transactions

	return nil
}

func (b Block) AppendFrames(from, to common.Address, frames []*Frame) error {
	if len(frames) == 0 {
		return nil
	}

	blockNum, err := b.GetBlockNumber()
	if err != nil {
		return err
	}
	blockHash, err := b.GetBlockHash()
	if err != nil {
		return err
	}

	buf := bytes.NewBuffer([]byte{BatcherTransactionVersionByte})
	for _, frame := range frames {
		err = frame.MarshalBinary(buf)
		if err != nil {
			return fmt.Errorf("error marshaling frame with FrameNumber: %d, :%w", frame.FrameNumber, err)
		}
	}
	calldata := "0x" + hex.EncodeToString(buf.Bytes())

	txn := NewBatcherTransaction(blockHash, blockNum, from.String(), to.String(), calldata)

	err = b.appendTransaction(txn)
	if err != nil {
		return err
	}

	return nil
}
