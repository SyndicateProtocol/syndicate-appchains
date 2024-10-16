package interfaces

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
)

type IBatchProvider interface {
	GetBatch(ctx context.Context, block types.Block) (*types.Batch, error)
	Close()
}
