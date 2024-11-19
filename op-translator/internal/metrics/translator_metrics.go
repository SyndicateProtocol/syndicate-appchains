package metrics

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

const opTranslatorNamespace = "op_translator"

type IOPTranslatorMetrics interface {
	RecordRPCRequest(method string)
	RecordTranslationLatency(method string, duration float64)
	RecordError(method, errorType string)
}

type OPTranslatorMetrics struct {
	rpcRequests        *prometheus.CounterVec
	translationLatency *prometheus.HistogramVec
	errors             *prometheus.CounterVec
}

func NewOPTranslatorMetrics() *OPTranslatorMetrics {
	return &OPTranslatorMetrics{
		rpcRequests: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: opTranslatorNamespace,
				Name:      "rpc_requests_total",
				Help:      "Total number of RPC requests processed",
			},
			[]string{"method"},
		),
		translationLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: opTranslatorNamespace,
				Name:      "translation_duration_seconds",
				Help:      "Time taken to translate blocks",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		errors: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: opTranslatorNamespace,
				Name:      "errors_total",
				Help:      "Total number of errors encountered",
			},
			[]string{"method", "error_type"},
		),
	}
}

func (m *OPTranslatorMetrics) RecordRPCRequest(method string) {
	m.rpcRequests.WithLabelValues(method).Inc()
}

func (m *OPTranslatorMetrics) RecordTranslationLatency(method string, duration float64) {
	m.translationLatency.WithLabelValues(method).Observe(duration)
}

func (m *OPTranslatorMetrics) RecordError(method, errorType string) {
	m.errors.WithLabelValues(method, errorType).Inc()
}
