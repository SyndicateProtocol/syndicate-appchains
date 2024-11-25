package mocks

import (
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockSigner struct {
	mock.Mock
}

func (m *MockSigner) NewSigner(cfg *translator.CLIConfig) {}
func (m *MockSigner) Sign(data []byte) ([]byte, error) {
	args := m.Called(data)
	return args.Get(0).([]byte), args.Error(1)
}

func (m *MockSigner) ChainID() int64 {
	args := m.Called()
	return args.Get(0).(int64)
}

func TestSigner(t *testing.T) *translator.Signer {
	signer, err := translator.NewSigner(TestingBatcherPrivateKey, big.NewInt(TestingSettlementChainID))
	assert.NoError(t, err)
	return signer
}
