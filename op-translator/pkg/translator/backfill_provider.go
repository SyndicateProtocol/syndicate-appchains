package translator

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
)

type BackFillProvider struct {
	Client        *http.Client
	MetafillerURL string
}

func InitBackFillerProvider(cfg *config.Config) *BackFillProvider {
	return &BackFillProvider{
		MetafillerURL: cfg.MetafillerURL,
		Client:        &http.Client{},
	}
}

func (b *BackFillProvider) GetBackFillBatch(blockNumber string) (*types.Batch, error) {
	fullURL := b.MetafillerURL + "/" + blockNumber
	resp, err := b.Client.Get(fullURL)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var batch types.Batch
	if err := json.Unmarshal(body, &batch); err != nil {
		return nil, err
	}

	return &batch, err
}
