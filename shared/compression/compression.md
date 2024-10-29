# File compression in the metabased rollup

The `op-translator` expects to receive byte arrays from the L2 smart contract corresponding to a `TransactionProcessed` event. The first byte of the array will denote the compression type, while the rest of the array will be the compressed data.

## Supported compression formats
- `zlib`
- `brotli`
- None

## `metabased-sequencer` batch compression format

The `zlib` compression format is used by the `metabased-sequencer` to submit batch transactions to the L2 smart contract. Compression is only fruitful when the input data is large enough, so the `metabased-sequencer` will first concatenate a batch of transactions together into a single byte array, along with metadata, before compressing it. It will be in the following format:

### Data Format:
```
| NumTransactions (4 bytes) | LengthTransaction 1 (4 bytes) | Transaction 1 (variable length) | LengthTransaction | Transaction 2 | ... |
```

### Explanation:
- The first 4 bytes represent the total number of transactions (NumTransactions).
- Each transaction is preceded by a 4-byte field indicating its length (LengthTransaction).
- Each transaction data segment follows its respective length field, with the length in bytes specified by LengthTransaction.
- This pattern repeats for the specified number of transactions.
Example:
```
| 00 00 00 02 | 00 00 00 05 | <Transaction 1 data> | 00 00 00 03 | <Transaction 2 data> |
```
- 2 transactions, with lengths 5 and 3 bytes respectively.