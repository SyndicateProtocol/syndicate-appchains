package mocks

import (
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/stretchr/testify/mock"
)

type MockReceiptsFetcher struct {
	mock.Mock
}

func (m *MockReceiptsFetcher) PickReceiptsMethod(txCount int) sources.ReceiptsFetchingMethod {
	args := m.Called(txCount)
	return args.Get(0).(sources.ReceiptsFetchingMethod)
}

func (m *MockReceiptsFetcher) OnReceiptsMethodErr(method sources.ReceiptsFetchingMethod, err error) {
	m.Called(method, err)
}
