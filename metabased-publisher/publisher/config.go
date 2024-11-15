package publisher

import (
	"errors"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/flags"
	"github.com/urfave/cli/v2"

	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
	"github.com/ethereum-optimism/optimism/op-service/txmgr"
)

type CLIConfig struct {
	SettlementChainRPCURL     string
	SequencingChainRPCURL     string
	SequencingContractAddress string
	L3RPCURL                  string
	AltDAURL                  string
	PprofConfig               oppprof.CLIConfig
	LogConfig                 oplog.CLIConfig
	MetricsConfig             opmetrics.CLIConfig
	TxMgrConfig               txmgr.CLIConfig
	PollInterval              time.Duration
}

func (c *CLIConfig) Check() error {
	// settlement chain
	if c.SettlementChainRPCURL == "" {
		return errors.New("empty settlement chain RPC URL")
	}

	// sequencing chain
	if c.SequencingChainRPCURL == "" {
		return errors.New("empty sequencing chain RPC URL")
	}
	if c.SequencingContractAddress == "" {
		return errors.New("empty sequencing contract address")
	}

	// L3 metabased chain
	if c.L3RPCURL == "" {
		return errors.New("empty L3 RPC URL")
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
	if err := c.TxMgrConfig.Check(); err != nil {
		return err
	}
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
	// override L1_RPC flag with the settlement chain RPC URL
	settlementChainRPCURL := ctx.String(flags.SettlementChainRPCURL.Name)
	txMgrConfig := txmgr.ReadCLIConfig(ctx)
	txMgrConfig.L1RPCURL = settlementChainRPCURL

	return &CLIConfig{
		// settlement chain
		SettlementChainRPCURL: settlementChainRPCURL,

		// sequencing chain
		SequencingChainRPCURL:     ctx.String(flags.SequencingChainRPCURL.Name),
		SequencingContractAddress: ctx.String(flags.SequencingContractAddress.Name),

		// L3 metabased chain
		L3RPCURL: ctx.String(flags.L3RPCURL.Name),

		// AltDA
		AltDAURL: ctx.String(flags.AltDAURL.Name),

		// operational configuration
		PollInterval: ctx.Duration(flags.PollInterval.Name),

		// from op-stack
		LogConfig:     oplog.ReadCLIConfig(ctx),
		MetricsConfig: opmetrics.ReadCLIConfig(ctx),
		PprofConfig:   oppprof.ReadCLIConfig(ctx),
		TxMgrConfig:   txMgrConfig,
	}
}
