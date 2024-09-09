package mocks

import "github.com/SyndicateProtocol/op-translator/internal/config"

const TESTINGPORT = 8080

var ConfigMock = &config.Config{
	Port:                TESTINGPORT,
	SettlementChainAddr: "http://localhost:8545",
	SequencingChainAddr: "http://localhost:8545",
}
