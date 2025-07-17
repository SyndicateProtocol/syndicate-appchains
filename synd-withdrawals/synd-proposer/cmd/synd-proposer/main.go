package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
)

func main() {
	var rootCmd = &cobra.Command{
		Use:   "synd-proposer",
		Short: "Syndicate's proposer service",
		RunE: func(cmd *cobra.Command, args []string) error {
			viper.AutomaticEnv()
			viper.SetEnvKeyReplacer(strings.NewReplacer("-", "_"))

			config, err := pkg.LoadConfig()
			if err != nil {
				return fmt.Errorf("failed to load config: %w", err)
			}
			log.Printf("Config: %+v\n", config)
			log.Printf("Health server will listen on /health at port %d", config.Port)

			ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
			defer stop()

			healthSrv := &http.Server{
				Addr: fmt.Sprintf(":%d", config.Port),
				Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					if r.URL.Path == "/health" {
						w.Header().Set("Content-Type", "application/json")
						w.WriteHeader(http.StatusOK)
						w.Write([]byte(`{"status":"ok"}`))
						return
					}
					http.NotFound(w, r)
				}),
			}
			go func() {
				<-ctx.Done()
				healthSrv.Shutdown(context.Background())
			}()
			go func() {
				if err := healthSrv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
					log.Printf("health server error: %v", err)
				}
			}()

			proposer := pkg.NewProposer(ctx, config)
			proposer.Run(ctx)
			log.Println("Synd-proposer service stopped.")
			return nil
		},
	}

	pkg.BindFlags(rootCmd.Flags())

	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}
