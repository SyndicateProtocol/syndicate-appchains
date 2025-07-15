package pkg

import (
	"crypto/ecdsa"
	"fmt"
	"strconv"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/spf13/pflag"
	"github.com/spf13/viper"
)

type Config struct {
	EthereumRPCURL    string
	SettlementRPCURL  string
	SettlementChainID uint64

	SequencingRPCURL string
	AppchainRPCURL   string
	EnclaveRPCURL    string
	EigenRPCUrl      string

	PrivateKey             *ecdsa.PrivateKey
	PollingInterval        time.Duration
	CloseChallengeInterval time.Duration
	Port                   int

	TeeModuleContractAddress common.Address

	AppchainBridgeAddress common.Address

	IsL1Chain bool

	EnclaveConfig    enclave.Config
	EnclaveTLSConfig TLSConfig
}

var ConfigKeys = map[string]struct {
	Description string
	Default     string
	Required    bool
}{
	"ethereum-rpc-url":            {"Ethereum RPC URL", "", true},
	"settlement-rpc-url":          {"Settlement RPC URL", "", true},
	"settlement-chain-id":         {"Settlement Chain ID", "", true},
	"sequencing-rpc-url":          {"Sequencing RPC URL", "", true},
	"appchain-rpc-url":            {"Appchain RPC URL", "", true},
	"eigen-rpc-url":               {"EigenDA RPC URL", "", true},
	"enclave-rpc-url":             {"Enclave RPC URL", "", true},
	"tee-module-contract-address": {"TEE Module Contract Address", "", true},
	"appchain-bridge-address":     {"Appchain Bridge Address", "", true},
	"private-key":                 {"Private Key", "", true},
	"polling-interval":            {"Polling interval", "10m", false},
	"close-challenge-interval":    {"Close challenge interval", "5m", false},
	"port":                        {"health and metrics port", "9292", false},
	"sequencing-contract-address": {"Sequencing Contract Address", "", true},
	"sequencing-bridge-address":   {"Sequencing Bridge Address", "", true},
	"settlement-delay":            {"Settlement Delay", "60", false},
	"mtls-client-cert-path":       {"mTLS client certificate path", "/etc/tls/tls.crt", false},
	"mtls-client-key-path":        {"mTLS client private key path", "/etc/tls/tls.key", false},
	"mtls-enabled-enclave":        {"mTLS enabled for enclave", "true", false},
}

func BindFlags(flags *pflag.FlagSet) {
	for key, meta := range ConfigKeys {
		switch key {
		case "port":
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

	port := viper.GetInt("port")

	privateKey, err := crypto.HexToECDSA(viper.GetString("private-key"))
	if err != nil {
		return nil, fmt.Errorf("invalid private-key: %v", err)
	}

	return &Config{
		EthereumRPCURL:           viper.GetString("ethereum-rpc-url"),
		SettlementRPCURL:         viper.GetString("settlement-rpc-url"),
		SettlementChainID:        viper.GetUint64("settlement-chain-id"),
		SequencingRPCURL:         viper.GetString("sequencing-rpc-url"),
		AppchainRPCURL:           viper.GetString("appchain-rpc-url"),
		EnclaveRPCURL:            viper.GetString("enclave-rpc-url"),
		EigenRPCUrl:              viper.GetString("eigen-rpc-url"),
		TeeModuleContractAddress: common.HexToAddress(viper.GetString("tee-module-contract-address")),
		AppchainBridgeAddress:    common.HexToAddress(viper.GetString("appchain-bridge-address")),
		PrivateKey:               privateKey,
		PollingInterval:          pollingInterval,
		CloseChallengeInterval:   closeChallengeInterval,
		Port:                     port,
		EnclaveConfig: enclave.Config{
			SequencingContractAddress: common.HexToAddress(viper.GetString("sequencing-contract-address")),
			SequencingBridgeAddress:   common.HexToAddress(viper.GetString("sequencing-bridge-address")),
			SettlementDelay:           viper.GetUint64("settlement-delay"),
		},
		EnclaveTLSConfig: TLSConfig{
			Enabled:        viper.GetBool("mtls-enabled-enclave"),
			ClientCertPath: viper.GetString("mtls-client-cert-path"),
			ClientKeyPath:  viper.GetString("mtls-client-key-path"),
		},
	}, nil
}

func mustAtoi(s string, fallback int) int {
	i, err := strconv.Atoi(s)
	if err != nil || i == 0 {
		return fallback
	}
	return i
}
