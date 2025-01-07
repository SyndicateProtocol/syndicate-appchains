package mocks

import "github.com/stretchr/testify/mock"

// Args0 returns the first argument of a mock call as a typed value. (used to avoid ignoring the "errcheck" lint everywhere)
func Args0[T any](args mock.Arguments) T {
	return args.Get(0).(T) //nolint:errcheck // mock safe cast
}
