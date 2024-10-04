package config

import (
	"errors"
	"strings"
	"testing"

	"github.com/SyndicateProtocol/op-translator/internal/constants"
	"github.com/hashicorp/go-multierror"
	"github.com/stretchr/testify/assert"
)

func validConfig() Config {
	return Config{
		port:                       1234,
		frameSize:                  100,
		sequencingChainAddr:        "http://example.com",
		settlementChainAddr:        "https://example.org",
		metaBasedChainAddr:         "https://example.io",
		logLevel:                   constants.Info.String(),
		pretty:                     false,
		sequencingContractAddress:  "0x0000000000000000000000000000000000000000",
		batcherAddress:             "0x0000000000000000000000000000000000000000",
		batchInboxAddress:          "0x0000000000000000000000000000000000000000",
		settlementStartBlock:       1,
		sequencingStartBlock:       2,
		sequencePerSettlementBlock: 2,
	}
}

func TestValidateConfigValues(t *testing.T) {
	tests := []struct {
		configChangeUnderTest func(*Config)
		name                  string
		expectedErrors        []string
	}{
		{
			name:                  "Valid config",
			configChangeUnderTest: func(c *Config) {},
			expectedErrors:        nil,
		},
		{
			name: "Invalid port",
			configChangeUnderTest: func(c *Config) {
				c.port = 0
			},
			expectedErrors: []string{"port must be a positive number"},
		},
		{
			name: "Invalid frameSize",
			configChangeUnderTest: func(c *Config) {
				c.frameSize = 0
			},
			expectedErrors: []string{"frameSize must be a positive number"},
		},
		{
			name: "Invalid sequencingChainAddr",
			configChangeUnderTest: func(c *Config) {
				c.sequencingChainAddr = "invalid sequencing chain address"
			},
			expectedErrors: []string{"invalid URL for sequencing chain address"},
		},
		{
			name: "Invalid settlementChainAddr",
			configChangeUnderTest: func(c *Config) {
				c.settlementChainAddr = "invalid settlement chain address"
			},
			expectedErrors: []string{"invalid URL for settlement chain address"},
		},
		{
			name: "Invalid metaBasedChainAddr",
			configChangeUnderTest: func(c *Config) {
				c.metaBasedChainAddr = "invalid meta based chain address"
			},
			expectedErrors: []string{"invalid URL for meta based chain address"},
		},
		{
			name: "Invalid log level",
			configChangeUnderTest: func(c *Config) {
				c.logLevel = "OTHER"
			},
			expectedErrors: []string{"invalid log level"},
		},
		{
			name: "Invalid settlementStartBlock",
			configChangeUnderTest: func(c *Config) {
				c.settlementStartBlock = 0
			},
			expectedErrors: []string{"settlementStartBlock must be a positive number"},
		},
		{
			name: "Invalid sequencingStartBlock",
			configChangeUnderTest: func(c *Config) {
				c.sequencingStartBlock = 0
			},
			expectedErrors: []string{"sequencingStartBlock must be a positive number"},
		},
		{
			name: "Invalid sequencePerSettlementBlock",
			configChangeUnderTest: func(c *Config) {
				c.sequencePerSettlementBlock = 0
			},
			expectedErrors: []string{"sequencePerSettlementBlock must be a positive number"},
		},
		{
			name: "Invalid sequencingContractAddress",
			configChangeUnderTest: func(c *Config) {
				c.sequencingContractAddress = "invalid sequencing contract address"
			},
			expectedErrors: []string{"sequencingContractAddress must be a valid hex address"},
		},
		{
			name: "Invalid batcherAddress",
			configChangeUnderTest: func(c *Config) {
				c.batcherAddress = "invalid batcher address"
			},
			expectedErrors: []string{"batcherAddress must be a valid hex address"},
		},
		{
			name: "Invalid batchInboxAddress",
			configChangeUnderTest: func(c *Config) {
				c.batchInboxAddress = "invalid batch inbox address"
			},
			expectedErrors: []string{"batchInboxAddress must be a valid hex address"},
		},
		{
			name: "Multiple invalid fields",
			configChangeUnderTest: func(c *Config) {
				c.port = -1
				c.frameSize = 0
				c.sequencingChainAddr = "invalid"
				c.settlementChainAddr = "also invalid"
				c.metaBasedChainAddr = "https://example.io"
				c.sequencePerSettlementBlock = 0
			},
			expectedErrors: []string{
				"port must be a positive number",
				"frameSize must be a positive number",
				"invalid URL for sequencing chain address",
				"invalid URL for settlement chain address",
				"sequencePerSettlementBlock must be a positive number",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := validConfig()
			tt.configChangeUnderTest(&cfg)
			result := validateConfigValues(&cfg)

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
						found := false
						for _, err := range merr.Errors {
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
