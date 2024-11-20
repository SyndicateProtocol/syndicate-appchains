package metrics

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

// Namespaces are prepended to all metric names in prometheus
const (
	opTranslatorNamespace  = "op_translator"
	batchProviderNamespace = "batch_provider"
)

type IMetrics interface {
	// OP Translator
	RecordOPTranslatorRPCRequest(method string)
	RecordOPTranslatorTranslationLatency(method string, duration float64)
	RecordOPTranslatorError(method, errorType string)

	// Batch Provider
	RecordBatchProviderBatchProcessed(method string)
	RecordBatchProviderBatchProcessingDuration(method string, duration float64)
	RecordBatchProviderError(method, errorType string)
	RecordBatchProviderInvalidTransactionsCount(state string, count int)
}

type Metrics struct {
	// OP Translator
	opTranslatorRPCRequests        *prometheus.CounterVec
	opTranslatorTranslationLatency *prometheus.HistogramVec
	opTranslatorErrors             *prometheus.CounterVec

	// Batch Provider
	batchProviderBatchProcessed           *prometheus.CounterVec
	batchProviderBatchProcessingDuration  *prometheus.HistogramVec
	batchProviderErrors                   *prometheus.CounterVec
	batchProviderInvalidTransactionsCount *prometheus.CounterVec
}

func NewMetrics() *Metrics {
	return &Metrics{
		// OP Translator
		opTranslatorRPCRequests: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: opTranslatorNamespace,
				Name:      "rpc_requests_total",
				Help:      "Total number of RPC requests processed",
			},
			[]string{"method"},
		),
		opTranslatorTranslationLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: opTranslatorNamespace,
				Name:      "translation_duration_seconds",
				Help:      "Time taken to translate blocks",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		opTranslatorErrors: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: opTranslatorNamespace,
				Name:      "errors_total",
				Help:      "Total number of errors encountered",
			},
			[]string{"method", "error_type"},
		),

		// Batch Provider
		batchProviderBatchProcessed: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "batch_processed_total",
				Help:      "Total number of batches processed",
			},
			[]string{"method"},
		),
		batchProviderBatchProcessingDuration: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: batchProviderNamespace,
				Name:      "batch_process_duration_seconds",
				Help:      "Duration of different batch processing methods",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		batchProviderErrors: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "errors_total",
				Help:      "Total number of batch processing errors encountered",
			},
			[]string{"method", "error_type"},
		),
		batchProviderInvalidTransactionsCount: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "invalid_transactions_count",
				Help:      "Total number of stateless and stateful invalid transactions removed",
			},
			[]string{"state"},
		),
	}
}

// OP Translator

func (m *Metrics) RecordOPTranslatorRPCRequest(method string) {
	m.opTranslatorRPCRequests.WithLabelValues(method).Inc()
}

func (m *Metrics) RecordOPTranslatorTranslationLatency(method string, duration float64) {
	m.opTranslatorTranslationLatency.WithLabelValues(method).Observe(duration)
}

func (m *Metrics) RecordOPTranslatorError(method, errorType string) {
	m.opTranslatorErrors.WithLabelValues(method, errorType).Inc()
}

// Batch Provider

func (m *Metrics) RecordBatchProviderBatchProcessed(method string) {
	m.batchProviderBatchProcessed.WithLabelValues(method).Inc()
}

func (m *Metrics) RecordBatchProviderBatchProcessingDuration(method string, duration float64) {
	m.batchProviderBatchProcessingDuration.WithLabelValues(method).Observe(duration)
}

func (m *Metrics) RecordBatchProviderError(method, errorType string) {
	m.batchProviderErrors.WithLabelValues(method, errorType).Inc()
}

func (m *Metrics) RecordBatchProviderInvalidTransactionsCount(state string, count int) {
	m.batchProviderInvalidTransactionsCount.WithLabelValues(state).Add(float64(count))
}
