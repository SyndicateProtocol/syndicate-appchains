// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package enclave

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strconv"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/state"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethdb/memorydb"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/params"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/triedb"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
	"github.com/offchainlabs/nitro/arbos"
	"github.com/offchainlabs/nitro/arbos/arbosState"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/cmd/chaininfo"
	"github.com/offchainlabs/nitro/daprovider"
	"github.com/offchainlabs/nitro/daprovider/das/dasutil"
	"github.com/offchainlabs/nitro/eigenda"
	"github.com/offchainlabs/nitro/execution"
)

func readMessage(ctx context.Context, wavm *wavmio.Wavm, delayedMessagesRead uint64, dasEnabled bool) (*arbostypes.MessageWithMetadata, error) {
	dapReaders := []daprovider.Reader{eigenda.NewReaderForEigenDA(&wavmio.EigenDAPreimageReader{Wavm: wavm}), daprovider.NewReaderForBlobReader(&wavmio.BlobPreimageReader{Wavm: wavm})}
	if dasEnabled {
		// DAS batch and keysets are all together in the same preimage binary.
		dapReaders = append(dapReaders, dasutil.NewReaderForDAS(&wavmio.PreimageDASReader{Wavm: wavm}, &wavmio.PreimageDASReader{Wavm: wavm}))
	}
	backend := &wavmio.WavmInbox{Wavm: wavm}
	keysetValidationMode := daprovider.KeysetPanicIfInvalid
	if backend.GetPositionWithinMessage() > 0 {
		keysetValidationMode = daprovider.KeysetDontValidate
	}
	inboxMultiplexer := arbstate.NewInboxMultiplexer(backend, delayedMessagesRead, dapReaders, keysetValidationMode)
	msg, err := inboxMultiplexer.Pop(ctx)
	if err != nil {
		return nil, fmt.Errorf("error reading from inbox multiplexer: %v", err.Error())
	}

	if msg.Message.Header.Kind == arbostypes.L1MessageType_BatchPostingReport && msg.Message.BatchGasCost == nil {
		_, _, batchHash, _, _, _, err := arbostypes.ParseBatchPostingReportMessageFields(bytes.NewReader(msg.Message.L2msg))
		if err != nil {
			return nil, fmt.Errorf("failed to parse batch posting report: %w", err)
		}
		batchData, err := wavm.ResolveTypedPreimage(arbutil.Keccak256PreimageType, batchHash)
		if err != nil {
			return nil, fmt.Errorf("failed to fetch batch mentioned by batch posting report: %w", err)
		}
		gas := arbostypes.ComputeBatchGasCost(batchData)
		msg.Message.BatchGasCost = &gas
	}
	return msg, nil
}

const L1_BLOCK_NUM_HARDFORK_TS = 1767571200

// getL1BlockNumHardforkTS returns the hardfork timestamp, supporting env var override for testing
func getL1BlockNumHardforkTS() uint64 {
	if val := os.Getenv("L1_BLOCK_NUM_HARDFORK_TS"); val != "" {
		if ts, err := strconv.ParseUint(val, 10, 64); err == nil {
			log.Warn("L1_BLOCK_NUM_HARDFORK_TS override", "value", ts)
			return ts
		}
	}
	return L1_BLOCK_NUM_HARDFORK_TS
}

