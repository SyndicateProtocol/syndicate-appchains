package config

import (
	"errors"
	"fmt"
	"net/url"
	"os"
	"strings"

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
	settlementChainAddr string `koanf:"settlement_chain_addr"`
	sequencingChainAddr string `koanf:"sequencing_chain_addr"`
	port                int    `koanf:"port"`
	frameSize           int    `koanf:"frame_size"`
}

type IConfig interface {
	SettlementChainAddr() string
	SequencingChainAddr() string
	Port() int
	FrameSize() int
}

// setCLIFlags sets all valid CLI flags for the app
func setCLIFlags(f *pflag.FlagSet) {
	f.Int("port", defaultPort, "Server port number for the app")
	f.String("settlement_chain_addr", "https://sepolia.base.org", "Settlement chain address")
	f.String("sequencing_chain_addr", "https://sepolia.base.org", "Sequencing chain address")
	f.Int("frame_size", defaultFrameSize, "Size of each frame in bytes. Max is 1,000,000")
}

func hydrateFromConfMap(config *Config) {
	config.port = k.Int("port")
	config.settlementChainAddr = k.String("settlement_chain_addr")
	config.sequencingChainAddr = k.String("sequencing_chain_addr")
	config.frameSize = k.Int("frame_size")
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

	var config Config
	hydrateFromConfMap(&config)

	// Validate config
	if err = validateConfigValues(config); err != nil {
		log.Panic().Err(err).Msg("error validating config")
	}
	return &config
}

func validateConfigValues(config Config) (result error) {
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
	return result
}

func (c *Config) SettlementChainAddr() string {
	return c.settlementChainAddr
}

func (c *Config) SequencingChainAddr() string {
	return c.sequencingChainAddr
}

func (c *Config) Port() int {
	return c.port
}

func (c *Config) FrameSize() int {
	return c.frameSize
}
