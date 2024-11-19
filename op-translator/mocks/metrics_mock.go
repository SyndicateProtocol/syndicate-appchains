package mocks

import (
	"github.com/stretchr/testify/mock"
)

type MockBatchProviderMetrics struct {
	mock.Mock
}

func (m *MockBatchProviderMetrics) RecordBatchProcessed(method string) {
	m.Called(method)
}

func (m *MockBatchProviderMetrics) RecordBatchProcessingDuration(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockBatchProviderMetrics) RecordError(method, errorType string) {
	m.Called(method, errorType)
}

func (m *MockBatchProviderMetrics) RecordInvalidTransactionsCount(state string, count int) {
	m.Called(state, count)
}

type MockOpTranslatorMetrics struct {
	mock.Mock
}

func (m *MockOpTranslatorMetrics) RecordRPCRequest(method string) {
	m.Called(method)
}

func (m *MockOpTranslatorMetrics) RecordTranslationLatency(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockOpTranslatorMetrics) RecordError(method, errorType string) {
	m.Called(method, errorType)
}
