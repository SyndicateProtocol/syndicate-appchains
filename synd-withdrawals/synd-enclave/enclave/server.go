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
	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/hf/nitrite"
	"github.com/hf/nsm"
	"github.com/hf/nsm/request"
	"github.com/holiman/uint256"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/execution"
)

const (
	// DefaultCARoots contains the PEM encoded roots for verifying Nitro
	// Enclave attestation signatures. You can download them from
	// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html
	DefaultCARoots       = "UEsDBBQAAAAIALkYV1GVtvolRwIAAAkDAAAIABwAcm9vdC5wZW1VVAkAA10ekl9dHpJfdXgLAAEESHEtDwQUAAAAZZJLk6JQDIX3/IrZW10Igo2LWdwXiBoE5HXZCSq0iNgKfYVfP9guJ8tTqS85Ofn4GAszy3b+EOYHtmkTFLCX+CGBbRMWEILSfYGEjVFh+8itnoe4yKq1XC7DDNptcJ2YXJCC2+smtYfzlCEBYhewjQSospASMlwCiSJ40gE5uHAijBrAldny5PaTnRkAan77iBDUiw4B+A9heZxKkedRilflYQZdVl+meW20aayfM8tU0wTEsswdCKonUFuDAPotRUo8ag59axIE3ls84xV4D0FG6gi1mFhF4cBcQNP35GIcGCvlsV504ImXnVffRqLjxpECT2tA6Xt1AFabs7zXu33i91mvXLLaefAkveQDVgEjC/ff1g60BSqYJeFdhzFCX0i1EXYFibZdTWA57Jf0q26/vZ+Ka3BbDVlz2chy2qv8wnYK9vVgVz1OWSZpBjFi3PTtp6li8Xlk7X7vTprSUrNr+FgspofpKlGNIHe9hDA3nWGE7WPgcsEaEqdMKo2LzhtPBHkoL9YOgTEgKkZ//jRA3lLGKBRIMCwP6PCyuPQ0ZhZeWJFYoYfKlPzJMRZ6Ns9vM7feX087nQta/ALcN8CjqLCsV4yEvL2Pd6JIrRBYnEjgkfOpn/hNXi+S7qjxq4hrZxUhTTuhqavH6vbGG7HYchL5e3b82RjdVkn4vdOfLbixdD8BGSFfhv6IcbYS63Vy2M3xrfXMLs2Cz1kjF7hUvsPnRb46d0UNtwY/iftcuJtsMnckW2yGmcz/Sr+fzRz637f/A1BLAQIeAxQAAAAIALkYV1GVtvolRwIAAAkDAAAIABgAAAAAAAEAAACkgQAAAAByb290LnBlbVVUBQADXR6SX3V4CwABBEhxLQ8EFAAAAFBLBQYAAAAAAQABAE4AAACJAgAAAAA="
	DefaultCARootsSHA256 = "8cf60e2b2efca96c6a9e71e851d00c1b6991cc09eadbe64a6a1d1b1eb9faff7c"
)

var defaultRoot = createAWSNitroRoot()

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

// Storage slot of the batch accumulator
// <https://github.com/SyndicateProtocol/nitro-contracts/blob/9a100a86242176b633a1d907e5efd41296922144/src/bridge/AbsBridge.sol#L51>
// Since the batch accumulator is a dynamic array, this slot contains the length of the array
var BATCH_ACCUMULATOR_STORAGE_SLOT = common.BigToHash(big.NewInt(7))

// Storage slot of the first element in the batch accumulator array
// Dynamic types are stored starting at the keccak256 of the original storage slot plus an offset
// This value is Keccak256("0x7")
var (
	BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT           = crypto.Keccak256Hash(BATCH_ACCUMULATOR_STORAGE_SLOT[:]).Big()
	BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE = new(big.Int).Sub(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, common.Big1)
)

// field offsets into the serialized arbostypes.L1IncomingMessage struct
const (
	DelayedMessageSenderOffset    = 13
	DelayedMessageTimestampOffset = 41
	DelayedMessageRequestIdOffset = 49
	DelayedMessageDataOffset      = 113
)

type KVDB map[common.Hash][]byte

func (k KVDB) Has(key []byte) (bool, error) {
	_, ok := k[common.BytesToHash(key)]
	return ok, nil
}

