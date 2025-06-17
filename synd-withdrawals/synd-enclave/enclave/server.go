package enclave

import (
	"archive/zip"
	"bytes"
	"context"
	"crypto/ecdsa"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"math"
	"math/big"
	"os"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/hf/nitrite"
	"github.com/hf/nsm"
	"github.com/hf/nsm/request"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/execution"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
)

const (
	// DefaultCARoots contains the PEM encoded roots for verifying Nitro
	// Enclave attestation signatures. You can download them from
	// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html
	DefaultCARoots       = "UEsDBBQAAAAIALkYV1GVtvolRwIAAAkDAAAIABwAcm9vdC5wZW1VVAkAA10ekl9dHpJfdXgLAAEESHEtDwQUAAAAZZJLk6JQDIX3/IrZW10Igo2LWdwXiBoE5HXZCSq0iNgKfYVfP9guJ8tTqS85Ofn4GAszy3b+EOYHtmkTFLCX+CGBbRMWEILSfYGEjVFh+8itnoe4yKq1XC7DDNptcJ2YXJCC2+smtYfzlCEBYhewjQSospASMlwCiSJ40gE5uHAijBrAldny5PaTnRkAan77iBDUiw4B+A9heZxKkedRilflYQZdVl+meW20aayfM8tU0wTEsswdCKonUFuDAPotRUo8ag59axIE3ls84xV4D0FG6gi1mFhF4cBcQNP35GIcGCvlsV504ImXnVffRqLjxpECT2tA6Xt1AFabs7zXu33i91mvXLLaefAkveQDVgEjC/ff1g60BSqYJeFdhzFCX0i1EXYFibZdTWA57Jf0q26/vZ+Ka3BbDVlz2chy2qv8wnYK9vVgVz1OWSZpBjFi3PTtp6li8Xlk7X7vTprSUrNr+FgspofpKlGNIHe9hDA3nWGE7WPgcsEaEqdMKo2LzhtPBHkoL9YOgTEgKkZ//jRA3lLGKBRIMCwP6PCyuPQ0ZhZeWJFYoYfKlPzJMRZ6Ns9vM7feX087nQta/ALcN8CjqLCsV4yEvL2Pd6JIrRBYnEjgkfOpn/hNXi+S7qjxq4hrZxUhTTuhqavH6vbGG7HYchL5e3b82RjdVkn4vdOfLbixdD8BGSFfhv6IcbYS63Vy2M3xrfXMLs2Cz1kjF7hUvsPnRb46d0UNtwY/iftcuJtsMnckW2yGmcz/Sr+fzRz637f/A1BLAQIeAxQAAAAIALkYV1GVtvolRwIAAAkDAAAIABgAAAAAAAEAAACkgQAAAAByb290LnBlbVVUBQADXR6SX3V4CwABBEhxLQ8EFAAAAFBLBQYAAAAAAQABAE4AAACJAgAAAAA="
	DefaultCARootsSHA256 = "8cf60e2b2efca96c6a9e71e851d00c1b6991cc09eadbe64a6a1d1b1eb9faff7c"
)

var (
	defaultRoot = createAWSNitroRoot()
)

func createAWSNitroRoot() *x509.CertPool {
	roots, err := base64.StdEncoding.DecodeString(DefaultCARoots)
	if err != nil {
		panic("error decoding AWS root cert")
	}
	sha := sha256.Sum256(roots)
	expected := common.HexToHash(DefaultCARootsSHA256)
	if !bytes.Equal(sha[:], expected[:]) {
		panic("DefaultCARoots checksum failed")
	}
	reader, err := zip.NewReader(bytes.NewReader(roots), int64(len(roots)))
	if err != nil {
		panic(fmt.Errorf("error creating zip reader: %w", err))
	}
	ca, err := reader.File[0].Open()
	if err != nil {
		panic(fmt.Errorf("error reading AWS root cert zip: %w", err))
	}
	pem, err := io.ReadAll(ca)
	if err != nil {
		panic(fmt.Errorf("error reading AWS root cert: %w", err))
	}
	pool := x509.NewCertPool()
	ok := pool.AppendCertsFromPEM(pem)
	if !ok {
		panic("error parsing AWS root cert")
	}
	return pool
}

type Server struct {
	pcr0          []byte
	signerKey     *ecdsa.PrivateKey
	decryptionKey *rsa.PrivateKey
}

