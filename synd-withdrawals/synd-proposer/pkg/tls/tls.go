package tls

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"net/http"
	"net/url"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/logger"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/pkg/errors"
	"github.com/rs/zerolog/log"
)

type Config struct {
	Enabled        bool
	ClientCertPath string
	ClientKeyPath  string
}

func CreateTLSClient(cfg *Config, rpcURL string) (*rpc.Client, error) {
	if cfg.ClientCertPath == "" || cfg.ClientKeyPath == "" {
		return nil, fmt.Errorf("TLS client certificate and key paths are required")
	}

	// Load client certificate and private key
	clientCert, err := tls.LoadX509KeyPair(cfg.ClientCertPath, cfg.ClientKeyPath)
	if err != nil {
		return nil, errors.Wrap(err, "Failed to load TLS client certificate and key pair")
	}

	tlsConfig := &tls.Config{
		Certificates:       []tls.Certificate{clientCert},
		InsecureSkipVerify: false,
		MinVersion:         tls.VersionTLS12,
	}

	httpClient := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: tlsConfig,
		},
	}

	client, err := rpc.DialOptions(
		context.Background(),
		rpcURL,
		rpc.WithHTTPClient(httpClient),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create enclave RPC client with TLS: %v", err)
	}

	return client, nil

}

// IsTLSErr returns true if err came from a failed TLS handshake / cert validation.
func IsTLSErr(err error) bool {
	var urlErr *url.Error
	if !errors.As(err, &urlErr) {
		return false
	}
	// Certificate validation errors, expect them as value types
	var certInvalid x509.CertificateInvalidError
	var unknownAuth x509.UnknownAuthorityError
	if errors.As(urlErr.Err, &certInvalid) || errors.As(urlErr.Err, &unknownAuth) {
		msg, wrappedErr := logger.WrapErrorWithMsg("TLS handshake / certificate error", err)
		log.Warn().Stack().Err(wrappedErr).Msg(msg)
		return true
	}
	// Any lower‐level TLS record error
	var recordHeaderError tls.RecordHeaderError
	if errors.As(urlErr.Err, &recordHeaderError) {
		msg, wrappedErr := logger.WrapErrorWithMsg("TLS record header error", err)
		log.Warn().Stack().Err(wrappedErr).Msg(msg)
		return true
	}
	return false
}

func HandleTLSErr(err error) error {
	if IsTLSErr(err) {
		// If the error is related to TLS, exit the program so k8s can restart it. That will automatically fix any cert expiry issues.
		log.Fatal().Err(err).Msg("TLS handshake / certificate error; exiting so k8s can rotate pod")
	}

	return errors.Wrap(err, "failed to call enclave")
}
