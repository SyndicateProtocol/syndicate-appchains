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
	Client            HTTPClient
	MetafillerURL     string
	GenesisEpochBlock uint64
	CutoverEpochBlock uint64
}

func NewBackfillerProvider(cfg *config.Config) *BackfillProvider {
	return &BackfillProvider{
		MetafillerURL:     cfg.MetafillerURL,
		Client:            &http.Client{},
		GenesisEpochBlock: uint64(cfg.SettlementStartBlock), //nolint:gosec // We validate the genesis block in the config package
		CutoverEpochBlock: uint64(cfg.CutoverEpochBlock),    //nolint:gosec // We validate the cutover block in the config package
	}
}

type BackfillData struct {
	Data      []string    `json:"data"` // Hex format
	EpochHash common.Hash `json:"epochHash"`
}

func (b *BackfillProvider) GetBackfillData(ctx context.Context, epochNumber uint64) (*BackfillData, error) {
	fullURL := b.MetafillerURL + "/" + strconv.FormatUint(epochNumber, 10)
	log.Debug().Msgf("Getting backfill data for epoch number: %d. Fetching from: %s", epochNumber, fullURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, fullURL, http.NoBody)
	if err != nil {
		return nil, err
	}

	// metafiller response status
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
	log.Debug().Msgf("Backfill data for epoch %d: %v", epochNumber, data)
	return data, nil
}

func (b *BackfillProvider) IsBlockInBackfillingWindow(block types.Block) bool {
	epochNumber, err := block.GetBlockNumber()
	if err != nil {
		return false
	}
	return epochNumber < b.CutoverEpochBlock
}

func (b *BackfillProvider) GetBackfillFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	epochNumber, err := block.GetBlockNumber()
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data - invalid block number: %w", err)
	}

	if epochNumber == b.GenesisEpochBlock {
		log.Debug().Msgf("Block number is genesis block, not backfilling, %d", epochNumber)
		return []*types.Frame{}, nil
	}

	epochHash, err := block.GetBlockHash()
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data - invalid block hash: %w", err)
	}

	// metafiller latency
	backfillData, err := b.GetBackfillData(ctx, epochNumber)
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data for epoch %d: %w", epochNumber, err)
	}

	if backfillData.EpochHash != common.HexToHash(epochHash) {
		return nil, fmt.Errorf("epoch hash mismatch: %s != %s", backfillData.EpochHash, epochHash)
	}

	frames := make([]*types.Frame, 0, len(backfillData.Data))
	for _, data := range backfillData.Data {
		byteData, err := utils.DecodeHexString(data)
		if err != nil {
			return nil, fmt.Errorf("failed to decode hex string: %s. error: %w", data, err)
		}
		frame, err := types.ToFrames(byteData, config.MaxFrameSize, backfillData.EpochHash)
		if err != nil {
			return nil, fmt.Errorf("failed to convert data to frames for epoch %d: %w", epochNumber, err)
		}
		frames = append(frames, frame...)
	}

	return frames, nil
}
