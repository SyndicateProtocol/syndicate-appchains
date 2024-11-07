package config_test

import (
	"errors"
	"strings"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/hashicorp/go-multierror"
	"github.com/stretchr/testify/assert"
)

func validConfig() config.Config {
	return config.Config{
		Port:                      1234,
		FrameSize:                 100,
		SequencingChainRPCURL:     "http://example.com",
		SettlementChainRPCURL:     "https://example.org",
		MetaBasedChainRPCURL:      "https://example.io",
		MetafillerURL:             "https://metafiller.io",
		LogLevel:                  constants.Info.String(),
		Pretty:                    false,
		SequencingContractAddress: "0x0000000000000000000000000000000000000000",
		BatcherAddress:            "0x0000000000000000000000000000000000000000",
		BatchInboxAddress:         "0x0000000000000000000000000000000000000000",
		SettlementStartBlock:      1,
		BatcherPrivateKey:         "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d",
		SettlementChainID:         84532,
		SettlementChainBlockTime:  2,
	}
}

func TestValidateConfigValues(t *testing.T) {
	tests := []struct {
		configChangeUnderTest func(*config.Config)
		name                  string
		expectedErrors        []string
	}{
		{
			name:                  "Valid config",
			configChangeUnderTest: func(c *config.Config) {},
			expectedErrors:        nil,
		},
		{
			name: "Invalid port",
			configChangeUnderTest: func(c *config.Config) {
				c.Port = 0
			},
			expectedErrors: []string{"port must be a positive number"},
		},
		{
			name: "Invalid frameSize",
			configChangeUnderTest: func(c *config.Config) {
				c.FrameSize = 0
			},
			expectedErrors: []string{"frameSize must be a positive number"},
		},
		{
			name: "Invalid sequencingChainAddr",
			configChangeUnderTest: func(c *config.Config) {
				c.SequencingChainRPCURL = "invalid sequencing chain address"
			},
			expectedErrors: []string{"invalid URL for sequencing chain address"},
		},
		{
			name: "Invalid settlementChainAddr",
			configChangeUnderTest: func(c *config.Config) {
				c.SettlementChainRPCURL = "invalid settlement chain address"
			},
			expectedErrors: []string{"invalid URL for settlement chain address"},
		},
		{
			name: "Invalid metaBasedChainAddr",
			configChangeUnderTest: func(c *config.Config) {
				c.MetaBasedChainRPCURL = "invalid meta based chain address"
			},
			expectedErrors: []string{"invalid URL for meta based chain address"},
		},
		{
			name: "Invalid log level",
			configChangeUnderTest: func(c *config.Config) {
				c.LogLevel = "OTHER"
			},
			expectedErrors: []string{"invalid log level"},
		},
		{
			name: "Invalid settlementStartBlock",
			configChangeUnderTest: func(c *config.Config) {
				c.SettlementStartBlock = 0
			},
			expectedErrors: []string{"settlementStartBlock must be a positive number"},
		},
		{
			name: "Invalid sequencingContractAddress",
			configChangeUnderTest: func(c *config.Config) {
				c.SequencingContractAddress = "invalid sequencing contract address"
			},
			expectedErrors: []string{"sequencingContractAddress must be a valid hex address"},
		},
		{
			name: "Invalid batcherAddress",
			configChangeUnderTest: func(c *config.Config) {
				c.BatcherAddress = "invalid batcher address"
			},
			expectedErrors: []string{"batcherAddress must be a valid hex address"},
		},
		{
			name: "Invalid batchInboxAddress",
			configChangeUnderTest: func(c *config.Config) {
				c.BatchInboxAddress = "invalid batch inbox address"
			},
			expectedErrors: []string{"batchInboxAddress must be a valid hex address"},
		},
		{
			name: "Multiple invalid fields",
			configChangeUnderTest: func(c *config.Config) {
				c.Port = -1
				c.FrameSize = 0
				c.SequencingChainRPCURL = "invalid"
				c.SettlementChainRPCURL = "also invalid"
				c.MetaBasedChainRPCURL = "https://example.io"
			},
			expectedErrors: []string{
				"port must be a positive number",
				"frameSize must be a positive number",
				"invalid URL for sequencing chain address",
				"invalid URL for settlement chain address",
			},
		},
		{
			name: "Invalid Batcher Private Key",
			configChangeUnderTest: func(c *config.Config) {
				c.BatcherPrivateKey = ""
			},
			expectedErrors: []string{"batcherPrivateKey cannot be blank"},
		},
		{
			name: "Invalid Settlement Chain ID",
			configChangeUnderTest: func(c *config.Config) {
				c.SettlementChainID = 0
			},
			expectedErrors: []string{"settlementChainID must be a positive number: 0"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := validConfig()
			tt.configChangeUnderTest(&cfg)
			result := config.ValidateConfigValues(&cfg)

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
