package mocks

import (
	"net/http"

	"github.com/stretchr/testify/mock"
)

type HTTPClientMock struct {
	mock.Mock
}

func (m *HTTPClientMock) Do(req *http.Request) (*http.Response, error) {
	args := m.Called(req)
	return args.Get(0).(*http.Response), args.Error(1)
}
