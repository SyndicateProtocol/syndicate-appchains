package interfaces

import "github.com/ethereum-optimism/optimism/op-service/sources"

type IReceiptsFetcher interface {
	PickReceiptsMethod(txCount int) sources.ReceiptsFetchingMethod
	OnReceiptsMethodErr(method sources.ReceiptsFetchingMethod, err error)
}