func Verify(
	ctx context.Context,
	data wavmio.ValidationInput,
	processor interface {
		ProcessBlock(block *types.Block, receipts types.Receipts, l1BlockNum uint64, timestamp uint64) error
	},
	appchain bool,
) (_ *execution.MessageResult, err error) {
	if data.BlockHash == (common.Hash{}) {
		return nil, errors.New("genesis block verification unsupported")
	}

	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("panic with error: %v", r)
		}
	}()

	wavm, err := wavmio.New(data)
	if err != nil {
		return nil, err
	}

	batchCount := uint64(len(data.Batches))

	header, err := wavm.GetBlockHeaderByHash(data.BlockHash)
	if err != nil {
		return nil, err
	}

	db := state.NewDatabase(triedb.NewDatabase(rawdb.WrapDatabaseWithWasm(rawdb.NewDatabase(&PreimageDb{wavm: wavm, memDb: memorydb.New()}), memorydb.New()), nil), nil)

	// TODO it seems arbitrum uses the nonce in the block header to indicate the number of delayed messages read. this is a bit obscure, should find docs that lay this out
	startDelayedMessagesRead := header.Nonce.Uint64()
	prevDelayedMessagesRead := startDelayedMessagesRead

	for wavm.GetInboxPosition() < batchCount {
		if err = ctx.Err(); err != nil {
			return nil, err
		}

		statedb, err := state.NewDeterministic(header.Root, db)
		if err != nil {
			return nil, fmt.Errorf("error opening state db for block %s: %v", header.Hash(), err.Error())
		}

		// ArbOS has already been initialized.
		// Load the chain config and then produce a block normally.
		initialArbosState, err := arbosState.OpenSystemArbosState(statedb, nil, true)
		if err != nil {
			return nil, fmt.Errorf("error opening initial ArbOS state: %v", err.Error())
		}
		chainId, err := initialArbosState.ChainId()
		if err != nil {
			return nil, fmt.Errorf("error getting chain ID from initial ArbOS state: %v", err.Error())
		}
		genesisBlockNum, err := initialArbosState.GenesisBlockNum()
		if err != nil {
			return nil, fmt.Errorf("error getting genesis block number from initial ArbOS state: %v", err.Error())
		}
		chainConfigJson, err := initialArbosState.ChainConfig()
		if err != nil {
			return nil, fmt.Errorf("error getting chain config from initial ArbOS state: %v", err.Error())
		}
		var chainConfig *params.ChainConfig
		if len(chainConfigJson) > 0 {
			chainConfig = &params.ChainConfig{}
			err = json.Unmarshal(chainConfigJson, chainConfig)
			if err != nil {
				return nil, fmt.Errorf("error parsing chain config: %v", err.Error())
			}
			if chainConfig.ChainID.Cmp(chainId) != 0 {
				return nil, fmt.Errorf("error: chain id mismatch, chainID: %v, chainConfig.ChainID: %v", chainId, chainConfig.ChainID)
			}
			if chainConfig.ArbitrumChainParams.GenesisBlockNum != genesisBlockNum {
				return nil, fmt.Errorf("error: genesis block number mismatch, genesisBlockNum: %v, chainConfig.ArbitrumParams.GenesisBlockNum: %v", genesisBlockNum, chainConfig.ArbitrumChainParams.GenesisBlockNum)
			}
		} else {
			log.Info("Falling back to hardcoded chain config.")
			chainConfig, err = chaininfo.GetChainConfig(chainId, "", genesisBlockNum, []string{}, "")
			if err != nil {
				return nil, err
			}
		}

		message, err := readMessage(ctx, wavm, header.Nonce.Uint64(), chainConfig.ArbitrumChainParams.DataAvailabilityCommittee)
		if err != nil {
			return nil, err
		}

		chainContext := wavmio.WavmChainContext{ChainConfig: chainConfig, Wavm: wavm}

		block, receipts, err := arbos.ProduceBlock(message.Message, message.DelayedMessagesRead, header, statedb, chainContext, false, core.NewMessageRecordingContext([]rawdb.WasmTarget{rawdb.LocalTarget()}))
		if err != nil {
			return nil, err
		}
		if block.NumberU64() != header.Number.Uint64()+1 {
			return nil, fmt.Errorf("unexpected block number: got %d, expected %d", block.NumberU64(), header.Number.Uint64()+1)
		}

		header = block.Header()
		bytes, err := rlp.EncodeToBytes(header)
		if err != nil {
			return nil, fmt.Errorf("error RLP encoding header: %v", err)
		}
		wavm.Preimages[arbutil.Keccak256PreimageType][crypto.Keccak256Hash(bytes)] = bytes

		result, err := statedb.Commit(block.NumberU64(), true, false)
		if err != nil {
			return nil, err
		}
		if result != header.Root {
			return nil, fmt.Errorf("bad commit root hash expected %v, got %v", header.Root, result)
		}

		// NOTE: l1BlockNum hardfork logic must match slotter.rs and server.go:parseAppBatches
		l1BlockNum := uint64(0)
		if !appchain {
			l1BlockNum = block.NumberU64()
		} else {
			// apply the l1_block_number hardfork logic only to derive the apchain
			hardforkTS := getL1BlockNumHardforkTS()
			log.Info("TOMATO verify.go hardfork check", "seq_block.Time()", block.Time(), "hardforkTS", hardforkTS, "seq_block.NumberU64()", block.NumberU64(), "message.DelayedMessagesRead", message.DelayedMessagesRead, "prevDelayedMessagesRead", prevDelayedMessagesRead, "startDelayedMessagesRead", startDelayedMessagesRead, "len(data.Messages)", len(data.Messages))
			if block.Time() < hardforkTS {
				l1BlockNum = block.NumberU64()
				log.Info("TOMATO verify.go pre-hardfork", "l1BlockNum", l1BlockNum)
			} else {
				// Get settlement block number from the last delayed message consumed in THIS block
				// Only if this block consumed new delayed messages (DelayedMessagesRead increased)
				if message.DelayedMessagesRead > prevDelayedMessagesRead && len(data.Messages) > 0 {
					lastMsgIdx := message.DelayedMessagesRead - 1 - startDelayedMessagesRead
					if lastMsgIdx < uint64(len(data.Messages)) {
						lastMsg := data.Messages[lastMsgIdx]
						l1BlockNum = data.MessagesBlockNum[lastMsgIdx]
						log.Info("TOMATO verify.go post-hardfork got l1_bloc_num", "l1BlockNum", l1BlockNum, "delayedMsgsRead", message.DelayedMessagesRead, "delayedMsg", hexutil.Encode(lastMsg))
					}
				} else {
					log.Info("TOMATO verify.go post-hardfork NO delayed messages for this block")
				}
			}
			prevDelayedMessagesRead = message.DelayedMessagesRead
		}

		if processor != nil {
			if err := processor.ProcessBlock(block, receipts, l1BlockNum, block.Time()); err != nil {
				return nil, err
			}
		}
	}

	if wavm.GetInboxPosition() != batchCount || wavm.GetPositionWithinMessage() != 0 {
		return nil, fmt.Errorf("invalid end state: batch count %d != %d or message position %d != 0", wavm.GetInboxPosition(), batchCount, wavm.GetPositionWithinMessage())
	}

	extraInfo := types.DeserializeHeaderExtraInformation(header)
	if extraInfo.ArbOSFormatVersion == 0 {
		return nil, fmt.Errorf("error deserializing header extra info: %+v", header)
	}

	return &execution.MessageResult{BlockHash: header.Hash(), SendRoot: extraInfo.SendRoot}, nil
}
