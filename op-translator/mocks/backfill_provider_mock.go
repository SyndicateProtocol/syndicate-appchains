package mocks

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/stretchr/testify/mock"
)

type BackfillProviderMock struct {
	mock.Mock
}

var _ translator.IBackfillProvider = (*BackfillProviderMock)(nil)

func NewMockBackfillProvider() *BackfillProviderMock {
	m := &BackfillProviderMock{}

	// always returns false
	m.On("IsBlockInBackfillingWindow", mock.Anything).Return(false)
	m.On("GetBackfillFrames", mock.Anything, mock.Anything).Return([]*types.Frame{}, nil)

	return m
}

func (b *BackfillProviderMock) GetBackfillFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	args := b.Called(ctx, block)
	return Args0[[]*types.Frame](args), args.Error(1)
}

func (b *BackfillProviderMock) IsBlockInBackfillingWindow(block types.Block) bool {
	args := b.Called(block)
	return Args0[bool](args)
}
