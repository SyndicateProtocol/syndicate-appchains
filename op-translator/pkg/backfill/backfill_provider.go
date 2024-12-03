package backfill

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	gethlog "github.com/ethereum/go-ethereum/log"
)

// TODO SEQ-141: spike: performant Go HTTP/JSON-RPC lib
type HTTPClient interface {
	Do(req *http.Request) (*http.Response, error)
}

type BackfillProvider struct {
	client            HTTPClient
	metrics           metrics.IMetrics
	log               gethlog.Logger
	metafillerURL     string
	genesisEpochBlock uint64
	cutoverEpochBlock uint64
}

func NewBackfillerProvider(
	metafillerURL string,
	genesisEpochBlock uint64, // SettlementStartBlock (?)
	cutoverEpochBlock uint64,
	client HTTPClient,
	metricsCollector metrics.IMetrics,
	log gethlog.Logger,
) *BackfillProvider {
	return &BackfillProvider{
		metafillerURL:     metafillerURL,
		client:            client,
		genesisEpochBlock: genesisEpochBlock,
		cutoverEpochBlock: cutoverEpochBlock,
		metrics:           metricsCollector,
		log:               log,
	}
}

type BackfillData struct {
	Data      []string    `json:"data"` // Hex format
	EpochHash common.Hash `json:"epochHash"`
}

func (b *BackfillProvider) GetBackfillData(ctx context.Context, epochNumber uint64) (*BackfillData, error) {
	start := time.Now()
	defer func() {
		duration := time.Since(start).Seconds()
		b.metrics.RecordBackfillProviderBackfillDuration("get_backfill_data", duration)
	}()

	fullURL := b.metafillerURL + "/" + strconv.FormatUint(epochNumber, 10)
	b.log.Debug("getting backfill data", "epoch_number", epochNumber, "url", fullURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, fullURL, http.NoBody)
	if err != nil {
		return nil, err
	}

	resp, err := b.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		b.metrics.RecordBackfillProviderBackfillResponseStatus("get_backfill_data", resp.StatusCode)
		b.log.Debug("received non-200 response from backfill data provider", "status", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var data *BackfillData
	err = json.Unmarshal(body, &data)
	if err != nil {
		return nil, err
	}
	b.log.Debug("backfill data", "epoch_number", epochNumber, "data", data)
	return data, nil
}

func (b *BackfillProvider) IsBlockInBackfillingWindow(block types.Block) bool {
	epochNumber, err := block.GetBlockNumber()
	if err != nil {
		return false
	}
	return epochNumber < b.cutoverEpochBlock
}

func (b *BackfillProvider) GetBackfillFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	epochNumber, err := block.GetBlockNumber()
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data - invalid block number: %w", err)
	}

	if epochNumber == b.genesisEpochBlock {
		b.log.Debug("block number is genesis block, not backfilling", "epoch_number", epochNumber)
		return []*types.Frame{}, nil
	}

	epochHash, err := block.GetBlockHash()
	if err != nil {
		return nil, fmt.Errorf("failed to get backfill data - invalid block hash: %w", err)
	}

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
		frame, err := types.ToFrames(byteData, flags.MaxFrameSize, backfillData.EpochHash)
		if err != nil {
			return nil, fmt.Errorf("failed to convert data to frames for epoch %d: %w", epochNumber, err)
		}
		frames = append(frames, frame...)
	}

	return frames, nil
}
