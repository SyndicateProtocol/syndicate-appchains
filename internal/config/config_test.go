package config

import (
	"errors"
	"fmt"
	"strings"
	"testing"

	"github.com/hashicorp/go-multierror"
	"github.com/stretchr/testify/assert"
)

func TestValidateConfigValues(t *testing.T) {
	tests := []struct {
		name           string
		expectedErrors []string
		config         Config
	}{
		{
			name: "Valid config",
			config: Config{
				port:                1234,
				frameSize:           100,
				sequencingChainAddr: "http://example.com",
				settlementChainAddr: "https://example.org",
			},
			expectedErrors: nil,
		},
		{
			name: "Invalid port",
			config: Config{
				port:                0,
				frameSize:           100,
				sequencingChainAddr: "http://example.com",
				settlementChainAddr: "https://example.org",
			},
			expectedErrors: []string{"port must be a positive number"},
		},
		{
			name: "Invalid frameSize",
			config: Config{
				port:                1234,
				frameSize:           0,
				sequencingChainAddr: "http://example.com",
				settlementChainAddr: "https://example.org",
			},
			expectedErrors: []string{"frameSize must be a positive number"},
		},
		{
			name: "Invalid sequencingChainAddr",
			config: Config{
				port:                1234,
				frameSize:           100,
				sequencingChainAddr: "not a valid url",
				settlementChainAddr: "https://example.org",
			},
			expectedErrors: []string{"invalid URL for sequencing chain address"},
		},
		{
			name: "Invalid settlementChainAddr",
			config: Config{
				port:                1234,
				frameSize:           100,
				sequencingChainAddr: "http://example.com",
				settlementChainAddr: "not a valid url",
			},
			expectedErrors: []string{"invalid URL for settlement chain address"},
		},
		{
			name: "Multiple invalid fields",
			config: Config{
				port:                -1,
				frameSize:           0,
				sequencingChainAddr: "invalid",
				settlementChainAddr: "also invalid",
			},
			expectedErrors: []string{
				"port must be a positive number",
				"frameSize must be a positive number",
				"invalid URL for sequencing chain address",
				"invalid URL for settlement chain address",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := validateConfigValues(tt.config)

			if tt.expectedErrors == nil {
				assert.NoError(t, result, "Expected no error, but got: %v", result)
			} else {
				assert.Error(t, result, "Expected an error, but got none")

				var merr *multierror.Error
				ok := errors.As(result, &merr)
				assert.True(t, ok, "Expected a multierror.Error, but got: %T", result)

				if ok {
					assert.Equal(t, len(tt.expectedErrors), len(merr.Errors), "Expected %d errors, but got %d", len(tt.expectedErrors), len(merr.Errors))

					for _, expectedErr := range tt.expectedErrors {
						fmt.Println(expectedErr)
						found := false
						for _, err := range merr.Errors {
							fmt.Println(err.Error())
							if strings.Contains(err.Error(), expectedErr) {
								found = true
								break
							}
						}
						assert.True(t, found, "Expected error '%s' not found in result", expectedErr)
					}
				}
			}
		})
	}
}
