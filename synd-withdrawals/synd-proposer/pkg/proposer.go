package pkg

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"maps"
	"math/big"
	"slices"
	"sync"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/teemodule"
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
)

type Proposer struct {
	Config              *Config
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
}

func NewProposer(cfg *Config) *Proposer {
	appchainClient, err := ethclient.Dial(cfg.AppchainRPCURL)
	if err != nil {
		log.Fatalf("Failed to create appchain provider: %v", err)
		return nil
	}
	sequencingClient, err := ethclient.Dial(cfg.SequencingRPCURL)
	if err != nil {
		log.Fatalf("Failed to create sequencing provider: %v", err)
		return nil
	}

	ethereumClient, err := ethclient.Dial(cfg.EthereumRPCURL)
	if err != nil {
		log.Fatalf("Failed to create ethereum provider: %v", err)
		return nil
	}

	var enclaveClient *rpc.Client
	if cfg.EnclaveTLSConfig.Enabled {
		enclaveClient, err = createTLSClient(&cfg.EnclaveTLSConfig, cfg.EnclaveRPCURL)
	} else {
		enclaveClient, err = rpc.Dial(cfg.EnclaveRPCURL)
	}
	if err != nil {
		log.Fatalf("Failed to create enclave provider: %v", err)
		return nil
	}

	settlementClient, err := ethclient.Dial(cfg.SettlementRPCURL)
	if err != nil {
		log.Fatalf("Failed to create settlement provider: %v", err)
		return nil
	}
	eigenClient, err := eigenda.NewEigenDA(&eigenda.EigenDAConfig{
		Enable: true,
		Rpc:    cfg.EigenRPCUrl,
	})
	if err != nil {
		log.Fatalf("Failed to create eignen provider: %v", err)
		return nil
	}

	settlementAuth, err := bind.NewKeyedTransactorWithChainID(cfg.PrivateKey, big.NewInt(int64(cfg.SettlementChainID)))
	if err != nil {
		log.Fatalf("Failed to create transactor: %v", err)
		return nil
	}
	teeModule, err := teemodule.NewTeemodule(cfg.TeeModuleContractAddress, settlementClient)
	if err != nil {
		log.Fatalf("Failed to create tee module: %v", err)
		return nil
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
			log.Println("Close challenge loop shutting down...")
			return
		case <-ticker.C:
			log.Println("Close challenge loop tick...")
			if _, err := p.TeeModule.CloseChallengeWindow(p.SettlementAuth); err != nil {
				log.Printf("Failed to close challenge window: %v", err)
			}
		}
	}
}

// PollingLoop runs the polling background process.
func (p *Proposer) pollingLoop(ctx context.Context) {
	ticker := time.NewTicker(p.Config.PollingInterval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			log.Println("Polling loop shutting down...")
			return
		case <-ticker.C:
			log.Println("Polling loop tick...")
			trustedInput, err := p.getTrustedInput(ctx)
			if err != nil {
				log.Printf("Failed to get trusted input: %v", err)
				continue
			}

			if p.PendingTeeInputHash != trustedInput.Hash() {
				log.Println("Proving new assertion...")
				appOutput, err := p.Prove(ctx, nil)
				if err != nil {
					log.Printf("Failed to prove: %v", err)
				}

				p.PendingAssertion = &teemodule.PendingAssertion{
					AppBlockHash: appOutput.AppchainBlockHash,
					AppSendRoot:  appOutput.AppchainSendRoot,
					SeqBlockHash: appOutput.SequencingBlockHash,
					L1BatchAcc:   appOutput.L1BatchAcc,
				}
				p.PendingTeeInputHash = trustedInput.Hash()
				p.PendingSignature = appOutput.Signature
			}

			transaction, err := p.TeeModule.SubmitAssertion(p.SettlementAuth, *p.PendingAssertion, p.PendingSignature, crypto.PubkeyToAddress(p.Config.PrivateKey.PublicKey))
			if err != nil {
				log.Printf("Failed to submit assertion: %v", err)
				continue
			}
			log.Println("Submitted assertion: ", transaction.Hash())
		}
	}
}

func (p *Proposer) getTrustedInput(ctx context.Context) (*enclave.TrustedInput, error) {
	contractTrustedInput, err := p.TeeModule.TeeTrustedInput(&bind.CallOpts{Context: ctx})
	if err != nil {
		return nil, err
	}
	trustedInput := enclave.TrustedInput{
		ConfigHash:           contractTrustedInput.ConfigHash,
		AppStartBlockHash:    contractTrustedInput.AppStartBlockHash,
		SeqStartBlockHash:    contractTrustedInput.SeqStartBlockHash,
		SetDelayedMessageAcc: contractTrustedInput.SetDelayedMessageAcc,
		L1StartBatchAcc:      contractTrustedInput.L1StartBatchAcc,
		L1EndHash:            contractTrustedInput.L1EndHash,
	}

	jsonInput, err := json.Marshal(trustedInput)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal trusted input: %v", err)
	}
	log.Println("Trusted input: ", string(jsonInput))
	return &trustedInput, nil
}

