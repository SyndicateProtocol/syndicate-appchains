package mocks

import (
	"context"

	"github.com/stretchr/testify/mock"
)

type Translator struct {
	mock.Mock
}

func (m *Translator) GetBlockByNumber(ctx context.Context, blockNumber string, fullTx bool) (map[string]any, error) {
	// TODO (SEQ-87): implement me if relevant
	result := map[string]any{"block": "0x123"}
	return result, nil
}

func (m *Translator) GetBlockByHash(ctx context.Context, blockHash string, fullTx bool) (map[string]any, error) {
	// TODO (SEQ-88): implement me if relevant
	result := map[string]any{"block": "0x123"}
	return result, nil
}
