package translator

import (
	"errors"
	"fmt"
	"net/url"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli/v2"
)

type CLIConfig struct {
	PprofConfig               oppprof.CLIConfig
	LogLevel                  string
	SequencingChainRPCURL     string
	MetaBasedChainRPCURL      string
	MetafillerURL             string
	SequencingContractAddress string
	SettlementChainRPCURL     string
	BatcherPrivateKey         string
	BatchInboxAddress         string
	LogConfig                 oplog.CLIConfig
	SettlementChainID         uint64
	SettlementStartBlock      uint64
	Port                      int
	FrameSize                 int
	SettlementChainBlockTime  time.Duration
	ReadTimeout               time.Duration
	WriteTimeout              time.Duration
	CutoverEpochBlock         uint64
}

func (c *CLIConfig) Check() error {
	var errs []error
	if c.Port <= 0 {
		errs = append(errs, errors.New("port must be a positive number"))
	}

	if c.FrameSize <= 0 {
		errs = append(errs, errors.New("frameSize must be a positive number"))
	}

	if c.FrameSize > flags.MaxFrameSize {
		errs = append(errs, errors.New("frameSize must be less than maximum"))
	}

	_, err := url.ParseRequestURI(c.SequencingChainRPCURL)
	if err != nil {
		errs = append(errs, fmt.Errorf("invalid URL for sequencing chain address: %w", err))
	}

	_, err = url.ParseRequestURI(c.SettlementChainRPCURL)
	if err != nil {
		errs = append(errs, fmt.Errorf("invalid URL for settlement chain address: %w", err))
	}

	_, err = url.ParseRequestURI(c.MetaBasedChainRPCURL)
	if err != nil {
		errs = append(errs, fmt.Errorf("invalid URL for meta based chain address: %w", err))
	}

	_, err = url.ParseRequestURI(c.MetafillerURL)
	if err != nil {
		errs = append(errs, fmt.Errorf("invalid URL for metafiller: %w", err))
	}

	if !common.IsHexAddress(c.SequencingContractAddress) {
		errs = append(errs, errors.New("sequencingContractAddress must be a valid hex address"))
	}

	if !common.IsHexAddress(c.BatchInboxAddress) {
		errs = append(errs, errors.New("batchInboxAddress must be a valid hex address"))
	}

	if c.SettlementStartBlock <= 0 {
		errs = append(errs, fmt.Errorf("settlementStartBlock must be a positive number: %d", c.SettlementStartBlock))
	}

	if c.BatcherPrivateKey == "" {
		errs = append(errs, errors.New("batcherPrivateKey cannot be blank"))
	}

	if c.SettlementChainID <= 0 {
		errs = append(errs, fmt.Errorf("settlementChainID must be a positive number: %d", c.SettlementChainID))
	}

	if c.SettlementChainBlockTime <= 0 {
		errs = append(errs, fmt.Errorf("settlementChainBlockTime must be positive: %d", c.SettlementChainBlockTime))
	}

	if err := c.PprofConfig.Check(); err != nil {
		errs = append(errs, err)
	}

	return errors.Join(errs...)
}

func NewConfig(ctx *cli.Context) *CLIConfig {
	return &CLIConfig{
		// required
		SettlementChainRPCURL:     ctx.String(flags.SettlementChainRPCURL.Name),
		SequencingChainRPCURL:     ctx.String(flags.SequencingChainRPCURL.Name),
		MetaBasedChainRPCURL:      ctx.String(flags.MetaBasedChainRPCURL.Name),
		MetafillerURL:             ctx.String(flags.MetafillerURL.Name),
		SequencingContractAddress: ctx.String(flags.SequencingContractAddress.Name),
		BatchInboxAddress:         ctx.String(flags.BatchInboxAddress.Name),
		BatcherPrivateKey:         ctx.String(flags.BatcherPrivateKey.Name),
		SettlementChainID:         ctx.Uint64(flags.SettlementChainID.Name),
		CutoverEpochBlock:         ctx.Uint64(flags.CutoverEpochBlock.Name),
		SettlementStartBlock:      ctx.Uint64(flags.SettlementStartBlock.Name),
		SettlementChainBlockTime:  ctx.Duration(flags.SettlementChainBlockTime.Name),

		// optional
		Port:         ctx.Int(flags.Port.Name),
		FrameSize:    ctx.Int(flags.FrameSize.Name),
		ReadTimeout:  ctx.Duration(flags.ReadTimeout.Name),
		WriteTimeout: ctx.Duration(flags.WriteTimeout.Name),

		// from op-stack
		LogConfig:   oplog.ReadCLIConfig(ctx),
		PprofConfig: oppprof.ReadCLIConfig(ctx),
	}
}
