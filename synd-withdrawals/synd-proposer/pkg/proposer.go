package pkg

import (
	"context"
	"log"
	"sync"
	"time"
)

// Config is defined in config.go in the same package.

// Proposer is the main service struct.
type Proposer struct {
	Config *Config
	// Add other dependencies here as needed
}

// NewProposer creates a new Proposer instance.
func NewProposer(cfg *Config) *Proposer {
	return &Proposer{
		Config: cfg,
	}
}

// Run starts the background processes for the proposer and waits for them to finish.
// It accepts a context and two functions: one for polling and one for close challenge.
func (p *Proposer) Run(ctx context.Context) {
	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		p.pollingLoop(ctx)
	}()

	go func() {
		defer wg.Done()
		p.closeChallengeLoop(ctx)
	}()

	wg.Wait()
}

// CloseChallengeLoop runs the close challenge background process.
func (p *Proposer) closeChallengeLoop(ctx context.Context) {
	ticker := time.NewTicker(p.Config.CloseChallengeInterval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			log.Println("Close challenge loop shutting down...")
			return
		case <-ticker.C:
			log.Println("Close challenge loop tick...")
			// TODO: Implement close challenge logic
		}
	}
}

// PollingLoop runs the polling background process.
func (p *Proposer) pollingLoop(ctx context.Context) {
	ticker := time.NewTicker(p.Config.PollingInterval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			log.Println("Polling loop shutting down...")
			return
		case <-ticker.C:
			log.Println("Polling loop tick...")
			// TODO: Implement polling logic
		}
	}
}
