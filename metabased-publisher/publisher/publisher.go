package publisher

import (
	"context"
	"sync"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	"github.com/ethereum/go-ethereum/common/hexutil"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/pkg/errors"
)

// TODO (SEQ-186): this should not be configurable - it is dangerous if the value is changed along the way
const MaxFrameSize = 120_000 - 1

type L3RPCAPI interface {
	BlockNumber(ctx context.Context) (uint64, error)
}

type AltDAProvider interface {
	GetInput(ctx context.Context, comm altda.CommitmentData) ([]byte, error)
	SetInput(ctx context.Context, img []byte) (altda.CommitmentData, error)
}

type Publisher struct {
	log                    gethlog.Logger
	metabasedBatchProvider translator.IBatchProvider
	ctx                    context.Context
	cancel                 context.CancelFunc
	metrics                metrics.Metricer
	l3                     L3RPCAPI
	altDA                  AltDAProvider
	wg                     sync.WaitGroup
	networkTimeout         time.Duration
	latestProcessedBlock   uint64
	pollInterval           time.Duration
}

func NewPublisher(
	l3 L3RPCAPI,
	metabasedBatchProvider translator.IBatchProvider,
	altDA AltDAProvider,
	pollInterval time.Duration,
	networkTimeout time.Duration,
	log gethlog.Logger,
	metr metrics.Metricer,
) *Publisher {
	return &Publisher{
		l3:                     l3,
		metabasedBatchProvider: metabasedBatchProvider,
		altDA:                  altDA,
		pollInterval:           pollInterval,
		networkTimeout:         networkTimeout,
		log:                    log,
		metrics:                metr,
	}
}

func (p *Publisher) Start(ctx context.Context) {
	p.log.Info("starting publisher")
	ctx, cancel := context.WithCancel(ctx)
	p.ctx = ctx
	p.cancel = cancel

	// TODO (SEQ-187): wait for all 3 chains RPCs to be synced

	// TODO (SEQ-194): determine the L3 metabased-sequencing starting block
	l3StartingBlock := uint64(15195256)

	// TODO (SEQ-188): `p.latestProcessedBlock` must be restored from somewhere, ideally we obtain it from upstream (altda)
	if l3StartingBlock > p.latestProcessedBlock {
		// only start uploading to altDA after the L3 is metabased sequenced
		p.latestProcessedBlock = l3StartingBlock
	}

	p.wg.Add(1)
	go p.loop()
}

func (p *Publisher) Stop() error {
	p.log.Info("stopping publisher")
	p.cancel()
	p.metabasedBatchProvider.Close()
	p.wg.Wait()
	return nil
}

func (p *Publisher) loop() {
	defer p.wg.Done()

	pollTicker := time.NewTicker(p.pollInterval)
	defer pollTicker.Stop()

	for {
		select {
		case <-pollTicker.C:
			// obtain new L3 blocks to process
			currentBlock, err := p.currentBlock()
			if err != nil {
				p.log.Error("failed to get latest block", "error", err)
				continue
			}
			p.log.Info("polling for new blocks", "L3Block", currentBlock, "latestProcessedBlock", p.latestProcessedBlock)
			for i := p.latestProcessedBlock + 1; i <= currentBlock; i++ {
				if err := p.processBlock(i); err != nil {
					p.log.Warn("stopped processing, will retry next interval", "lastProcessed", p.latestProcessedBlock, "failedBlock", i, "error", err)
					break // stop processing here, wait for next ticker to retry from block `i`
				}
				p.latestProcessedBlock = i
			}
		case <-p.ctx.Done():
			return
		}
	}
}

// TODO (SEQ-190): we are using L3 blocks as the guiding principle to publish data (assumption seems okay, but must be discussed)
func (p *Publisher) currentBlock() (uint64, error) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	blockNumber, err := p.l3.BlockNumber(contextWithTimeout)
	return blockNumber, err
}

func (p *Publisher) callDataForBlock(block uint64) ([]byte, error) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	p.log.Debug("processing block", "block", block)
	// TODO (SEQ-304): the batch has already been built by the translator and is part of the L3 block. we should be able to just use it (thus removing all dependencies of the publisher except L3 RPC)
	batch, err := p.metabasedBatchProvider.GetBatch(contextWithTimeout, map[string]any{"number": hexutil.EncodeUint64(block)})
	if err != nil {
		return nil, errors.Wrap(err, "failed to get batch")
	}
	frames, err := batch.GetFrames(MaxFrameSize)
	if err != nil {
		return nil, errors.Wrap(err, "failed to get batch")
	}
	callData, err := types.ToData(frames)
	if err != nil {
		return nil, errors.Wrap(err, "failed to convert frames to calldata")
	}
	return callData, nil
}

// TODO (SEQ-191): add more tests
func (p *Publisher) processBlock(block uint64) error {
	p.log.Debug("processing block", "block", block)
	callData, err := p.callDataForBlock(block)
	if err != nil {
		return err // something went wrong
	}
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	commitment := altda.NewGenericCommitment(callData)
	if _, err := p.altDA.SetInput(contextWithTimeout, commitment); err != nil {
		return errors.Wrap(err, "unable to upload commitment to altDA")
	}
	p.log.Debug("successfully published DA for block", "block", block)
	return nil
}
