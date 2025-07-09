package main

import (
	"context"
	"encoding/binary"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"maps"
	"math/big"
	"slices"
	"strings"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/offchainlabs/nitro/arbnode"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/daprovider"
	"github.com/offchainlabs/nitro/eigenda"
	"github.com/offchainlabs/nitro/execution"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
)

// keep in sync with the nitro node
type ValidationData struct {
	BatchStartBlockNum uint64
	BatchEndBlockNum   uint64
	BatchStartIndex    uint64
	BatchEndIndex      uint64
	DelayedMessages    [][]byte
	StartDelayedAcc    common.Hash
	PreimageData       [][]byte
}

var messageDeliveredID common.Hash
var inboxMessageDeliveredID common.Hash

func init() {
	parsedIBridgeABI, err := bridgegen.IBridgeMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	messageDeliveredID = parsedIBridgeABI.Events["MessageDelivered"].ID
	parsedIMessageProviderABI, err := bridgegen.IDelayedMessageProviderMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	inboxMessageDeliveredID = parsedIMessageProviderABI.Events["InboxMessageDelivered"].ID
}

func getLogs(ctx context.Context, c *ethclient.Client, startBlock uint64, endBlock uint64, addresses []common.Address, topics [][]common.Hash, maxQty uint64) ([]types.Log, error) {
	if startBlock > endBlock {
		return nil, errors.New("start block > end block")
	}
	logs, err := c.FilterLogs(ctx, ethereum.FilterQuery{
		FromBlock: big.NewInt(int64(startBlock)),
		ToBlock:   big.NewInt(int64(endBlock)),
		Addresses: addresses,
		Topics:    topics,
	})
	// TODO: check if a valid error response object is received before retrying
	if err == nil {
		return logs, nil
	}
	if startBlock == endBlock {
		return nil, err
	}
	mid := (startBlock + endBlock) / 2
	logs, err = getLogs(ctx, c, mid+1, endBlock, addresses, topics, maxQty)
	if err != nil {
		return nil, err
	}
	if maxQty > 0 && uint64(len(logs)) >= maxQty {
		return logs, nil
	}
	prevLogs, err := getLogs(ctx, c, startBlock, mid, addresses, topics, maxQty)
	if err != nil {
		return nil, err
	}
	prevLogs = append(prevLogs, logs...)
	return prevLogs, nil
}

func getBatches(ctx context.Context, c *ethclient.Client, sequencerInbox common.Address, start uint64, end uint64, startBlock uint64, endBlock uint64) ([][]byte, error) {
	inbox, err := arbnode.NewSequencerInbox(c, sequencerInbox, 0)
	if err != nil {
		return nil, err
	}
	// TODO: use custom client that auto bisects the log range
	batches, err := inbox.LookupBatchesInRange(ctx, big.NewInt(int64(startBlock)), big.NewInt(int64(endBlock)))
	if err != nil {
		return nil, err
	}
	var data [][]byte
	for _, batch := range batches {
		if batch.SequenceNumber >= start && batch.SequenceNumber <= end {
			// TODO: can this be sped up? probably not since the tx receipt needs to be fetched in general
			raw, err := batch.Serialize(ctx, c)
			if err != nil {
				return nil, err
			}
			data = append(data, raw)
		}
	}
	return data, nil
}

func getBatchPreimageData(ctx context.Context, batch []byte, dapReaders []daprovider.Reader, preimages map[arbutil.PreimageType]map[common.Hash][]byte) error {
	if len(batch) > 40 {
		for _, dapReader := range dapReaders {
			if dapReader != nil && dapReader.IsValidHeaderByte(ctx, batch[40]) {
				// TODO: try to speed this up - can disable validation as well if it is slow.
				_, preimagesRecorded, err := dapReader.RecoverPayloadFromBatch(ctx, 0, common.Hash{}, batch, nil, true)
				if err != nil {
					// Matches the way keyset validation was done inside DAS readers i.e logging the error
					//  But other daproviders might just want to return the error
					if strings.Contains(err.Error(), daprovider.ErrSeqMsgValidation.Error()) && daprovider.IsDASMessageHeaderByte(batch[40]) {
						log.Error(err.Error())
					} else {
						return err
					}
				}

				for ty, images := range preimagesRecorded {
					if preimages[ty] == nil {
						preimages[ty] = images
					} else {
						maps.Copy(preimages[ty], images)
					}
				}
				return nil
			}
		}
		if daprovider.IsDASMessageHeaderByte(batch[40]) {
			log.Error("No DAS Reader configured, but sequencer message found with DAS header")
		}
	}
	return nil
}

