package metrics

import (
	"io"
	"math/big"

	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	txmetrics "github.com/ethereum-optimism/optimism/op-service/txmgr/metrics"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/log"
)

type noopMetrics struct {
	opmetrics.NoopRefMetrics
	txmetrics.NoopTxMetrics
	opmetrics.NoopRPCMetrics
}

var NoopMetrics Metricer = new(noopMetrics)

func (n *noopMetrics) StartBalanceMetrics(l log.Logger, client *ethclient.Client, account common.Address) io.Closer {
	return nil
}

func (n *noopMetrics) RPCError()                              {}
func (n *noopMetrics) RecordBaseFee(*big.Int)                 {}
func (n *noopMetrics) RecordBlobBaseFee(*big.Int)             {}
func (n *noopMetrics) RecordGasBumpCount(int)                 {}
func (n *noopMetrics) RecordInfo(version string)              {}
func (n *noopMetrics) RecordNonce(uint64)                     {}
func (n *noopMetrics) RecordPendingTx(pending int64)          {}
func (n *noopMetrics) RecordTipCap(*big.Int)                  {}
func (n *noopMetrics) RecordTxConfirmationLatency(int64)      {}
func (n *noopMetrics) RecordUp()                              {}
func (n *noopMetrics) TxConfirmed(*types.Receipt)             {}
func (n *noopMetrics) TxPublished(string)                     {}
func (n *noopMetrics) Document() []opmetrics.DocumentedMetric { return nil }
