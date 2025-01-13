package publisher

import (
	"context"
	"errors"
	"fmt"
	"sync/atomic"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/flags"
	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	opservice "github.com/ethereum-optimism/optimism/op-service"

	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum-optimism/optimism/op-service/httputil"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
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
	rpcClient        RPCAPI
	metrics         metrics.Metricer
	log             gethlog.Logger
	pprofService    *oppprof.Service
	altDAClient     *altda.DAClient
	publisher       *Publisher
	metricsServer   *httputil.HTTPServer
	version         string
	pollInterval    time.Duration
	networkTimeout  time.Duration
	blobUploadTimeout time.Duration
	stopped         atomic.Bool
	batchInboxAddress common.Address
	batcherAddress   common.Address
}

func (p *PublisherService) initFromCLIConfig(_ context.Context, version string, cfg *CLIConfig, log gethlog.Logger) error {
	p.version = version
	p.log = log

	p.log.Info("Starting publisher with the following config", "config", fmt.Sprintf("%+v", *cfg))

	p.initMetrics(cfg)

	if err := p.initAddresses(cfg); err != nil {
		return fmt.Errorf("failed to init sequencing contract address: %w", err)
	}
	if err := p.initRPCClient(cfg); err != nil {
		return err
	}
	if err := p.initMetricsServer(cfg); err != nil {
		return fmt.Errorf("failed to start metrics server: %w", err)
	}
	if err := p.initPProf(cfg); err != nil {
		return fmt.Errorf("failed to init profiling: %w", err)
	}

	p.pollInterval = cfg.PollInterval
	p.networkTimeout = cfg.NetworkTimeout
	p.blobUploadTimeout = cfg.BlobUploadTimeout
	// NOTE: we set `verify` to false because we will only be writing to the DA
	// NOTE: we set `precompute` to false because we will use `GenericCommitments` and let the da proxy generate the commitments for us
	p.altDAClient = altda.NewDAClient(cfg.AltDAURL, false, false)
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

// initRPCClient creates the RPC client for the op-translator
func (p *PublisherService) initRPCClient(cfg *CLIConfig) error {
	rpcClient, err := NewGethRPCAPI(cfg.OpTranslatorRPCURL)
	if err != nil {
		return fmt.Errorf("failed to dial L3 chain RPC: %w", err)
	}
	p.rpcClient = rpcClient
	return nil
}

func (p *PublisherService) initAddresses(cfg *CLIConfig) error {
	batcherAddress, err := opservice.ParseAddress(cfg.BatcherAddress)
	if err != nil {
		return err
	}
	p.batcherAddress = batcherAddress

	batchInboxAddress, err := opservice.ParseAddress(cfg.BatchInboxAddress)
	if err != nil {
		return err
	}
	p.batchInboxAddress = batchInboxAddress
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

func (p *PublisherService) initPublisher() {
	p.publisher = NewPublisher(
		p.rpcClient,
		p.altDAClient,
		p.batcherAddress,
		p.batchInboxAddress,
		p.pollInterval,
		p.networkTimeout,
		p.blobUploadTimeout,
		p.log,
		p.metrics,
	)
}

// Start implements cliapp.Lifecycle.
func (p *PublisherService) Start(ctx context.Context) error {
	if err := p.publisher.Start(ctx); err != nil {
		return fmt.Errorf("failed to start publisher: %w", err)
	}
	return nil
}

// Stop implements cliapp.Lifecycle.
func (p *PublisherService) Stop(ctx context.Context) error {
	if p.stopped.Load() {
		return ErrAlreadyStopped
	}
	p.log.Info("Stopping publisher")

	var result error

	if p.publisher != nil {
		if err := p.publisher.Stop(); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop publisher: %w", err))
		}
	}

	if p.pprofService != nil {
		if err := p.pprofService.Stop(ctx); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop PProf server: %w", err))
		}
	}

	if p.metricsServer != nil {
		if err := p.metricsServer.Stop(ctx); err != nil {
			result = errors.Join(result, fmt.Errorf("failed to stop metrics server: %w", err))
		}
	}

	if p.rpcClient != nil {
		if closer, ok := p.rpcClient.(*GethRPCAPI); ok {
			closer.Close()
		}
	}

	if result == nil {
		p.stopped.Store(true)
		p.log.Info("Publisher stopped")
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
