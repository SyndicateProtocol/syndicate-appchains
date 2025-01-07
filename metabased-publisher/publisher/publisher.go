package publisher

import (
	"context"
	"fmt"
	"math/big"
	"sync"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"

	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/pkg/errors"
)

// TODO (SEQ-186): this should not be configurable - it is dangerous if the value is changed along the way
const MaxFrameSize = 120_000 - 1

type RPCAPI interface {
	BlockNumber(ctx context.Context) (uint64, error)
	BlockByNumber(ctx context.Context, number *big.Int) (*types.Block, error)
	ChainID(ctx context.Context) (*big.Int, error)
}

type AltDAProvider interface {
	GetInput(ctx context.Context, comm altda.CommitmentData) ([]byte, error)
	SetInput(ctx context.Context, img []byte) (altda.CommitmentData, error)
}

type Publisher struct {
	translatedBlockSigner types.Signer
	altDA                 AltDAProvider
	log                   gethlog.Logger
	ctx                   context.Context
	metrics               metrics.Metricer
	opTranslatorClient    RPCAPI
	cancel                context.CancelFunc
	wg                    sync.WaitGroup
	latestProcessedBlock  uint64
	pollInterval          time.Duration
	networkTimeout        time.Duration
	blobUploadTimeout     time.Duration
	batcherAddress        common.Address
	batchInboxAddress     common.Address
}

func NewPublisher(
	opTranslatorCLient RPCAPI,
	altDA AltDAProvider,
	batcherAddress common.Address,
	batchInboxAddress common.Address,
	pollInterval time.Duration,
	networkTimeout time.Duration,
	blobUploadTimeout time.Duration,
	log gethlog.Logger,
	metr metrics.Metricer,
) *Publisher {
	return &Publisher{
		opTranslatorClient: opTranslatorCLient,
		altDA:              altDA,
		batcherAddress:     batcherAddress,
		batchInboxAddress:  batchInboxAddress,
		pollInterval:       pollInterval,
		networkTimeout:     networkTimeout,
		blobUploadTimeout:  blobUploadTimeout,
		log:                log,
		metrics:            metr,
	}
}

func (p *Publisher) Start(ctx context.Context) {
	ctx, cancel := context.WithCancel(ctx)
	p.ctx = ctx
	p.cancel = cancel

	// TODO (SEQ-187): wait for all 3 chains RPCs to be synced

	// TODO (SEQ-194): determine the translated block starting block
	startingBlock := uint64(0)

	// TODO (SEQ-188): `p.latestProcessedBlock` must be restored from somewhere, ideally we obtain it from upstream (altda)
	if startingBlock > p.latestProcessedBlock {
		// only start uploading to altDA after the L3 is metabased sequenced
		p.latestProcessedBlock = startingBlock
	}

	p.wg.Add(1)
	go p.loop()
}

func (p *Publisher) Stop() error {
	p.cancel()
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
			// obtain new translated blocks to process
			currentBlock, err := p.currentBlockNumber()
			if err != nil {
				p.log.Error("failed to get latest translated block number", "error", err)
				continue
			}
			p.log.Debug("polling for new blocks", "currentTranslatedBlock", currentBlock, "latestProcessedBlock", p.latestProcessedBlock)
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

// TODO (SEQ-190): we are using translated blocks as the guiding principle to publish data (assumption seems okay, but must be discussed)
func (p *Publisher) currentBlockNumber() (uint64, error) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	return p.opTranslatorClient.BlockNumber(contextWithTimeout)
}

func (p *Publisher) signer() types.Signer {
	if p.translatedBlockSigner != nil {
		return p.translatedBlockSigner
	}
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	chainID, err := p.opTranslatorClient.ChainID(contextWithTimeout)
	if err != nil {
		return nil
	}
	return types.NewCancunSigner(chainID)
}

func (p *Publisher) callDataForBlock(blockNumber uint64) ([]byte, error) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	block, err := p.opTranslatorClient.BlockByNumber(contextWithTimeout, new(big.Int).SetUint64(blockNumber))
	if err != nil {
		return nil, errors.Wrap(err, fmt.Sprintf("failed to get translated block %d", blockNumber))
	}
	var callData []byte
	for _, tx := range block.Transactions() {
		// filter for transactions from the batcher address to the batch inbox contract
		if tx.To() != nil && *tx.To() == p.batchInboxAddress {
			from, err := types.Sender(p.signer(), tx)
			if err != nil {
				return nil, errors.Wrap(err, "failed to get sender from transaction")
			}
			if from == p.batcherAddress {
				callData = append(callData, tx.Data()...) // TODO (SEQ-317) this will work for a single tx, but we might have to collect the data differently if many txs are used in a single block
			}
		}
	}
	return callData, nil
}

// TODO (SEQ-191): add more tests
func (p *Publisher) processBlock(block uint64) error {
	p.log.Info("processing block", "block", block)
	callData, err := p.callDataForBlock(block)
	if err != nil {
		return err
	}
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.blobUploadTimeout)
	defer cancel()
	commitment := altda.NewGenericCommitment(callData)
	if _, err := p.altDA.SetInput(contextWithTimeout, commitment); err != nil {
		return errors.Wrap(err, "unable to upload commitment to altDA")
	}
	p.log.Info("successfully published DA for block", "block", block)
	return nil
}
