package mocks

import (
	"github.com/stretchr/testify/mock"
)

type MockMetrics struct {
	mock.Mock
}

func NewMockMetrics() *MockMetrics {
	m := &MockMetrics{}

	// OP Translator
	m.On("RecordOPTranslatorRPCRequest", mock.Anything).Return()
	m.On("RecordOPTranslatorTranslationLatency", mock.Anything, mock.Anything).Return()
	m.On("RecordOPTranslatorError", mock.Anything, mock.Anything).Return()

	// Batch Provider
	m.On("RecordBatchProviderBatchProcessed", mock.Anything).Return()
	m.On("RecordBatchProviderBatchProcessingDuration", mock.Anything, mock.Anything).Return()
	m.On("RecordBatchProviderError", mock.Anything, mock.Anything).Return()
	m.On("RecordBatchProviderInvalidTransactionsCount", mock.Anything, mock.Anything).Return()

	// Backfill Provider
	m.On("RecordBackfillProviderBackfillDuration", mock.Anything, mock.Anything).Return()
	m.On("RecordBackfillProviderBackfillResponseStatus", mock.Anything, mock.Anything).Return()
	m.On("RecordBackfillProviderBackfillingWindow", mock.Anything).Return()

	// RPC Client
	m.On("RecordRPCRequestDuration", mock.Anything, mock.Anything, mock.Anything).Return()
	m.On("RecordRPCRequestError", mock.Anything, mock.Anything, mock.Anything).Return()

	return m
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

// Backfill Provider

func (m *MockMetrics) RecordBackfillProviderBackfillDuration(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockMetrics) RecordBackfillProviderBackfillResponseStatus(method string, status int) {
	m.Called(method, status)
}

func (m *MockMetrics) RecordBackfillProviderBackfillingWindow(inWindow bool) {
	m.Called(inWindow)
}

// RPC Client

func (m *MockMetrics) RecordRPCRequestDuration(clientType string, method string, duration float64) {
	m.Called(clientType, method, duration)
}

func (m *MockMetrics) RecordRPCRequestError(clientType string, method string, errMsg string) {
	m.Called(clientType, method, errMsg)
}
