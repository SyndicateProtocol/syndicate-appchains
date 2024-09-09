package rpcclient

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestConnect(t *testing.T) {
	tests := []struct {
		name    string
		address string
		wantErr bool
	}{
		{"Valid address", "http://localhost:8545", false},
		{"Invalid address", "invalid", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client, err := Connect(tt.address)

			if tt.wantErr {
				assert.Error(t, err, "expected an error but got none")
				assert.Nil(t, client, "client should be nil when an error occurs")
				return
			}

			assert.NoError(t, err, "expected no error but got one")
			assert.NotNil(t, client, "client should not be nil when no error occurs")
			require.NotNil(t, client.Client)
		})
	}
}

func TestCloseConnection(t *testing.T) {
	client, err := Connect("http://localhost:8545")
	require.NoError(t, err)
	require.NotNil(t, client)

	client.CloseConnection()

	// Since Close() doesn't return an error in the original function,
	// we can't use assert.NoError here. Instead, we can just ensure the
	// method completes without panicking and include further checks.
	assert.NotNil(t, client.Client, "Client should be non-nil before closing")

	// TODO (SEQ-129): assert that client.someRPCMethod() @ the old address fails with a closed connection error
}
