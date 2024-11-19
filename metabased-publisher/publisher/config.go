package publisher

import (
	"errors"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/flags"
	"github.com/urfave/cli/v2"

	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
)

type CLIConfig struct {
	BatcherAddress     string
	BatchInboxAddress  string
	OpTranslatorRPCURL string
	AltDAURL           string
	PprofConfig        oppprof.CLIConfig
	LogConfig          oplog.CLIConfig
	MetricsConfig      opmetrics.CLIConfig
	PollInterval       time.Duration
	NetworkTimeout     time.Duration
}

func (c *CLIConfig) Check() error {
	// op-translator
	if c.OpTranslatorRPCURL == "" {
		return errors.New("empty op-translator RPC URL")
	}

	if c.BatcherAddress == "" {
		return errors.New("empty batcher address")
	}
	if c.BatchInboxAddress == "" {
		return errors.New("empty batch inbox address")
	}

	// AltDA
	if c.AltDAURL == "" {
		return errors.New("empty AltDA URL")
	}

	// operational configuration
	if c.PollInterval == 0 {
		return errors.New("poll interval not set")
	}

	// from op-stack
	if err := c.MetricsConfig.Check(); err != nil {
		return err
	}
	if err := c.PprofConfig.Check(); err != nil {
		return err
	}
	return nil
}

// NewConfig parses the Config from the provided flags or environment variables.
func NewConfig(ctx *cli.Context) *CLIConfig {
	return &CLIConfig{
		// op-translator
		OpTranslatorRPCURL: ctx.String(flags.OpTranslatorRPCURL.Name),
		BatcherAddress:     ctx.String(flags.BatcherAddress.Name),
		BatchInboxAddress:  ctx.String(flags.BatchInboxAddress.Name),

		// AltDA
		AltDAURL: ctx.String(flags.AltDAURL.Name),

		// operational configuration
		PollInterval:   ctx.Duration(flags.PollInterval.Name),
		NetworkTimeout: ctx.Duration(flags.NetworkTimeout.Name),

		// from op-stack
		LogConfig:     oplog.ReadCLIConfig(ctx),
		MetricsConfig: opmetrics.ReadCLIConfig(ctx),
		PprofConfig:   oppprof.ReadCLIConfig(ctx),
	}
}