func NewServer() (*Server, error) {
	var random io.Reader
	var pcr0 []byte
	session, err := nsm.OpenDefaultSession()
	var signerKeyEnv string
	if err != nil {
		log.Warn("failed to open Nitro Secure Module session, running in local mode", "error", err)
		random = rand.Reader
		// only allow a signer key to be set in local mode
		signerKeyEnv = os.Getenv("OP_ENCLAVE_SIGNER_KEY")
	} else {
		defer func() {
			_ = session.Close()
		}()
		pcr, err := session.Send(&request.DescribePCR{
			Index: 0,
		})
		if err != nil {
			return nil, fmt.Errorf("failed to describe PCR: %w", err)
		}
		if pcr.Error != "" {
			return nil, fmt.Errorf("NSM device returned an error: %s", pcr.Error)
		}
		if pcr.DescribePCR == nil || pcr.DescribePCR.Data == nil || len(pcr.DescribePCR.Data) == 0 {
			return nil, errors.New("NSM device did not return PCR data")
		}
		pcr0 = pcr.DescribePCR.Data
		random = session
	}

	decryptionKey, err := rsa.GenerateKey(random, 4096)
	if err != nil {
		return nil, fmt.Errorf("failed to generate decryption key: %w", err)
	}
	signerKey, err := ecdsa.GenerateKey(crypto.S256(), random)
	if err != nil {
		return nil, fmt.Errorf("failed to generate signer key: %w", err)
	}
	if signerKeyEnv != "" {
		signerKey, err = crypto.HexToECDSA(signerKeyEnv)
		if err != nil {
			return nil, fmt.Errorf("failed to parse signer key: %w", err)
		}
	}
	log.Info("Generated signer key", "address", crypto.PubkeyToAddress(signerKey.PublicKey).Hex())
	return &Server{
		pcr0:          pcr0,
		signerKey:     signerKey,
		decryptionKey: decryptionKey,
	}, nil
}

func (s *Server) SignerPublicKey(ctx context.Context) (hexutil.Bytes, error) {
	return crypto.FromECDSAPub(&s.signerKey.PublicKey), nil
}

func (s *Server) SignerAttestation(ctx context.Context) (hexutil.Bytes, error) {
	return s.publicKeyAttestation(ctx, s.SignerPublicKey)
}

func (s *Server) DecryptionPublicKey(ctx context.Context) (hexutil.Bytes, error) {
	return x509.MarshalPKIXPublicKey(s.decryptionKey.Public())
}

func (s *Server) DecryptionAttestation(ctx context.Context) (hexutil.Bytes, error) {
	return s.publicKeyAttestation(ctx, s.DecryptionPublicKey)
}

