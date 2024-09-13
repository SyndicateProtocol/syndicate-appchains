package mocks

import (
	"github.com/stretchr/testify/mock"
)

const TestingPort = 8080
const TestingFrameSize = 1024

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
