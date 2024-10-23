package translator

import (
	"context"
	"encoding/json"
	"io"
	"net/http"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
)

type BackfillProvider struct {
	Client        *http.Client
	MetafillerURL string
}

func NewBackfillerProvider(cfg *config.Config) *BackfillProvider {
	return &BackfillProvider{
		MetafillerURL: cfg.MetafillerURL,
		Client:        &http.Client{},
	}
}

type BackfillData struct {
	Data      string      `json:"data"` // Hex format
	EpochHash common.Hash `json:"epochHash"`
}

func (b *BackfillProvider) GetBackfillData(ctx context.Context, epochNumber string) (*BackfillData, error) {
	fullURL := b.MetafillerURL + "/" + epochNumber
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, fullURL, http.NoBody)
	if err != nil {
		return nil, err
	}

	resp, err := b.Client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var data *BackfillData
	err = json.Unmarshal(body, &data)
	if err != nil {
		return nil, err
	}
	return data, nil
}

func (b *BackfillProvider) GetBackfillFrames(ctx context.Context, epochNumber string) ([]*types.Frame, error) {
	backfillData, err := b.GetBackfillData(ctx, epochNumber)
	if err != nil {
		return nil, err
	}

	frames, err := types.ToFrames([]byte(backfillData.Data), config.MaxFrameSize, backfillData.EpochHash)
	if err != nil {
		return nil, err
	}

	return frames, nil
}