func (p *Proposer) getBatchCount(ctx context.Context, l1Hash common.Hash) (uint64, error) {
	var batchCount uint64
	if p.Config.IsL1Chain {
		if err := p.SequencingClient.Client().CallContext(ctx, &batchCount, "synd_batchFromAcc", l1Hash); err != nil {
			return 0, err
		}
	} else {
		count, err := p.EthereumClient.StorageAtHash(ctx, p.Config.EnclaveConfig.SequencingBridgeAddress, enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, l1Hash)
		if err != nil {
			return 0, err
		}
		batchCount = common.BytesToHash(count).Big().Uint64()
	}
	if batchCount == 0 {
		return 0, errors.New("end batch count is 0")
	}
	return batchCount, nil
}

// TODO (SEQ-1061): Replace enclave types with auto-generated types from the bindings
func (p *Proposer) Prove(ctx context.Context, trustedInput *enclave.TrustedInput) (*enclave.VerifyAppchainOutput, error) {
	// get trusted input
	if trustedInput == nil {
		var err error
		trustedInput, err = p.getTrustedInput(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to get trusted input: %v", err)
		}
	}

	// get the batch count
	endBatchCount, err := p.getBatchCount(ctx, trustedInput.L1EndHash)
	if err != nil {
		return nil, fmt.Errorf("failed to get batch count: %v", err)
	}

	// get the start block
	header, err := p.SequencingClient.HeaderByHash(ctx, trustedInput.SeqStartBlockHash)
	if err != nil {
		return nil, fmt.Errorf("failed to get sequencing header: %v", err)
	}

	// get validation data
	var valData ValidationData
	log.Println("Getting validation data from start block: ", header.Number.Uint64(), " to end batch: ", endBatchCount-1)
	if err := p.SequencingClient.Client().CallContext(ctx, &valData, "synd_validationData", header.Number.Uint64(), endBatchCount-1, false); err != nil {
		return nil, fmt.Errorf("failed to get validation data: %v", err)
	}

	preimages := make(map[arbutil.PreimageType]map[common.Hash][]byte)
	preimages[arbutil.Keccak256PreimageType] = make(map[common.Hash][]byte)
	for _, preimage := range valData.PreimageData {
		preimages[arbutil.Keccak256PreimageType][crypto.Keccak256Hash(preimage)] = preimage
	}

	// get batches
	log.Println("Getting batches...")
	var batches [][]byte
	if valData.BatchEndIndex >= valData.BatchStartIndex {
		ibridge, err := bridgegen.NewIBridgeCaller(p.Config.EnclaveConfig.SequencingBridgeAddress, p.EthereumClient)
		if err != nil {
			return nil, fmt.Errorf("failed to create ibridge caller: %v", err)
		}

		seqInbox, err := ibridge.SequencerInbox(&bind.CallOpts{Context: ctx})
		if err != nil {
			return nil, fmt.Errorf("failed to get sequencer inbox: %v", err)
		}

		batches, err = getBatches(ctx, p.EthereumClient, seqInbox, valData.BatchStartIndex, valData.BatchEndIndex, valData.BatchStartBlockNum, valData.BatchEndBlockNum)
		if err != nil {
			return nil, fmt.Errorf("failed to get batches: %v", err)
		}
		if len(batches) == 0 {
			return nil, fmt.Errorf("found 0 batches")
		}
		// update preimages
		for _, batch := range batches {
			if err := getBatchPreimageData(ctx, batch, p.DapReaders, preimages); err != nil {
				return nil, fmt.Errorf("failed to get batch preimage data: %v", err)
			}
		}
	}

	log.Println("Getting proof...")
	var proof *enclave.AccountResult
	if !p.Config.IsL1Chain {
		// get the end block header
		if header, err = p.EthereumClient.HeaderByHash(ctx, trustedInput.L1EndHash); err != nil {
			return nil, fmt.Errorf("failed to get sequencing header: %v", err)
		}

		// get merkle proof
		accSlot := common.BigToHash(new(big.Int).Add(enclave.BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE, big.NewInt(int64(endBatchCount))))
		if err := p.EthereumClient.Client().CallContext(ctx, &proof, "eth_getProof", p.Config.EnclaveConfig.SequencingBridgeAddress, []common.Hash{enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, accSlot}, trustedInput.L1EndHash); err != nil {
			return nil, fmt.Errorf("failed to get proof: %v", err)
		}
	}

	preimageData := make(map[arbutil.PreimageType][][]byte)
	for ty, images := range preimages {
		preimageData[ty] = slices.Collect(maps.Values(images))
	}

	// derive sequencing chain
	log.Println("Verifying sequencing chain...")
	var seqOutput enclave.VerifySequencingChainOutput
	if err := p.handleEnclaveCall(&seqOutput, "enclave_verifySequencingChain", enclave.VerifySequencingChainInput{
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
		return nil, fmt.Errorf("failed to verify sequencing chain: %v", err)
	}

	// get appchain start block
	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return nil, fmt.Errorf("failed to get appchain header: %v", err)
	}

	// get delayed messages
	startAcc, msgs, isDummy, err := GetDelayedMessages(ctx, p.SettlementClient, p.Config.AppchainBridgeAddress, header.Nonce.Uint64(), trustedInput.SetDelayedMessageAcc)
	if err != nil {
		return nil, fmt.Errorf("failed to get delayed messages: %v", err)
	}

	// get the number of batches. ignore the delayed message if it is a dummy one
	var realMsgs [][]byte
	if !isDummy {
		realMsgs = msgs
	}
	numBatches := getNumBatches(seqOutput.Batches, realMsgs, p.Config.EnclaveConfig.SettlementDelay)

	// get appchain preimage data
	var appPreimages [][]byte
	if err := p.AppchainClient.Client().CallContext(ctx, &appPreimages, "synd_preimageData", header.Number, numBatches, true); err != nil {
		return nil, fmt.Errorf("failed to get appchain preimage data: %v", err)
	}

	// derive appchain
	log.Println("Verifying appchain...")
	var appOutput enclave.VerifyAppchainOutput
	if err := p.handleEnclaveCall(&appOutput, "enclave_verifyAppchain", enclave.VerifyAppchainInput{
		TrustedInput:                    *trustedInput,
		Config:                          p.Config.EnclaveConfig,
		DelayedMessages:                 msgs,
		StartDelayedMessagesAccumulator: startAcc,
		VerifySequencingChainOutput:     seqOutput,
		AppStartBlockHeader:             *header,
		PreimageData: map[arbutil.PreimageType][][]byte{
			arbutil.Keccak256PreimageType: appPreimages},
	}); err != nil {
		return nil, fmt.Errorf("failed to verify appchain: %v", err)
	}
	return &appOutput, nil
}

