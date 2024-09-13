package types

import (
	"bytes"
	"encoding/hex"
	"fmt"
)

// BatcherTransactionVersionByte 0x00 is the version for frames
// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batcher-transaction-format
const BatcherTransactionVersionByte = 0x00

type Block map[string]any

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

func (b Block) appendTransaction(txn any) error {
	transactions, ok := b["transactions"].([]any)
	if !ok {
		return fmt.Errorf("parsing error: transactions")
	}

	transactions = append(transactions, txn)
	b["transactions"] = transactions

	return nil
}

func (b Block) AppendFrames(from, to string, frames []*Frame) error {
	if len(frames) == 0 {
		return fmt.Errorf("no frames to append")
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

	txn := NewBatcherTransaction(blockHash, blockNum, from, to, calldata)

	err = b.appendTransaction(txn)
	if err != nil {
		return fmt.Errorf("error appending txn to batch: %w", err)
	}

	return nil
}
