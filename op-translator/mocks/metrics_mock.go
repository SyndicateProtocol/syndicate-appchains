package mocks

import (
	"github.com/stretchr/testify/mock"
)

type MockMetrics struct {
	mock.Mock
}

func (m *MockMetrics) RecordRPCRequest(method string) {
	m.Called(method)
}

func (m *MockMetrics) RecordTranslationLatency(method string, duration float64) {
	m.Called(method, duration)
}

func (m *MockMetrics) RecordError(method, errorType string) {
	m.Called(method, errorType)
}
