package interfaces

import (
	"context"
)

type IRawRPCClient interface {
	CallContext(ctx context.Context, result any, method string, args ...any) error
}