func (k KVDB) Get(key []byte) ([]byte, error) {
	val, ok := k[common.BytesToHash(key)]
	if !ok {
		return nil, errors.New("could not find key")
	}
	return val, nil
}

func verify(root common.Hash, key []byte, value []byte, proof []hexutil.Bytes) error {
	db := make(KVDB, len(proof))
	for _, v := range proof {
		db[crypto.Keccak256Hash(v)] = v
	}

	res, err := trie.VerifyProof(root, crypto.Keccak256(key), db)
	if err != nil {
		return err
	}
	if !bytes.Equal(value, res) {
		return fmt.Errorf("verification failed: value mismatch: %s != %s", common.Bytes2Hex(value), common.Bytes2Hex(res))
	}
	return nil
}

func verifyProof(proof *AccountResult, stateRoot common.Hash) error {
	// verify account proof
	value, err := rlp.EncodeToBytes(&types.StateAccount{
		Nonce:    uint64(proof.Nonce),
		Balance:  (*uint256.Int)(&proof.Balance),
		Root:     proof.StorageHash,
		CodeHash: proof.CodeHash[:],
	})
	if err != nil {
		return err
	}
	if err := verify(stateRoot, proof.Address.Bytes(), value, proof.AccountProof); err != nil {
		return fmt.Errorf("account proof verification failed: %w", err)
	}
	// verify storage proofs
	for _, p := range proof.StorageProof {
		value, err := rlp.EncodeToBytes(p.Value.ToInt())
		if err != nil {
			return err
		}
		if err := verify(proof.StorageHash, p.Key[:], value, p.Proof); err != nil {
			return err
		}
	}
	return nil
}

func verifyBatchAccProof(proof *AccountResult, stateRoot common.Hash, sequencingBridgeAddress common.Address, acc common.Hash) error {
	if proof == nil {
		return errors.New("missing end batch accumulator proof")
	}
	if proof.Address != sequencingBridgeAddress {
		return errors.New("merkle proof address is not the bridge contract")
	}
	if len(proof.StorageProof) != 2 {
		return fmt.Errorf("invalid number of proofs: got %d, expected 2", len(proof.StorageProof))
	}
	if proof.StorageProof[0].Value.ToInt().Sign() == 0 {
		return errors.New("batch count at end block is zero")
	}
	if acc.Big().Cmp(proof.StorageProof[1].Value.ToInt()) != 0 {
		return fmt.Errorf("batch acc does not match merkle proof value: %s != %s", acc.Big(), proof.StorageProof[1].Value.ToInt())
	}
	storageSlot := new(big.Int).Add(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT_MINUS_ONE, proof.StorageProof[0].Value.ToInt())
	if proof.StorageProof[0].Key != BATCH_ACCUMULATOR_STORAGE_SLOT || proof.StorageProof[1].Key.Big().Cmp(storageSlot) != 0 {
		return fmt.Errorf("invalid storage proof keys: %s != %s or %s != %s", proof.StorageProof[0].Key, BATCH_ACCUMULATOR_STORAGE_SLOT, proof.StorageProof[1].Key.Big(), storageSlot)
	}
	if err := verifyProof(proof, stateRoot); err != nil {
		return fmt.Errorf("failed to verify merkle proof: %w", err)
	}
	return nil
}

func delayedMessageAccumulate(acc common.Hash, msg []byte) common.Hash {
	return crypto.Keccak256Hash(acc[:], crypto.Keccak256(msg[:1], msg[DelayedMessageSenderOffset:DelayedMessageDataOffset], crypto.Keccak256(msg[DelayedMessageDataOffset:])))
}

func validateDelayedMessages(msgs [][]byte) (uint64, error) {
	var start uint64
	for i, msg := range msgs {
		if len(msg) < DelayedMessageDataOffset {
			return 0, fmt.Errorf("delayed message %d too short: len=%d", i, len(msg))
		}
		requestId := common.Hash(msg[DelayedMessageRequestIdOffset : DelayedMessageRequestIdOffset+32]).Big()
		if !requestId.IsUint64() {
			return 0, fmt.Errorf("delayed message %d request id overflow: got %d", i, requestId)
		}
		// request id must increment by 1 each message
		if i == 0 {
			start = requestId.Uint64()
		} else if requestId.Uint64() != start+uint64(i) {
			return 0, fmt.Errorf("delayed message %d invalid request id: got %d, expected %d", i, requestId.Uint64(), start+uint64(i))
		}
		// the first 12 bytes of the 32-byte word used to hold the 20-byte address should be zero
		for _, b := range msg[1:DelayedMessageSenderOffset] {
			if b != 0 {
				return 0, fmt.Errorf("delayed message %d request id %d invalid address: got %s", i, requestId.Uint64(), common.Bytes2Hex(msg[1:1+32]))
			}
		}
	}
	return start, nil
}

