package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os/signal"
	"syscall"

	"github.com/SyndicateProtocol/synd-proposer/pkg"
)

func main() {
	log.Println("Starting synd-proposer-go service...")

	config, err := pkg.LoadConfig()
	if err != nil {
		log.Fatalf("failed to load config: %v", err)
	}
	log.Printf("Config: %+v\n", config)
	log.Printf("Health server will listen on /health at port %d", config.MetricsPort)

	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	// Start health server
	healthSrv := &http.Server{
		Addr: fmt.Sprintf(":%d", config.MetricsPort),
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

	proposer := pkg.NewProposer(config)
	proposer.Run(ctx)

	log.Println("Synd-proposer-go service stopped.")
}
