package mocks

import (
	"github.com/stretchr/testify/mock"
)

const TestingPort = 8080
const TestingFrameSize = 1024

const TestingSequencingContractAddress = "0x0000000000000000000000000000000000000000"
const TestingBatcherAddress = "0x0000000000000000000000000000000000000000"
const TestingBatchInboxAddress = "0x0000000000000000000000000000000000000000"
const TestingSettlementStartBlock = 1
const TestingSequencingStartBlock = 2
const TestingSequencePerSettlementBlock = 2

type MockConfig struct {
	mock.Mock
}

func (m *MockConfig) SettlementChainAddr() string {
	args := m.Called()
	return args.Get(0).(string)
}
func (m *MockConfig) SequencingChainAddr() string {
	args := m.Called()
	return args.Get(0).(string)
}
func (m *MockConfig) MetaBasedChainAddr() string {
	args := m.Called()
	return args.Get(0).(string)
}
func (m *MockConfig) Port() int {
	m.Called()
	// return args.Get(0).(int) // bring this back for more sophisticated testing
	return TestingPort
}
func (m *MockConfig) FrameSize() int {
	m.Called()
	// return args.Get(0).(int) // bring this back for more sophisticated testing
	return TestingFrameSize
}
func (m *MockConfig) LogLevel() string {
	args := m.Called()
	return args.Get(0).(string)
}
func (m *MockConfig) Pretty() bool {
	args := m.Called()
	return args.Get(0).(bool)
}

func (m *MockConfig) SequencingContractAddress() string {
	m.Called()
	return TestingSequencingContractAddress
}

func (m *MockConfig) BatcherAddress() string {
	m.Called()
	return TestingBatcherAddress
}

func (m *MockConfig) BatchInboxAddress() string {
	m.Called()
	return TestingBatchInboxAddress
}

func (m *MockConfig) SettlementStartBlock() int {
	m.Called()
	return TestingSettlementStartBlock
}

func (m *MockConfig) SequencingStartBlock() int {
	m.Called()
	return TestingSequencingStartBlock
}

func (m *MockConfig) SequencePerSettlementBlock() int {
	m.Called()
	return TestingSequencePerSettlementBlock
}
