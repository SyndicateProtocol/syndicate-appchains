// Copyright 2021-2022, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE.md

package enclave

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/consensus"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/state"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/kzg4844"
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
	"github.com/offchainlabs/nitro/daprovider/das/dastree"
	"github.com/offchainlabs/nitro/daprovider/das/dasutil"
	"github.com/offchainlabs/nitro/eigenda"
	"github.com/offchainlabs/nitro/execution"
	"github.com/offchainlabs/nitro/gethhook"
)

type WavmChainContext struct {
	chainConfig *params.ChainConfig
	wavm        *wavmio.Wavm
}

func (c WavmChainContext) Config() *params.ChainConfig {
	return c.chainConfig
}

func (c WavmChainContext) Engine() consensus.Engine {
	return arbos.Engine{}
}

func (c WavmChainContext) GetHeader(hash common.Hash, num uint64) *types.Header {
	header, err := c.wavm.GetBlockHeaderByHash(hash)
	if err != nil {
		panic(fmt.Sprintf("missing preimage data for block header hash %v", hash))
	}
	if !header.Number.IsUint64() || header.Number.Uint64() != num {
		panic(fmt.Sprintf("retrieved wrong block number for header hash %v -- requested %v but got %v", hash, num, header.Number.String()))
	}
	return header
}

type WavmInbox struct {
	wavm *wavmio.Wavm
}

func (i WavmInbox) PeekSequencerInbox() ([]byte, common.Hash, error) {
	pos := i.wavm.GetInboxPosition()
	res, err := i.wavm.ReadInboxMessage(pos)
	if err != nil {
		return nil, common.Hash{}, err
	}
	log.Info("PeekSequencerInbox", "pos", pos, "res[:8]", res[:8])
	// Our BlobPreimageReader doesn't need the block hash
	return res, common.Hash{}, nil
}

func (i WavmInbox) GetSequencerInboxPosition() uint64 {
	pos := i.wavm.GetInboxPosition()
	log.Info("GetSequencerInboxPosition", "pos", pos)
	return pos
}

func (i WavmInbox) AdvanceSequencerInbox() {
	log.Info("AdvanceSequencerInbox")
	i.wavm.AdvanceInboxMessage()
}

func (i WavmInbox) GetPositionWithinMessage() uint64 {
	pos := i.wavm.GetPositionWithinMessage()
	log.Info("GetPositionWithinMessage", "pos", pos)
	return pos
}

func (i WavmInbox) SetPositionWithinMessage(pos uint64) {
	log.Info("SetPositionWithinMessage", "pos", pos)
	i.wavm.SetPositionWithinMessage(pos)
}

func (i WavmInbox) ReadDelayedInbox(seqNum uint64) (*arbostypes.L1IncomingMessage, error) {
	log.Info("ReadDelayedMsg", "seqNum", seqNum)
	data, err := i.wavm.ReadDelayedInboxMessage(seqNum)
	if err != nil {
		return nil, err
	}
	return arbostypes.ParseIncomingL1Message(bytes.NewReader(data), nil)
}

type PreimageDASReader struct {
	wavm *wavmio.Wavm
}

func (dasReader *PreimageDASReader) GetByHash(ctx context.Context, hash common.Hash) ([]byte, error) {
	oracle := func(hash common.Hash) ([]byte, error) {
		return dasReader.wavm.ResolveTypedPreimage(arbutil.Keccak256PreimageType, hash)
	}
	return dastree.Content(hash, oracle)
}

func (dasReader *PreimageDASReader) GetKeysetByHash(ctx context.Context, hash common.Hash) ([]byte, error) {
	return dasReader.GetByHash(ctx, hash)
}

func (dasReader *PreimageDASReader) HealthCheck(ctx context.Context) error {
	return nil
}

func (dasReader *PreimageDASReader) ExpirationPolicy(ctx context.Context) (dasutil.ExpirationPolicy, error) {
	return dasutil.DiscardImmediately, nil
}

type BlobPreimageReader struct {
	wavm *wavmio.Wavm
}

