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
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/log"
	"github.com/hf/nitrite"
	"github.com/hf/nsm"
	"github.com/hf/nsm/request"
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

// used to generate the signature
type SeqTrustedInput struct {
	SeqConfigHash     common.Hash
	L1StartBlockHash  common.Hash
	L1EndBlockHash    common.Hash
	SeqStartBlockHash common.Hash
}

func (input *SeqTrustedInput) hash() common.Hash {
	return crypto.Keccak256Hash(input.SeqConfigHash[:], input.L1StartBlockHash[:], input.L1EndBlockHash[:], input.SeqStartBlockHash[:])
}

type SeqVerifyInput struct {
	TrustedInput SeqTrustedInput

	// trustless preimage data
	PreimageData [][]byte

	// the output of the verifier - should be replaced with verifier inputs instead
	Batches []wavmio.Batch
}

type SeqVerifyOutput struct {
	BlockHash common.Hash
	Signature []byte
}

func (output *SeqVerifyOutput) sign(input common.Hash, key *ecdsa.PrivateKey) (err error) {
	payload := crypto.Keccak256(input[:], crypto.Keccak256(output.BlockHash[:]))
	output.Signature, err = crypto.Sign(payload, key)
	return
}

func (output *SeqVerifyOutput) validate(input common.Hash, key *ecdsa.PublicKey) bool {
	payload := crypto.Keccak256(input[:], crypto.Keccak256(output.BlockHash[:]))
	return crypto.VerifySignature(crypto.FromECDSAPub(key), payload, output.Signature)
}

type AppVerifyExtraInput struct {
	// trustless old output
	Output AppVerifyOutput
	// trusted new inputs which replace the old ones
	L1EndBlockHash       common.Hash
	SetDelayedMessageAcc common.Hash
	// trustless message hashes to derive the new accumulator from the old one
	DelayedMessageHashes []common.Hash
}

// used to generate the signature
type AppTrustedInput struct {
	SeqTrustedInput      SeqTrustedInput
	AppConfigHash        common.Hash
	AppStartBlockHash    common.Hash
	SetDelayedMessageAcc common.Hash
}

func (input *AppTrustedInput) hash() common.Hash {
	return crypto.Keccak256Hash(input.AppConfigHash[:], input.AppStartBlockHash[:], input.SeqTrustedInput.SeqConfigHash[:], input.SeqTrustedInput.SeqStartBlockHash[:], input.SetDelayedMessageAcc[:], input.SeqTrustedInput.L1StartBlockHash[:], input.SeqTrustedInput.L1EndBlockHash[:])
}

type AppVerifyInput struct {
	// appchain trusted input
	TrustedInput AppTrustedInput

	// optional extra input data to concat results with a previous AppVerifyOutput
	ExtraInput *AppVerifyExtraInput

	// seq trustless output for either TrustedInput or ExtraInput
	SeqOutput SeqVerifyOutput

	// trustless preimage data
	PreimageData [][]byte

	// the output of the verifier - should be replaced with verifier inputs instead
	Batches []wavmio.Batch
}

type AppVerifyOutput struct {
	BlockHash    common.Hash
	SendRoot     common.Hash
	SeqBlockHash common.Hash
	Signature    []byte
}

func (output *AppVerifyOutput) sign(input common.Hash, priv *ecdsa.PrivateKey) (err error) {
	payload := crypto.Keccak256(input[:], crypto.Keccak256(output.BlockHash[:], output.SendRoot[:], output.SeqBlockHash[:]))
	output.Signature, err = crypto.Sign(payload, priv)
	return
}

func (output *AppVerifyOutput) validate(input common.Hash, key *ecdsa.PublicKey) bool {
	payload := crypto.Keccak256(input[:], crypto.Keccak256(output.BlockHash[:], output.SendRoot[:], output.SeqBlockHash[:]))
	return crypto.VerifySignature(crypto.FromECDSAPub(key), payload, output.Signature)
}

// skips all rust code
func (s *Server) TestVerifySequencingChain(ctx context.Context, input SeqVerifyInput) (*SeqVerifyOutput, error) {
	// todo: add verifier code here before calling the block verifier Verify function

	result, err := Verify(wavmio.ValidationInput{
		BlockHash:    input.TrustedInput.SeqStartBlockHash,
		PreimageData: input.PreimageData,
		Batches:      input.Batches,
	})
	if err != nil {
		return nil, err
	}
	output := SeqVerifyOutput{
		BlockHash: result.BlockHash,
	}
	if err := output.sign(input.TrustedInput.hash(), s.signerKey); err != nil {
		return nil, err
	}
	return &output, nil
}

