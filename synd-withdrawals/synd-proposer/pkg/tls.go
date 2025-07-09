package pkg

import (
	"context"
	"crypto/tls"
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"

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

func isTLSErr(err error) bool {
	return strings.Contains(err.Error(), "tls") || strings.Contains(err.Error(), "certificate")
}

func handleTLSErr(err error) error {
	if isTLSErr(err) {
		log.Println("TLS error, exiting program so k8s can restart it. Error: ", err)
		// If the error is related to TLS, exit the program so k8s can restart it. That will automatically fix any cert expiry issues.
		os.Exit(1)
	}

	return fmt.Errorf("failed to call enclave: %v", err)
}
