package metrics

import (
	"io"
	"math/big"

	"github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/metrics/doc"
	txmetrics "github.com/ethereum-optimism/optimism/op-service/txmgr/metrics"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/log"
)

const Namespace = "mb_publisher"

type Metricer interface {
	doc.Metrics
	RecordInfo(version string)
	RecordUp()

	// opmetrics.RefMetricer // TODO (SEQ-185): implement a custom version that keeps track of the 3 chains

	// records tx data
	txmetrics.TxMetricer

	StartBalanceMetrics(l log.Logger, client *ethclient.Client, account common.Address) io.Closer
}

type Metrics struct{}

var _ Metricer = (*Metrics)(nil)

func NewMetrics(procName string) *Metrics {
	// TODO (SEQ-180): implement metrics
	return &Metrics{}
}

func (m *Metrics) RPCError() {
	panic("unimplemented")
}

func (m *Metrics) RecordBaseFee(*big.Int) {
	panic("unimplemented")
}

func (m *Metrics) RecordBlobBaseFee(*big.Int) {
	panic("unimplemented")
}

func (m *Metrics) RecordGasBumpCount(int) {
	panic("unimplemented")
}

func (m *Metrics) RecordNonce(uint64) {
	panic("unimplemented")
}

func (m *Metrics) RecordPendingTx(pending int64) {
	panic("unimplemented")
}

func (m *Metrics) RecordTipCap(*big.Int) {
	panic("unimplemented")
}

func (m *Metrics) RecordTxConfirmationLatency(int64) {
	panic("unimplemented")
}

func (m *Metrics) TxConfirmed(*types.Receipt) {
	panic("unimplemented")
}

func (m *Metrics) TxPublished(string) {
	panic("unimplemented")
}

func (m *Metrics) RecordInfo(version string) {
	panic("unimplemented")
}

func (m *Metrics) RecordUp() {
	panic("unimplemented")
}

func (m *Metrics) StartBalanceMetrics(l log.Logger, client *ethclient.Client, account common.Address) io.Closer {
	panic("unimplemented")
}

func (m *Metrics) Document() []metrics.DocumentedMetric {
	panic("unimplemented")
}