// skips all rust code
func (s *Server) TestVerifyAppchain(ctx context.Context, input AppVerifyInput) (*AppVerifyOutput, error) {
	// backup old start values
	l1StartBlockHash := input.TrustedInput.SeqTrustedInput.L1StartBlockHash
	appStartBlockHash := input.TrustedInput.AppStartBlockHash
	seqStartBlockHash := input.TrustedInput.SeqTrustedInput.SeqStartBlockHash

	if input.ExtraInput != nil {
		// verify old start values
		if !input.ExtraInput.Output.validate(input.TrustedInput.hash(), &s.signerKey.PublicKey) {
			return nil, errors.New("output validation failed")
		}

		// verify the new delayed message acc contains the old one
		acc := input.TrustedInput.SetDelayedMessageAcc
		for _, hash := range input.ExtraInput.DelayedMessageHashes {
			acc = crypto.Keccak256Hash(acc[:], hash[:])
		}
		if acc != input.ExtraInput.SetDelayedMessageAcc {
			return nil, errors.New("set delayed message acc validation failed")
		}

		// temporarily update to new start values
		input.TrustedInput.SeqTrustedInput.L1StartBlockHash = input.TrustedInput.SeqTrustedInput.L1EndBlockHash
		input.TrustedInput.AppStartBlockHash = input.ExtraInput.Output.BlockHash
		input.TrustedInput.SeqTrustedInput.SeqStartBlockHash = input.ExtraInput.Output.SeqBlockHash

		// permanently update end values
		input.TrustedInput.SeqTrustedInput.L1EndBlockHash = input.ExtraInput.L1EndBlockHash
		input.TrustedInput.SetDelayedMessageAcc = input.ExtraInput.SetDelayedMessageAcc
	}
	if !input.SeqOutput.validate(input.TrustedInput.SeqTrustedInput.hash(), &s.signerKey.PublicKey) {
		return nil, errors.New("seq output validation failed")
	}

	// todo: add verifier code here before calling the block verifier Verify function

	result, err := Verify(wavmio.ValidationInput{
		BlockHash:    input.TrustedInput.AppStartBlockHash,
		PreimageData: input.PreimageData,
		Batches:      input.Batches,
	})
	if err != nil {
		return nil, err
	}

	// restore old start values
	input.TrustedInput.SeqTrustedInput.L1StartBlockHash = l1StartBlockHash
	input.TrustedInput.AppStartBlockHash = appStartBlockHash
	input.TrustedInput.SeqTrustedInput.SeqStartBlockHash = seqStartBlockHash

	output := AppVerifyOutput{
		BlockHash:    result.BlockHash,
		SendRoot:     result.SendRoot,
		SeqBlockHash: input.SeqOutput.BlockHash,
	}
	if err := output.sign(input.TrustedInput.hash(), s.signerKey); err != nil {
		return nil, err
	}
	return &output, nil
}

