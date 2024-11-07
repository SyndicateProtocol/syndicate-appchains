package mocks

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/stretchr/testify/mock"
)

const TestingPort = 8080
const TestingFrameSize = 1024

const TestingSequencingContractAddress = "0x0000000000000000000000000000000000000000"
const TestingBatcherAddress = "0x0000000000000000000000000000000000000000"
const TestingBatchInboxAddress = "0x0000000000000000000000000000000000000000"
const TestingSettlementStartBlock = 1
const TestingBatcherPrivateKey = "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
const TestingSettlementChainID = 84532
const TestingCutoverBlock = 10

type MockConfig struct {
	mock.Mock
}

var DefaultTestingConfig = &config.Config{
	Port:                      TestingPort,
	FrameSize:                 TestingFrameSize,
	LogLevel:                  "info",
	SequencingContractAddress: TestingSequencingContractAddress,
	SettlementStartBlock:      TestingSettlementStartBlock,
	BatcherPrivateKey:         TestingBatcherPrivateKey,
	SettlementChainRPCURL:     "http://localhost:8545",
	SequencingChainRPCURL:     "http://localhost:8545",
	MetaBasedChainRPCURL:      "http://localhost:8545",
	MetafillerURL:             "http://localhost:8080",
	BatchInboxAddress:         TestingBatchInboxAddress,
	BatcherAddress:            TestingBatcherAddress,
	SettlementChainID:         TestingSettlementChainID,
	CutoverBlock:              TestingCutoverBlock,
}
