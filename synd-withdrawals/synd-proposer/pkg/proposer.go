package pkg

import (
	"context"
	"encoding/json"
	"fmt"
	"maps"
	"math/big"
	"slices"
	"strconv"
	"sync"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/logger"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/metrics"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/tls"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/offchainlabs/nitro/arbnode"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/daprovider"
	"github.com/offchainlabs/nitro/eigenda"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
	"github.com/pkg/errors"
	"github.com/rs/zerolog/log"
)

type Proposer struct {
	Config              *config.Config
	AppchainClient      *ethclient.Client
	SequencingClient    *ethclient.Client
	EthereumClient      *ethclient.Client
	SettlementClient    *ethclient.Client
	SettlementAuth      *bind.TransactOpts
	EnclaveClient       *rpc.Client
	DapReaders          []daprovider.Reader
	TeeModule           *teemodule.Teemodule
	PendingAssertion    *teemodule.PendingAssertion
	PendingTeeInputHash common.Hash
	PendingSignature    []byte
	Metrics             *metrics.Metrics
}

func NewProposer(ctx context.Context, cfg *config.Config, metrics *metrics.Metrics) *Proposer {
	appchainClient, err := ethclient.DialContext(ctx, cfg.AppchainRPCURL)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create appchain provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}
	sequencingClient, err := ethclient.DialContext(ctx, cfg.SequencingRPCURL)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create sequencing provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}

	ethereumClient, err := ethclient.DialContext(ctx, cfg.EthereumRPCURL)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create ethereum provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}

	var enclaveClient *rpc.Client
	if cfg.EnclaveTLSConfig.Enabled {
		enclaveClient, err = tls.CreateTLSClient(&cfg.EnclaveTLSConfig, cfg.EnclaveRPCURL)
	} else {
		enclaveClient, err = rpc.DialContext(ctx, cfg.EnclaveRPCURL)
	}
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create enclave provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}

	settlementClient, err := ethclient.DialContext(ctx, cfg.SettlementRPCURL)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create settlement provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}
	eigenClient, err := eigenda.NewEigenDA(&eigenda.EigenDAConfig{
		Enable: true,
		Rpc:    cfg.EigenRPCUrl,
	})
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create Eigen provider", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}

	settlementAuth, err := bind.NewKeyedTransactorWithChainID(cfg.PrivateKey, big.NewInt(int64(cfg.SettlementChainID)))
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create settlement transactor", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}
	teeModule, err := teemodule.NewTeemodule(cfg.TeeModuleContractAddress, settlementClient)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to create TEE module", err)
		log.Fatal().Stack().Err(wrappedErr).Msg(msg)
	}

	return &Proposer{
		Config:           cfg,
		AppchainClient:   appchainClient,
		SequencingClient: sequencingClient,
		EthereumClient:   ethereumClient,
		EnclaveClient:    enclaveClient,
		SettlementClient: settlementClient,
		SettlementAuth:   settlementAuth,
		DapReaders:       []daprovider.Reader{eigenda.NewReaderForEigenDA(eigenClient)},
		TeeModule:        teeModule,
		Metrics:          metrics,

		PendingAssertion:    nil,
		PendingSignature:    nil,
		PendingTeeInputHash: common.Hash{},
	}
}

// Run starts the background processes for the proposer and waits for them to finish.
// It accepts a context and two functions: one for polling and one for close challenge.
func (p *Proposer) Run(ctx context.Context) {
	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		p.pollingLoop(ctx)
	}()

	go func() {
		defer wg.Done()
		p.closeChallengeLoop(ctx)
	}()

	wg.Wait()
}

// CloseChallengeLoop runs the close challenge background process.
func (p *Proposer) closeChallengeLoop(ctx context.Context) {
	ticker := time.NewTicker(p.Config.CloseChallengeInterval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			log.Info().Msg("Close challenge loop shutting down...")

			return
		case <-ticker.C:
			log.Info().Msg("Close challenge loop tick...")
			if _, err := p.TeeModule.CloseChallengeWindow(p.SettlementAuth); err != nil {
				log.Error().Err(err).Msg("Failed to close challenge window")
			}
		}
	}
}

