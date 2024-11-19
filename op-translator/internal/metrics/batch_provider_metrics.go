package metrics

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

const batchProviderNamespace = "batch_provider"

type IMetaBasedBatchProviderMetrics interface {
	RecordBatchProcessed(method string)
	RecordBatchProcessingDuration(method string, duration float64)
	RecordError(method, errorType string)
	RecordInvalidTransactionsCount(state string, count int)
}

type MetaBasedBatchProviderMetrics struct {
	batchProcessed           *prometheus.CounterVec
	batchProcessingDuration  *prometheus.HistogramVec
	errors                   *prometheus.CounterVec
	invalidTransactionsCount *prometheus.CounterVec
}

func NewMetaBasedBatchProviderMetrics() *MetaBasedBatchProviderMetrics {
	return &MetaBasedBatchProviderMetrics{
		batchProcessed: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "batch_processed_total",
				Help:      "Total number of batches processed",
			},
			[]string{"method"},
		),
		batchProcessingDuration: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: batchProviderNamespace,
				Name:      "batch_process_duration_seconds",
				Help:      "Duration of different batch processing methods",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		errors: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "errors_total",
				Help:      "Total number of batch processing errors encountered",
			},
			[]string{"method", "error_type"},
		),
		invalidTransactionsCount: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: batchProviderNamespace,
				Name:      "invalid_transactions_count",
				Help:      "Total number of stateless and stateful invalid transactions removed",
			},
			[]string{"state"},
		),
	}
}

func (m *MetaBasedBatchProviderMetrics) RecordBatchProcessed(method string) {
	m.batchProcessed.WithLabelValues(method).Inc()
}

func (m *MetaBasedBatchProviderMetrics) RecordBatchProcessingDuration(method string, duration float64) {
	m.batchProcessingDuration.WithLabelValues(method).Observe(duration)
}

func (m *MetaBasedBatchProviderMetrics) RecordError(method, errorType string) {
	m.errors.WithLabelValues(method, errorType).Inc()
}

func (m *MetaBasedBatchProviderMetrics) RecordInvalidTransactionsCount(state string, count int) {
	m.invalidTransactionsCount.WithLabelValues(state).Add(float64(count))
}
