package publisher

import (
	"context"
	"errors"
	"fmt"
	"io"
	"sync/atomic"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/flags"
	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum-optimism/optimism/op-service/httputil"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
	"github.com/ethereum-optimism/optimism/op-service/txmgr"
	"github.com/ethereum/go-ethereum/common"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/urfave/cli/v2"
)

// Main is the entrypoint into the Batch Submitter.
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

		l := oplog.NewLogger(oplog.AppOut(cliCtx), cfg.LogConfig)
		oplog.SetGlobalLogHandler(l.Handler())
		opservice.ValidateEnvVars(flags.EnvVarPrefix, flags.Flags, l)

		l.Info("Initializing Metabase Publisher")
		return PublisherServiceFromCLIConfig(cliCtx.Context, version, cfg, l)
	}
}

var ErrAlreadyStopped = errors.New("already stopped")

type PublisherService struct {
	metrics                   metrics.Metricer
	settlementChainClient     rpc.IRPCClient
	l3Client                  rpc.IRPCClient
	sequencingChainClient     rpc.IRPCClient
	balanceMetricer           io.Closer
	txManager                 txmgr.TxManager
	log                       gethlog.Logger
	sequencingContractAddress *common.Address
	batchInboxAddress         *common.Address
	metricsServer             *httputil.HTTPServer
	pprofService              *oppprof.Service
	publisher                 *Publisher
	version                   string
	pollInterval              time.Duration
	stopped                   atomic.Bool
}

func (p *PublisherService) initFromCLIConfig(_ context.Context, version string, cfg *CLIConfig, log gethlog.Logger) error {
	p.version = version
	p.log = log

	p.initMetrics(cfg)

	// settlement chain
	if err := p.initBatchInboxAddress(cfg); err != nil {
		return fmt.Errorf("failed to init batch inbox address: %w", err)
	}

	// sequencing chain
	if err := p.initSequencingContractAddress(cfg); err != nil {
		return fmt.Errorf("failed to init batch inbox address: %w", err)
	}

	if err := p.initRPCClients(cfg); err != nil {
		return err
	}
	if err := p.initTxManager(cfg); err != nil {
		return fmt.Errorf("failed to init Tx manager: %w", err)
	}
	p.initBalanceMonitor(cfg)
	if err := p.initMetricsServer(cfg); err != nil {
		return fmt.Errorf("failed to start metrics server: %w", err)
	}
	if err := p.initPProf(cfg); err != nil {
		return fmt.Errorf("failed to init profiling: %w", err)
	}

	// TODO (SEQ-193): serve admin API with start/stop publishing and a "txManager" (see `initRPCServer` on op-batcher)
	// if err := p.initRPCServer(cfg); err != nil {
	// 	return fmt.Errorf("failed to start RPC server: %w", err)
	// }

	p.pollInterval = cfg.PollInterval
	p.initPublisher()

	p.metrics.RecordInfo(p.version)
	p.metrics.RecordUp()
	return nil
}

func (p *PublisherService) initMetrics(cfg *CLIConfig) {
	if cfg.MetricsConfig.Enabled {
		procName := "default"
		p.metrics = metrics.NewMetrics(procName)
	} else {
		p.metrics = metrics.NoopMetrics
	}
}

// initRPCClients creates the RPC clients for the settlement, sequencing chains and L3 metabased chain
func (p *PublisherService) initRPCClients(cfg *CLIConfig) error {
	seqChainClient, err := rpc.Connect(cfg.SequencingChainRPCURL)
	if err != nil {
		return fmt.Errorf("failed to dial sequencing chain RPC: %w", err)
	}
	p.sequencingChainClient = seqChainClient

	settlementChainClient, err := rpc.Connect(cfg.SettlementChainRPCURL)
	if err != nil {
		return fmt.Errorf("failed to dial settlement chain RPC: %w", err)
	}
	p.settlementChainClient = settlementChainClient

	l3Client, err := rpc.Connect(cfg.L3RPCURL)
	if err != nil {
		return fmt.Errorf("failed to dial L3 chain RPC: %w", err)
	}
	p.l3Client = l3Client
	return nil
}

func (p *PublisherService) initBatchInboxAddress(cfg *CLIConfig) error {
	if cfg.BatchInboxAddress == "" {
		return errors.New("empty batch inbox address")
	}
	batchInboxAddress, err := opservice.ParseAddress(cfg.BatchInboxAddress)
	if err != nil {
		return err
	}
	p.batchInboxAddress = &batchInboxAddress
	return nil
}

func (p *PublisherService) initSequencingContractAddress(cfg *CLIConfig) error {
	if cfg.SequencingContractAddress == "" {
		return errors.New("empty sequencing contract address")
	}
	sequencingContractAddress, err := opservice.ParseAddress(cfg.SequencingContractAddress)
	if err != nil {
		return err
	}
	p.sequencingContractAddress = &sequencingContractAddress
	return nil
}

func (p *PublisherService) initTxManager(cfg *CLIConfig) error {
	txManager, err := txmgr.NewSimpleTxManager("publisher", p.log, p.metrics, cfg.TxMgrConfig)
	if err != nil {
		return err
	}
	p.txManager = txManager
	return nil
}

