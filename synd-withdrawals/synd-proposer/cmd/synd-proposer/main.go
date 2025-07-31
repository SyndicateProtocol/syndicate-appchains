package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/logger"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/server"
	"github.com/rs/zerolog/log"
)

func main() {

	rootCmd := &cobra.Command{
		Use:   "synd-proposer",
		Short: "Syndicate's Proposer service",
		RunE: func(cmd *cobra.Command, args []string) error {
			viper.AutomaticEnv()
			viper.SetEnvKeyReplacer(strings.NewReplacer("-", "_"))
			logger.Init()

			cfg, err := config.LoadConfig()
			if err != nil {
				return fmt.Errorf("failed to load config: %w", err)
			}
			log.Info().Msgf("Config: %+v", cfg)

			ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
			defer stop()

			// Initialize server
			proposerServer := server.InitServer(cfg.Port)

			// Start the server
			if err := proposerServer.Start(ctx); err != nil {
				return fmt.Errorf("failed to start server: %w", err)
			}

			// Run proposer with server
			return proposerServer.RunProposer(ctx, cfg)
		},
	}

	config.BindFlags(rootCmd.Flags())

	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}
