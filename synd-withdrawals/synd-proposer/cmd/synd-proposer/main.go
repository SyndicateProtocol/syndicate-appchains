package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/logger"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/metrics"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/rs/zerolog/log"
)

func main() {
	logger.Init()

	rootCmd := &cobra.Command{
		Use:   "synd-proposer",
		Short: "Syndicate's Proposer service",
		RunE: func(cmd *cobra.Command, args []string) error {
			viper.AutomaticEnv()
			viper.SetEnvKeyReplacer(strings.NewReplacer("-", "_"))

			config, err := config.LoadConfig()
			if err != nil {
				return fmt.Errorf("failed to load config: %w", err)
			}
			log.Info().Msgf("Config: %+v", config)

			ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
			defer stop()

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

			// TODO(SEQ-1077) should we just remove this?
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

			server := &http.Server{
				Addr:    fmt.Sprintf(":%d", config.Port),
				Handler: mux,
			}

			go func() {
				<-ctx.Done()
				server.Shutdown(context.Background())
			}()
			go func() {
				if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
					log.Error().Err(err).Msg("health server error")
				}
			}()
			log.Info().Int("port", config.Port).Msg("Server listening on /health and /metrics")

			metrics := metrics.NewMetrics(registry)
			proposer := pkg.NewProposer(ctx, config, metrics)
			proposer.Run(ctx)
			log.Info().Msg("Synd-proposer service stopped.")
			return nil
		},
	}

	config.BindFlags(rootCmd.Flags())

	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}
