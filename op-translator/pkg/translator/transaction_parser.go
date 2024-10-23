package translator

import (
	"fmt"
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/rs/zerolog/log"
)

const (
	ZlibCM8            = 8
	ZlibCM15           = 15
	VersionBrotli byte = 0x1
	NoCompression byte = 0x0
)

type TransactionProcessed struct {
	EncodedData []byte
	Sender      common.Address // indexed
}

const (
	TransactionProcessedABI  = `[{"anonymous":false,"inputs":[{"indexed":true,"name":"Sender","type":"address"},{"indexed":false,"name":"EncodedTxn","type":"bytes"}],"name":"TransactionProcessed","type":"event"}]`
	TransactionProcessedName = "TransactionProcessed"
	TransactionProcessedSig  = "TransactionProcessed(address,bytes)"
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

func (l *L3TransactionParser) ParseTransactionProcessed(ethLog *ethtypes.Log) (*TransactionProcessed, error) {
	event := TransactionProcessed{}
	event.Sender = common.HexToAddress(ethLog.Topics[1].Hex())

	err := l.sequencingContractABI.UnpackIntoInterface(&event, TransactionProcessedName, ethLog.Data)
	if err != nil {
		return nil, err
	}

	return &event, nil
}

func (l *L3TransactionParser) DecodeTransactionData(data []byte) ([]byte, error) {
	if len(data) == 0 {
		return nil, fmt.Errorf("empty data provided")
	}

	compressionType := data[0]
	compressedData := data[1:] // skip the first byte (compressionType)

	switch {
	case compressionType == NoCompression:
		return compressedData, nil

	case compressionType&0x0F == ZlibCM8 || compressionType&0x0F == ZlibCM15:
		return utils.DecompressZlib(data) // zlib needs the compression type byte

	case compressionType == VersionBrotli:
		return utils.DecompressBrotli(compressedData)

	default:
		return nil, fmt.Errorf("cannot distinguish the compression algo used given type byte %v", compressionType)
	}
}
