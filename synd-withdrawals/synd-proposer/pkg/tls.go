package pkg

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"errors"
	"fmt"
	"log"
	"net/http"
	"net/url"

	"github.com/ethereum/go-ethereum/rpc"
)

type TLSConfig struct {
	ClientCertPath string
	ClientKeyPath  string
}

func createTLSClient(cfg *TLSConfig, rpcURL string) (*rpc.Client, error) {
	if cfg.ClientCertPath == "" || cfg.ClientKeyPath == "" {
		return nil, fmt.Errorf("TLS client certificate and key paths are required")
	}

	// Load client certificate and private key
	clientCert, err := tls.LoadX509KeyPair(cfg.ClientCertPath, cfg.ClientKeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load client certificate: %v", err)
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

// isTLSErr returns true if err came from a failed TLS handshake / cert validation.
func isTLSErr(err error) bool {
	var urlErr *url.Error
	if !errors.As(err, &urlErr) {
		return false
	}
	// Certificate validation errors
	var certInvalid x509.CertificateInvalidError
	var unknownAuth x509.UnknownAuthorityError
	if errors.As(urlErr.Err, &certInvalid) || errors.As(urlErr.Err, &unknownAuth) {
		log.Printf("TLS handshake / certificate error: %v", err)
		return true
	}
	// Any lower‐level TLS record error
	if _, ok := urlErr.Err.(tls.RecordHeaderError); ok {
		log.Printf("TLS record header error: %v", err)
		return true
	}
	return false
}

func handleTLSErr(err error) error {
	if isTLSErr(err) {
		// If the error is related to TLS, exit the program so k8s can restart it. That will automatically fix any cert expiry issues.
		log.Fatalf("TLS handshake / certificate error; exiting so k8s can rotate pod: %v", err)
	}

	return fmt.Errorf("failed to call enclave: %v", err)
}
