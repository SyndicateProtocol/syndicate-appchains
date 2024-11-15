package metrics

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

const Namespace = "op_translator"

type IMetrics interface {
	RecordRPCRequest(method string)
	RecordTranslationLatency(method string, duration float64)
	RecordError(method, errorType string)
}

type PrometheusMetrics struct {
	rpcRequests        *prometheus.CounterVec
	translationLatency *prometheus.HistogramVec
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

// Error counts by method and type
func (m *PrometheusMetrics) RecordError(method, errorType string) {
	m.errors.WithLabelValues(method, errorType).Inc()
}
