package mocks

import (
	"github.com/stretchr/testify/mock"
)

type MockMetrics struct {
	mock.Mock
}

// OP Translator

func (m *MockMetrics) RecordOPTranslatorRPCRequest(method string) {
	m.Called(method)
}

func (m *MockMetrics) RecordOPTranslatorTranslationLatency(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockMetrics) RecordOPTranslatorError(method, errorType string) {
	m.Called(method, errorType)
}

// Batch Provider

func (m *MockMetrics) RecordBatchProviderBatchProcessed(method string) {
	m.Called(method)
}

func (m *MockMetrics) RecordBatchProviderBatchProcessingDuration(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockMetrics) RecordBatchProviderError(method, errorType string) {
	m.Called(method, errorType)
}

func (m *MockMetrics) RecordBatchProviderInvalidTransactionsCount(state string, count int) {
	m.Called(state, count)
}
