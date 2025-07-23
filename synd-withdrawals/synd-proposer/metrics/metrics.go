package metrics

import (
	"time"

	"github.com/prometheus/client_golang/prometheus"
)

// Metrics holds all Prometheus metrics for the proposer
type Metrics struct {
	// Polling loop metrics
	PollingLoopDuration prometheus.Histogram

	// Prove function metrics
	ProveTotal      prometheus.Counter
	ProveBatchCount prometheus.Gauge

	// Assertion submission metrics
	AssertionSubmissions        prometheus.Counter
	AssertionSubmissionDuration prometheus.Histogram

	// Enclave call metrics
	EnclaveCalls        *prometheus.CounterVec
	EnclaveCallDuration *prometheus.HistogramVec
}

// NewMetrics creates and registers all Prometheus metrics
func NewMetrics(registry *prometheus.Registry) *Metrics {
	metrics := &Metrics{
		// Polling loop metrics
		PollingLoopDuration: prometheus.NewHistogram(prometheus.HistogramOpts{
			Name:    "synd_proposer_polling_loop_duration_seconds",
			Help:    "Duration of polling loop iterations",
			Buckets: prometheus.DefBuckets,
		}),
		// Prove function metrics
		ProveTotal: prometheus.NewCounter(prometheus.CounterOpts{
			Name: "synd_proposer_prove_total",
			Help: "Total number of proofs generated",
		}),
		ProveBatchCount: prometheus.NewGauge(prometheus.GaugeOpts{
			Name: "synd_proposer_prove_batch_count",
			Help: "Number of batches processed in last proof",
		}),

		// Assertion submission metrics
		AssertionSubmissions: prometheus.NewCounter(prometheus.CounterOpts{
			Name: "synd_proposer_assertion_submissions_total",
			Help: "Total number of assertion submissions",
		}),
		AssertionSubmissionDuration: prometheus.NewHistogram(prometheus.HistogramOpts{
			Name:    "synd_proposer_assertion_submission_duration_seconds",
			Help:    "Duration of assertion submissions",
			Buckets: []float64{0.1, 0.5, 1, 2, 5, 10, 30},
		}),

		// Enclave call metrics
		EnclaveCalls: prometheus.NewCounterVec(prometheus.CounterOpts{
			Name: "synd_proposer_enclave_calls_total",
			Help: "Total number of enclave calls",
		}, []string{"method"}),
		EnclaveCallDuration: prometheus.NewHistogramVec(prometheus.HistogramOpts{
			Name:    "synd_proposer_enclave_call_duration_seconds",
			Help:    "Duration of enclave calls",
			Buckets: []float64{0.1, 0.5, 1, 5, 10, 30, 60, 120},
		}, []string{"method"}),
	}

	// Register all metrics
	registry.MustRegister(
		metrics.PollingLoopDuration,
		metrics.ProveTotal,
		metrics.ProveBatchCount,
		metrics.AssertionSubmissions,
		metrics.AssertionSubmissionDuration,
		metrics.EnclaveCalls,
		metrics.EnclaveCallDuration,
	)

	return metrics
}

// Timer helper for measuring durations
type Timer struct {
	start time.Time
}

// NewTimer creates a new timer
func NewTimer() *Timer {
	return &Timer{start: time.Now()}
}

// ObserveHistogram records the duration in a histogram
func (t *Timer) ObserveHistogram(histogram prometheus.Histogram) {
	histogram.Observe(time.Since(t.start).Seconds())
}

// ObserveHistogramVec records the duration in a histogram vec
func (t *Timer) ObserveHistogramVec(histogram prometheus.Observer) {
	histogram.Observe(time.Since(t.start).Seconds())
}

// Duration returns the elapsed time since the timer was created
func (t *Timer) Duration() time.Duration {
	return time.Since(t.start)
}
