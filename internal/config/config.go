package config

import (
	"errors"
	"fmt"
	"net/url"
	"os"
	"strings"

	"github.com/SyndicateProtocol/op-translator/internal/constants"
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
	settlementChainAddr        string `koanf:"settlement_chain_addr"`
	sequencingChainAddr        string `koanf:"sequencing_chain_addr"`
	metaBasedChainAddr         string `koanf:"meta_based_chain_addr"`
	sequencingContractAddress  string `koanf:"sequencing_contract_address"`
	batcherAddress             string `koanf:"batcher_address"`
	batchInboxAddress          string `koanf:"batch_inbox_address"`
	batcherPrivateKey          string `koanf:"batcher_private_key"`
	logLevel                   string `koanf:"log_level"`
	settlementChainID          int64  `koanf:"settlement_chain_id"`
	settlementStartBlock       int    `koanf:"settlement_start_block"`
	sequencingStartBlock       int    `koanf:"sequencing_start_block"`
	sequencePerSettlementBlock int    `koanf:"sequence_per_settlement_block"`
	port                       int    `koanf:"port"`
	frameSize                  int    `koanf:"frame_size"`
	pretty                     bool   `koanf:"pretty"`
}

type IConfig interface {
	SettlementChainAddr() string
	SequencingChainAddr() string
	MetaBasedChainAddr() string
	LogLevel() string
	Port() int
	FrameSize() int
	Pretty() bool

	SequencingContractAddress() string
	BatcherAddress() string
	BatchInboxAddress() string
	BatcherPrivateKey() string
	SettlementChainID() int64
	SettlementStartBlock() int
	SequencingStartBlock() int
	SequencePerSettlementBlock() int
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
	config.port = k.Int("port")
	config.settlementChainAddr = k.String("settlement_chain_addr")
	config.sequencingChainAddr = k.String("sequencing_chain_addr")
	config.metaBasedChainAddr = k.String("meta_based_chain_addr")
	config.frameSize = k.Int("frame_size")
	config.logLevel = k.String("log_level")
	config.pretty = k.Bool("pretty")

	config.sequencingContractAddress = k.String("sequencing_contract_address")
	config.batcherAddress = k.String("batcher_address")
	config.batchInboxAddress = k.String("batch_inbox_address")
	config.settlementStartBlock = k.Int("settlement_start_block")
	config.sequencingStartBlock = k.Int("sequencing_start_block")
	config.sequencePerSettlementBlock = k.Int("sequence_per_settlement_block")
	config.batcherPrivateKey = k.String("batcher_private_key")
	config.settlementChainID = k.Int64("settlement_chain_id")
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
	if config.port <= 0 {
		result = multierror.Append(result, errors.New("port must be a positive number"))
	}

	if config.frameSize <= 0 {
		result = multierror.Append(result, errors.New("frameSize must be a positive number"))
	}

	if config.frameSize > MaxFrameSize {
		result = multierror.Append(result, errors.New("frameSize must be less than maximum"))
	}

	_, err := url.ParseRequestURI(config.sequencingChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for sequencing chain address: %w", err))
	}

	_, err = url.ParseRequestURI(config.settlementChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for settlement chain address: %w", err))
	}

	_, err = url.ParseRequestURI(config.metaBasedChainAddr)
	if err != nil {
		result = multierror.Append(result, fmt.Errorf("invalid URL for meta based chain address: %w", err))
	}

	if !constants.IsValidLogLevel(config.logLevel) {
		result = multierror.Append(result, errors.New("invalid log level"))
	}

	if !common.IsHexAddress(config.sequencingContractAddress) {
		result = multierror.Append(result, errors.New("sequencingContractAddress must be a valid hex address"))
	}

	if !common.IsHexAddress(config.batcherAddress) {
		result = multierror.Append(result, errors.New("batcherAddress must be a valid hex address"))
	}

	if !common.IsHexAddress(config.batchInboxAddress) {
		result = multierror.Append(result, errors.New("batchInboxAddress must be a valid hex address"))
	}

	if config.sequencePerSettlementBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("sequencePerSettlementBlock must be a positive number: %d", config.sequencePerSettlementBlock))
	}

	if config.settlementStartBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("settlementStartBlock must be a positive number: %d", config.settlementStartBlock))
	}

	if config.sequencingStartBlock <= 0 {
		result = multierror.Append(result, fmt.Errorf("sequencingStartBlock must be a positive number: %d", config.sequencingStartBlock))
	}

	if config.batcherPrivateKey == "" {
		result = multierror.Append(result, fmt.Errorf("batcherPrivateKey cannot be blank"))
	}

	if config.settlementChainID <= 0 {
		result = multierror.Append(result, fmt.Errorf("settlementChainID must be a positive number: %d", config.settlementChainID))
	}

	return result
}

func (c *Config) SettlementChainAddr() string {
	return c.settlementChainAddr
}

func (c *Config) SequencingChainAddr() string {
	return c.sequencingChainAddr
}

func (c *Config) MetaBasedChainAddr() string {
	return c.metaBasedChainAddr
}

func (c *Config) Port() int {
	return c.port
}

func (c *Config) FrameSize() int {
	return c.frameSize
}
func (c *Config) LogLevel() string {
	return c.logLevel
}
func (c *Config) Pretty() bool {
	return c.pretty
}

func (c *Config) SequencingContractAddress() string {
	return c.sequencingContractAddress
}

func (c *Config) BatcherAddress() string {
	return c.batcherAddress
}

func (c *Config) BatchInboxAddress() string {
	return c.batchInboxAddress
}

func (c *Config) SettlementStartBlock() int {
	return c.settlementStartBlock
}

func (c *Config) SequencingStartBlock() int {
	return c.sequencingStartBlock
}

func (c *Config) SequencePerSettlementBlock() int {
	return c.sequencePerSettlementBlock
}

func (c *Config) BatcherPrivateKey() string {
	return c.batcherPrivateKey
}

func (c *Config) SettlementChainID() int64 {
	return c.settlementChainID
}
