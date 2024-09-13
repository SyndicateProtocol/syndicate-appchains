package test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/SyndicateProtocol/op-translator/internal/server"
	"github.com/SyndicateProtocol/op-translator/internal/translator"
	"github.com/SyndicateProtocol/op-translator/mocks"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
)

type IntegrationTestSuite struct {
	suite.Suite
}

func TestIntegrationSuite(t *testing.T) {
	suite.Run(t, new(IntegrationTestSuite))
}

// Simple test that includes the 2 RPC endpoints which OP-node will hit
// TODO update this going forward
func TestOPNodeCalls(t *testing.T) {
	mockConfig := &mocks.MockConfig{}
	mockConfig.On("SettlementChainAddr").Return("http://localhost:8545")
	mockConfig.On("SequencingChainAddr").Return("http://localhost:8545")
	opTranslator := translator.Init(mockConfig)
	s, err := server.TranslatorHandler(mockConfig, opTranslator)
	assert.NoError(t, err)

	// eth_getBlockByNumber
	getBlockByNumberReq := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(`{"method": "eth_getBlockByNumber"}`))
	getBlockByNumberReq.Header.Set("Content-Type", "application/json")
	getBlockByNumberResponse, err := postRequest(t, s, getBlockByNumberReq)

	assert.NoError(t, err)
	assert.Equal(t, "2.0", getBlockByNumberResponse["jsonrpc"])
	assert.Equal(t, "invalid request", getBlockByNumberResponse["error"].(map[string]any)["message"])

	// eth_getBlockByHash
	getBlockByHashReq := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(`{"method": "eth_getBlockByHash"}`))
	getBlockByHashReq.Header.Set("Content-Type", "application/json")
	getBlockByHashResponse, err := postRequest(t, s, getBlockByHashReq)

	assert.NoError(t, err)
	assert.Equal(t, "2.0", getBlockByHashResponse["jsonrpc"])
	assert.Equal(t, "invalid request", getBlockByHashResponse["error"].(map[string]any)["message"])
}

func postRequest(t *testing.T, s *http.ServeMux, req *http.Request) (map[string]any, error) {
	t.Helper()
	w := httptest.NewRecorder()
	s.ServeHTTP(w, req)
	resp := w.Result()
	defer resp.Body.Close()
	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]any
	err := json.NewDecoder(resp.Body).Decode(&response)
	if err != nil {
		return nil, err
	}
	return response, err
}