func (s *Server) publicKeyAttestation(ctx context.Context, publicKey func(ctx context.Context) (hexutil.Bytes, error)) (hexutil.Bytes, error) {
	session, err := nsm.OpenDefaultSession()
	if err != nil {
		return nil, fmt.Errorf("failed to open session: %w", err)
	}
	defer func() {
		_ = session.Close()
	}()
	public, err := publicKey(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to get public key: %w", err)
	}
	res, err := session.Send(&request.Attestation{
		PublicKey: public,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to get attestation: %w", err)
	}
	if res.Error != "" {
		return nil, fmt.Errorf("NSM device returned an error: %s", res.Error)
	}
	if res.Attestation == nil || res.Attestation.Document == nil {
		return nil, errors.New("NSM device did not return an attestation")
	}
	return res.Attestation.Document, nil
}

func (s *Server) EncryptedSignerKey(ctx context.Context, attestation hexutil.Bytes) (hexutil.Bytes, error) {
	verification, err := nitrite.Verify(
		attestation,
		nitrite.VerifyOptions{
			Roots:       defaultRoot,
			CurrentTime: time.Now(),
		},
	)
	if err != nil {
		return nil, fmt.Errorf("failed to verify attestation: %w", err)
	}
	if !bytes.Equal(verification.Document.PCRs[0], s.pcr0) {
		return nil, errors.New("attestation does not match PCR0")
	}
	publicKey, err := x509.ParsePKIXPublicKey(verification.Document.PublicKey)
	if err != nil {
		return nil, fmt.Errorf("failed to parse public key: %w", err)
	}
	public, ok := publicKey.(*rsa.PublicKey)
	if !ok {
		return nil, errors.New("public key is not RSA")
	}
	session, err := nsm.OpenDefaultSession()
	if err != nil {
		return nil, fmt.Errorf("failed to open session: %w", err)
	}
	defer func() {
		_ = session.Close()
	}()
	ciphertext, err := rsa.EncryptPKCS1v15(session, public, crypto.FromECDSA(s.signerKey))
	if err != nil {
		return nil, fmt.Errorf("failed to encrypt key: %w", err)
	}
	return ciphertext, nil
}

func (s *Server) SetSignerKey(ctx context.Context, encrypted hexutil.Bytes) error {
	session, err := nsm.OpenDefaultSession()
	if err != nil {
		return fmt.Errorf("failed to open session: %w", err)
	}
	defer func() {
		_ = session.Close()
	}()
	decrypted, err := rsa.DecryptPKCS1v15(session, s.decryptionKey, encrypted)
	if err != nil {
		return fmt.Errorf("failed to decrypt key: %w", err)
	}
	key, err := crypto.ToECDSA(decrypted)
	if err != nil {
		return fmt.Errorf("failed to convert key: %w", err)
	}
	s.signerKey = key
	return nil
}

func accumulate(msg arbostypes.L1IncomingMessage, acc common.Hash) common.Hash {
	blockNumber := make([]byte, 8)
	binary.BigEndian.PutUint64(blockNumber, msg.Header.BlockNumber)
	timestamp := make([]byte, 8)
	binary.BigEndian.PutUint64(timestamp, msg.Header.Timestamp)
	l1BaseFee := common.BigToHash(msg.Header.L1BaseFee)
	return crypto.Keccak256Hash(acc[:], crypto.Keccak256([]byte{msg.Header.Kind}, msg.Header.Poster[:], blockNumber, timestamp, msg.Header.RequestId[:], l1BaseFee[:], crypto.Keccak256(msg.L2msg)))
}

// Storage slot of the batch accumulator
// <https://github.com/SyndicateProtocol/nitro-contracts/blob/9a100a86242176b633a1d907e5efd41296922144/src/bridge/AbsBridge.sol#L51>
// Since the batch accumulator is a dynamic array, this slot contains the length of the array
var BATCH_ACCUMULATOR_STORAGE_SLOT = common.BigToHash(big.NewInt(7))

// Storage slot of the first element in the batch accumulator array
// Dynamic types are stored starting at the keccak256 of the original storage slot plus an offset
// This value is Keccak256("0x7")
var BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT = crypto.Keccak256Hash(BATCH_ACCUMULATOR_STORAGE_SLOT[:]).Big()

type KVDB map[common.Hash][]byte

func (k KVDB) Has(key []byte) (bool, error) {
	_, ok := k[crypto.Keccak256Hash(key)]
	return ok, nil
}

func (k KVDB) Get(key []byte) ([]byte, error) {
	val, ok := k[crypto.Keccak256Hash(key)]
	if !ok {
		return nil, errors.New("could not find key")
	}
	return val, nil
}

func verify(root common.Hash, key []byte, value []byte, proof []string) error {
	db := make(KVDB, len(proof))
	for _, v := range proof {
		value := common.Hex2Bytes(v)
		db[crypto.Keccak256Hash(value)] = value
	}

	res, err := trie.VerifyProof(root, crypto.Keccak256(key), db)
	if err != nil {
		return err
	}
	if !bytes.Equal(value, res) {
		return errors.New("verification failed: value mismatch")
	}
	return nil
}

func verifyProof(proof *AccountResult, stateRoot common.Hash) error {
	// verify account proof
	value, err := rlp.EncodeToBytes([]interface{}{
		proof.Nonce, proof.Balance.ToInt(), proof.StorageHash, proof.CodeHash,
	})
	if err != nil {
		return err
	}
	if err := verify(stateRoot, proof.Address.Bytes(), value, proof.AccountProof); err != nil {
		return err
	}
	// verify storage proofs
	for _, p := range proof.StorageProof {
		value, err := rlp.EncodeToBytes(p.Value)
		if err != nil {
			return err
		}
		if err := verify(proof.StorageHash, common.Hex2Bytes(p.Key), value, p.Proof); err != nil {
			return err
		}
	}
	return nil
}

func ValidateDelayedMessages(acc common.Hash, msgs []arbostypes.L1IncomingMessage, endAcc common.Hash) error {
	log.Warn("checking dmsg", "startAcc", acc, "endAcc", endAcc, "count", len(msgs))
	if len(msgs) > 0 {
		offset := msgs[0].Header.RequestId.Big().Uint64()
		for i := range msgs {
			// clear the batch gas cost field in case it is set
			msgs[i].BatchGasCost = nil

			// make sure the request id matches the index
			if msgs[i].Header.RequestId.Big().Uint64() != offset+uint64(i) {
				return errors.New("unexpected delayed message request id")
			}

			// update accumulator
			acc = accumulate(msgs[i], acc)
			log.Warn("dmsg acc", "acc", acc)
		}
	}

	if acc != endAcc {
		return errors.New("delayed message accumulator mismatch")
	}

	return nil
}

func parseSeqBatches(input VerifySequencingChainInput) ([]wavmio.Batch, SyndicateAccumulator, common.Hash, error) {
	// make sure config matches the hash
	if input.Config.Hash() != input.TrustedInput.ConfigHash {
		return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("config hash mismatch")
	}

	var batches []wavmio.Batch
	acc := input.TrustedInput.L1StartBatchAcc
	log.Warn("batch acc", "acc", acc)

	batchCount := len(input.Batches)
	if batchCount > 0 {
		// make sure the end delayed message accumulator matches the one in the last batch
		if err := ValidateDelayedMessages(input.StartDelayedMessagesAccumulator, input.DelayedMessages, input.Batches[batchCount-1].DelayedAcc); err != nil {
			return nil, SyndicateAccumulator{}, common.Hash{}, err
		}

		// prepare batches & calculate the batch accumulator
		var i uint64
		msgCount := uint64(len(input.DelayedMessages))
		var offset uint64
		if msgCount > 0 {
			offset = input.DelayedMessages[0].Header.RequestId.Big().Uint64()
		}
		for _, batch := range input.Batches {
			acc = batch.accumulate(acc)
			log.Warn("batch acc", "acc", acc)
			j := batch.AfterDelayedMessagesRead - offset
			if j > msgCount {
				return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("missing delayed message")
			}
			batches = append(batches, wavmio.Batch{
				TimeBounds: batch.TimeBounds,
				Data:       batch.Data,
				Messages:   input.DelayedMessages[i:j],
			})
			i = j
		}
	}

	// make sure the end batch accumulator matches the trusted input end hash
	if input.IsL1Chain {
		if acc != input.TrustedInput.L1EndHash {
			return nil, SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("batch accumulator mismatch: got %s, expected %s", acc, input.TrustedInput.L1EndHash)
		}
	} else {
		if input.L1EndBlockHeader == nil {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("missing end block header")
		}
		if input.L1EndBlockHeader.Hash() != input.TrustedInput.L1EndHash {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("block header does not match end block hash")
		}
		if input.EndBatchAccumulatorMerkleProof == nil {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("missing end batch accumulator proof")
		}
		storageProof := input.EndBatchAccumulatorMerkleProof.StorageProof
		if len(storageProof) != 2 {
			return nil, SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("invalid number of proofs: got %d, expected 2", len(storageProof))
		}
		storageSlot := common.BigToHash(new(big.Int).Add(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, storageProof[0].Value.Big()))
		if common.HexToHash(storageProof[0].Key) != BATCH_ACCUMULATOR_STORAGE_SLOT || common.HexToHash(storageProof[1].Key) != storageSlot {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("invalid storage proof keys")
		}
		if input.EndBatchAccumulatorMerkleProof.Address != input.Config.SequencingBridgeAddress {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("merkle proof address is not the bridge contract")
		}
		if storageProof[1].Value != acc {
			return nil, SyndicateAccumulator{}, common.Hash{}, errors.New("batch acc does not match merkle proof value")
		}
		if err := verifyProof(input.EndBatchAccumulatorMerkleProof, input.L1EndBlockHeader.Root); err != nil {
			return nil, SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("failed to verify merkle proof: %w", err)
		}
	}

	return batches, SyndicateAccumulator{
		Address: common.Address(input.Config.SequencingContractAddress),
	}, acc, nil
}

func (s *Server) VerifySequencingChain(ctx context.Context, input VerifySequencingChainInput) (*VerifySequencingChainOutput, error) {
	batches, acc, l1BatchAcc, err := parseSeqBatches(input)
	if err != nil {
		return nil, fmt.Errorf("Failed to verify sequencing input data: %w", err)
	}

	data := &execution.MessageResult{
		BlockHash: input.TrustedInput.SeqStartBlockHash,
	}

	if len(batches) > 0 {
		blockVerifierInput := wavmio.ValidationInput{
			BlockHash:    input.TrustedInput.SeqStartBlockHash,
			PreimageData: input.PreimageData,
			Batches:      batches,
		}

		log.Debug("Sequencing Chain BlockVerifierInput Input", "input", blockVerifierInput)

		data, err = Verify(blockVerifierInput, &acc)
		if err != nil {
			return nil, fmt.Errorf("Failed to verify sequencing chain: %w", err)
		}
	}

	output := VerifySequencingChainOutput{
		L1BatchAcc:            l1BatchAcc,
		SequencingBlockHash:   data.BlockHash,
		SequencingBlockNumber: acc.BlockNum,
		Batches:               acc.Batches,
		Signature:             []byte{},
	}
	if err := output.sign(&input.TrustedInput, s.signerKey); err != nil {
		return nil, err
	}
	log.Debug("Sequencing Chain BlockVerifierOutput Output", "output", output)
	return &output, nil
}

func processMessage(msg *arbostypes.L1IncomingMessage, ts uint64, blockNum uint64) error {
	msg.Header.BlockNumber = blockNum
	msg.Header.Timestamp = ts
	if msg.Header.Kind == arbostypes.L1MessageType_Initialize {
		return errors.New("unexpected init message")
	}
	// TODO: check other message types as well
	if msg.Header.Kind == arbostypes.L1MessageType_BatchPostingReport {
		msg.Header.Kind = arbostypes.L1MessageType_EndOfBlock
		msg.L2msg = nil
		msg.Header.Poster = common.Address{}
		msg.Header.L1BaseFee = nil
	}
	return nil
}

func parseAppBatches(input VerifyAppchainInput) ([]wavmio.Batch, error) {
	// make sure config matches the hash
	if input.Config.Hash() != input.TrustedInput.ConfigHash {
		return nil, errors.New("config hash mismatch")
	}

	if input.AppStartBlockHeader.Hash() != input.TrustedInput.AppStartBlockHash {
		return nil, errors.New("appchain start block header mismatch")
	}

	if len(input.DelayedMessages) == 0 {
		return nil, errors.New("must include at least one delayed message")
	}

	// verify delayed messages
	if err := ValidateDelayedMessages(input.StartDelayedMessagesAccumulator, input.DelayedMessages, input.TrustedInput.SetDelayedMessageAcc); err != nil {
		return nil, err
	}

	// remove dummy delayed message used to verify the count of the accumulator
	if len(input.DelayedMessages) == 1 && input.AppStartBlockHeader.Nonce.Uint64() == input.DelayedMessages[0].Header.RequestId.Big().Uint64() {
		input.DelayedMessages = nil
	}

	// build batches
	msgCount := len(input.DelayedMessages)
	i := 0
	var batches []wavmio.Batch
	blockNum := input.VerifySequencingChainOutput.SequencingBlockNumber - uint64(len(input.VerifySequencingChainOutput.Batches))
	for _, batch := range input.VerifySequencingChainOutput.Batches {
		blockNum++
		j := i
		for i < msgCount && input.DelayedMessages[i].Header.Timestamp+input.Config.SettlementDelay <= batch.Timestamp {
			if err := processMessage(&input.DelayedMessages[i], batch.Timestamp, blockNum); err != nil {
				return nil, fmt.Errorf("Failed to process delayed message: %w", err)
			}
			i++
		}
		if i != j || len(batch.Data) > 0 {
			batches = append(batches, wavmio.Batch{
				TimeBounds: bridgegen.IBridgeTimeBounds{
					MinTimestamp:   0,
					MaxTimestamp:   math.MaxUint64,
					MinBlockNumber: 0,
					MaxBlockNumber: math.MaxUint64,
				},
				Data:     batch.Data,
				Messages: input.DelayedMessages[j:i],
			})
		}
	}

	return batches, nil
}

func (s *Server) VerifyAppchain(ctx context.Context, input VerifyAppchainInput) (*VerifyAppchainOutput, error) {
	if !input.VerifySequencingChainOutput.validate(&input.TrustedInput, &s.signerKey.PublicKey) {
		return nil, errors.New("seq output validation failed")
	}
	batches, err := parseAppBatches(input)
	if err != nil {
		return nil, fmt.Errorf("Failed to verify appchain input data: %w", err)
	}
	blockVerifierInput := wavmio.ValidationInput{
		BlockHash:    input.TrustedInput.AppStartBlockHash,
		PreimageData: input.PreimageData,
		Batches:      batches,
	}
	log.Debug("Appchain BlockVerifierInput Input", "input", blockVerifierInput)
	result, err := Verify(blockVerifierInput, nil)
	if err != nil {
		return nil, err
	}
	output := VerifyAppchainOutput{
		L1BatchAcc:          input.VerifySequencingChainOutput.L1BatchAcc,
		SequencingBlockHash: input.VerifySequencingChainOutput.SequencingBlockHash,
		AppchainBlockHash:   result.BlockHash,
		AppchainSendRoot:    result.SendRoot,
	}
	if err := output.sign(&input.TrustedInput, s.signerKey); err != nil {
		return nil, err
	}
	log.Debug("Appchain BlockVerifierOutput Output", "output", output)
	return &output, nil
}

// combines two partial appchain proofs together
// TODO: impl this function
func (s *Server) CombineAppchainProofs(input CombineAppchainInput) (*VerifyAppchainOutput, error) {
	return nil, nil
}
