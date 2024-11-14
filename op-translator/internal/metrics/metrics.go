package metrics

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

const Namespace = "op_translator"

type Metrics interface {
	RecordRPCRequest(method string)
	RecordTranslationLatency(method string, duration float64)
	RecordBatchSize(size int)
	RecordError(method, errorType string)
}

type PrometheusMetrics struct {
	rpcRequests        *prometheus.CounterVec
	translationLatency *prometheus.HistogramVec
	batchSizes         prometheus.Histogram
	errors             *prometheus.CounterVec
}

func NewMetrics() *PrometheusMetrics {
	return &PrometheusMetrics{
		rpcRequests: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: Namespace,
				Name:      "rpc_requests_total",
				Help:      "Total number of RPC requests processed",
			},
			[]string{"method"},
		),
		translationLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: Namespace,
				Name:      "translation_duration_seconds",
				Help:      "Time taken to translate blocks",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		batchSizes: promauto.NewHistogram(
			prometheus.HistogramOpts{
				Namespace: Namespace,
				Name:      "batch_size_bytes",
				Help:      "Size of processed batches in bytes",
				Buckets:   []float64{1000, 5000, 10000, 50000, 100000, 500000},
			},
		),
		errors: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: Namespace,
				Name:      "errors_total",
				Help:      "Total number of errors encountered",
			},
			[]string{"method", "error_type"},
		),
	}
}

// Total number of RPC requests by method
func (m *PrometheusMetrics) RecordRPCRequest(method string) {
	m.rpcRequests.WithLabelValues(method).Inc()
}

// Translation operation latencies with distribution
func (m *PrometheusMetrics) RecordTranslationLatency(method string, duration float64) {
	m.translationLatency.WithLabelValues(method).Observe(duration)
}

// Batch size distributions
func (m *PrometheusMetrics) RecordBatchSize(size int) {
	m.batchSizes.Observe(float64(size))
}

// Error counts by method and type
func (m *PrometheusMetrics) RecordError(method, errorType string) {
	m.errors.WithLabelValues(method, errorType).Inc()
}
