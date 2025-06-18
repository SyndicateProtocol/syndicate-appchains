package pkg

import (
	"fmt"
	"time"

	"github.com/spf13/pflag"
	"github.com/spf13/viper"
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
	MetricsPort              int
}

var ConfigKeys = map[string]struct {
	Description string
	Default     string
	Required    bool
}{
	"ethereum-rpc-url":            {"Ethereum RPC URL", "", true},
	"settlement-rpc-url":          {"Settlement RPC URL", "", true},
	"sequencing-rpc-url":          {"Sequencing RPC URL", "", true},
	"appchain-rpc-url":            {"Appchain RPC URL", "", true},
	"enclave-rpc-url":             {"Enclave RPC URL", "", true},
	"tee-module-contract-address": {"TEE Module Contract Address", "", true},
	"arbitrum-bridge-address":     {"Arbitrum Bridge Address", "", true},
	"inbox-address":               {"Inbox Address", "", true},
	"sequencer-inbox-address":     {"Sequencer Inbox Address", "", true},
	"private-key":                 {"Private Key", "", true},
	"polling-interval":            {"Polling interval", "10m", false},
	"close-challenge-interval":    {"Close challenge interval", "5m", false},
	"metrics-port":                {"Metrics port", "9292", false},
}

func BindFlags(flags *pflag.FlagSet) {
	for key, meta := range ConfigKeys {
		switch key {
		case "metrics-port":
			flags.Int(key, mustAtoi(meta.Default, 9292), meta.Description)
		default:
			flags.String(key, meta.Default, meta.Description)
		}
		if err := viper.BindPFlag(key, flags.Lookup(key)); err != nil {
			panic(fmt.Sprintf("failed to bind flag %s: %v", key, err))
		}
	}
}

func LoadConfig() (*Config, error) {
	for key, meta := range ConfigKeys {
		if meta.Required && (!viper.IsSet(key) || viper.GetString(key) == "") {
			return nil, fmt.Errorf("missing required config: --%s", key)
		}
	}

	pollingInterval, err := time.ParseDuration(viper.GetString("polling-interval"))
	if err != nil {
		return nil, fmt.Errorf("invalid polling-interval: %v", err)
	}

	closeChallengeInterval, err := time.ParseDuration(viper.GetString("close-challenge-interval"))
	if err != nil {
		return nil, fmt.Errorf("invalid close-challenge-interval: %v", err)
	}

	metricsPort := viper.GetInt("metrics-port")
	if metricsPort == 0 {
		metricsPort = 9292
	}

	return &Config{
		EthereumRPCURL:           viper.GetString("ethereum-rpc-url"),
		SettlementRPCURL:         viper.GetString("settlement-rpc-url"),
		SequencingRPCURL:         viper.GetString("sequencing-rpc-url"),
		AppchainRPCURL:           viper.GetString("appchain-rpc-url"),
		EnclaveRPCURL:            viper.GetString("enclave-rpc-url"),
		TeeModuleContractAddress: viper.GetString("tee-module-contract-address"),
		ArbitrumBridgeAddress:    viper.GetString("arbitrum-bridge-address"),
		InboxAddress:             viper.GetString("inbox-address"),
		SequencerInboxAddress:    viper.GetString("sequencer-inbox-address"),
		PrivateKey:               viper.GetString("private-key"),
		PollingInterval:          pollingInterval,
		CloseChallengeInterval:   closeChallengeInterval,
		MetricsPort:              metricsPort,
	}, nil
}

func mustAtoi(s string, fallback int) int {
	var i int
	_, err := fmt.Sscanf(s, "%d", &i)
	if err != nil || i == 0 {
		return fallback
	}
	return i
}