func parseSeqBatches(input VerifySequencingChainInput) (SyndicateAccumulator, common.Hash, error) {
	// make sure config matches the hash
	if input.Config.Hash() != input.TrustedInput.ConfigHash {
		return SyndicateAccumulator{}, common.Hash{}, errors.New("config hash mismatch")
	}

	acc := input.TrustedInput.L1StartBatchAcc
	delayedAcc := input.StartDelayedMessagesAccumulator

	// verify delayed messages
	startIndex, err := validateDelayedMessages(input.DelayedMessages)
	if err != nil {
		return SyndicateAccumulator{}, common.Hash{}, err
	}

	// prepare batches & calculate the batch accumulator
	msgCount := uint64(len(input.DelayedMessages))
	var i uint64
	for j, batch := range input.Batches {
		if len(batch) < 40 {
			return SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("batch %d too short", j)
		}
		if msgCount > 0 {
			afterDelayedMessagesRead := binary.BigEndian.Uint64(batch[32:40])
			if afterDelayedMessagesRead > msgCount+startIndex {
				return SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("missing delayed messages: have %d, need %d", msgCount, afterDelayedMessagesRead-startIndex)
			}
			for i+startIndex < afterDelayedMessagesRead {
				delayedAcc = delayedMessageAccumulate(delayedAcc, input.DelayedMessages[i])
				i++
			}
		}
		acc = crypto.Keccak256Hash(acc[:], crypto.Keccak256(batch), delayedAcc[:])
	}

	if i != msgCount {
		return SyndicateAccumulator{}, common.Hash{}, errors.New("extra delayed messages included in input")
	}

	// make sure the end batch accumulator matches the trusted input end hash
	if input.IsL1Chain {
		if acc != input.TrustedInput.L1EndHash {
			return SyndicateAccumulator{}, common.Hash{}, fmt.Errorf("batch accumulator mismatch: got %s, expected %s", acc, input.TrustedInput.L1EndHash)
		}
	} else {
		if input.L1EndBlockHeader == nil {
			return SyndicateAccumulator{}, common.Hash{}, errors.New("missing end block header")
		}
		if input.L1EndBlockHeader.Hash() != input.TrustedInput.L1EndHash {
			return SyndicateAccumulator{}, common.Hash{}, errors.New("block header does not match end block hash")
		}
		if err := verifyBatchAccProof(input.EndBatchAccumulatorMerkleProof, input.L1EndBlockHeader.Root, input.Config.SequencingBridgeAddress, acc); err != nil {
			return SyndicateAccumulator{}, common.Hash{}, err
		}
	}

	return SyndicateAccumulator{
		Address: common.Address(input.Config.SequencingContractAddress),
	}, acc, nil
}