func (p *PublisherService) initPProf(cfg *CLIConfig) error {
	p.pprofService = oppprof.New(
		cfg.PprofConfig.ListenEnabled,
		cfg.PprofConfig.ListenAddr,
		cfg.PprofConfig.ListenPort,
		cfg.PprofConfig.ProfileType,
		cfg.PprofConfig.ProfileDir,
		cfg.PprofConfig.ProfileFilename,
	)

	if err := p.pprofService.Start(); err != nil {
		return fmt.Errorf("failed to start pprof service: %w", err)
	}
	return nil
}

func (p *PublisherService) initMetricsServer(cfg *CLIConfig) error {
	if !cfg.MetricsConfig.Enabled {
		p.log.Info("Metrics disabled")
		return nil
	}
	m, ok := p.metrics.(opmetrics.RegistryMetricer)
	if !ok {
		return fmt.Errorf("metrics were enabled, but metricer %T does not expose registry for metrics-server", p.metrics)
	}
	p.log.Debug("Starting metrics server", "addr", cfg.MetricsConfig.ListenAddr, "port", cfg.MetricsConfig.ListenPort)
	metricsServer, err := opmetrics.StartServer(m.Registry(), cfg.MetricsConfig.ListenAddr, cfg.MetricsConfig.ListenPort)
	if err != nil {
		return fmt.Errorf("failed to start metrics server: %w", err)
	}
	p.log.Info("Started metrics server", "addr", metricsServer.Addr())
	p.metricsServer = metricsServer
	return nil
}

// initBalanceMonitor depends on Metrics, L1Client and TxManager to start background-monitoring of the batcher balance.
func (p *PublisherService) initBalanceMonitor(cfg *CLIConfig) {
	if cfg.MetricsConfig.Enabled {
		p.balanceMetricer = p.metrics.StartBalanceMetrics(p.log, p.settlementChainClient.AsEthClient(), p.txManager.From())
	}
}

func (p *PublisherService) initPublisher() {
	metabasedBatchProvider := translator.NewMetaBasedBatchProvider(
		p.settlementChainClient,
		p.sequencingChainClient,
		*p.sequencingContractAddress,
		0, // TODO (SEQ-194): SettlementStartBlock
		0, // TODO (SEQ-194): SequencingStartBlock
		1, // TODO (SEQ-194): SequencePerSettlementBlock
	)

	p.publisher = NewPublisher(
		p.settlementChainClient.AsEthClient(),
		p.batchInboxAddress,
		p.sequencingChainClient.AsEthClient(),
		p.sequencingContractAddress,
		p.l3Client.AsEthClient(),
		metabasedBatchProvider,
		p.pollInterval,
		p.txManager,
		p.log,
		p.metrics,
	)
}

// Start implements cliapp.Lifecycle.
func (p *PublisherService) Start(ctx context.Context) error {
	p.publisher.Start(ctx)
	return nil
}

// Stop implements cliapp.Lifecycle.
func (p *PublisherService) Stop(ctx context.Context) error {
	if p.stopped.Load() {
		return ErrAlreadyStopped
	}
	p.log.Info("Stopping batcher")

	var result error

	if p.publisher != nil {
		if err := p.publisher.Stop(); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop publisher: %w", err))
		}
	}

	// TODO (SEQ-193): stop the RPC admin API
	// if p.rpcServer != nil {
	// 	// TODO(7685): the op-service RPC server is not built on top of op-service httputil Server, and has poor shutdown
	// 	if err := p.rpcServer.Stop(); err != nil {
	// 		result = errors.Join(result, fmt.Errorf("failed to stop RPC server: %w", err))
	// 	}
	// }
	if p.pprofService != nil {
		if err := p.pprofService.Stop(ctx); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop PProf server: %w", err))
		}
	}
	if p.balanceMetricer != nil {
		if err := p.balanceMetricer.Close(); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to close balance metricer: %w", err))
		}
	}

	if p.metricsServer != nil {
		if err := p.metricsServer.Stop(ctx); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop metrics server: %w", err))
		}
	}

	if p.settlementChainClient != nil {
		p.settlementChainClient.CloseConnection()
	}

	if p.sequencingChainClient != nil {
		p.sequencingChainClient.CloseConnection()
	}

	if p.l3Client != nil {
		p.l3Client.CloseConnection()
	}

	if result == nil {
		p.stopped.Store(true)
		p.log.Info("Batch Submitter stopped")
	}
	return result
}

// Stopped implements cliapp.Lifecycle.
func (p *PublisherService) Stopped() bool {
	return p.stopped.Load()
}

// guarantees that the cliapp.Lifecycle interface is implemented by PublisherService
var _ cliapp.Lifecycle = (*PublisherService)(nil)

// BatcherServiceFromCLIConfig creates a new BatcherService from a CLIConfig.
// The service components are fully started, except for the driver,
// which will not be submitting batches (if it was configured to) until the Start part of the lifecycle.
func PublisherServiceFromCLIConfig(ctx context.Context, version string, cfg *CLIConfig, log gethlog.Logger) (*PublisherService, error) {
	var p PublisherService
	if err := p.initFromCLIConfig(ctx, version, cfg, log); err != nil {
		return nil, errors.Join(err, p.Stop(ctx)) // try to clean up our failed initialization attempt
	}
	return &p, nil
}