// PollingLoop runs the polling background process.
func (p *Proposer) pollingLoop(ctx context.Context) {
	ticker := time.NewTicker(p.Config.PollingInterval)
	// check if the appchain settles to an arbitrum rollup by querying the code at ArbSys precompile address
	code, err := p.SettlementClient.CodeAt(ctx, ArbSysPrecompileAddress, nil)
	if err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg("Failed to get code at ArbSys precompile address", err)
		log.Warn().Stack().Err(wrappedErr).Msg(msg)
	}
	settlesToArbitrumRollup := len(code) > 0
	log.Info().Bool("settlesToArbitrumRollup", settlesToArbitrumRollup).Msg("Settles to Arbitrum Rollup")

	for {
		select {
		case <-ctx.Done():
			log.Info().Msg("Polling loop shutting down...")

			return
		case <-ticker.C:
			log.Info().Msg("Polling loop tick...")

			pollingLoopTimer := metrics.NewTimer()

			trustedInput, err := p.getTrustedInput(ctx)
			if err != nil {
				msg, wrappedErr := logger.WrapErrorWithMsg("Failed to get trusted input", err)
				log.Error().Stack().Err(wrappedErr).Msg(msg)

				continue
			}

			if p.PendingTeeInputHash != trustedInput.Hash() {
				log.Info().Msg("Proving new assertion...")
				appOutput, err := p.Prove(ctx, trustedInput, settlesToArbitrumRollup)
				if err != nil {
					msg, wrappedErr := logger.WrapErrorWithMsg("Failed to prove", err)
					log.Error().Stack().Err(wrappedErr).Msg(msg)

					continue
				}
				p.PendingAssertion = &appOutput.PendingAssertion

				p.PendingTeeInputHash = trustedInput.Hash()
				p.PendingSignature = appOutput.Signature
				log.Debug().Msgf("Trusted input: %v", ToHexForLogsTrustedInput(*trustedInput))
				log.Debug().Msgf("Pending assertion: %v", ToHexForLogsPendingAssertion(*p.PendingAssertion))
			}

			submissionTimer := metrics.NewTimer()
			transaction, err := p.TeeModule.SubmitAssertion(p.SettlementAuth, *p.PendingAssertion, p.PendingSignature,
				crypto.PubkeyToAddress(p.Config.PrivateKey.PublicKey))
			if err != nil {
				msg, wrappedErr := logger.WrapErrorWithMsg("Failed to submit assertion", err)
				log.Error().Stack().Err(wrappedErr).Msg(msg)

				continue
			}
			p.Metrics.AssertionSubmissions.Inc()
			submissionTimer.ObserveHistogram(p.Metrics.AssertionSubmissionDuration)

			log.Debug().
				Str("transactionHash", transaction.Hash().Hex()).
				Str("seqHash", common.BytesToHash(p.PendingAssertion.SeqBlockHash[:]).Hex()).
				Str("appHash", common.BytesToHash(p.PendingAssertion.AppBlockHash[:]).Hex()).
				Str("l1Acc", common.BytesToHash(p.PendingAssertion.L1BatchAcc[:]).Hex()).
				Msg("Submitted assertion")
			pollingLoopTimer.ObserveHistogram(p.Metrics.PollingLoopDuration)
		}
	}
}

func (p *Proposer) getTrustedInput(ctx context.Context) (*enclave.TrustedInput, error) {
	contractTrustedInput, err := p.TeeModule.TeeTrustedInput(&bind.CallOpts{Context: ctx})
	if err != nil {
		return nil, errors.Wrap(err, "failed to get trusted input from contract")
	}
	trustedInput := enclave.TrustedInput(contractTrustedInput)

	return &trustedInput, nil
}

func (p *Proposer) getBatchCount(ctx context.Context, l1Hash common.Hash) (uint64, error) {
	var batchCount uint64
	if p.Config.IsL1Chain {
		if err := p.SequencingClient.Client().CallContext(ctx, &batchCount, "synd_batchFromAcc", l1Hash); err != nil {
			return 0, errors.Wrap(err, "failed to get batch from seq acc")
		}
	} else {
		count, err := p.EthereumClient.StorageAtHash(ctx, p.Config.EnclaveConfig.SequencingBridgeAddress,
			enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, l1Hash)
		if err != nil {
			return 0, errors.Wrap(err, "failed to get batch count from l1")
		}
		batchCount = common.BytesToHash(count).Big().Uint64()
	}
	if batchCount == 0 {
		return 0, errors.New("end batch count is 0")
	}

	return batchCount, nil
}

