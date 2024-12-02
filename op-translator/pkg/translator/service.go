package translator

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"net/http"
	"sync/atomic"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/server"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/ethereum/go-ethereum/common"
	gethlog "github.com/ethereum/go-ethereum/log"
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

		log := oplog.NewLogger(oplog.AppOut(cliCtx), cfg.LogConfig)
		oplog.SetGlobalLogHandler(log.Handler())
		opservice.ValidateEnvVars(flags.EnvVarPrefix, flags.Flags, log)

		log.Info("Initializing op-translator")
		return TranslatorServiceFromCLIConfig(cliCtx.Context, version, cfg, log)
	}
}

func TranslatorServiceFromCLIConfig(ctx context.Context, version string, cfg *CLIConfig, log gethlog.Logger) (*TranslatorService, error) {
	var t TranslatorService
	if err := t.initFromCLIConfig(ctx, version, cfg, log); err != nil {
		log.Error("unable to initialize op-translator", "error", err)
		return nil, errors.Join(err, t.Stop(ctx)) // try to clean up our failed initialization attempt
	}
	return &t, nil
}

type TranslatorService struct {
	log                       gethlog.Logger
	metaBasedBatchProvider    *MetaBasedBatchProvider
	batchInboxAddress         *common.Address
	batcherSigner             *Signer
	settlementChainRPC        *rpc.RPCClient
	sequencingChainRPC        *rpc.RPCClient
	metaBasedChainRPC         *rpc.RPCClient
	backfillProvider          *backfill.BackfillProvider
	opTranslator              *OPTranslator
	server                    *server.Server
	metricsCollector          *metrics.Metrics
	sequencingContractAddress *common.Address
	version                   string
	stopped                   atomic.Bool
}

// guarantees that the cliapp.Lifecycle interface is implemented by TranslatorService
var _ cliapp.Lifecycle = (*TranslatorService)(nil)

func (t *TranslatorService) initFromCLIConfig(ctx context.Context, version string, cfg *CLIConfig, log gethlog.Logger) error {
	t.log = log
	t.version = version

	if err := t.initRPCServers(ctx, cfg); err != nil {
		return fmt.Errorf("failed to initialize RPC servers: %w", err)
	}
	if err := t.initAddresses(cfg); err != nil {
		return fmt.Errorf("failed to initialize addresses: %w", err)
	}
	if err := t.initBatchSigner(cfg); err != nil {
		return fmt.Errorf("failed to initialize batch signer: %w", err)
	}

	t.initBackfillProvider(cfg)
	t.initMetrics()
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
		t.log,
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
		t.log,
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
		t.log,
	)
}

func (t *TranslatorService) initServer(cfg *CLIConfig) error {
	srv, err := server.NewServer(
		cfg.Port,
		cfg.ReadTimeout,
		cfg.WriteTimeout,
		cfg.SettlementChainRPCURL,
		t.opTranslator,
		t.log,
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
	t.log.Info("Stopping op-translator")

	t.metaBasedChainRPC.CloseConnection()
	t.settlementChainRPC.CloseConnection()
	t.sequencingChainRPC.CloseConnection()
	t.server.Stop(ctx)

	t.stopped.Store(true)
	t.log.Info("op-translator stopped")
	return nil
}

// Stopped implements cliapp.Lifecycle.
func (t *TranslatorService) Stopped() bool {
	return t.stopped.Load()
}
