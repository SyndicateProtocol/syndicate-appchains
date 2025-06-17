package main

import (
	"context"
	"log"
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

	proposer := pkg.NewProposer(config)

	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	proposer.Run(ctx)

	log.Println("Synd-proposer-go service stopped.")
}
