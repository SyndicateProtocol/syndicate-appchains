package server

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/SyndicateProtocol/op-translator/internal/config"

	"github.com/stretchr/testify/assert"
)

type MockTranslator struct{}

func (mt *MockTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, fullTx bool) (map[string]interface{}, error) {
	result := map[string]interface{}{"block": "0x123"}
	return result, nil
}

func getMocks() (*config.Config, *MockTranslator) {
	mockCfg := &config.Config{
		Port:                8080,
		SettlementChainAddr: "http://localhost:8545",
		SequencingChainAddr: "http://localhost:8545",
	}
	mockTranslator := &MockTranslator{}

	return mockCfg, mockTranslator
}

func TestParseMethod(t *testing.T) {
	tests := []struct {
		name       string
		body       string
		wantMethod string
	}{
		{"Valid method", `{"method": "eth_blockNumber"}`, "eth_blockNumber"},
		{"Invalid method type", `{"method": 123}`, ""},
		{"Empty body", `{}`, ""},
		{"Invalid JSON", `{"method": "eth_blockNumber"`, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest("POST", "/", bytes.NewBufferString(tt.body))
			gotMethod := parseMethod(req)
			assert.Equal(t, tt.wantMethod, gotMethod)
		})
	}
}

func TestHealthEndpoint(t *testing.T) {
	mockCfg, mockTranslator := getMocks()

	router, err := Init(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest("GET", "/health", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	resp := w.Result()
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]string
	err = json.NewDecoder(resp.Body).Decode(&response)
	assert.NoError(t, err)
	assert.Equal(t, "Healthy", response["status"])
}

func TestProxyEndpoint(t *testing.T) {
	mockCfg, mockTranslator := getMocks()

	mockBackend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"result": "success"}`))
	}))
	defer mockBackend.Close()

	mockCfg.SettlementChainAddr = mockBackend.URL

	router, err := Init(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest("POST", "/", bytes.NewBufferString(`{"method": "eth_getTransactionReceipt"}`))
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	resp := w.Result()
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]interface{}
	err = json.NewDecoder(resp.Body).Decode(&response)
	assert.NoError(t, err)
	assert.Equal(t, "success", response["result"])
}

func TestTranslatedEndpoint(t *testing.T) {
	mockCfg, mockTranslator := getMocks()

	router, err := Init(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest("POST", "/", bytes.NewBufferString(`{"id": 1, "jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": ["0x123", false]}`))
	req.Header.Set("Content-Type", "application/json")

	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	resp := w.Result()
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]interface{}
	err = json.NewDecoder(resp.Body).Decode(&response)
	assert.NoError(t, err)
	assert.Equal(t, "0x123", response["result"].(map[string]interface{})["block"])
}
