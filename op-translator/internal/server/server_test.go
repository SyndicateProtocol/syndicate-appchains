package server_test

import (
	"bytes"
	"encoding/json"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/server"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/stretchr/testify/assert"
)

func TestParseMethod(t *testing.T) {
	tests := []struct {
		name          string
		body          string
		wantMethod    string
		expectedError string
	}{
		{"Valid method", `{"method": "eth_blockNumber"}`, "eth_blockNumber", ""},
		{"Invalid method type", `{"method": 123}`, "", "invalid method type"},
		{"Empty body", `{}`, "", "invalid method type"},
		{"Invalid JSON", `{"method": "eth_blockNumber"`, "", "error decoding JSON"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(tt.body))
			gotMethod, err := server.ParseMethod(req)
			if tt.expectedError != "" {
				assert.Error(t, err)
				assert.Contains(t, err.Error(), tt.expectedError)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.wantMethod, gotMethod)
			}
		})
	}
}

func TestHealthEndpoint(t *testing.T) {
	mockTranslator := &mocks.Translator{}

	router, err := server.TranslatorHandler(
		mocks.DefaultTestingConfig.SettlementChainRPCURL,
		mockTranslator,
		testlog.Logger(t, slog.LevelDebug),
	)
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
	mockTranslator := &mocks.Translator{}

	mockBackend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, err := w.Write([]byte(`{"result": "success"}`))
		assert.NoError(t, err)
	}))
	defer mockBackend.Close()

	router, err := server.TranslatorHandler(
		mockBackend.URL,
		mockTranslator,
		testlog.Logger(t, slog.LevelDebug),
	)
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
	mockTranslator := &mocks.Translator{}
	router, err := server.TranslatorHandler(
		mocks.DefaultTestingConfig.SettlementChainRPCURL,
		mockTranslator,
		testlog.Logger(t, slog.LevelDebug),
	)
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

func TestShouldTranslate(t *testing.T) {
	tests := []struct {
		method   string
		expected bool
	}{
		{"eth_getBlockByNumber", true},
		{"eth_getBlockByHash", true},
		{"eth_getTransactionByHash", false},
		{"eth_blockNumber", false},
	}

	for _, tt := range tests {
		t.Run(tt.method, func(t *testing.T) {
			result := server.ShouldTranslate(tt.method)
			assert.Equal(t, tt.expected, result)
		})
	}
}
