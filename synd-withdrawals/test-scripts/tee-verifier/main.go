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
	seqUrl := flag.String("seq-url", "https://risa-testnet.us-central1.gcp.testnet.syndicate.io", "sequencing chain rpc url")
	enclaveUrl := flag.String("enclave-url", "https://verifier.direct.us-east-2.aws.testnet.syndicate.io", "enclave rpc url")
	appUrl := flag.String("app-url", "https://rpc.testnet.manchego.syndicate.io", "appchain rpc url")

	// config flags - for risa testnet
	seqContractFlag := flag.String("seq-contract", "0x1e491B3C0A53492F72dBE5A48C6cd6ffe19b643E", "sequencing contract address for appchain")
	seqBridgeFlag := flag.String("seq-bridge", "0x1043E08195914c32ec3a4a075d9Eb2B0DC2fB1aA", "sequencing chain bridge contract address")
	appBridgeFlag := flag.String("app-bridge", "0x646eD51Ef2daD941733b004961d9ceC2B32BACF8", "appchain bridge address")

	// config flags - for risa devnet
	// seqContractFlag := flag.String("seq-contract", "0xb89D1d2E9bc9A14855e6C8509dd5435422CcDd8f", "sequencing contract address for appchain")
	// seqBridgeFlag := flag.String("seq-bridge", "0x765E6EC7f3A8c8A2712EA230754E5968E45E124b", "sequencing chain bridge contract address")
	// appBridgeFlag := flag.String("app-bridge", "0xC5432874Fe53da9185a34eCdf48A3a2a2A8Bd241", "appchain bridge address")

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
		Port:                     8080,
		EnclaveConfig: enclave.Config{
			SequencingContractAddress: seqContractAddress,
			SequencingBridgeAddress:   seqBridgeAddress,
			SettlementDelay:           *setDelay,
		},
	}

	proposer := pkg.NewProposer(ctx, proposerConfig)

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

	fmt.Println("Trusted input SeqStartBlockHash: ", common.Hash(trustedInput.SeqStartBlockHash))
	fmt.Println("Trusted input AppStartBlockHash: ", common.Hash(trustedInput.AppStartBlockHash))
	fmt.Println("Trusted input L1StartBatchAcc: ", common.Hash(trustedInput.L1StartBatchAcc))
	fmt.Println("ready in", time.Since(now))
	now = time.Now()
	proveOutput, err := proposer.Prove(ctx, trustedInput, true)
	if err != nil {
		panic(err)
	}
	out, err := json.Marshal(proveOutput)
	if err != nil {
		panic(err)
	}
	println("Proof output: ", string(out))
	fmt.Println("proof took", time.Since(now))
	now = time.Now()
	verifyOutput, err := proposer.Verify(ctx, trustedInput)
	if err != nil {
		panic(err)
	}
	out, err = json.Marshal(verifyOutput)
	if err != nil {
		panic(err)
	}
	println("Verify output: ", string(out))
	fmt.Println("verify took", time.Since(now))

	if verifyOutput.PendingAssertion.AppBlockHash != proveOutput.PendingAssertion.AppBlockHash {
		panic("appchain block hash mismatch")
	}
	if verifyOutput.PendingAssertion.AppSendRoot != proveOutput.PendingAssertion.AppSendRoot {
		panic("appchain send root mismatch")
	}
	if verifyOutput.PendingAssertion.SeqBlockHash != proveOutput.PendingAssertion.SeqBlockHash {
		panic("sequencing block hash mismatch")
	}
	if verifyOutput.PendingAssertion.L1BatchAcc != proveOutput.PendingAssertion.L1BatchAcc {
		panic("l1 batch acc mismatch")
	}

	println("✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ All good! 🎉")
}
