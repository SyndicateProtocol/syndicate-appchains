package rpcclient

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewSettlementClient(t *testing.T) {
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
			client, err := NewSettlementClient(tt.address)

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
