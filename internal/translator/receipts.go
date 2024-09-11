package translator

import (
	"fmt"

	"github.com/SyndicateProtocol/op-translator/contracts/l2"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/hashicorp/go-multierror"
)

var (
	// ref: OP `crossdomain` package
	AddressType, _              = abi.NewType("address", "", nil)
	indexedAddressTypeArgs      = abi.Arguments{{Name: "Sender", Type: AddressType, Indexed: true}}
	TransactionProcessedABI     = l2.TransactionProcessedMetaData.ABI
	TransactionProcessedABIHash = crypto.Keccak256Hash([]byte(TransactionProcessedABI))
)

// ParseTransactionProcessed is a method to parse a log into the TransactionProcessed event
func ParseTransactionProcessed(log *types.Log) (*l2.TransactionProcessed, error) {
	event := new(l2.TransactionProcessed)
	if err := abi.ParseTopics(event, indexedAddressTypeArgs, log.Topics[1:]); err != nil {
		return nil, err
	}
	event.EncodedTxn = log.Data
	return event, nil
}

func FilterReceipts(receipts []*types.Receipt, sequencingContractAddr common.Address) (txns []hexutil.Bytes, result error) {
	for i, rec := range receipts {
		if rec.Status != types.ReceiptStatusSuccessful {
			continue
		}
		for j, log := range rec.Logs {
			if log.Address == sequencingContractAddr && len(log.Topics) > 0 && log.Topics[0] == TransactionProcessedABIHash {
				proc, err := ParseTransactionProcessed(log)
				if err != nil {
					result = multierror.Append(result, fmt.Errorf("malformatted l2 receipt log in receipt %d, log %d: %w", i, j, err))
				} else {
					txns = append(txns, proc.EncodedTxn)
				}
			}
		}
	}
	return txns, result
}