func (s *Server) VerifySequencingChain(ctx context.Context, verifyInput VerifySequencingChainInput) (VerifySequencingChainOutput, error) {
	// Sanitize to ensure non-nil slices for correct JSON serialization
	SanitizeVerifySequencingChainInput(&verifyInput)

	// Execute Sequencing Chain Verifier Rust Binary
	config, err := json.Marshal(verifyInput.VerifySequencingChainConfig)
	if err != nil {
		return VerifySequencingChainOutput{}, fmt.Errorf("failed to marshal verify sequencing chain config: %w", err)
	}
	sequencingChainInput, err := json.Marshal(verifyInput.L1ChainInput)
	if err != nil {
		return VerifySequencingChainOutput{}, fmt.Errorf("failed to marshal sequencing chain input: %w", err)
	}
	cmd := exec.Command("cargo", "run", "--bin", "synd-seqchain-verifier", "--",
		"--config", string(config),
		"--sequencing-chain-input", string(sequencingChainInput),
	)
	out, err := cmd.CombinedOutput()
	if err != nil {
		return VerifySequencingChainOutput{}, fmt.Errorf("failed to run synd-seqchain-verifier: %w. Output: %s", err, string(out))
	}

	log.Debug("VerifySequencingChain Output", "output", string(out))

	lines := strings.Split(string(out), "\n")
	var outputLine string
	for i := len(lines) - 1; i >= 0; i-- {
		trimmed := strings.TrimSpace(lines[i])
		if trimmed != "" {
			outputLine = trimmed
			break
		}
	}

	var batches []wavmio.Batch
	log.Debug("VerifySequencingChain Output Line", "outputLine", outputLine)
	if err := json.Unmarshal([]byte(outputLine), &batches); err != nil {
		return VerifySequencingChainOutput{}, fmt.Errorf("failed to unmarshal batches: %w. Raw: %s", err, outputLine)
	}

	var blockVerifierInput = wavmio.ValidationInput{
		BlockHash:    verifyInput.L1ChainInput.StartBlockHash,
		PreimageData: verifyInput.SequencingPreImageData,
		Batches:      batches,
	}

	log.Debug("Sequencing Chain BlockVerifierInput Input", "input", blockVerifierInput)
	data, err := Verify(blockVerifierInput)
	if err != nil {
		return VerifySequencingChainOutput{}, fmt.Errorf("Failed to verify sequencing chain: %w", err)
	}

	output := VerifySequencingChainOutput{
		L1SequencingBlockHash: data.BlockHash,
		L1EndBlockHash:        verifyInput.L1ChainInput.EndBlockHash
	}
	log.Debug("Sequencing Chain BlockVerifierOutput Output", "output", output)
	return output, nil

}

func (s *Server) VerifyAppchain(ctx context.Context, verifyInput VerifyAppchainInput) (VerifyAppchainOutput, error) {
	// Sanitize to ensure non-nil slices for correct JSON serialization
	SanitizeVerifyAppchainInput(&verifyInput)

	// Execute Appchain Verifier Rust Binary
	config, err := json.Marshal(verifyInput.VerifyAppchainConfig)
	if err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("failed to marshal verify appchain config: %w", err)
	}
	sequencingChainInput, err := json.Marshal(verifyInput.SequencingChainInput)
	if err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("failed to marshal sequencing chain input: %w", err)
	}
	settlementChainInput, err := json.Marshal(verifyInput.SettlementChainInput)
	if err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("failed to marshal settlement chain input: %w", err)
	}
	cmd := exec.Command("cargo", "run", "--bin", "synd-appchain-verifier", "--",
		"--config", string(config),
		"--sequencing-chain-input", string(sequencingChainInput),
		"--settlement-chain-input", string(settlementChainInput),
		"--appchain-config-hash", verifyInput.AppchainConfigHash.Hex(),
	)

	out, err := cmd.CombinedOutput()
	if err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("failed to run synd-appchain-verifier: %w. Output: %s", err, string(out))
	}

	log.Debug("VerifyAppchain Output", "output", string(out))

	lines := strings.Split(string(out), "\n")
	var outputLine string
	for i := len(lines) - 1; i >= 0; i-- {
		trimmed := strings.TrimSpace(lines[i])
		if trimmed != "" {
			outputLine = trimmed
			break
		}
	}

	var batches []wavmio.Batch
	log.Debug("VerifyAppchain Output Line", "outputLine", outputLine)
	if err := json.Unmarshal([]byte(outputLine), &batches); err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("failed to unmarshal batches: %w. Raw: %s", err, outputLine)
	}

	var blockVerifierInput = wavmio.ValidationInput{
		BlockHash:    common.Hash{},
		PreimageData: verifyInput.AppchainPreImageData,
		Batches:      batches,
	}

	log.Debug("Appchain BlockVerifierInput Input", "input", blockVerifierInput)
	data, err := Verify(blockVerifierInput)
	if err != nil {
		return VerifyAppchainOutput{}, fmt.Errorf("Failed to verify appchain: %w", err)
	}

	// Sign & return
	output := VerifyAppchainOutput{
		LastAppchainBlockHash: data.BlockHash,
		LastAppchainSendRoot:  data.SendRoot,
		// TODO: set output properly
		L1SequencingBlockHash: verifyInput.VerifySequencingChainOutput.L1SequencingBlockHash,
		L1EndBlockHash:        verifyInput.VerifySequencingChainOutput.L1EndBlockHash,
	}
	log.Debug("Appchain BlockVerifierOutput Output", "output", output)
	return output, nil
}
