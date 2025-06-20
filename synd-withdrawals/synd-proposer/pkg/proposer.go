package pkg

import (
	"context"
	"log"
	"math/big"
	"sync"
	"time"

	"github.com/SyndicateProtocol/synd-proposer/teemodule"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
)

type Proposer struct {
	Config           *Config
	AppchainClient   *ethclient.Client
	SequencingClient *ethclient.Client
	EthereumClient   *ethclient.Client
	SettlementClient *ethclient.Client
	SettlementAuth   *bind.TransactOpts
	EnclaveClient    *rpc.Client
	TeeModule        *teemodule.Teemodule
}

func NewProposer(cfg *Config) *Proposer {
	appchainClient, err := ethclient.Dial(cfg.AppchainRPCURL)
	if err != nil {
		log.Fatalf("Failed to create appchain provider: %v", err)
		return nil
	}
	sequencingClient, err := ethclient.Dial(cfg.SequencingRPCURL)
	if err != nil {
		log.Fatalf("Failed to create sequencing provider: %v", err)
		return nil
	}

	ethereumClient, err := ethclient.Dial(cfg.EthereumRPCURL)
	if err != nil {
		log.Fatalf("Failed to create ethereum provider: %v", err)
		return nil
	}
	enclaveClient, err := rpc.Dial(cfg.EnclaveRPCURL)
	if err != nil {
		log.Fatalf("Failed to create enclave provider: %v", err)
		return nil
	}
	settlementClient, err := ethclient.Dial(cfg.SettlementRPCURL)
	if err != nil {
		log.Fatalf("Failed to create settlement provider: %v", err)
		return nil
	}
	pk, err := crypto.HexToECDSA(cfg.PrivateKey)
	if err != nil {
		log.Fatalf("Failed to create private key: %v", err)
		return nil
	}
	settlementAuth, err := bind.NewKeyedTransactorWithChainID(pk, big.NewInt(int64(cfg.SettlementChainID)))
	if err != nil {
		log.Fatalf("Failed to create transactor: %v", err)
		return nil
	}
	teeAddress := common.HexToAddress(cfg.TeeModuleContractAddress)
	teeModule, err := teemodule.NewTeemodule(teeAddress, settlementClient)
	if err != nil {
		log.Fatalf("Failed to create tee module: %v", err)
		return nil
	}

	return &Proposer{
		Config:           cfg,
		AppchainClient:   appchainClient,
		SequencingClient: sequencingClient,
		EthereumClient:   ethereumClient,
		EnclaveClient:    enclaveClient,
		SettlementClient: settlementClient,
		SettlementAuth:   settlementAuth,
		TeeModule:        teeModule,
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
			p.TeeModule.CloseChallengeWindow(p.SettlementAuth)
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
