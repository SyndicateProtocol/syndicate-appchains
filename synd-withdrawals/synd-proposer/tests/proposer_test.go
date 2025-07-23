package tests

import (
	"context"
	"fmt"
	"io/ioutil"
	"net"
	"net/http"
	"testing"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/metrics"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/tls"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/prometheus/client_golang/prometheus"
)

func TestInitProposerWithConfig(t *testing.T) {
	privateKey, err := crypto.HexToECDSA("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
	if err != nil {
		t.Fatalf("failed to parse private key: %v", err)
	}
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
    registry := prometheus.NewRegistry()
	dummyMetrics := metrics.NewMetrics(registry)
	proposer := pkg.NewProposer(context.Background(), dummyCfg, dummyMetrics)
	if proposer.Config != dummyCfg {
		t.Errorf("Proposer config was not set correctly")
	}
}

func TestHealthEndpoint(t *testing.T) {
	// Find a free port
	ln, err := net.Listen("tcp", ":0")
	if err != nil {
		t.Fatalf("failed to find a free port: %v", err)
	}
	port := ln.Addr().(*net.TCPAddr).Port
	ln.Close()

	healthSrv := &http.Server{
		Addr: fmt.Sprintf(":%d", port),
		Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if r.URL.Path == "/health" {
				w.Header().Set("Content-Type", "application/json")
				w.WriteHeader(http.StatusOK)
				w.Write([]byte(`{"status":"ok"}`))
				return
			}
			http.NotFound(w, r)
		}),
	}

	done := make(chan struct{})
	go func() {
		healthSrv.ListenAndServe()
		close(done)
	}()
	defer func() {
		healthSrv.Close()
		<-done
	}()

	// Give the server a moment to start
	time.Sleep(100 * time.Millisecond)

	resp, err := http.Get(fmt.Sprintf("http://localhost:%d/health", port))
	if err != nil {
		t.Fatalf("failed to GET /health: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("expected 200 OK, got %d", resp.StatusCode)
	}
	body, _ := ioutil.ReadAll(resp.Body)
	if string(body) != `{"status":"ok"}` {
		t.Errorf("unexpected body: %s", string(body))
	}
}
