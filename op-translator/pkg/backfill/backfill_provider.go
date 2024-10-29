package backfill

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
)

// TODO SEQ-141: spike: performant Go HTTP/JSON-RPC lib
type HTTPClient interface {
	Do(req *http.Request) (*http.Response, error)
}

type BackfillProvider struct {
	Client        HTTPClient
	MetafillerURL string
}

func NewBackfillerProvider(cfg *config.Config) *BackfillProvider {
	return &BackfillProvider{
		MetafillerURL: cfg.MetafillerURL,
		Client:        &http.Client{},
	}
}

type BackfillData struct {
	Data      []string    `json:"data"` // Hex format
	EpochHash common.Hash `json:"epochHash"`
}

func (b *BackfillProvider) GetBackfillData(ctx context.Context, epochNumber string) (*BackfillData, error) {
	intEpochNumber, err := utils.HexToInt(epochNumber)
	if err != nil {
		return nil, err
	}
	fullURL := b.MetafillerURL + "/" + strconv.Itoa(intEpochNumber)
	log.Debug().Msgf("Getting backfill data for epoch %s. Int epoch number: %d. Fetching from: %s", epochNumber, intEpochNumber, fullURL)

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
	log.Debug().Msgf("Backfill data for epoch %d: %v", intEpochNumber, data)
	return data, nil
}

func (b *BackfillProvider) GetBackfillFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	epochNumber, err := block.GetBlockNumber()
	if err != nil {
		return nil, err
	}

	epochHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	backfillData, err := b.GetBackfillData(ctx, epochNumber)
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data for epoch %s: %w", epochNumber, err)
	}

	if backfillData.EpochHash != common.HexToHash(epochHash) {
		return nil, fmt.Errorf("epoch hash mismatch: %s != %s", backfillData.EpochHash, epochHash)
	}

	frames := make([]*types.Frame, 0, len(backfillData.Data))
	for _, data := range backfillData.Data {
		frame, err := types.ToFrames([]byte(data), config.MaxFrameSize, backfillData.EpochHash)
		if err != nil {
			return nil, fmt.Errorf("failed to convert data to frames for epoch %s: %w", epochNumber, err)
		}
		frames = append(frames, frame...)
	}

	return frames, nil
}
