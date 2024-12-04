package mocks

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/stretchr/testify/mock"
)

const (
	TestingPort      = 8080
	TestingFrameSize = 1024
)

const (
	TestingSequencingContractAddress = "0x0000000000000000000000000000000000000000"
	TestingBatcherAddress            = "0x0000000000000000000000000000000000000000"
	TestingBatchInboxAddress         = "0x0000000000000000000000000000000000000000"
	TestingSettlementStartBlock      = 1
	TestingBatcherPrivateKey         = "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
	TestingSettlementChainID         = 84532
	TestingCutoverEpochBlock         = 10
)

type MockConfig struct {
	mock.Mock
}

var DefaultTestingConfig = &translator.CLIConfig{
	Port:                      TestingPort,
	FrameSize:                 TestingFrameSize,
	SequencingContractAddress: TestingSequencingContractAddress,
	SettlementStartBlock:      TestingSettlementStartBlock,
	BatcherPrivateKey:         TestingBatcherPrivateKey,
	SettlementChainRPCURL:     "http://localhost:8545",
	SequencingChainRPCURL:     "http://localhost:8545",
	MetaBasedChainRPCURL:      "http://localhost:8545",
	MetafillerURL:             "http://localhost:8080",
	BatchInboxAddress:         TestingBatchInboxAddress,
	SettlementChainID:         TestingSettlementChainID,
	CutoverEpochBlock:         TestingCutoverEpochBlock,
}
