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
	return Args0[[]byte](args), args.Error(1)
}

func (m *MockSigner) ChainID() int64 {
	args := m.Called()
	return Args0[int64](args)
}

func TestSigner(t *testing.T) *translator.Signer {
	signer, err := translator.NewSigner(TestingBatcherPrivateKey, big.NewInt(TestingSettlementChainID))
	assert.NoError(t, err)
	return signer
}