func (r *BlobPreimageReader) GetBlobs(
	ctx context.Context,
	batchBlockHash common.Hash,
	versionedHashes []common.Hash,
) ([]kzg4844.Blob, error) {
	var blobs []kzg4844.Blob
	for _, h := range versionedHashes {
		var blob kzg4844.Blob
		preimage, err := r.wavm.ResolveTypedPreimage(arbutil.EthVersionedHashPreimageType, h)
		if err != nil {
			return nil, err
		}
		if len(preimage) != len(blob) {
			return nil, fmt.Errorf("for blob %v got back preimage of length %v but expected blob length %v", h, len(preimage), len(blob))
		}
		copy(blob[:], preimage)
		blobs = append(blobs, blob)
	}
	return blobs, nil
}

func (r *BlobPreimageReader) Initialize(ctx context.Context) error {
	return nil
}

type EigenDAPreimageReader struct {
	wavm *wavmio.Wavm
}

// QueryBlob returns the blob for the given cert from the preimage oracle using the hash of the
// certificate kzg commitment for identifying the preimage.
func (dasReader *EigenDAPreimageReader) QueryBlob(ctx context.Context, cert *eigenda.EigenDAV1Cert, domain string) ([]byte, error) {
	hash, err := cert.PreimageHash()
	if err != nil {
		return nil, err
	}

	preimage, err := dasReader.wavm.ResolveTypedPreimage(arbutil.EigenDaPreimageType, *hash)
	if err != nil {
		return nil, err
	}

	decodedBlob, err := eigenda.GenericDecodeBlob(preimage)
	if err != nil {
		return nil, err
	}

	return decodedBlob, nil
}

// To generate:
// key, _ := crypto.HexToECDSA("0000000000000000000000000000000000000000000000000000000000000001")
// sig, _ := crypto.Sign(make([]byte, 32), key)
// println(hex.EncodeToString(sig))
const sampleSignature = "a0b37f8fba683cc68f6574cd43b39f0343a50008bf6ccea9d13231d9e7e2e1e411edc8d307254296264aebfc3dc76cd8b668373a072fd64665b50000e9fcce5201"

// We call this early to populate the secp256k1 ecc basepoint cache in the cached early machine state.
// That means we don't need to re-compute it for every block.
func populateEcdsaCaches() {
	signature, err := hex.DecodeString(sampleSignature)
	if err != nil {
		log.Warn("failed to decode sample signature to populate ECDSA cache", "err", err)
		return
	}
	_, err = crypto.Ecrecover(make([]byte, 32), signature)
	if err != nil {
		log.Warn("failed to recover signature to populate ECDSA cache", "err", err)
		return
	}
}

func readMessage(ctx context.Context, wavm *wavmio.Wavm, delayedMessagesRead uint64, dasEnabled bool, eigenDAEnabled bool) (*arbostypes.MessageWithMetadata, error) {
	var dasReader dasutil.DASReader
	var eigenDAReader *EigenDAPreimageReader
	var dasKeysetFetcher dasutil.DASKeysetFetcher
	if dasEnabled {
		// DAS batch and keysets are all together in the same preimage binary.
		dasReader = &PreimageDASReader{wavm}
		dasKeysetFetcher = &PreimageDASReader{wavm}
	}
	if eigenDAEnabled {
		eigenDAReader = &EigenDAPreimageReader{wavm}
	}
	backend := WavmInbox{wavm}
	keysetValidationMode := daprovider.KeysetPanicIfInvalid
	if backend.GetPositionWithinMessage() > 0 {
		keysetValidationMode = daprovider.KeysetDontValidate
	}
	var dapReaders []daprovider.Reader
	if dasReader != nil {
		dapReaders = append(dapReaders, dasutil.NewReaderForDAS(dasReader, dasKeysetFetcher))
	}
	if eigenDAReader != nil {
		dapReaders = append(dapReaders, eigenda.NewReaderForEigenDA(eigenDAReader))
	}

	dapReaders = append(dapReaders, daprovider.NewReaderForBlobReader(&BlobPreimageReader{wavm}))
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
	gethhook.RequireHookedGeth()

	glogger := log.NewGlogHandler(
		log.NewTerminalHandler(io.Writer(os.Stderr), false),
	)
	glogger.Verbosity(log.LevelError)
	log.SetDefault(log.NewLogger(glogger))

	populateEcdsaCaches()

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

		chainContext := WavmChainContext{chainConfig: chainConfig, wavm: wavm}

		block, receipts, err := arbos.ProduceBlock(message.Message, message.DelayedMessagesRead, header, statedb, chainContext, false, core.MessageReplayMode)
		if err != nil {
			return nil, err
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
