package publisher

import (
	"context"
	"sync"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	altda "github.com/ethereum-optimism/optimism/op-alt-da"
	"github.com/ethereum-optimism/optimism/op-service/txmgr"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/ethclient"
	gethlog "github.com/ethereum/go-ethereum/log"
)

// TODO (SEQ-186): this should not be configurable - it is dangerous if the value is changed in op-translator along the way
const MaxFrameSize = 120_000 - 1

type Publisher struct {
	log                        gethlog.Logger
	txManager                  txmgr.TxManager
	metabasedBatchProvider     translator.BatchProvider
	ctx                        context.Context
	cancel                     context.CancelFunc
	metrics                    metrics.Metricer
	l3ChainClient              *ethclient.Client
	sequencingChainClient      *ethclient.Client
	settlementChainClient      *ethclient.Client
	batchInboxAddress          *common.Address
	sequencingContractAddress  *common.Address
	wg                         sync.WaitGroup
	networkTimeout             time.Duration
	sequencingStartBlock       uint64
	sequencePerSettlementBlock uint64
	latestProcessedBlock       uint64
	pollInterval               time.Duration
}

func NewPublisher(
	settlementChainClient *ethclient.Client,
	batchInboxAddress *common.Address,
	sequencingChainClient *ethclient.Client,
	sequencingContractAddress *common.Address,
	l3ChainClient *ethclient.Client,
	metabasedBatchProvider translator.BatchProvider,
	pollInterval time.Duration,
	txManager txmgr.TxManager,
	log gethlog.Logger,
	metr metrics.Metricer,
) *Publisher {
	return &Publisher{
		settlementChainClient:     settlementChainClient,
		batchInboxAddress:         batchInboxAddress,
		sequencingChainClient:     sequencingChainClient,
		sequencingContractAddress: sequencingContractAddress,
		l3ChainClient:             l3ChainClient,
		metabasedBatchProvider:    metabasedBatchProvider,
		pollInterval:              pollInterval,
		txManager:                 txManager,
		log:                       log,
		metrics:                   metr,
	}
}

func (p *Publisher) Start(ctx context.Context) {
	ctx, cancel := context.WithCancel(ctx)
	p.ctx = ctx
	p.cancel = cancel

	// TODO (SEQ-187): wait for all 3 chains RPCs to be synced

	// TODO (SEQ-188): determine the latest block that has been processed
	// NOTE: This value could be set in a config flag, but ideally we should obtain it from the chain info
	p.sequencingStartBlock = 0
	p.sequencePerSettlementBlock = 1

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
			// obtain new blocks to process
			currentBlock, err := p.currentBlock()
			if err != nil {
				p.log.Error("failed to get latest block", "error", err)
				continue
			}
			for i := p.latestProcessedBlock + 1; i <= currentBlock; i++ {
				// TODO (SEQ-189): we might want to limit the number of go routines
				go p.processBlock(i)
			}
			p.latestProcessedBlock = currentBlock
		case <-p.ctx.Done():
			return
		}
	}
}

// TODO (SEQ-190): we are using L3 blocks as the guiding principle to publish data (assumption seems okay, but must be discussed)
func (p *Publisher) currentBlock() (uint64, error) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	blockNumber, err := p.l3ChainClient.BlockNumber(contextWithTimeout)
	return blockNumber, err
}

// TODO (SEQ-191): this functionality is wildly untested
func (p *Publisher) processBlock(block uint64) {
	contextWithTimeout, cancel := context.WithTimeout(p.ctx, p.networkTimeout)
	defer cancel()
	p.log.Debug("processing block", "block", block)
	batch, err := p.metabasedBatchProvider.GetBatch(contextWithTimeout, map[string]any{"number": hexutil.EncodeUint64(block)})
	if err != nil {
		p.log.Error("failed to get batch", "error", err, "block", block)
		return
	}
	frames, err := batch.ToFrames(MaxFrameSize)
	if err != nil {
		p.log.Error("failed to convert batch to frames", "error", err)
		return
	}
	callData, err := types.ToData(frames)
	if err != nil {
		p.log.Error("failed to convert frames to calldata", "error", err)
		return
	}

	commitment := altda.NewKeccak256Commitment(callData)
	txCandidate := daCommitmentTxCandidate(commitment.TxData(), p.batchInboxAddress)

	tx, err := p.txManager.Send(contextWithTimeout, *txCandidate) // TODO (SEQ-192): use `txmgr.Queue` instead ?
	if err != nil {
		p.log.Error("failed to send transaction", "error", err)
		return
	}

	p.log.Debug("transaction sent", "transaction", tx.TxHash.Hex())
}

func daCommitmentTxCandidate(commitmentData []byte, batchInboxAddress *common.Address) *txmgr.TxCandidate {
	return &txmgr.TxCandidate{
		To:     batchInboxAddress,
		TxData: commitmentData,
	}
}
