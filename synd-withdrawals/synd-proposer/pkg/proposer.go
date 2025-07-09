package pkg

import (
	"context"
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
)

type Proposer struct {
	Config           *Config
	AppchainClient   *ethclient.Client
	SequencingClient *ethclient.Client
	EthereumClient   *ethclient.Client
	SettlementClient *ethclient.Client
	SettlementAuth   *bind.TransactOpts
	EnclaveClient    *rpc.Client
	TeeModule        *teemodule.Teemodule
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
	enclaveClient, err := rpc.Dial(cfg.EnclaveRPCURL)
	if err != nil {
		log.Fatalf("Failed to create enclave provider: %v", err)
		return nil
	}
	settlementClient, err := ethclient.Dial(cfg.SettlementRPCURL)
	if err != nil {
		log.Fatalf("Failed to create settlement provider: %v", err)
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
			// TODO (SEQ-1060): Optimize this to only prove if the last assertion is not the same as the current one. Use `Verify` to check if the last assertion is the same as the current one.
			log.Println("Polling loop tick...")
			appOutput, err := p.Prove(ctx, nil)
			if err != nil {
				log.Printf("Failed to prove: %v", err)
			}
			pendingAssertion := teemodule.PendingAssertion{
				AppBlockHash: appOutput.AppchainBlockHash,
				AppSendRoot:  appOutput.AppchainSendRoot,
				SeqBlockHash: appOutput.SequencingBlockHash,
				L1BatchAcc:   appOutput.L1BatchAcc,
			}
			fmt.Println("Appchain output: ", appOutput)
			p.TeeModule.SubmitAssertion(p.SettlementAuth, pendingAssertion, appOutput.Signature, crypto.PubkeyToAddress(p.Config.PrivateKey.PublicKey))
			if err != nil {
				log.Printf("Failed to submit assertion: %v", err)
			}
		}
	}
}

func (p *Proposer) getTrustedInput() (enclave.TrustedInput, error) {
	contractTrustedInput, err := p.TeeModule.TeeTrustedInput(nil)
	if err != nil {
		return enclave.TrustedInput{}, err
	}
	trustedInput := enclave.TrustedInput{
		ConfigHash:           contractTrustedInput.ConfigHash,
		AppStartBlockHash:    contractTrustedInput.AppStartBlockHash,
		SeqStartBlockHash:    contractTrustedInput.SeqStartBlockHash,
		SetDelayedMessageAcc: contractTrustedInput.SetDelayedMessageAcc,
		L1StartBatchAcc:      contractTrustedInput.L1StartBatchAcc,
		L1EndHash:            contractTrustedInput.L1EndHash,
	}

	fmt.Println("Trusted input: ", trustedInput)
	return trustedInput, nil
}

