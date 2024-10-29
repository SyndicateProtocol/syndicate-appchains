package backfill_test

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestGetBackfillFrames(t *testing.T) {
	mockHTTPClient := new(mocks.HTTPClientMock)
	jsonData, err := json.Marshal(backfill.BackfillData{
		Data:      []string{"data"},
		EpochHash: common.HexToHash("0x123"),
	})
	assert.NoError(t, err)
	mockHTTPClient.On("Do", mock.Anything).Return(&http.Response{
		StatusCode: http.StatusOK,
		Body:       io.NopCloser(bytes.NewBuffer(jsonData)),
		Header:     make(http.Header),
	}, nil)

	ctx := context.Background()
	backfillProvider := backfill.BackfillProvider{
		Client:        mockHTTPClient,
		MetafillerURL: "http://metafiller.io",
	}
	epoch := "0x1"
	frames, err := backfillProvider.GetBackfillFrames(ctx, epoch)

	assert.NoError(t, err)
	assert.Len(t, frames, 1)

	frame := frames[0]
	assert.NotNil(t, frame.ID)
	assert.Equal(t, uint16(0), frame.FrameNumber)
	assert.Equal(t, []byte("data"), frame.Data)
	assert.Equal(t, true, frame.IsLast)
}