func (p *Proposer) Prove(
	ctx context.Context,
	trustedInput *enclave.TrustedInput,
	settlesToArbitrumRollup bool,
) (*enclave.VerifyAppchainOutput, error) {
	p.Metrics.ProveTotal.Inc()

	// get trusted input
	if trustedInput == nil {
		var err error
		trustedInput, err = p.getTrustedInput(ctx)
		if err != nil {
			return nil, errors.Wrap(err, "failed to get trusted input")
		}
	}

	// get the batch count
	endBatchCount, err := p.getBatchCount(ctx, trustedInput.L1EndHash)
	if err != nil {
		return nil, errors.Wrap(err, "failed to get batch count")
	}

	// get the start block
	header, err := p.SequencingClient.HeaderByHash(ctx, trustedInput.SeqStartBlockHash)
	if err != nil {
		return nil, errors.Wrap(err, "failed to get sequencing header")
	}

	// get validation data
	var valData ValidationData
	log.Debug().
		Str("start_block", header.Number.String()).
		Str("end_batch", strconv.FormatUint(endBatchCount-1, 10)).
		Msg("Getting validation data from start block to end batch")

	if err = p.SequencingClient.Client().
		CallContext(
			ctx,
			&valData,
			"synd_validationData",
			header.Number.Uint64(),
			endBatchCount-1,
			false,
		); err != nil {
		return nil, errors.Wrap(err, "failed to get validation data")
	}

	preimages := make(map[arbutil.PreimageType]map[common.Hash][]byte)
	preimages[arbutil.Keccak256PreimageType] = make(map[common.Hash][]byte)
	for _, preimage := range valData.PreimageData {
		preimages[arbutil.Keccak256PreimageType][crypto.Keccak256Hash(preimage)] = preimage
	}

	// get batches
	log.Debug().Msg("Getting batches...")
	var batches [][]byte
	if valData.BatchEndIndex >= valData.BatchStartIndex {
		ibridge, err := bridgegen.NewIBridgeCaller(p.Config.EnclaveConfig.SequencingBridgeAddress, p.EthereumClient)
		if err != nil {
			return nil, errors.Wrap(err, "failed to create ibridge caller")
		}

		seqInbox, err := ibridge.SequencerInbox(&bind.CallOpts{Context: ctx})
		if err != nil {
			return nil, errors.Wrap(err, "failed to get sequencer inbox")
		}

		batches, err = getBatches(ctx, p.EthereumClient, seqInbox, valData.BatchStartIndex, valData.BatchEndIndex,
			valData.BatchStartBlockNum, valData.BatchEndBlockNum)
		if err != nil {
			return nil, errors.Wrap(err, "failed to get batches")
		}
		if len(batches) == 0 {
			return nil, errors.New("found 0 batches")
		}

		// Record batch metrics
		p.Metrics.ProveBatchCount.Set(float64(len(batches)))

		// update preimages
		for _, batch := range batches {
			if err := getBatchPreimageData(ctx, batch, p.DapReaders, preimages); err != nil {
				return nil, errors.Wrap(err, "failed to get batch preimage data")
			}
		}
	}

	log.Debug().Msg("Getting proof...")
	var proof *enclave.AccountResult
	if !p.Config.IsL1Chain {
		// get the end block header
		if header, err = p.EthereumClient.HeaderByHash(ctx, trustedInput.L1EndHash); err != nil {
			return nil, errors.Wrap(err, "failed to get sequencing header")
		}

		// get merkle proof
		accSlot := common.BigToHash(new(big.Int).Add(
			enclave.BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE,
			big.NewInt(int64(endBatchCount)),
		))
		if err := p.EthereumClient.Client().CallContext(ctx,
			&proof,
			"eth_getProof",
			p.Config.EnclaveConfig.SequencingBridgeAddress,
			[]common.Hash{enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, accSlot},
			common.Hash(trustedInput.L1EndHash),
		); err != nil {
			return nil, errors.Wrap(err, "failed to get proof")
		}
	}

	preimageData := make(map[arbutil.PreimageType][][]byte)
	for ty, images := range preimages {
		preimageData[ty] = slices.Collect(maps.Values(images))
	}

	// derive sequencing chain
	log.Debug().Msg("Verifying sequencing chain...")
	sequencingChainTimer := metrics.NewTimer()
	var seqOutput enclave.VerifySequencingChainOutput
	if err = p.handleEnclaveCall(&seqOutput, "enclave_verifySequencingChain", enclave.VerifySequencingChainInput{
		TrustedInput:                    *trustedInput,
		Config:                          p.Config.EnclaveConfig,
		DelayedMessages:                 valData.DelayedMessages,
		StartDelayedMessagesAccumulator: valData.StartDelayedAcc,
		Batches:                         batches,
		IsL1Chain:                       p.Config.IsL1Chain,
		PreimageData:                    preimageData,
		EndBatchAccumulatorMerkleProof:  proof,
		L1EndBlockHeader:                header,
	}); err != nil {
		return nil, errors.Wrap(err, "failed to verify sequencing chain")
	}
	log.Debug().Dur("duration", sequencingChainTimer.Duration()).Msg("Sequencing chain verification completed")

	// get appchain start block
	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		msg, wrappedErr := logger.WrapErrorWithMsg(fmt.Sprintf("failed to get appchain header, hash: %v",
			common.BytesToHash(trustedInput.AppStartBlockHash[:]).Hex()), err)

		return nil, errors.Wrap(wrappedErr, msg)
	}

	// get delayed messages
	startAcc, msgs, isDummy, err := GetDelayedMessages(
		ctx,
		p.SettlementClient,
		p.Config.AppchainBridgeAddress,
		header.Nonce.Uint64(),
		trustedInput.SetDelayedMessageAcc,
		settlesToArbitrumRollup,
	)
	if err != nil {
		return nil, errors.Wrap(err, "failed to get delayed messages")
	}

	// get the number of batches. ignore the delayed message if it is a dummy one
	var realMsgs [][]byte
	if !isDummy {
		realMsgs = msgs
	}
	numBatches := getNumBatches(seqOutput.Batches, realMsgs, p.Config.EnclaveConfig.SettlementDelay)

	// get appchain preimage data
	var appPreimages [][]byte
	if err = p.AppchainClient.Client().CallContext(ctx, &appPreimages, "synd_preimageData", header.Number,
		numBatches, true); err != nil {
		return nil, errors.Wrap(err, "failed to get appchain preimage data")
	}

	// derive appchain
	log.Debug().Msg("Verifying appchain...")
	appchainTimer := metrics.NewTimer()
	var appOutput enclave.VerifyAppchainOutput
	if err := p.handleEnclaveCall(&appOutput, "enclave_verifyAppchain", enclave.VerifyAppchainInput{
		TrustedInput:                    *trustedInput,
		Config:                          p.Config.EnclaveConfig,
		DelayedMessages:                 msgs,
		StartDelayedMessagesAccumulator: startAcc,
		VerifySequencingChainOutput:     seqOutput,
		AppStartBlockHeader:             *header,
		PreimageData: map[arbutil.PreimageType][][]byte{
			arbutil.Keccak256PreimageType: appPreimages,
		},
	}); err != nil {
		return nil, errors.Wrap(err, "failed to verify appchain")
	}
	log.Debug().Dur("appchainVerificationDuration", appchainTimer.Duration()).Msg("Appchain verification completed")

	return &appOutput, nil
}

