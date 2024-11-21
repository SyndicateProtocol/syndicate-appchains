package test

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/server"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/test/stubs"
	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/suite"
)

type IntegrationTestSuite struct {
	suite.Suite
}

func TestIntegrationSuite(t *testing.T) {
	suite.Run(t, new(IntegrationTestSuite))
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

func getMockClient() *mocks.MockRPCClient {
	mockClient := new(mocks.MockRPCClient)

	blockNumHex := "0xe730a8"
	blockHash := common.HexToHash("0x5214c19f0635af3e8c98ea12e3748d2cd8c20f933aa46b5de778f8a1ea6075c4")
	blockWithFullTxns := types.Block{
		"hash":         blockHash.String(),
		"number":       blockNumHex,
		"transactions": stubs.TransactionsBlock0xe730a8Full,
	}
	blockWithHashes := types.Block{
		"hash":         blockHash.String(),
		"number":       blockNumHex,
		"transactions": stubs.TransactionsBlock0xe730a8Hashes,
	}
	backfillHash := common.HexToHash("0x123")
	backfillBlockNumber := "0x2"
	backfillBlock := types.Block{
		"hash":         backfillHash.String(),
		"number":       backfillBlockNumber,
		"transactions": stubs.TransactionsBlock0xe730a8Full,
	}
	ctx := mock.Anything

	// Mock settlement block
	mockClient.On("GetBlockByNumber", ctx, blockNumHex, true).Return(blockWithFullTxns, nil)
	mockClient.On("GetBlockByNumber", ctx, blockNumHex, false).Return(blockWithHashes, nil)
	mockClient.On("GetBlockByHash", ctx, blockHash, true).Return(blockWithFullTxns, nil)
	mockClient.On("GetBlockByHash", ctx, blockHash, false).Return(blockWithHashes, nil)
	mockClient.On("GetBlockByNumber", ctx, backfillBlockNumber, true).Return(backfillBlock, nil)

	// Mock sequencing block
	seqBlockNumbers := []string{"0xe730a8"}
	mockClient.On("GetBlocksByNumbers", ctx, seqBlockNumbers, false).Return(blockWithHashes, nil)

	// Mock metabase block
	parentBlockNumHex := "0xe7309d"
	mockClient.On("GetBlockByNumber", ctx, parentBlockNumHex, false).Return(stubs.MetabasedBlock0xE7309D, nil)

	// Mock GetReceiptsByHashes
	senderAddr := common.HexToAddress("0x1234567890123456789012345678901234567890")
	encodedTxn := []byte{1, 2, 3, 4, 5}
	receipt := ethtypes.Receipt{
		Status: 1,
		Logs: []*ethtypes.Log{
			{
				Address: common.HexToAddress("0x123"),
				Topics: []common.Hash{
					translator.TransactionProcessedSigHash,
					common.BytesToHash(senderAddr.Bytes()),
				},
				Data: encodedTxn,
			},
		},
	}
	receipts := []*ethtypes.Receipt{&receipt}
	mockClient.On("GetReceiptsByHashes", ctx, stubs.TransactionsBlock0xe730a8Hashes).Return(receipts, nil)

	return mockClient
}

func getBackfillHTTPMock() *mocks.HTTPClientMock {
	mockHTTPClient := new(mocks.HTTPClientMock)
	jsonData, _ := json.Marshal(backfill.BackfillData{
		Data:      []string{"0x789c015100aeffb84f00f84ca0fea5a65ee8a5e073600da29f4651e0f631594f85b2b1e1ff3eb30b6716e1d4ec839d0070a0a423d903194b28bff3bf0949c9721a1c1cd1fda79d972ba7f9da94a674bf7bc38465c45dc3c0e86c2b7e"},
		EpochHash: common.HexToHash("0x123"),
	})
	mockHTTPClient.On("Do", mock.Anything).Return(&http.Response{
		StatusCode: http.StatusOK,
		Body:       io.NopCloser(bytes.NewBuffer(jsonData)),
		Header:     make(http.Header),
	}, nil)

	return mockHTTPClient
}

func TestOPNodeCalls(t *testing.T) {
	mockConfig := mocks.DefaultTestingConfig
	testCases := []struct {
		expectedResult   types.Block
		requestBody      string
		name             string
		expectedError    string
		expectedStatus   int
		expectedBackfill bool
	}{
		{
			name:             "eth_getBlockByNumber - Invalid request - missing params",
			requestBody:      `{"method": "eth_getBlockByNumber"}`,
			expectedStatus:   http.StatusOK,
			expectedError:    "invalid request",
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByNumber - Invalid request - missing params",
			requestBody:      `{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "id": 1}`,
			expectedStatus:   http.StatusOK,
			expectedError:    "missing value for required argument 0",
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByNumber - Valid request - specific block number",
			requestBody:      `{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": ["0xe730a8", true], "id": 1}`,
			expectedStatus:   http.StatusOK,
			expectedResult:   stubs.ExpectedBlock0xE7309D,
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByHash - Invalid request - missing params",
			requestBody:      `{"method": "eth_getBlockByHash"}`,
			expectedStatus:   http.StatusOK,
			expectedError:    "invalid request",
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByHash - Invalid request - missing params",
			requestBody:      `{"jsonrpc": "2.0", "method": "eth_getBlockByHash", "id": 1}`,
			expectedStatus:   http.StatusOK,
			expectedError:    "missing value for required argument 0",
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByHash - Valid request - specific block hash",
			requestBody:      `{"jsonrpc": "2.0", "method": "eth_getBlockByHash", "params": ["0x5214c19f0635af3e8c98ea12e3748d2cd8c20f933aa46b5de778f8a1ea6075c4", true], "id": 1}`,
			expectedStatus:   http.StatusOK,
			expectedResult:   stubs.ExpectedBlock0xE7309D,
			expectedBackfill: false,
		},
		{
			name:             "eth_getBlockByNumber - Valid request - backfill",
			requestBody:      `{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": ["0x2", true], "id": 1}`,
			expectedStatus:   http.StatusOK,
			expectedResult:   stubs.ExpectedBackfillBlock,
			expectedBackfill: true,
		},
	}

	for _, tc := range testCases {
		mockClient := getMockClient()
		mockHTTPBackfillClient := getBackfillHTTPMock()
		mockMetrics := mocks.NewMockMetrics()

		opTranslator := &translator.OPTranslator{
			SettlementChain:     mockClient,
			BatcherInboxAddress: common.HexToAddress("0x123"),
			BatcherAddress:      common.HexToAddress("0x123"),
			BatchProvider:       &mocks.MockBatchProvider{},
			Signer:              *translator.NewSigner(mockConfig),
			BackfillProvider: &backfill.BackfillProvider{
				MetafillerURL:     mockConfig.MetafillerURL,
				Client:            mockHTTPBackfillClient,
				GenesisEpochBlock: uint64(mockConfig.SettlementStartBlock), //nolint:gosec // precision loss is known
				CutoverEpochBlock: uint64(mockConfig.CutoverEpochBlock),    //nolint:gosec // precision loss is known
				Metrics:           mockMetrics,
			},
			Metrics: mockMetrics,
		}

		s, err := server.TranslatorHandler(mockConfig, opTranslator)
		assert.NoError(t, err)

		t.Run(tc.name, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(tc.requestBody))
			req.Header.Set("Content-Type", "application/json")
			response, err := postRequest(t, s, req)

			assert.NoError(t, err)
			assert.Equal(t, "2.0", response["jsonrpc"])

			if tc.expectedError != "" {
				errorData, ok := response["error"].(map[string]any)
				assert.True(t, ok)
				assert.Contains(t, errorData["message"], tc.expectedError)
			} else {
				assert.Equal(t, tc.expectedBackfill, len(mockHTTPBackfillClient.Calls) > 0)
				blockData, ok := response["result"].(map[string]any)
				assert.True(t, ok, "result is not a map[string]any")
				block := types.Block(blockData)

				blockHash, err := block.GetBlockHash()
				assert.NoError(t, err)
				assert.NotEmpty(t, blockHash)

				blockNumber, err := block.GetBlockNumber()
				assert.NoError(t, err)
				assert.NotEmpty(t, blockNumber)

				transactions, err := block.GetTransactions()
				assert.NoError(t, err)
				assert.NotNil(t, transactions)

				assert.Equal(t, tc.expectedResult["hash"], block["hash"])
				assert.Equal(t, tc.expectedResult["number"], block["number"])

				expectedTransactions, _ := tc.expectedResult["transactions"].([]any)
				actualTransactions, _ := block["transactions"].([]any)
				assert.Equal(t, len(expectedTransactions), len(actualTransactions))

				for i := range expectedTransactions {
					expectedTx, _ := expectedTransactions[i].(map[string]any)
					actualTx, _ := actualTransactions[i].(map[string]any)

					assert.Equal(t, expectedTx["gas"], actualTx["gas"])
					assert.Equal(t, expectedTx["hash"], actualTx["hash"])
					assert.Equal(t, expectedTx["mint"], actualTx["mint"])
					assert.Equal(t, expectedTx["nonce"], actualTx["nonce"])
					assert.Equal(t, expectedTx["r"], actualTx["r"])
					assert.Equal(t, expectedTx["s"], actualTx["s"])
					assert.Equal(t, expectedTx["to"], actualTx["to"])
					assert.Equal(t, expectedTx["type"], actualTx["type"])
					assert.Equal(t, expectedTx["v"], actualTx["v"])
					assert.Equal(t, expectedTx["value"], actualTx["value"])
					assert.Equal(t, expectedTx["maxFeePerGas"], actualTx["maxFeePerGas"])
					assert.Equal(t, expectedTx["maxPriorityFeePerGas"], actualTx["maxPriorityFeePerGas"])
				}
			}
		})
	}
}
