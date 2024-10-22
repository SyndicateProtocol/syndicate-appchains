package translator

import (
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/rs/zerolog/log"
)

type TransactionProcessed struct {
	EncodedTxn []byte
	Sender     common.Address // indexed
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