func (p *Proposer) Verify(ctx context.Context, trustedInput *enclave.TrustedInput) (*enclave.VerifyAppchainOutput, error) {
	// get trusted input
	if trustedInput == nil {
		var err error
		trustedInput, err = p.getTrustedInput(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to get trusted input: %v", err)
		}
	}

	// get the batch count
	endBatchCount, err := p.getBatchCount(ctx, trustedInput.L1EndHash)
	if err != nil {
		return nil, fmt.Errorf("failed to get batch count: %v", err)
	}
	log.Println("Getting batch metadata for batch: ", endBatchCount-1)

	var metadata arbnode.BatchMetadata
	if err := p.SequencingClient.Client().CallContext(ctx, &metadata, "synd_batchMetadata", endBatchCount-1); err != nil {
		return nil, fmt.Errorf("failed to get batch metadata: %v", err)
	}

	if metadata.MessageCount == 0 {
		return nil, errors.New("message count is 0")
	}

	// get the end block
	header, err := p.SequencingClient.HeaderByNumber(ctx, big.NewInt(int64(metadata.MessageCount-1)))
	if err != nil {
		return nil, fmt.Errorf("failed to get sequencing header: %v", err)
	}
	sequencingBlockHash := header.Hash()

	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return nil, fmt.Errorf("failed to get appchain header: %v", err)
	}

	// binary search to find the appchain end block
	appEndBlock, err := FindBlock(ctx, p.AppchainClient, header.Number.Uint64(), uint64(metadata.MessageCount-1))
	if err != nil {
		return nil, fmt.Errorf("failed to find appchain end block: %v", err)
	}

	return &enclave.VerifyAppchainOutput{
		L1BatchAcc:          metadata.Accumulator,
		SequencingBlockHash: sequencingBlockHash,
		AppchainBlockHash:   appEndBlock.BlockHash,
		AppchainSendRoot:    appEndBlock.SendRoot,
	}, nil
}

func (p *Proposer) handleEnclaveCall(output interface{}, method string, input interface{}) error {
	if err := p.EnclaveClient.Call(output, method, input); err != nil {
		return handleTLSErr(err)
	}
	return nil
}
