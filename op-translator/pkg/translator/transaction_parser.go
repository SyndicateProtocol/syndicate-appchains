package translator

import (
	"encoding/binary"
	"fmt"
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/rs/zerolog/log"
)

type TransactionProcessed struct {
	EncodedData []byte
	Sender      common.Address // indexed
}

const (
	TransactionProcessedABI  = `[{"anonymous":false,"inputs":[{"indexed":true,"name":"Sender","type":"address"},{"indexed":false,"name":"EncodedData","type":"bytes"}],"name":"TransactionProcessed","type":"event"}]`
	TransactionProcessedName = "TransactionProcessed"
	TransactionProcessedSig  = "TransactionProcessed(address,bytes)"
	NumTransactionsBytes     = 4
	LengthTransactionBytes   = 4
)

var TransactionProcessedSigHash = crypto.Keccak256Hash([]byte(TransactionProcessedSig))

type L3TransactionParser struct {
	sequencingContractABI     abi.ABI
	sequencingContractAddress common.Address
}

func InitL3TransactionParser(cfg *config.Config) *L3TransactionParser {
	return NewL3TransactionParser(common.HexToAddress(cfg.SequencingContractAddress))
}

func NewL3TransactionParser(sequencingContractAddress common.Address) *L3TransactionParser {
	sequencingContractABI, err := abi.JSON(strings.NewReader(TransactionProcessedABI))
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize sequencing contract ABI")
	}

	return &L3TransactionParser{
		sequencingContractAddress: sequencingContractAddress,
		sequencingContractABI:     sequencingContractABI,
	}
}

func (l *L3TransactionParser) IsLogTransactionProcessed(ethLog *ethtypes.Log) bool {
	return ethLog.Address == l.sequencingContractAddress && ethLog.Topics[0] == TransactionProcessedSigHash
}

func (l *L3TransactionParser) GetEventTransactions(ethLog *ethtypes.Log) ([]hexutil.Bytes, error) {
	event := TransactionProcessed{}
	event.Sender = common.HexToAddress(ethLog.Topics[1].Hex())

	err := l.sequencingContractABI.UnpackIntoInterface(&event, TransactionProcessedName, ethLog.Data)
	if err != nil {
		return nil, err
	}

	transactions, err := DecodeEventData(event.EncodedData)
	if err != nil {
		return nil, err
	}

	return transactions, nil
}

func DecodeEventData(data []byte) ([]hexutil.Bytes, error) {
	if len(data) <= 1 {
		return nil, fmt.Errorf("no data provided for decoding")
	}

	compressionType := utils.GetCompressionType(data[0])
	compressedData := data[1:]
	var decompressedData []byte
	var err error

	switch compressionType {
	case utils.CompressionTypeNone:
		// No decompression needed
		return []hexutil.Bytes{hexutil.Bytes(compressedData)}, nil

	case utils.CompressionTypeZlib:
		// Zlib  expects the compression type byte included.
		decompressedData, err = utils.DecompressZlib(data)

	case utils.CompressionTypeBrotli:
		// skip the first byte (compressionType).
		decompressedData, err = utils.DecompressBrotli(compressedData)

	case utils.CompressionTypeUnknown:
		return nil, fmt.Errorf("unknown compression type byte %v", compressionType)
	}

	if err != nil {
		return nil, fmt.Errorf("decompression failed: %w", err)
	}

	return ParseEventData(decompressedData)
}

func ParseEventData(data []byte) ([]hexutil.Bytes, error) {
	/*
		Data Format:
		| NumTransactions (4 bytes) | LengthTransaction 1 (4 bytes) | Transaction 1 (variable length) | LengthTransaction | Transaction 2 | ... |

		Explanation:
		- The first 4 bytes represent the total number of transactions (NumTransactions).
		- Each transaction is preceded by a 4-byte field indicating its length (LengthTransaction).
		- Each transaction data segment follows its respective length field, with the length in bytes specified by LengthTransaction.
		- This pattern repeats for the specified number of transactions.

		Example:
		| 00 00 00 02 | 00 00 00 05 | <Transaction 1 data> | 00 00 00 03 | <Transaction 2 data> |
		- 2 transactions, with lengths 5 and 3 bytes respectively.
	*/

	if len(data) < NumTransactionsBytes+LengthTransactionBytes {
		return nil, fmt.Errorf("insufficient data length to contain transaction details")
	}

	numTransactions := int(binary.BigEndian.Uint32(data[:NumTransactionsBytes+LengthTransactionBytes]))
	pos := NumTransactionsBytes
	transactions := make([]hexutil.Bytes, 0, numTransactions)

	for i := 0; i < numTransactions; i++ {
		if pos+LengthTransactionBytes-1 > len(data) {
			return nil, fmt.Errorf("data truncated before expected transaction length")
		}
		lengthTransaction := int(binary.BigEndian.Uint32(data[pos : pos+LengthTransactionBytes]))
		pos += LengthTransactionBytes

		if pos+lengthTransaction > len(data) {
			return nil, fmt.Errorf("transaction data length exceeds data boundary")
		}

		transactions = append(transactions, data[pos:pos+lengthTransaction])
		pos += lengthTransaction
	}
	return transactions, nil
}