// if count is zero, fetches the count instead
func getMessageAcc(ctx context.Context, c *ethclient.Client, bridge common.Address, count uint64) (common.Hash, uint64, error) {
	slot := common.BigToHash(big.NewInt(6))
	if count == 0 {
		countBytes, err := c.StorageAt(ctx, bridge, slot, nil)
		if err != nil {
			return common.Hash{}, 0, err
		}
		count = common.Hash(countBytes).Big().Uint64()
	}
	if count == 0 {
		return common.Hash{}, 0, errors.New("delayed acc count is zero")
	}
	slot = common.BigToHash(new(big.Int).Add(crypto.Keccak256Hash(slot[:]).Big(), big.NewInt(int64(count-1))))
	acc, err := c.StorageAt(ctx, bridge, slot, nil)
	if err != nil {
		return common.Hash{}, 0, err
	}
	if common.Hash(acc).Cmp(common.Hash{}) == 0 {
		return common.Hash{}, 0, errors.New("acc value of zero found")
	}
	return common.Hash(acc), count - 1, nil
}

func getDelayedMessages(ctx context.Context, c *ethclient.Client, bridge common.Address, start uint64, endAcc common.Hash) (common.Hash, [][]byte, bool, error) {
	endBlock, err := c.BlockNumber(ctx)
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	acc, end, err := getMessageAcc(ctx, c, bridge, 0)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	if acc != endAcc {
		logs, err := getLogs(ctx, c, 0, endBlock, []common.Address{bridge}, [][]common.Hash{{messageDeliveredID}, nil, {endAcc}}, 1)
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		if len(logs) != 1 {
			return common.Hash{}, nil, false, fmt.Errorf("unexpected number of logs found: got %d, expected 1", len(logs))
		}
		end = logs[0].Topics[1].Big().Uint64()
		if end == 0 {
			return common.Hash{}, nil, false, errors.New("unexpected message index 0 found")
		}
		end--
		endBlock = logs[0].BlockNumber
	}
	var dummy bool
	// include a dummy message if the start count is after the end message index
	indexes := []common.Hash{common.BigToHash(big.NewInt(int64(end)))}
	if start == end+1 {
		start--
		dummy = true
	} else {
		if start > end {
			return common.Hash{}, nil, false, fmt.Errorf("start message %d is after end %d", start, end)
		}
		if start < end {
			indexes = append(indexes, common.BigToHash(big.NewInt(int64(start))))
		}
	}
	logs, err := getLogs(ctx, c, 0, endBlock, []common.Address{bridge}, [][]common.Hash{{messageDeliveredID}, indexes}, uint64(len(indexes)))
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	if len(logs) != len(indexes) {
		return common.Hash{}, nil, false, fmt.Errorf("unexpected number of logs found: got %d, expected %d", len(logs), len(indexes))
	}

	ibridge, err := bridgegen.NewBridge(bridge, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	seqInbox, err := ibridge.SequencerInbox(&bind.CallOpts{Context: ctx})
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	addrs := []common.Address{bridge, seqInbox}

	var i int64
	for {
		inbox, err := ibridge.AllowedDelayedInboxList(&bind.CallOpts{Context: ctx}, big.NewInt(i))
		if err != nil {
			if strings.Contains(err.Error(), vm.ErrExecutionReverted.Error()) {
				break
			}
			return common.Hash{}, nil, false, err
		}
		addrs = append(addrs, inbox)
		i++
	}
	if i == 0 {
		return common.Hash{}, nil, false, errors.New("no inbox addresses found")
	}

	logs, err = getLogs(ctx, c, logs[0].BlockNumber, logs[len(logs)-1].BlockNumber,
		addrs,
		[][]common.Hash{
			{messageDeliveredID, inboxMessageDeliveredID},
		}, 0)

	if err != nil {
		return common.Hash{}, nil, false, err
	}

	if len(logs)%2 != 0 {
		for _, log := range logs {
			fmt.Println("LOG: ", log.Topics[0], log.TxHash, log.Address, log.Topics[1].Big().Uint64())
		}
		return common.Hash{}, nil, false, fmt.Errorf("even number of logs expected: got %d", len(logs))
	}

	iinbox, err := bridgegen.NewIDelayedMessageProvider(common.Address{}, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	var msgs [][]byte
	var prevAcc *common.Hash
	for i := 0; i < len(logs); i += 2 {
		log, err := ibridge.ParseMessageDelivered(logs[i])
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		dataLog, err := iinbox.ParseInboxMessageDelivered(logs[i+1])
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		if log.MessageIndex.Cmp(dataLog.MessageNum) != 0 {
			return common.Hash{}, nil, false, errors.New("event log msg index mismatch")
		}
		if log.Raw.BlockNumber != dataLog.Raw.BlockNumber {
			return common.Hash{}, nil, false, errors.New("event log block number mismatch")
		}
		// skip events prior to the start one
		if log.MessageIndex.Cmp(big.NewInt(int64(start))) != 0 {
			continue
		}
		// exit once we have processed the end message
		if start > end {
			break
		}
		start++
		if prevAcc == nil {
			hash := common.Hash(log.BeforeInboxAcc)
			prevAcc = &hash
		}
		requestId := common.BigToHash(log.MessageIndex)
		msg := arbostypes.L1IncomingMessage{
			Header: &arbostypes.L1IncomingMessageHeader{
				Kind:        log.Kind,
				Poster:      log.Sender,
				BlockNumber: log.Raw.BlockNumber,
				Timestamp:   log.Timestamp,
				RequestId:   &requestId,
				L1BaseFee:   log.BaseFeeL1,
			},
			L2msg: dataLog.Data,
		}
		data, err := msg.Serialize()
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		msgs = append(msgs, data)
	}
	if start != end+1 || prevAcc == nil {
		return common.Hash{}, nil, false, fmt.Errorf("missing message: got %d, expected %d", start, end+1)
	}
	return *prevAcc, msgs, dummy, nil
}

func getNumBatches(batches []enclave.SyndicateBatch, dmsgs [][]byte, setDelay uint64) uint64 {
	var batchCount uint64
	i := 0
	for _, b := range batches {
		hasMsg := false
		for i < len(dmsgs) && binary.BigEndian.Uint64(dmsgs[i][enclave.DelayedMessageTimestampOffset:enclave.DelayedMessageTimestampOffset+8])+setDelay <= b.Timestamp {
			i++
			hasMsg = true
		}
		if b.Data != nil || hasMsg {
			batchCount++
		}
	}
	return batchCount
}

type Client struct {
	l1Client      *ethclient.Client
	seqClient     *ethclient.Client
	setClient     *ethclient.Client
	appClient     *ethclient.Client
	enclaveClient *rpc.Client
	cfg           enclave.Config
	appBridge     common.Address
	dapReaders    []daprovider.Reader
}

// find the last block <= l1 number. return an error if start is greater than l1 number.
func findBlock(ctx context.Context, c *ethclient.Client, start uint64, l1Number uint64) (*execution.MessageResult, error) {
	end, err := c.BlockNumber(ctx)
	if err != nil {
		return nil, err
	}
	end++
	var extraInfo types.HeaderInfo
	for end-start > 1 {
		mid := (start + end) / 2
		header, err := c.HeaderByNumber(ctx, big.NewInt(int64(mid)))
		if err != nil {
			return nil, err
		}
		extraInfo = types.DeserializeHeaderExtraInformation(header)
		if extraInfo.L1BlockNumber <= l1Number {
			start = mid
		} else {
			end = mid
		}
	}

	header, err := c.HeaderByNumber(ctx, big.NewInt(int64(start)))
	if err != nil {
		return nil, err
	}
	extraInfo = types.DeserializeHeaderExtraInformation(header)
	if extraInfo.L1BlockNumber > l1Number {
		return nil, errors.New("block not found")
	}
	return &execution.MessageResult{BlockHash: header.Hash(), SendRoot: extraInfo.SendRoot}, nil
}

func Verify(ctx context.Context, appClient *ethclient.Client, seqClient *ethclient.Client, l1Client *ethclient.Client, sequencingBridgeAddress common.Address, trustedInput enclave.TrustedInput, isL1Chain bool) (*enclave.VerifyAppchainOutput, error) {
	// get the batch count
	var endBatchCount uint64
	if isL1Chain {
		if err := seqClient.Client().CallContext(ctx, &endBatchCount, "synd_batchFromAcc", trustedInput.L1EndHash); err != nil {
			return nil, err
		}
	} else {
		count, err := l1Client.StorageAtHash(ctx, sequencingBridgeAddress, enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, trustedInput.L1EndHash)
		if err != nil {
			return nil, err
		}
		endBatchCount = common.BytesToHash(count).Big().Uint64()
	}

	if endBatchCount == 0 {
		return nil, errors.New("end batch count is 0")
	}

	var metadata arbnode.BatchMetadata
	if err := seqClient.Client().CallContext(ctx, &metadata, "synd_batchMetadata", endBatchCount-1); err != nil {
		return nil, err
	}

	if metadata.MessageCount == 0 {
		return nil, errors.New("message count is 0")
	}

	// get the end block
	header, err := seqClient.HeaderByNumber(ctx, big.NewInt(int64(metadata.MessageCount-1)))
	if err != nil {
		return nil, err
	}
	sequencingBlockHash := header.Hash()

	if header, err = appClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return nil, err
	}

	// binary search to find the appchain end block
	appEndBlock, err := findBlock(ctx, appClient, header.Number.Uint64(), uint64(metadata.MessageCount-1))
	if err != nil {
		return nil, err
	}

	return &enclave.VerifyAppchainOutput{
		L1BatchAcc:          metadata.Accumulator,
		SequencingBlockHash: sequencingBlockHash,
		AppchainBlockHash:   appEndBlock.BlockHash,
		AppchainSendRoot:    appEndBlock.SendRoot,
	}, nil
}

func (c *Client) Prove(ctx context.Context, trustedInput enclave.TrustedInput, isL1Chain bool) (*enclave.VerifyAppchainOutput, error) {
	// get the batch count
	now := time.Now()
	var endBatchCount uint64
	if isL1Chain {
		if err := c.seqClient.Client().CallContext(ctx, &endBatchCount, "synd_batchFromAcc", trustedInput.L1EndHash); err != nil {
			return nil, err
		}
	} else {
		count, err := c.l1Client.StorageAtHash(ctx, c.cfg.SequencingBridgeAddress, enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, trustedInput.L1EndHash)
		if err != nil {
			return nil, err
		}
		endBatchCount = common.BytesToHash(count).Big().Uint64()
	}
	fmt.Println("get end batch count took", time.Since(now))

	if endBatchCount == 0 {
		return nil, errors.New("end batch count is 0")
	}

	// get the start block
	header, err := c.seqClient.HeaderByHash(ctx, trustedInput.SeqStartBlockHash)
	if err != nil {
		return nil, err
	}

	// get validation data
	now = time.Now()
	var valData ValidationData
	if err := c.seqClient.Client().CallContext(ctx, &valData, "synd_validationData", header.Number.Uint64(), endBatchCount-1, false); err != nil {
		return nil, err
	}
	fmt.Println("synd_validationData took", time.Since(now))
	preimages := make(map[arbutil.PreimageType]map[common.Hash][]byte)
	preimages[arbutil.Keccak256PreimageType] = make(map[common.Hash][]byte)
	for _, preimage := range valData.PreimageData {
		preimages[arbutil.Keccak256PreimageType][crypto.Keccak256Hash(preimage)] = preimage
	}

	// get batches
	var batches [][]byte
	if valData.BatchEndIndex >= valData.BatchStartIndex {
		ibridge, err := bridgegen.NewIBridgeCaller(c.cfg.SequencingBridgeAddress, c.l1Client)
		if err != nil {
			return nil, err
		}

		seqInbox, err := ibridge.SequencerInbox(&bind.CallOpts{Context: ctx})
		if err != nil {
			return nil, err
		}

		now = time.Now()
		batches, err = getBatches(ctx, c.l1Client, seqInbox, valData.BatchStartIndex, valData.BatchEndIndex, valData.BatchStartBlockNum, valData.BatchEndBlockNum)
		if err != nil {
			return nil, err
		}
		fmt.Println("getBatches() took", time.Since(now))
		if len(batches) == 0 {
			return nil, errors.New("found 0 batches")
		}
		// update preimages
		now = time.Now()
		for _, batch := range batches {
			if err := getBatchPreimageData(ctx, batch, c.dapReaders, preimages); err != nil {
				return nil, err
			}
		}
		fmt.Println("getBatchPreimageData() took", time.Since(now))
	}

	var proof *enclave.AccountResult
	if !isL1Chain {
		// get the end block header
		if header, err = c.l1Client.HeaderByHash(ctx, trustedInput.L1EndHash); err != nil {
			return nil, err
		}

		// get merkle proof
		now = time.Now()
		accSlot := common.BigToHash(new(big.Int).Add(enclave.BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE, big.NewInt(int64(endBatchCount))))
		if err := c.l1Client.Client().CallContext(ctx, &proof, "eth_getProof", c.cfg.SequencingBridgeAddress, []common.Hash{enclave.BATCH_ACCUMULATOR_STORAGE_SLOT, accSlot}, trustedInput.L1EndHash); err != nil {
			return nil, err
		}
		fmt.Println("eth_getProof took", time.Since(now))
	}

	preimagesData := make(map[arbutil.PreimageType][][]byte)
	for ty, images := range preimages {
		preimagesData[ty] = slices.Collect(maps.Values(images))
	}

	// derive sequencing chain
	now = time.Now()
	var seqOutput enclave.VerifySequencingChainOutput
	fmt.Println("deriving seq chain")
	if err := c.enclaveClient.Call(&seqOutput, "enclave_verifySequencingChain", enclave.VerifySequencingChainInput{
		TrustedInput:                    trustedInput,
		Config:                          c.cfg,
		DelayedMessages:                 valData.DelayedMessages,
		StartDelayedMessagesAccumulator: valData.StartDelayedAcc,
		Batches:                         batches,
		IsL1Chain:                       isL1Chain,
		PreimageData:                    preimagesData,
		EndBatchAccumulatorMerkleProof:  proof,
		L1EndBlockHeader:                header,
	}); err != nil {
		return nil, err
	}
	fmt.Println("seq enclave runtime: ", time.Since(now))

	// get appchain start block
	if header, err = c.appClient.HeaderByHash(ctx, trustedInput.AppStartBlockHash); err != nil {
		return nil, err
	}

	// get delayed messages
	now = time.Now()
	startAcc, msgs, isDummy, err := getDelayedMessages(ctx, c.setClient, c.appBridge, header.Nonce.Uint64(), trustedInput.SetDelayedMessageAcc)
	if err != nil {
		return nil, err
	}
	fmt.Println("getDelayedMessages() took", time.Since(now))

	// get the number of batches. ignore the delayed message if it is a dummy one
	var realMsgs [][]byte
	if !isDummy {
		realMsgs = msgs
	}
	numBatches := getNumBatches(seqOutput.Batches, realMsgs, c.cfg.SettlementDelay)

	// get preimage data
	now = time.Now()
	var preimageData [][]byte
	if err := c.appClient.Client().CallContext(ctx, &preimageData, "synd_preimageData", header.Number, numBatches, true); err != nil {
		return nil, err
	}
	fmt.Println("synd_preimageData took", time.Since(now))

	// derive appchain
	now = time.Now()
	var appOutput enclave.VerifyAppchainOutput
	if err := c.enclaveClient.Call(&appOutput, "enclave_verifyAppchain", enclave.VerifyAppchainInput{
		TrustedInput:                    trustedInput,
		Config:                          c.cfg,
		DelayedMessages:                 msgs,
		StartDelayedMessagesAccumulator: startAcc,
		VerifySequencingChainOutput:     seqOutput,
		AppStartBlockHeader:             *header,
		PreimageData: map[arbutil.PreimageType][][]byte{
			arbutil.Keccak256PreimageType: preimageData},
	}); err != nil {
		return nil, err
	}
	fmt.Println("app enclave runtime: ", time.Since(now))
	return &appOutput, nil
}

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

	seqClient, err := ethclient.Dial(*seqUrl)
	if err != nil {
		panic(err)
	}
	enclaveClient, err := rpc.Dial(*enclaveUrl)
	if err != nil {
		panic(err)
	}

	// normally this comes from the tee contract instead
	var startMetadata arbnode.BatchMetadata
	if err := seqClient.Client().CallContext(ctx, &startMetadata, "synd_batchMetadata", l1StartBatch); err != nil {
		panic(err)
	}
	var endMetadata arbnode.BatchMetadata
	if err := seqClient.Client().CallContext(ctx, &endMetadata, "synd_batchMetadata", l1EndBatch); err != nil {
		panic(err)
	}

	cfg := enclave.Config{
		SequencingContractAddress: seqContractAddress,
		SequencingBridgeAddress:   seqBridgeAddress,
		SettlementDelay:           *setDelay,
	}

	startSeqNum := uint64(startMetadata.MessageCount) - 1

	header, err := seqClient.HeaderByNumber(ctx, big.NewInt(int64(startSeqNum)))
	if err != nil {
		panic(err)
	}

	startSeqBlock := header.Hash()

	appClient, err := ethclient.Dial(*appUrl)
	if err != nil {
		panic(err)
	}

	// binary search to find the start block
	result, err := findBlock(ctx, appClient, 0, startSeqNum)
	if err != nil {
		panic(err)
	}

	l1Client, err := ethclient.Dial(*l1Url)
	if err != nil {
		panic(err)
	}

	// can add an arbitrary offset to the end block
	if header, err = l1Client.HeaderByNumber(ctx, big.NewInt(int64(endMetadata.ParentChainBlock))); err != nil {
		panic(err)
	}

	setClient, err := ethclient.Dial(*setUrl)
	if err != nil {
		panic(err)
	}

	setDelayedAcc, _, err := getMessageAcc(ctx, setClient, appBridge, *setMsgs)
	if err != nil {
		panic(err)
	}

	trustedInput := enclave.TrustedInput{
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

	eigenClient, err := eigenda.NewEigenDA(&eigenda.EigenDAConfig{
		Enable: true,
		Rpc:    *eigenUrl,
	})

	if err != nil {
		panic(err)
	}

	p := Client{
		l1Client, seqClient, setClient, appClient, enclaveClient,
		cfg, appBridge, []daprovider.Reader{eigenda.NewReaderForEigenDA(eigenClient)},
	}

	fmt.Println("ready in", time.Since(now))
	now = time.Now()
	appOutput, err := p.Prove(ctx, trustedInput, false)
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
	verifyOutput, err := Verify(ctx, p.appClient, p.seqClient, p.l1Client, p.cfg.SequencingBridgeAddress, trustedInput, false)
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