func (p *Proposer) Verify(ctx context.Context, trustedInput *enclave.TrustedInput) (*enclave.VerifyAppchainOutput, error) {
	// get trusted input
	if trustedInput == nil {
		var err error
		trustedInput, err = p.getTrustedInput(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to get trusted input: %w", err)
		}
	}

	// get the batch count
	endBatchCount, err := p.getBatchCount(ctx, trustedInput.L1EndHash)
	if err != nil {
		return nil, fmt.Errorf("failed to get batch count: %w", err)
	}
	log.Debug().
		Str("batch", strconv.FormatUint(endBatchCount-1, 10)).
		Msg("Getting batch metadata")

	var metadata arbnode.BatchMetadata
	if err := p.SequencingClient.Client().CallContext(ctx, &metadata, "synd_batchMetadata", endBatchCount-1); err != nil {
		return nil, fmt.Errorf("failed to get batch metadata: %w", err)
	}

	if metadata.MessageCount == 0 {
		return nil, errors.New("message count is 0")
	}

	// get the end block
	header, err := p.SequencingClient.HeaderByNumber(ctx, big.NewInt(int64(metadata.MessageCount-1)))
	if err != nil {
		return nil, fmt.Errorf("failed to get sequencing header: %w", err)
	}
	sequencingBlockHash := header.Hash()

	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return nil, fmt.Errorf("failed to get appchain header: %w", err)
	}

	// binary search to find the appchain end block
	appEndBlock, err := FindBlock(ctx, p.AppchainClient, header.Number.Uint64(), uint64(metadata.MessageCount-1))
	if err != nil {
		return nil, fmt.Errorf("failed to find appchain end block: %w", err)
	}

	return &enclave.VerifyAppchainOutput{
		PendingAssertion: teemodule.PendingAssertion{
			AppBlockHash: appEndBlock.BlockHash,
			AppSendRoot:  appEndBlock.SendRoot,
			SeqBlockHash: sequencingBlockHash,
			L1BatchAcc:   metadata.Accumulator,
		},
		// Intentionally unset
		Signature: nil,
	}, nil
}

