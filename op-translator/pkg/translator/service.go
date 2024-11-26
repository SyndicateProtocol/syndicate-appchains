package translator

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"net/http"
	"sync/atomic"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/logger"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/server"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
	"github.com/urfave/cli/v2"
)

// Main is the entrypoint into the op-translator.
// This method returns a cliapp.LifecycleAction, to create an op-service CLI-lifecycle-managed batch-submitter with.
func Main(version string) cliapp.LifecycleAction {
	return func(cliCtx *cli.Context, closeApp context.CancelCauseFunc) (cliapp.Lifecycle, error) {
		if err := flags.CheckRequired(cliCtx); err != nil {
			return nil, err
		}
		cfg := NewConfig(cliCtx)
		if err := cfg.Check(); err != nil {
			return nil, fmt.Errorf("invalid CLI flags: %w", err)
		}

		// TODO [SEQ-329] logger
		// l := oplog.NewLogger(oplog.AppOut(cliCtx), cfg.LogConfig)
		// oplog.SetGlobalLogHandler(l.Handler())
		// opservice.ValidateEnvVars(flags.EnvVarPrefix, flags.Flags, log)

		// l.Info("Initializing op-translator")
		return TranslatorServiceFromCLIConfig(cliCtx.Context, version, cfg)
	}
}

func TranslatorServiceFromCLIConfig(ctx context.Context, version string, cfg *CLIConfig) (*TranslatorService, error) {
	var t TranslatorService
	if err := t.initFromCLIConfig(ctx, version, cfg); err != nil {
		return nil, errors.Join(err, t.Stop(ctx)) // try to clean up our failed initialization attempt
	}
	return &t, nil
}

type TranslatorService struct {
	metaBasedChainRPC         *rpc.RPCClient
	sequencingContractAddress *common.Address
	batchInboxAddress         *common.Address
	batcherSigner             *Signer
	settlementChainRPC        *rpc.RPCClient
	sequencingChainRPC        *rpc.RPCClient
	metaBasedBatchProvider    *MetaBasedBatchProvider
	backfillProvider          *backfill.BackfillProvider
	opTranslator              *OPTranslator
	server                    *server.Server
	metricsCollector          *metrics.Metrics
	version                   string
	stopped                   atomic.Bool
}

// guarantees that the cliapp.Lifecycle interface is implemented by TranslatorService
var _ cliapp.Lifecycle = (*TranslatorService)(nil)

func (t *TranslatorService) initFromCLIConfig(ctx context.Context, version string, cfg *CLIConfig) error {
	t.version = version
	logger.Init(cfg.LogLevel, cfg.Pretty) // TODO [SEQ-329] logger

	if err := t.initRPCServers(ctx, cfg); err != nil {
		return fmt.Errorf("failed to initialize RPC servers: %w", err)
	}
	if err := t.initAddresses(cfg); err != nil {
		return fmt.Errorf("failed to initialize addresses: %w", err)
	}
	if err := t.initBatchSigner(cfg); err != nil {
		return fmt.Errorf("failed to initialize batch signer: %w", err)
	}
	t.initMetrics()
	t.initBackfillProvider(cfg)
	t.initBatchProvider(cfg)
	t.initOPTranslator()

	if err := t.initServer(cfg); err != nil {
		return fmt.Errorf("failed to initialize server: %w", err)
	}
	return nil
}

func (t *TranslatorService) initRPCServers(_ context.Context, cfg *CLIConfig) error {
	settlementChain, err := rpc.Connect(cfg.SettlementChainRPCURL)
	if err != nil {
		return errors.New("failed to initialize settlement chain")
	}
	t.settlementChainRPC = settlementChain

	sequencingChain, err := rpc.Connect(cfg.SequencingChainRPCURL)
	if err != nil {
		return errors.New("failed to initialize sequencing chain")
	}
	t.sequencingChainRPC = sequencingChain

	metaBasedChain, err := rpc.Connect(cfg.MetaBasedChainRPCURL)
	if err != nil {
		return errors.New("failed to initialize metabased chain")
	}
	t.metaBasedChainRPC = metaBasedChain

	return nil
}

func (t *TranslatorService) initAddresses(cfg *CLIConfig) error {
	if cfg.SequencingContractAddress == "" {
		return errors.New("empty sequencing contract address")
	}
	sequencingContractAddress, err := opservice.ParseAddress(cfg.SequencingContractAddress)
	if err != nil {
		return err
	}
	t.sequencingContractAddress = &sequencingContractAddress

	if cfg.BatchInboxAddress == "" {
		return errors.New("empty batch inbox address")
	}
	batchInboxAddress, err := opservice.ParseAddress(cfg.BatchInboxAddress)
	if err != nil {
		return err
	}
	t.batchInboxAddress = &batchInboxAddress

	return nil
}

func (t *TranslatorService) initBatchSigner(cfg *CLIConfig) error {
	signer, err := NewSigner(cfg.BatcherPrivateKey, new(big.Int).SetUint64(cfg.SettlementChainID))
	if err != nil {
		return fmt.Errorf("failed to initialize batch signer: %w", err)
	}
	t.batcherSigner = signer
	return nil
}

func (t *TranslatorService) initBackfillProvider(cfg *CLIConfig) {
	t.backfillProvider = backfill.NewBackfillerProvider(
		cfg.MetafillerURL,
		cfg.SettlementStartBlock,
		cfg.CutoverEpochBlock,
		&http.Client{},
		t.metricsCollector,
	)
}

func (t *TranslatorService) initMetrics() {
	t.metricsCollector = metrics.NewMetrics()
}

func (t *TranslatorService) initBatchProvider(cfg *CLIConfig) {
	t.metaBasedBatchProvider = NewMetaBasedBatchProvider(
		t.settlementChainRPC,
		t.sequencingChainRPC,
		*t.sequencingContractAddress,
		cfg.SettlementStartBlock,
		int(cfg.SettlementChainBlockTime.Seconds()),
		t.metricsCollector,
	)
}

func (t *TranslatorService) initOPTranslator() {
	t.opTranslator = NewOPTranslator(
		t.settlementChainRPC,
		t.metaBasedBatchProvider,
		t.backfillProvider,
		t.batcherSigner,
		t.batchInboxAddress,
		t.metricsCollector,
	)
}

func (t *TranslatorService) initServer(cfg *CLIConfig) error {
	srv, err := server.NewServer(
		cfg.Port,
		cfg.ReadTimeout,
		cfg.WriteTimeout,
		cfg.SettlementChainRPCURL,
		t.opTranslator,
		cfg.LogLevel,
	)
	if err != nil {
		return fmt.Errorf("failed to create server: %w", err)
	}
	t.server = srv
	return nil
}

// Start implements cliapp.Lifecycle.
func (t *TranslatorService) Start(ctx context.Context) error {
	t.server.Start(ctx)
	return nil
}

var ErrAlreadyStopped = errors.New("already stopped")

// Stop implements cliapp.Lifecycle.
func (t *TranslatorService) Stop(ctx context.Context) error {
	if t.stopped.Load() {
		return ErrAlreadyStopped
	}
	log.Info().Msg("Stopping op-translator")

	t.metaBasedChainRPC.CloseConnection()
	t.settlementChainRPC.CloseConnection()
	t.sequencingChainRPC.CloseConnection()
	t.server.Stop(ctx)

	t.stopped.Store(true)
	log.Info().Msg("op-translator stopped")
	return nil
}

// Stopped implements cliapp.Lifecycle.
func (t *TranslatorService) Stopped() bool {
	return t.stopped.Load()
}
