package server

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/metrics"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/rs/zerolog/log"
)

// Server wraps HTTP server with its registry
type Server struct {
	*http.Server
	Registry *prometheus.Registry
}

// InitServer creates and configures the HTTP server with metrics
func InitServer(port int) *Server {
	registry := prometheus.NewRegistry()
	mux := http.NewServeMux()

	// Health endpoint
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"ok"}`))
	})

	// Metrics endpoint - use the custom registry
	mux.Handle("/metrics", promhttp.HandlerFor(registry, promhttp.HandlerOpts{
		EnableOpenMetrics: true,
	}))

	// Default handler for unknown paths
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.NotFound(w, r)
			return
		}
		w.Header().Set("Content-Type", "text/plain")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Syndicate Proposer\nAvailable endpoints:\n- /health\n- /metrics\n"))
	})

	return &Server{
		Server: &http.Server{
			Addr:    fmt.Sprintf(":%d", port),
			Handler: mux,
		},
		Registry: registry,
	}
}

// Start begins listening on the configured port
func (s *Server) Start(ctx context.Context) error {
	errCh := make(chan error, 1)

	go func() {
		if err := s.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			errCh <- err
		}
	}()

	go func() {
		<-ctx.Done()
		s.Shutdown(context.Background())
	}()

	// Check for immediate errors
	select {
	case err := <-errCh:
		return err
	default:
		return nil
	}
}

// RunProposer runs the `synd-proposer` service with the given configuration
func (s *Server) RunProposer(ctx context.Context, cfg *config.Config) error {
	log.Info().Int("port", cfg.Port).Msg("Server listening on /health and /metrics")

	// Create metrics and proposer
	m := metrics.NewMetrics(s.Registry)
	proposer := pkg.NewProposer(ctx, cfg, m)

	// Run the proposer
	proposer.Run(ctx)

	log.Info().Msg("Synd-proposer service stopped.")
	return nil
}
