package config

import (
	"errors"
	"fmt"
	"net/url"
	"os"
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/ethereum/go-ethereum/common"
	"github.com/hashicorp/go-multierror"
	"github.com/knadh/koanf/parsers/dotenv"
	"github.com/knadh/koanf/providers/file"
	"github.com/rs/zerolog/log"

	"github.com/knadh/koanf/providers/posflag"
	"github.com/knadh/koanf/v2"
	"github.com/spf13/pflag"
)

var (
	k                = koanf.New(".")
	defaultPort      = 8546
	defaultFrameSize = 1024
	// Documentation: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#frame-format
	MaxFrameSize = 1_000_000
)

type Config struct {
	SettlementChainAddr        string `koanf:"settlement_chain_addr"`
	SequencingChainAddr        string `koanf:"sequencing_chain_addr"`
	MetaBasedChainAddr         string `koanf:"meta_based_chain_addr"`
	SequencingContractAddress  string `koanf:"sequencing_contract_address"`
	BatcherAddress             string `koanf:"batcher_address"`
	BatchInboxAddress          string `koanf:"batch_inbox_address"`
	BatcherPrivateKey          string `koanf:"batcher_private_key"`
	LogLevel                   string `koanf:"log_level"`
	SettlementChainID          int64  `koanf:"settlement_chain_id"`
	SettlementStartBlock       int    `koanf:"settlement_start_block"`
	SequencingStartBlock       int    `koanf:"sequencing_start_block"`
	SequencePerSettlementBlock int    `koanf:"sequence_per_settlement_block"`
	Port                       int    `koanf:"port"`
	FrameSize                  int    `koanf:"frame_size"`
	Pretty                     bool   `koanf:"pretty"`
}

// setCLIFlags sets all valid CLI flags for the app
func setCLIFlags(f *pflag.FlagSet) {
	f.Int("port", defaultPort, "Server port number for the app")
	f.String("settlement_chain_addr", "https://sepolia.base.org", "Settlement chain address")
	f.String("sequencing_chain_addr", "https://sepolia.base.org", "Sequencing chain address")
	f.String("meta_based_chain_addr", "https://sepolia.base.org", "Meta based chain address")
	f.String("log_level", constants.Info.String(), "Log level for the app")
	f.Int("frame_size", defaultFrameSize, "Size of each frame in bytes. Max is 1,000,000")
	f.Bool("pretty", false, "Pretty print JSON log responses")
	f.String("sequencing_contract_address", "0x123", "Sequencing contract address")
	f.String("batcher_address", "0x123", "Batcher address")
	f.String("batch_inbox_address", "0x123", "Batch inbox address")
	f.Int("settlement_start_block", 0, "Settlement chain start block")
	f.Int("sequencing_start_block", 0, "Sequencing chain start block")
	f.Int("sequence_per_settlement_block", 0, "Number of sequencing blocks per settlement block")
	f.String("batcher_private_key", "", "Batcher private key")
	f.Int("settlement_chain_id", 1, "Settlement chain id")
}

// hydrateFromConfMap sets the Config values from the koanf conf map
func hydrateFromConfMap(config *Config) {
	config.Port = k.Int("port")
	config.SettlementChainAddr = k.String("settlement_chain_addr")
	config.SequencingChainAddr = k.String("sequencing_chain_addr")
	config.MetaBasedChainAddr = k.String("meta_based_chain_addr")
	config.FrameSize = k.Int("frame_size")
	config.LogLevel = k.String("log_level")
	config.Pretty = k.Bool("pretty")

	config.SequencingContractAddress = k.String("sequencing_contract_address")
	config.BatcherAddress = k.String("batcher_address")
	config.BatchInboxAddress = k.String("batch_inbox_address")
	config.SettlementStartBlock = k.Int("settlement_start_block")
	config.SequencingStartBlock = k.Int("sequencing_start_block")
	config.SequencePerSettlementBlock = k.Int("sequence_per_settlement_block")
	config.BatcherPrivateKey = k.String("batcher_private_key")
	config.SettlementChainID = k.Int64("settlement_chain_id")
}

func Init() *Config {
	// Load .env file and lowercase all keys
	err := k.Load(file.Provider(".env"), dotenv.ParserEnv("", "", strings.ToLower))
	if err != nil {
		log.Error().Err(err).Msg("error loading dotenv config")
	} else {
		log.Debug().Msg("Loaded .env")
	}

	// Define CLI flags
	f := pflag.NewFlagSet("config", pflag.ContinueOnError)
	f.Usage = func() {
		fmt.Println(f.FlagUsages())
		os.Exit(0)
	}
	setCLIFlags(f)
	err = f.Parse(os.Args[1:])
	if err != nil {
		log.Error().Err(err).Msg("Error parsing flags")
	} else {
		log.Debug().Msg("Parsed CLI flags")
	}

	// Load defaults
	if err = k.Load(posflag.Provider(f, ".", k), nil); err != nil {
		log.Error().Err(err).Msg("error loading default values")
	}

	// Set config values
	var config Config
	hydrateFromConfMap(&config)

	// Validate config
	if err = validateConfigValues(&config); err != nil {
		log.Panic().Err(err).Msg("error validating config")
	}
	return &config
}

func validateConfigValues(config *Config) (result error) {
	if config.Port <= 0 {
		result = multierror.Append(result, errors.New("port must be a positive number"))
	}

	if config.FrameSize <= 0 {
		result = multierror.Append(result, errors.New("frameSize must be a positive number"))
	}

	if config.FrameSize > MaxFrameSize {
		result = multierror.Append(result, errors.New("frameSize must be less than maximum"))
	}

	_, err := url.ParseRequestURI(config.SequencingChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for sequencing chain address: %w", err))
	}

	_, err = url.ParseRequestURI(config.SettlementChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for settlement chain address: %w", err))
	}

	_, err = url.ParseRequestURI(config.MetaBasedChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for meta based chain address: %w", err))
	}

	if !constants.IsValidLogLevel(config.LogLevel) {
		result = multierror.Append(result, errors.New("invalid log level"))
	}

	if !common.IsHexAddress(config.SequencingContractAddress) {
		result = multierror.Append(result, errors.New("sequencingContractAddress must be a valid hex address"))
	}

	if !common.IsHexAddress(config.BatcherAddress) {
		result = multierror.Append(result, errors.New("batcherAddress must be a valid hex address"))
	}

	if !common.IsHexAddress(config.BatchInboxAddress) {
		result = multierror.Append(result, errors.New("batchInboxAddress must be a valid hex address"))
	}

	if config.SequencePerSettlementBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("sequencePerSettlementBlock must be a positive number: %d", config.SequencePerSettlementBlock))
	}

	if config.SettlementStartBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("settlementStartBlock must be a positive number: %d", config.SettlementStartBlock))
	}

	if config.SequencingStartBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("sequencingStartBlock must be a positive number: %d", config.SequencingStartBlock))
	}

	if config.BatcherPrivateKey == "" {
		result = multierror.Append(result, fmt.Errorf("batcherPrivateKey cannot be blank"))
	}

	if config.SettlementChainID <= 0 {
		result = multierror.Append(result, fmt.Errorf("settlementChainID must be a positive number: %d", config.SettlementChainID))
	}

	return result
}
