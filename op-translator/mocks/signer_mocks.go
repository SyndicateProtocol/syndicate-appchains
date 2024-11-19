package mocks

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/stretchr/testify/mock"
)

type MockSigner struct {
	mock.Mock
}

func (m *MockSigner) NewSigner(cfg *config.Config) {}
func (m *MockSigner) Sign(data []byte) ([]byte, error) {
	args := m.Called(data)
	return args.Get(0).([]byte), args.Error(1) //nolint:errcheck // mock safe cast
}

func (m *MockSigner) ChainID() int64 {
	args := m.Called()
	return args.Get(0).(int64) //nolint:errcheck // mock safe cast
}
