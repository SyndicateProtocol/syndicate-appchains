package main

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"encoding/json"
	"flag"
	"fmt"
	"math/big"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
	"github.com/ethereum/go-ethereum/common"
	"github.com/offchainlabs/nitro/arbnode"
)

func main() {
	now := time.Now()

	// config flags - optional. urls
	eigenUrl := flag.String("eigenda-url", "https://risa-testnet-eigenda-mirror.rollups.alchemy.com", "eigenda proxy url")
	l1Url := flag.String("l1-url", "https://eth-sepolia.g.alchemy.com/v2/xZF7o-Vl3z94HOqwaQtrZP06swu4_E15", "l1 rpc url")
	setUrl := flag.String("set-url", "https://base-sepolia.g.alchemy.com/v2/FFOCYExawZ3K46YRNHqaUEo3pbqS5F1F", "settlement rpc url")
	seqUrl := flag.String("seq-url", "http://localhost:8545", "sequencing chain rpc url")
	enclaveUrl := flag.String("enclave-url", "http://localhost:1234", "enclave rpc url")
	appUrl := flag.String("app-url", "http://localhost:8546", "appchain rpc url")

	// config flags - optional. addrs
	seqContractFlag := flag.String("seq-contract", "0x7f389b0827d38D047c98fAbBfbf004a966dB8Dc1", "sequencing contract address for appchain")
	seqBridgeFlag := flag.String("seq-bridge", "0x1043E08195914c32ec3a4a075d9Eb2B0DC2fB1aA", "sequencing chain bridge contract address")
	appBridgeFlag := flag.String("app-bridge", "0x509e8942e6C1626dA3d45060aB39B86e8F246E98", "appchain bridge address")

	// config flags - optional. settlement
	setMsgs := flag.Uint64("set-msg-count", 0, "settlement delayed message count")
	setDelay := flag.Uint64("set-delay", 60, "settlement chain delay, in seconds")

	// config flags - required
	l1StartBatch := flag.Uint64("start-batch", 0, "l1 start batch")
	l1EndBatch := flag.Uint64("end-batch", 0, "l1 end batch")

	flag.Parse()
	appBridge := common.HexToAddress(*appBridgeFlag)
	seqContractAddress := common.HexToAddress(*seqContractFlag)
	seqBridgeAddress := common.HexToAddress(*seqBridgeFlag)

	ctx := context.Background()

	var teeModuleAddress common.Address // Don't need to set this value for this script

	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		panic(err)
	}

	proposerConfig := &pkg.Config{
		EthereumRPCURL:           *l1Url,
		SettlementRPCURL:         *setUrl,
		SettlementChainID:        85432,
		SequencingRPCURL:         *seqUrl,
		AppchainRPCURL:           *appUrl,
		EnclaveRPCURL:            *enclaveUrl,
		EigenRPCUrl:              *eigenUrl,
		TeeModuleContractAddress: teeModuleAddress,
		AppchainBridgeAddress:    appBridge,
		PrivateKey:               privateKey,
		PollingInterval:          1000,
		CloseChallengeInterval:   1000,
		MetricsPort:              8080,
		EnclaveConfig: enclave.Config{
			SequencingContractAddress: seqContractAddress,
			SequencingBridgeAddress:   seqBridgeAddress,
			SettlementDelay:           *setDelay,
		},
	}

	proposer := pkg.NewProposer(proposerConfig)

	// normally this comes from the tee contract instead
	var startMetadata arbnode.BatchMetadata
	if err := proposer.SequencingClient.Client().CallContext(ctx, &startMetadata, "synd_batchMetadata", l1StartBatch); err != nil {
		panic(err)
	}
	var endMetadata arbnode.BatchMetadata
	if err := proposer.SequencingClient.Client().CallContext(ctx, &endMetadata, "synd_batchMetadata", l1EndBatch); err != nil {
		panic(err)
	}

	cfg := enclave.Config{
		SequencingContractAddress: seqContractAddress,
		SequencingBridgeAddress:   seqBridgeAddress,
		SettlementDelay:           *setDelay,
	}

	startSeqNum := uint64(startMetadata.MessageCount) - 1

	header, err := proposer.SequencingClient.HeaderByNumber(ctx, big.NewInt(int64(startSeqNum)))
	if err != nil {
		panic(err)
	}

	startSeqBlock := header.Hash()

	// binary search to find the start block
	result, err := pkg.FindBlock(ctx, proposer.AppchainClient, 0, startSeqNum)
	if err != nil {
		panic(err)
	}

	// can add an arbitrary offset to the end block
	if header, err = proposer.EthereumClient.HeaderByNumber(ctx, big.NewInt(int64(endMetadata.ParentChainBlock))); err != nil {
		panic(err)
	}

	setDelayedAcc, _, err := pkg.GetMessageAcc(ctx, proposer.SettlementClient, appBridge, *setMsgs)
	if err != nil {
		panic(err)
	}

	trustedInput := &enclave.TrustedInput{
		ConfigHash:           cfg.Hash(),
		AppStartBlockHash:    result.BlockHash,
		SeqStartBlockHash:    startSeqBlock,
		SetDelayedMessageAcc: common.Hash(setDelayedAcc),
		L1StartBatchAcc:      startMetadata.Accumulator,
		L1EndHash:            header.Hash(),
	}

	trustedInputJson, err := json.Marshal(trustedInput)
	if err != nil {
		panic(err)
	}
	fmt.Println("Trusted input: ", string(trustedInputJson))
	fmt.Println("ready in", time.Since(now))
	now = time.Now()
	appOutput, err := proposer.Prove(ctx, trustedInput, false)
	if err != nil {
		panic(err)
	}
	out, err := json.Marshal(appOutput)
	if err != nil {
		panic(err)
	}
	println("Proof output: ", string(out))
	fmt.Println("proof took", time.Since(now))
	now = time.Now()
	verifyOutput, err := proposer.Verify(ctx, trustedInput, false)
	if err != nil {
		panic(err)
	}
	out, err = json.Marshal(verifyOutput)
	if err != nil {
		panic(err)
	}
	println("Verify output: ", string(out))
	fmt.Println("verify took", time.Since(now))
}
