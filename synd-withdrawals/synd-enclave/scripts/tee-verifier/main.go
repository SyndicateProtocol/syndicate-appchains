package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbutil"
)

type ValidationData struct {
	Batches         []enclave.ArbitrumBatch
	DelayedMessages []arbostypes.L1IncomingMessage
	StartDelayedAcc common.Hash
	PreimageData    [][]byte
}

type BatchMetadata struct {
	Accumulator         common.Hash
	MessageCount        arbutil.MessageIndex
	DelayedMessageCount uint64
	ParentChainBlock    uint64
}

func main() {
	// config flags - optional
	l1Url := flag.String("l1-url", "", "l1 rpc url")
	_ = l1Url
	seqUrl := flag.String("seq-url", "http://localhost:8545", "sequencing chain rpc url")
	seqContractFlag := flag.String("seq-contract", "0xb89D1d2E9bc9A14855e6C8509dd5435422CcDd8f", "sequencing contract address for appchain")
	seqBridgeFlag := flag.String("seq-bridge", "0x765E6EC7f3A8c8A2712EA230754E5968E45E124b", "sequencing chain bridge contract address")
	setDelay := flag.Uint64("set-delay", 60, "settlement chain delay")
	enclaveUrl := flag.String("enclave-url", "http://localhost:1234", "enclave rpc url")
	setDelayedMessagesFlag := flag.String("set-delayed-msg-acc", "", "settlement chain delayed message accumulator")
	appUrl := flag.String("rpc-url", "", "appchain rpc url")
	_ = appUrl

	// config flags - required
	l1StartBatch := flag.Uint64("start-batch", 0, "l1 start batch")
	l1EndBatch := flag.Uint64("end-batch", 0, "l1 end batch")
	appStartFlag := flag.String("app-start-block", "", "app start block hash")

	flag.Parse()
	seqContractAddress := common.HexToAddress(*seqContractFlag)
	seqBridgeAddress := common.HexToAddress(*seqBridgeFlag)
	setDelayedMessagesAcc := common.HexToHash(*setDelayedMessagesFlag)
	appStartBlockHash := common.HexToHash(*appStartFlag)

	seqClient, err := rpc.Dial(*seqUrl)
	if err != nil {
		panic(err)
	}
	enclaveClient, err := rpc.Dial(*enclaveUrl)
	if err != nil {
		panic(err)
	}

	// normally this comes from the tee contract instead
	var startMetadata BatchMetadata
	if err := seqClient.Call(&startMetadata, "synd_batchMetadata", l1StartBatch); err != nil {
		panic(err)
	}
	var endMetadata BatchMetadata
	if err := seqClient.Call(&endMetadata, "synd_batchMetadata", l1EndBatch); err != nil {
		panic(err)
	}

	cfg := enclave.Config{
		SequencingContractAddress: seqContractAddress,
		SequencingBridgeAddress:   seqBridgeAddress,
		SettlementDelay:           *setDelay,
	}

	var resp types.Header

	startSeqNum := uint64(startMetadata.MessageCount) - 1

	if err := seqClient.Call(&resp, "eth_getBlockByNumber", hexutil.Uint64(startSeqNum), false); err != nil {
		panic(err)
	}

	trustedInput := enclave.TrustedInput{
		ConfigHash:           cfg.Hash(),
		AppStartBlockHash:    appStartBlockHash,
		SeqStartBlockHash:    resp.Hash(),
		SetDelayedMessageAcc: setDelayedMessagesAcc,
		L1StartBatchAcc:      startMetadata.Accumulator,
		L1EndHash:            endMetadata.Accumulator,
	}

	var valData ValidationData
	if err := seqClient.Call(&valData, "synd_validationData", startSeqNum, *l1EndBatch-*l1StartBatch); err != nil {
		panic(err)
	}

	now := time.Now()
	var seqOutput enclave.VerifySequencingChainOutput
	if err := enclaveClient.Call(&seqOutput, "enclave_verifySequencingChain", enclave.VerifySequencingChainInput{
		TrustedInput:                    trustedInput,
		Config:                          cfg,
		DelayedMessages:                 valData.DelayedMessages,
		StartDelayedMessagesAccumulator: valData.StartDelayedAcc,
		Batches:                         valData.Batches,
		IsL1Chain:                       true,
		PreimageData:                    valData.PreimageData,
	}); err != nil {
		panic(err)
	}
	fmt.Println("enclave runtime: ", time.Now().Sub(now))
	out, err := json.Marshal(seqOutput)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(out))

	/*
		l1Client, err := rpc.Dial(l1Url)
		if err != nil {
			panic(err)
		}
		appClient, err := rpc.Dial(appUrl)
		if err != nil {
			panic(err)
		}

		if err := appClient.Call(&valData, "synd_validationData", startSeqNum, 0, false); err != nil {
			panic(err)
		}

		if err := seqClient.Call(&resp, "eth_getBlockByHash", appStartBlockHash, false); err != nil {
			panic(err)
		}

		var appOutput enclave.VerifyAppchainOutput
		if err := enclaveClient.Call(&seqOutput, "enclave_verifyAppchain", enclave.VerifyAppchainInput{
			TrustedInput: trustedInput,
			Config:       cfg,
			// this is wrong - need to fetch events manually for substitutions to work properly
			DelayedMessages: valData.DelayedMessages,
			// this is wrong - acc is different on-chain
			StartDelayedMessagesAccumulator: valData.StartDelayedAcc,
			VerifySequencingChainOutput:     seqOutput,
			AppStartBlockHeader:             resp,
			PreimageData:                    valData.PreimageData,
		}); err != nil {
			panic(err)
		}
		println("Final output: ", appOutput)
	*/
}
