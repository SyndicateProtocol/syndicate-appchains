package backfill_test

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"log/slog"
	"net/http"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestGetBackfillFramesMultipleCases(t *testing.T) {
	mockMetrics := mocks.NewMockMetrics()

	tests := []struct {
		mockResponseData   *backfill.BackfillData
		block              types.Block
		name               string
		expectedFrameCount int
		expectedStatusCode int
		expectedErr        bool
	}{
		{
			name: "Single Frame - Matching Epoch Hash",
			mockResponseData: &backfill.BackfillData{
				Data:      []string{"0x1234"},
				EpochHash: common.HexToHash("0x123"),
			},
			block: types.Block{
				"number":       "0x1",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 1,
			expectedStatusCode: http.StatusOK,
			expectedErr:        false,
		},
		{
			name: "Multiple Frames - Matching Epoch Hash",
			mockResponseData: &backfill.BackfillData{
				Data:      []string{"0x1234", "0x1234", "0x1234"},
				EpochHash: common.HexToHash("0x123"),
			},
			block: types.Block{
				"number":       "0x1",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 3,
			expectedErr:        false,
			expectedStatusCode: http.StatusOK,
		},
		{
			name: "Epoch Hash Mismatch",
			mockResponseData: &backfill.BackfillData{
				Data:      []string{"data"},
				EpochHash: common.HexToHash("0x456"),
			},
			block: types.Block{
				"number":       "0x1",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 0,
			expectedErr:        true,
			expectedStatusCode: http.StatusOK,
		},
		{
			name: "No Data in Backfill",
			mockResponseData: &backfill.BackfillData{
				Data:      []string{},
				EpochHash: common.HexToHash("0x123"),
			},
			block: types.Block{
				"number":       "0x1",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 0,
			expectedErr:        false,
			expectedStatusCode: http.StatusOK,
		},
		{
			name:             "Non-200 Status Code - Genesis Block",
			mockResponseData: nil,
			block: types.Block{
				"number":       "0x0",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 0,
			expectedErr:        false,
			expectedStatusCode: http.StatusNotFound,
		},
		{
			name:             "Non-200 Status Code - Non-Genesis Block",
			mockResponseData: nil,
			block: types.Block{
				"number":       "0x01",
				"hash":         "0x123",
				"transactions": []any{},
			},
			expectedFrameCount: 0,
			expectedErr:        true,
			expectedStatusCode: http.StatusNotFound,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockHTTPClient := new(mocks.HTTPClientMock)
			var body io.ReadCloser
			if tt.mockResponseData == nil {
				body = io.NopCloser(nil)
			} else {
				jsonData, err := json.Marshal(tt.mockResponseData)
				assert.NoError(t, err)
				body = io.NopCloser(bytes.NewBuffer(jsonData))
			}
			mockHTTPClient.On("Do", mock.Anything).Return(&http.Response{
				StatusCode: tt.expectedStatusCode,
				Body:       body,
				Header:     make(http.Header),
			}, nil)

			ctx := context.Background()
			backfillProvider := backfill.NewBackfillerProvider(
				"http://metafiller.io",
				0,
				0,
				mockHTTPClient,
				mockMetrics,
				testlog.Logger(t, slog.LevelDebug),
			)

			frames, err := backfillProvider.GetBackfillFrames(ctx, tt.block)
			if tt.expectedErr {
				assert.Error(t, err)
				assert.Nil(t, frames)
			} else {
				assert.NoError(t, err)
				assert.Len(t, frames, tt.expectedFrameCount)

				for i, frame := range frames {
					assert.NotNil(t, frame.ID)
					mockDataBytes, err := utils.DecodeHexString(tt.mockResponseData.Data[i])
					assert.NoError(t, err)
					assert.Equal(t, mockDataBytes, frame.Data)
				}
			}
			mockHTTPClient.AssertExpectations(t)
		})
	}
}
