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
	"github.com/offchainlabs/nitro/execution"
)

const (
	// DefaultCARoots contains the PEM encoded roots for verifying Nitro
	// Enclave attestation signatures. You can download them from
	// https://docs.aws.amazon.com/enclaves/latest/user/verify-root.html
	DefaultCARoots         = "UEsDBBQAAAAIALkYV1GVtvolRwIAAAkDAAAIABwAcm9vdC5wZW1VVAkAA10ekl9dHpJfdXgLAAEESHEtDwQUAAAAZZJLk6JQDIX3/IrZW10Igo2LWdwXiBoE5HXZCSq0iNgKfYVfP9guJ8tTqS85Ofn4GAszy3b+EOYHtmkTFLCX+CGBbRMWEILSfYGEjVFh+8itnoe4yKq1XC7DDNptcJ2YXJCC2+smtYfzlCEBYhewjQSospASMlwCiSJ40gE5uHAijBrAldny5PaTnRkAan77iBDUiw4B+A9heZxKkedRilflYQZdVl+meW20aayfM8tU0wTEsswdCKonUFuDAPotRUo8ag59axIE3ls84xV4D0FG6gi1mFhF4cBcQNP35GIcGCvlsV504ImXnVffRqLjxpECT2tA6Xt1AFabs7zXu33i91mvXLLaefAkveQDVgEjC/ff1g60BSqYJeFdhzFCX0i1EXYFibZdTWA57Jf0q26/vZ+Ka3BbDVlz2chy2qv8wnYK9vVgVz1OWSZpBjFi3PTtp6li8Xlk7X7vTprSUrNr+FgspofpKlGNIHe9hDA3nWGE7WPgcsEaEqdMKo2LzhtPBHkoL9YOgTEgKkZ//jRA3lLGKBRIMCwP6PCyuPQ0ZhZeWJFYoYfKlPzJMRZ6Ns9vM7feX087nQta/ALcN8CjqLCsV4yEvL2Pd6JIrRBYnEjgkfOpn/hNXi+S7qjxq4hrZxUhTTuhqavH6vbGG7HYchL5e3b82RjdVkn4vdOfLbixdD8BGSFfhv6IcbYS63Vy2M3xrfXMLs2Cz1kjF7hUvsPnRb46d0UNtwY/iftcuJtsMnckW2yGmcz/Sr+fzRz637f/A1BLAQIeAxQAAAAIALkYV1GVtvolRwIAAAkDAAAIABgAAAAAAAEAAACkgQAAAAByb290LnBlbVVUBQADXR6SX3V4CwABBEhxLQ8EFAAAAFBLBQYAAAAAAQABAE4AAACJAgAAAAA="
	DefaultCARootsSHA256   = "8cf60e2b2efca96c6a9e71e851d00c1b6991cc09eadbe64a6a1d1b1eb9faff7c"
	maxSequencerDriftFjord = 1800
)

var (
	defaultRoot                = createAWSNitroRoot()
	l2ToL1MessagePasserAddress = common.HexToAddress("0x4200000000000000000000000000000000000016")
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
	ca, err := reader.File[0].Open()
	if err != nil {
		panic("error reading AWS root cert zip")
	}
	pem, err := io.ReadAll(ca)
	if err != nil {
		panic("error reading AWS root cert")
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

var _ RPC = (*Server)(nil)

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

// skips all rust code
func (s *Server) TestVerifySequencingChain(ctx context.Context, input wavmio.ValidationInput) (*execution.MessageResult, error) {
	return Verify(input)
}

// skips all rust code
func (s *Server) TestVerifyAppchain(ctx context.Context, input wavmio.ValidationInput) (*execution.MessageResult, error) {
	return Verify(input)
}

func (s *Server) VerifySequencingChain(ctx context.Context, verifyInput string) (string, error) {
	// TODO (SEQ-961): Implement Sequencing Chain Verifier Component
	// Sequencing chain verification
	// Sequencing chain block verifier
	// Sign & return
	return "", nil
}

func (s *Server) VerifyAppchain(ctx context.Context, config string, sequencingChainInput string, settlementChainInput string, appchainConfigHash string) (string, error) {
	// Execute Appchain Verifier Rust Binary
	cmd := exec.Command("cargo", "run", "--bin", "synd-appchain-verifier", "--",
		"--config", config,
		"--sequencing-chain-input", sequencingChainInput,
		"--settlement-chain-input", settlementChainInput,
		"--appchain-config-hash", appchainConfigHash,
	)

	out, err := cmd.CombinedOutput()
	if err != nil {
		return "", fmt.Errorf("failed to run synd-appchain-verifier: %w. Output: %s", err, string(out))
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
		return "", fmt.Errorf("failed to unmarshal batches: %w. Raw: %s", err, outputLine)
	}

	// TODO: set block hash, preimage data properly
	var input = wavmio.ValidationInput{
		BlockHash:    common.Hash{},
		PreimageData: nil,
		Batches:      batches,
	}

	// Appchain block verifier
	// TODO (SEQ-781): Block verifier
	log.Debug("VerifyAppchain Input", "input", input)

	data, err := Verify(input)
	if err != nil {
		return "", err
	}

	// Sign & return
	return fmt.Sprintf("%s %s", data.BlockHash, data.SendRoot), nil
}
