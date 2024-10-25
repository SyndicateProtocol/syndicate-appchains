package translator

import (
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/stretchr/testify/assert"
)

var TestBlock = types.Block{
	"timestamp": "0x1",
}

func TestGetSequencingBlocks(t *testing.T) {
	fetcher := NewSequencingBlockFetcher(nil, 2)

	blocks, err := fetcher.GetSequencingBlocks(TestBlock)
	assert.NoError(t, err)
	// TODO [SEQ-242]: Implement getting sequencing blocks by timewindow
	assert.Empty(t, blocks)
}
