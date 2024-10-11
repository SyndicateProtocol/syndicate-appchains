package mocks

import (
	"github.com/SyndicateProtocol/op-translator/internal/config"
	"github.com/stretchr/testify/mock"
)

type MockSigner struct {
	mock.Mock
}

func (m *MockSigner) NewSigner(cfg config.IConfig) {}
func (m *MockSigner) Sign(data []byte) ([]byte, error) {
	args := m.Called(data)
	return args.Get(0).([]byte), args.Error(1)
}

func (m *MockSigner) ChainID() int64 {
	args := m.Called()
	return args.Get(0).(int64)
}
