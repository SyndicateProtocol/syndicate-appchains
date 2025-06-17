package tests

import (
	"testing"
	"time"

	"github.com/SyndicateProtocol/synd-proposer/pkg"
)

func TestDummy(t *testing.T) {
	t.Log("Dummy test running.")
}

func TestInitProposerWithConfig(t *testing.T) {
	dummyCfg := &pkg.Config{
		EthereumRPCURL:           "http://localhost:8545",
		SettlementRPCURL:         "http://localhost:8546",
		SequencingRPCURL:         "http://localhost:8547",
		AppchainRPCURL:           "http://localhost:8548",
		EnclaveRPCURL:            "http://localhost:8549",
		TeeModuleContractAddress: "0x123",
		ArbitrumBridgeAddress:    "0x456",
		InboxAddress:             "0x789",
		SequencerInboxAddress:    "0xabc",
		PrivateKey:               "dummykey",
		PollingInterval:          10 * time.Second,
		CloseChallengeInterval:   5 * time.Second,
		MetricsPort:              9292,
	}
	proposer := pkg.NewProposer(dummyCfg)
	if proposer.Config != dummyCfg {
		t.Errorf("Proposer config was not set correctly")
	}
}
