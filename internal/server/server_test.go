package server

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/SyndicateProtocol/op-translator/mocks"
	"github.com/stretchr/testify/assert"
)

func getMocks() (*mocks.MockConfig, *mocks.Translator) {
	return &mocks.MockConfig{}, &mocks.Translator{}
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
			req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(tt.body))
			gotMethod := parseMethod(req)
			assert.Equal(t, tt.wantMethod, gotMethod)
		})
	}
}

func TestHealthEndpoint(t *testing.T) {
	mockCfg, mockTranslator := getMocks()
	mockCfg.On("SettlementChainAddr").Return("http://localhost:8545")
	mockCfg.On("LogLevel").Return("info")

	router, err := TranslatorHandler(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest(http.MethodGet, "/health", http.NoBody)
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
	mockCfg.On("LogLevel").Return("info")

	mockBackend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, err := w.Write([]byte(`{"result": "success"}`))
		assert.NoError(t, err)
	}))
	defer mockBackend.Close()

	mockCfg.On("SettlementChainAddr").Return(mockBackend.URL)

	router, err := TranslatorHandler(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(`{"method": "eth_getBalance"}`))
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	resp := w.Result()
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]any
	err = json.NewDecoder(resp.Body).Decode(&response)
	assert.NoError(t, err)
	assert.Equal(t, "success", response["result"])
}

func TestTranslatedEndpoint(t *testing.T) {
	mockCfg, mockTranslator := getMocks()
	mockCfg.On("LogLevel").Return("info")
	mockCfg.On("SettlementChainAddr").Return("http://localhost:8545")

	router, err := TranslatorHandler(mockCfg, mockTranslator)
	assert.NoError(t, err)

	req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(`{"id": 1, "jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": ["0x123", false]}`))
	req.Header.Set("Content-Type", "application/json")

	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	resp := w.Result()
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)

	var response map[string]any
	err = json.NewDecoder(resp.Body).Decode(&response)
	assert.NoError(t, err)
	assert.Equal(t, "0x123", response["result"].(map[string]any)["block"])
}