// TODO (SEQ-1061): Replace enclave types with auto-generated types from the bindings
func (p *Proposer) Prove(ctx context.Context, trustedInputParam *enclave.TrustedInput) (enclave.VerifyAppchainOutput, error) {
	var appOutput enclave.VerifyAppchainOutput
	var trustedInput enclave.TrustedInput

	if trustedInputParam == nil {
		var err error
		trustedInput, err = p.getTrustedInput()
		if err != nil {
			return appOutput, err
		}
	} else {
		trustedInput = *trustedInputParam
	}

	// Prove method
	count, err := p.EthereumClient.StorageAtHash(ctx, p.Config.EnclaveConfig.SequencingBridgeAddress, enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, trustedInput.L1EndHash)
	if err != nil {
		return appOutput, err
	}
	endBatchCount := common.BytesToHash(count).Big().Uint64()
	fmt.Println("End batch count: ", endBatchCount)

	if endBatchCount == 0 {
		return appOutput, fmt.Errorf("end batch count is 0")
	}

	// get the start block
	header, err := p.SequencingClient.HeaderByHash(ctx, trustedInput.SeqStartBlockHash)
	if err != nil {
		return appOutput, err
	}

	// get validation data
	var valData ValidationData
	if err := p.SequencingClient.Client().CallContext(ctx, &valData, "synd_validationData", header.Number.Uint64(), endBatchCount-1, false); err != nil {
		return appOutput, err
	}

	fmt.Println("Validation data: ", valData)

	// get preimages
	preimages := make(map[common.Hash][]byte)
	for _, preimage := range valData.PreimageData {
		preimages[crypto.Keccak256Hash(preimage)] = preimage
	}

	fmt.Println("Preimages: ", preimages)

	// get batches
	var batches [][]byte
	if valData.BatchEndIndex >= valData.BatchStartIndex {
		batches, err = getBatches(ctx, p.EthereumClient, p.Config.SequencingInboxAddress, valData.BatchStartIndex, valData.BatchEndIndex, valData.BatchStartBlockNum, valData.BatchEndBlockNum)
		if err != nil {
			return appOutput, err
		}
		if len(batches) == 0 {
			return appOutput, fmt.Errorf("found 0 batches")
		}
		// update preimages
		for _, batch := range batches {
			// TODO (SEQ-1062): add dapReaders to this function call
			if err := getBatchPreimageData(ctx, batch, nil, preimages); err != nil {
				return appOutput, err
			}
		}
	}

	var proof *enclave.AccountResult

	// get the end block header
	if header, err = p.EthereumClient.HeaderByHash(ctx, trustedInput.L1EndHash); err != nil {
		return appOutput, err
	}

	// get merkle proof
	accSlot := common.BigToHash(new(big.Int).Add(enclave.BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE, big.NewInt(int64(endBatchCount))))
	if err := p.EthereumClient.Client().CallContext(ctx, &proof, "eth_getProof", p.Config.EnclaveConfig.SequencingBridgeAddress, []common.Hash{enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, accSlot}, trustedInput.L1EndHash); err != nil {
		return appOutput, err
	}

	// derive sequencing chain
	var seqOutput enclave.VerifySequencingChainOutput
	if err := p.EnclaveClient.Call(&seqOutput, "enclave_verifySequencingChain", enclave.VerifySequencingChainInput{
		TrustedInput:                    trustedInput,
		Config:                          p.Config.EnclaveConfig,
		DelayedMessages:                 valData.DelayedMessages,
		StartDelayedMessagesAccumulator: valData.StartDelayedAcc,
		Batches:                         batches,
		IsL1Chain:                       false,
		PreimageData:                    slices.Collect(maps.Values(preimages)),
		EndBatchAccumulatorMerkleProof:  proof,
		L1EndBlockHeader:                header,
	}); err != nil {
		return appOutput, err
	}

	fmt.Println("Sequencing chain: ", seqOutput)

	// get appchain start block
	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return appOutput, err
	}

	// get delayed messages
	startAcc, msgs, isDummy, err := getDelayedMessages(ctx, p.SettlementClient, p.Config.AppchainBridgeAddress, p.Config.AppchainInboxAddress, header.Nonce.Uint64(), trustedInput.SetDelayedMessageAcc)
	if err != nil {
		return appOutput, err
	}
	fmt.Println("Delayed messages: ", msgs)

	// get the number of batches. ignore the delayed message if it is a dummy one
	var realMsgs [][]byte
	if !isDummy {
		realMsgs = msgs
	}
	numBatches := getNumBatches(seqOutput.Batches, realMsgs, p.Config.EnclaveConfig.SettlementDelay)

	// get preimage data
	var preimageData [][]byte
	if err := p.AppchainClient.Client().CallContext(ctx, &preimageData, "synd_preimageData", header.Number, numBatches, true); err != nil {
		return appOutput, err
	}

	// derive appchain
	if err := p.EnclaveClient.Call(&appOutput, "enclave_verifyAppchain", enclave.VerifyAppchainInput{
		TrustedInput:                    trustedInput,
		Config:                          p.Config.EnclaveConfig,
		DelayedMessages:                 msgs,
		StartDelayedMessagesAccumulator: startAcc,
		VerifySequencingChainOutput:     seqOutput,
		AppStartBlockHeader:             *header,
		PreimageData:                    preimageData,
	}); err != nil {
		return appOutput, err
	}
	fmt.Println("Appchain output: ", appOutput)

	return appOutput, nil
}

func (p *Proposer) Verify(ctx context.Context, trustedInputParam *enclave.TrustedInput) (enclave.VerifyAppchainOutput, error) {
	var appOutput enclave.VerifyAppchainOutput
	var trustedInput enclave.TrustedInput

	if trustedInputParam == nil {
		var err error
		trustedInput, err = p.getTrustedInput()
		if err != nil {
			return appOutput, err
		}
	} else {
		trustedInput = *trustedInputParam
	}

	count, err := p.EthereumClient.StorageAtHash(ctx, p.Config.EnclaveConfig.SequencingBridgeAddress, enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, trustedInput.L1EndHash)
	if err != nil {
		return appOutput, err
	}
	endBatchCount := common.BytesToHash(count).Big().Uint64()
	if endBatchCount == 0 {
		return appOutput, fmt.Errorf("end batch count is 0")
	}

	var metadata arbnode.BatchMetadata
	if err := p.SequencingClient.Client().CallContext(ctx, &metadata, "synd_batchMetadata", endBatchCount-1); err != nil {
		return appOutput, err
	}

	if metadata.MessageCount == 0 {
		return appOutput, fmt.Errorf("message count is 0")
	}

	header, err := p.SequencingClient.HeaderByNumber(ctx, big.NewInt(int64(metadata.MessageCount-1)))
	if err != nil {
		return appOutput, err
	}
	sequencingBlockHash := header.Hash()

	if header, err = p.AppchainClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return appOutput, err
	}

	// binary search to find the appchain end block
	appEndBlock, err := FindBlock(ctx, p.AppchainClient, header.Number.Uint64(), uint64(metadata.MessageCount-1))
	if err != nil {
		return appOutput, err
	}
	appOutput = enclave.VerifyAppchainOutput{
		L1BatchAcc:          metadata.Accumulator,
		SequencingBlockHash: sequencingBlockHash,
		AppchainBlockHash:   appEndBlock.BlockHash,
		AppchainSendRoot:    appEndBlock.SendRoot,
	}
	return appOutput, nil
}