func (s *Server) VerifySequencingChain(ctx context.Context, input VerifySequencingChainInput) (*VerifySequencingChainOutput, error) {
	acc, l1BatchAcc, err := parseSeqBatches(input)
	if err != nil {
		return nil, fmt.Errorf("Failed to verify sequencing input data: %w", err)
	}

	data := &execution.MessageResult{
		BlockHash: input.TrustedInput.SeqStartBlockHash,
	}

	if len(input.Batches) > 0 {
		blockVerifierInput := wavmio.ValidationInput{
			BlockHash:    input.TrustedInput.SeqStartBlockHash,
			PreimageData: input.PreimageData,
			Batches:      input.Batches,
			Messages:     input.DelayedMessages,
		}

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
	return &output, nil
}

var allowedMsgs = map[byte]struct{}{
	arbostypes.L1MessageType_L2Message:          {},
	arbostypes.L1MessageType_L2FundedByL1:       {},
	arbostypes.L1MessageType_SubmitRetryable:    {},
	arbostypes.L1MessageType_EthDeposit:         {},
	arbostypes.L1MessageType_BatchPostingReport: {},
}

func processMessage(msg []byte, blockNum uint64, ts uint64) ([]byte, error) {
	if _, ok := allowedMsgs[msg[0]]; !ok {
		return nil, fmt.Errorf("unexpected message: type %d", msg[0])
	}
	if msg[0] == arbostypes.L1MessageType_BatchPostingReport {
		requestId := msg[DelayedMessageRequestIdOffset : DelayedMessageRequestIdOffset+32]
		msg = make([]byte, DelayedMessageDataOffset)
		copy(msg[DelayedMessageRequestIdOffset:DelayedMessageRequestIdOffset+32], requestId)
		msg[0] = arbostypes.L1MessageType_EndOfBlock
	}
	binary.BigEndian.PutUint64(msg[33:41], blockNum)
	binary.BigEndian.PutUint64(msg[41:49], ts)
	return msg, nil
}

func buildArbBatch(afterDelayedMessagesRead uint64, data []byte) []byte {
	var msg []byte

	// Set header values
	for _, value := range []uint64{
		0, math.MaxUint64, 0, math.MaxUint64, afterDelayedMessagesRead,
	} {
		var buffer [8]byte
		binary.BigEndian.PutUint64(buffer[:], value)
		msg = append(msg, buffer[:]...)
	}

	// Append the batch data
	msg = append(msg, data...)

	return msg
}

// returns batches, modifies delayed messages in place
func parseAppBatches(input *VerifyAppchainInput) ([][]byte, error) {
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

	fmt.Println("input.DelayedMessage length", len(input.DelayedMessages))
	// fmt.Println("input.DelayedMessages", input.DelayedMessages)
	fmt.Println("input.StartDelayedMessagesAccumulator", input.StartDelayedMessagesAccumulator)

	// verify delayed messages
	startIndex, err := validateDelayedMessages(input.DelayedMessages)
	if err != nil {
		return nil, err
	}

	// verify delayed message accumulator
	acc := input.StartDelayedMessagesAccumulator
	for _, msg := range input.DelayedMessages {
		acc = delayedMessageAccumulate(acc, msg)
	}
	if acc != input.TrustedInput.SetDelayedMessageAcc {
		return nil, fmt.Errorf("delayed message accumulator mismatch: got %s, expected %s", common.Bytes2Hex(acc.Bytes()), common.Bytes2Hex(input.TrustedInput.SetDelayedMessageAcc[:]))
	}

	// remove dummy delayed message used to verify the count of the accumulator
	if len(input.DelayedMessages) == 1 && input.AppStartBlockHeader.Nonce.Uint64() == startIndex+1 {
		input.DelayedMessages = nil
		startIndex++
	}

	// build batches & update delayed messages
	msgCount := uint64(len(input.DelayedMessages))
	var i uint64
	var batches [][]byte
	blockNum := input.VerifySequencingChainOutput.SequencingBlockNumber - uint64(len(input.VerifySequencingChainOutput.Batches))
	for _, batch := range input.VerifySequencingChainOutput.Batches {
		blockNum++
		var hasDelayedMessage bool
		for i < msgCount {
			timestamp := binary.BigEndian.Uint64(input.DelayedMessages[i][DelayedMessageTimestampOffset : DelayedMessageTimestampOffset+8])
			if timestamp+input.Config.SettlementDelay > batch.Timestamp {
				break
			}
			var err error
			input.DelayedMessages[i], err = processMessage(input.DelayedMessages[i], blockNum, batch.Timestamp)
			if err != nil {
				return nil, fmt.Errorf("Failed to process delayed message: %w", err)
			}
			i++
			hasDelayedMessage = true
		}
		if hasDelayedMessage || len(batch.Data) > 0 {
			batches = append(batches, buildArbBatch(startIndex+i, batch.Data))
		}
		batch.Data = nil
	}

	// remove unused delayed messages
	input.DelayedMessages = input.DelayedMessages[:i]

	return batches, nil
}

func (s *Server) VerifyAppchain(ctx context.Context, input VerifyAppchainInput) (*VerifyAppchainOutput, error) {
	if err := input.VerifySequencingChainOutput.validate(&input.TrustedInput, &s.signerKey.PublicKey); err != nil {
		return nil, fmt.Errorf("seq output validation failed: %w", err)
	}
	batches, err := parseAppBatches(&input)
	if err != nil {
		return nil, fmt.Errorf("Failed to verify appchain input data: %w", err)
	}
	info := types.DeserializeHeaderExtraInformation(&input.AppStartBlockHeader)
	result := &execution.MessageResult{
		BlockHash: input.TrustedInput.AppStartBlockHash,
		SendRoot:  info.SendRoot,
	}
	if len(batches) > 0 {
		blockVerifierInput := wavmio.ValidationInput{
			BlockHash:    input.TrustedInput.AppStartBlockHash,
			PreimageData: input.PreimageData,
			Batches:      batches,
			Messages:     input.DelayedMessages,
		}
		result, err = Verify(blockVerifierInput, nil)
		if err != nil {
			return nil, err
		}
	}
	output := VerifyAppchainOutput{
		PendingAssertion: teemodule.PendingAssertion{
			AppBlockHash: result.BlockHash,
			AppSendRoot:  result.SendRoot,
			SeqBlockHash: input.VerifySequencingChainOutput.SequencingBlockHash,
			L1BatchAcc:   input.VerifySequencingChainOutput.L1BatchAcc,
		},
	}
	if err := output.sign(&input.TrustedInput, s.signerKey); err != nil {
		return nil, err
	}
	return &output, nil
}

// combines two partial appchain proofs together
func (s *Server) CombineAppchainProofs(input CombineAppchainInput) (*CombineAppchainOutput, error) {
	if err := input.Outputs[0].validate(&input.Inputs[0], &s.signerKey.PublicKey); err != nil {
		return nil, fmt.Errorf("first output validation failed: %w", err)
	}
	if err := input.Outputs[1].validate(&input.Inputs[1], &s.signerKey.PublicKey); err != nil {
		return nil, fmt.Errorf("second output validation failed: %w", err)
	}

	if input.Inputs[0].ConfigHash != input.Inputs[1].ConfigHash || input.Config.Hash() != input.Inputs[0].ConfigHash {
		return nil, errors.New("config hash mismatch")
	}
	if input.Outputs[0].PendingAssertion.SeqBlockHash != input.Inputs[1].SeqStartBlockHash ||
		input.Outputs[0].PendingAssertion.AppBlockHash != input.Inputs[1].AppStartBlockHash ||
		input.Outputs[0].PendingAssertion.L1BatchAcc != input.Inputs[1].L1StartBatchAcc {
		return nil, errors.New("input output mismatch")
	}

	if input.Inputs[0].SetDelayedMessageAcc != input.Inputs[1].SetDelayedMessageAcc {
		if _, err := validateDelayedMessages([][]byte{input.SetFirstDelayedMessage}); err != nil {
			return nil, err
		}
		hash := crypto.Keccak256(input.SetFirstDelayedMessage[:DelayedMessageDataOffset], crypto.Keccak256(input.SetFirstDelayedMessage[DelayedMessageDataOffset:]))
		acc := crypto.Keccak256Hash(input.Inputs[0].SetDelayedMessageAcc[:], hash)
		for _, hash := range input.SetRemainingDelayedMessageHashes {
			acc = crypto.Keccak256Hash(acc[:], hash[:])
		}
		if acc != input.Inputs[1].SetDelayedMessageAcc {
			return nil, errors.New("set delayed message acc derivation failure")
		}
		timestamp := binary.BigEndian.Uint64(input.SetFirstDelayedMessage[DelayedMessageTimestampOffset : DelayedMessageTimestampOffset+8])
		if input.SeqFirstEndBlockHeader == nil {
			return nil, errors.New("missing seq first end block header")
		}
		if input.SeqFirstEndBlockHeader.Hash() != input.Outputs[0].PendingAssertion.SeqBlockHash {
			return nil, errors.New("seq first end block header hash mismatch")
		}
		if timestamp+input.Config.SettlementDelay <= input.SeqFirstEndBlockHeader.Time {
			return nil, errors.New("first proof is missing settlement blocks")
		}
	}

	input.Inputs[0].SetDelayedMessageAcc = input.Inputs[1].SetDelayedMessageAcc
	input.Inputs[0].L1EndHash = input.Inputs[1].L1EndHash
	input.Outputs[1].sign(&input.Inputs[0], s.signerKey)
	return &CombineAppchainOutput{
		Input:  input.Inputs[0],
		Output: input.Outputs[1],
	}, nil
}
