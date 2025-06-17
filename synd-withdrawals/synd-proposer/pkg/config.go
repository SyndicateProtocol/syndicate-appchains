package pkg

import (
	"fmt"
	"log"
	"os"
	"time"
)

type Config struct {
	EthereumRPCURL           string
	SettlementRPCURL         string
	SequencingRPCURL         string
	AppchainRPCURL           string
	EnclaveRPCURL            string
	TeeModuleContractAddress string
	ArbitrumBridgeAddress    string
	InboxAddress             string
	SequencerInboxAddress    string
	PrivateKey               string
	PollingInterval          time.Duration
	CloseChallengeInterval   time.Duration
	Port                     int
	MetricsPort              int
}

func LoadConfig() (*Config, error) {
	getEnv := func(key string, required bool) string {
		val := os.Getenv(key)
		if required && val == "" {
			log.Fatalf("missing required environment variable: %s", key)
		}
		return val
	}
	parseDuration := func(key, def string) time.Duration {
		val := os.Getenv(key)
		if val == "" {
			val = def
		}
		d, err := time.ParseDuration(val)
		if err != nil {
			log.Fatalf("invalid duration for %s: %v", key, err)
		}
		return d
	}
	parseInt := func(key string, def int) int {
		val := os.Getenv(key)
		if val == "" {
			return def
		}
		var i int
		_, err := fmt.Sscanf(val, "%d", &i)
		if err != nil {
			log.Fatalf("invalid int for %s: %v", key, err)
		}
		return i
	}
	return &Config{
		EthereumRPCURL:           getEnv("ETH_RPC_URL", true),
		SettlementRPCURL:         getEnv("SETTLEMENT_RPC_URL", true),
		SequencingRPCURL:         getEnv("SEQUENCING_RPC_URL", true),
		AppchainRPCURL:           getEnv("APPCHAIN_RPC_URL", true),
		EnclaveRPCURL:            getEnv("ENCLAVE_RPC_URL", true),
		TeeModuleContractAddress: getEnv("TEE_MODULE_CONTRACT_ADDRESS", true),
		ArbitrumBridgeAddress:    getEnv("ARBITRUM_BRIDGE_ADDRESS", true),
		InboxAddress:             getEnv("INBOX_ADDRESS", true),
		SequencerInboxAddress:    getEnv("SEQUENCER_INBOX_ADDRESS", true),
		PrivateKey:               getEnv("PROPOSER_PRIVATE_KEY", true),
		PollingInterval:          parseDuration("PROPOSER_POLLING_INTERVAL", "10m"),
		CloseChallengeInterval:   parseDuration("PROPOSER_CLOSE_CHALLENGE_INTERVAL", "5m"),
		Port:                     parseInt("PROPOSER_PORT", 8080),
		MetricsPort:              parseInt("PROPOSER_METRICS_PORT", 9292),
	}, nil
}
