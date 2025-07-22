package tests

import (
	"context"
	"crypto/ecdsa"
	"fmt"
	"io"
	"net/http"
	"testing"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/metrics"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/tls"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/server"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func dummyCfg(privateKey *ecdsa.PrivateKey) *config.Config {
	dummyCfg := &config.Config{
		EthereumRPCURL:           "http://localhost:8545",
		SettlementRPCURL:         "http://localhost:8546",
		SequencingRPCURL:         "http://localhost:8547",
		AppchainRPCURL:           "http://localhost:8548",
		EnclaveRPCURL:            "http://localhost:8549",
		TeeModuleContractAddress: common.HexToAddress("0x41F2F55571f9e8e3Ba511Adc48879Bd67626A2b4"),
		AppchainBridgeAddress:    common.HexToAddress("0x41F2F55571f9e8e3Ba511Adc48879Bd67626A2b5"),
		PrivateKey:               privateKey,
		PollingInterval:          10 * time.Second,
		CloseChallengeInterval:   5 * time.Second,
		Port:                     9292,
		SettlementChainID:        9999,
		EnclaveTLSConfig: tls.TLSConfig{
			Enabled:        false,
			ClientCertPath: "/etc/tls/tls.crt",
			ClientKeyPath:  "/etc/tls/tls.key",
		},
	}
	return dummyCfg
}

func TestInitProposerWithConfig(t *testing.T) {
	privateKey, err := crypto.HexToECDSA("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
	if err != nil {
		t.Fatalf("failed to parse private key: %v", err)
	}
	dummyCfg := dummyCfg(privateKey)
	registry := prometheus.NewRegistry()
	dummyMetrics := metrics.NewMetrics(registry)
	proposer := pkg.NewProposer(context.Background(), dummyCfg, dummyMetrics)
	if proposer.Config != dummyCfg {
		t.Errorf("Proposer config was not set correctly")
	}
}

func TestServerInit(t *testing.T) {
	// Test server initialization
	port := 18080
	server := server.InitServer(port)

	assert.NotNil(t, server)
	assert.NotNil(t, server.Server)
	assert.NotNil(t, server.Registry)
	assert.Equal(t, fmt.Sprintf(":%d", port), server.Addr)
}

func TestServerEndpoints(t *testing.T) {
	privateKey, err := crypto.HexToECDSA("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
	if err != nil {
		t.Fatalf("failed to parse private key: %v", err)
	}
	dummyCfg := dummyCfg(privateKey)

	server := server.InitServer(dummyCfg.Port)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Start server
	err = server.Start(ctx)
	require.NoError(t, err)

	// Run proposer with server, allow it to fail
	go server.RunProposer(ctx, dummyCfg, server)

	// Wait for server to be ready
	baseURL := fmt.Sprintf("http://localhost:%d", dummyCfg.Port)

	tests := []struct {
		name           string
		endpoint       string
		expectedStatus int
		expectedBody   string
		bodyContains   []string
	}{
		{
			name:           "health endpoint",
			endpoint:       "/health",
			expectedStatus: http.StatusOK,
			expectedBody:   `{"status":"ok"}`,
		},
		{
			name:           "metrics endpoint",
			endpoint:       "/metrics",
			expectedStatus: http.StatusOK,
			bodyContains:   []string{"# HELP", "# TYPE"},
		},
		{
			name:           "root endpoint",
			endpoint:       "/",
			expectedStatus: http.StatusOK,
			expectedBody:   "Syndicate Proposer\nAvailable endpoints:\n- /health\n- /metrics\n",
		},
		{
			name:           "unknown endpoint",
			endpoint:       "/unknown",
			expectedStatus: http.StatusNotFound,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			resp, err := http.Get(baseURL + tt.endpoint)
			require.NoError(t, err)
			defer resp.Body.Close()

			assert.Equal(t, tt.expectedStatus, resp.StatusCode)

			body, err := io.ReadAll(resp.Body)
			require.NoError(t, err)

			if tt.expectedBody != "" {
				assert.Equal(t, tt.expectedBody, string(body))
			}

			for _, contains := range tt.bodyContains {
				assert.Contains(t, string(body), contains)
			}
		})
	}
}