func (p *Proposer) handleEnclaveCall(output interface{}, method string, input interface{}) error {
	timer := metrics.NewTimer()
	defer func() {
		timer.ObserveHistogramVec(p.Metrics.EnclaveCallDuration.WithLabelValues(method))
	}()

	p.Metrics.EnclaveCalls.WithLabelValues(method).Inc()

	if err := p.EnclaveClient.Call(output, method, input); err != nil {
		return errors.Wrap(tls.HandleTLSErr(err), "tls error")
	}

	return nil
}

// ToHexForLogsTrustedInput  converts TeeTrustedInput to a hex-encoded version
func ToHexForLogsTrustedInput(t enclave.TrustedInput) string {
	hexInput := TeeTrustedInputHex{
		ConfigHash:           common.Hash(t.ConfigHash).Hex(),
		AppStartBlockHash:    common.Hash(t.AppStartBlockHash).Hex(),
		SeqStartBlockHash:    common.Hash(t.SeqStartBlockHash).Hex(),
		SetDelayedMessageAcc: common.Hash(t.SetDelayedMessageAcc).Hex(),
		L1StartBatchAcc:      common.Hash(t.L1StartBatchAcc).Hex(),
		L1EndHash:            common.Hash(t.L1EndHash).Hex(),
	}
	jsonInput, _ := json.Marshal(hexInput)

	return string(jsonInput)
}

// ToHexForLogsPendingAssertion converts Pending assertion to a hex-encoded version
func ToHexForLogsPendingAssertion(t teemodule.PendingAssertion) string {
	hexInput := PendingAssertionHex{
		AppBlockHash: common.Hash(t.AppBlockHash).Hex(),
		AppSendRoot:  common.Hash(t.AppSendRoot).Hex(),
		SeqBlockHash: common.Hash(t.SeqBlockHash).Hex(),
		L1BatchAcc:   common.Hash(t.L1BatchAcc).Hex(),
	}
	jsonInput, _ := json.Marshal(hexInput)

	return string(jsonInput)
}

// PendingAssertionHex is a hex-encoded version for logging
type PendingAssertionHex struct {
	AppBlockHash string `json:"appBlockHash"`
	AppSendRoot  string `json:"appSendRoot"`
	SeqBlockHash string `json:"seqBlockHash"`
	L1BatchAcc   string `json:"l1BatchAcc"`
}

// TeeTrustedInputHex is a hex-encoded version for logging
type TeeTrustedInputHex struct {
	ConfigHash           string `json:"configHash"`
	AppStartBlockHash    string `json:"appStartBlockHash"`
	SeqStartBlockHash    string `json:"seqStartBlockHash"`
	SetDelayedMessageAcc string `json:"setDelayedMessageAcc"`
	L1StartBatchAcc      string `json:"l1StartBatchAcc"`
	L1EndHash            string `json:"l1EndHash"`
}
