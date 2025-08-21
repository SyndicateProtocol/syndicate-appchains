// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package enclave

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/state"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethdb"
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

func readMessage(ctx context.Context, wavm *wavmio.Wavm, delayedMessagesRead uint64, dasEnabled bool, eigenDAEnabled bool) (*arbostypes.MessageWithMetadata, error) {
	var dasReader dasutil.DASReader
	var eigenDAReader *wavmio.EigenDAPreimageReader
	var dasKeysetFetcher dasutil.DASKeysetFetcher
	if dasEnabled {
		// DAS batch and keysets are all together in the same preimage binary.
		dasReader = &wavmio.PreimageDASReader{Wavm: wavm}
		dasKeysetFetcher = &wavmio.PreimageDASReader{Wavm: wavm}
	}
	if eigenDAEnabled {
		eigenDAReader = &wavmio.EigenDAPreimageReader{Wavm: wavm}
	}
	backend := &wavmio.WavmInbox{Wavm: wavm}
	keysetValidationMode := daprovider.KeysetPanicIfInvalid
	if backend.GetPositionWithinMessage() > 0 {
		keysetValidationMode = daprovider.KeysetDontValidate
	}
	var dapReaders []daprovider.Reader
	if eigenDAReader != nil {
		dapReaders = append(dapReaders, eigenda.NewReaderForEigenDA(eigenDAReader))
	}
	if dasReader != nil {
		dapReaders = append(dapReaders, dasutil.NewReaderForDAS(dasReader, dasKeysetFetcher))
	}

	dapReaders = append(dapReaders, daprovider.NewReaderForBlobReader(&wavmio.BlobPreimageReader{Wavm: wavm}))
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

func Verify(
	ctx context.Context,
	data wavmio.ValidationInput,
	processor interface {
		ProcessBlock(*types.Block, types.Receipts) error
	},
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

	// use the rust LRU cache (0) for wasm programs
	db := state.NewDatabase(triedb.NewDatabase(rawdb.WrapDatabaseWithWasm(rawdb.NewDatabase(&PreimageDb{wavm: wavm, memDb: memorydb.New()}), memorydb.New(), 0, []ethdb.WasmTarget{rawdb.LocalTarget()}), nil), nil)

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

		message, err := readMessage(ctx, wavm, header.Nonce.Uint64(), chainConfig.ArbitrumChainParams.DataAvailabilityCommittee, chainConfig.ArbitrumChainParams.EigenDA)
		if err != nil {
			return nil, err
		}

		chainContext := wavmio.WavmChainContext{ChainConfig: chainConfig, Wavm: wavm}

		block, receipts, err := arbos.ProduceBlock(message.Message, message.DelayedMessagesRead, header, statedb, chainContext, false, core.MessageReplayMode)
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

		if processor != nil {
			if err := processor.ProcessBlock(block, receipts); err != nil {
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
