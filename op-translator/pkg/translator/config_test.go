package translator

import (
	"strings"
	"testing"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/stretchr/testify/assert"
)

func validConfig() CLIConfig {
	return CLIConfig{
		Port:                      1234,
		FrameSize:                 100,
		SequencingChainRPCURL:     "http://example.com",
		SettlementChainRPCURL:     "https://example.org",
		MetaBasedChainRPCURL:      "https://example.io",
		MetafillerURL:             "https://metafiller.io",
		LogLevel:                  constants.Info.String(),
		Pretty:                    false,
		SequencingContractAddress: "0x0000000000000000000000000000000000000000",
		BatchInboxAddress:         "0x0000000000000000000000000000000000000000",
		SettlementStartBlock:      1,
		BatcherPrivateKey:         "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d",
		SettlementChainID:         84532,
		SettlementChainBlockTime:  2 * time.Second,
	}
}

func TestValidateConfigValues(t *testing.T) {
	tests := []struct {
		configChangeUnderTest func(*CLIConfig)
		name                  string
		expectedErrors        []string
	}{
		{
			name:                  "Valid config",
			configChangeUnderTest: func(c *CLIConfig) {},
			expectedErrors:        nil,
		},
		{
			name: "Invalid port",
			configChangeUnderTest: func(c *CLIConfig) {
				c.Port = 0
			},
			expectedErrors: []string{"port must be a positive number"},
		},
		{
			name: "Invalid frameSize",
			configChangeUnderTest: func(c *CLIConfig) {
				c.FrameSize = 0
			},
			expectedErrors: []string{"frameSize must be a positive number"},
		},
		{
			name: "Invalid sequencingChainAddr",
			configChangeUnderTest: func(c *CLIConfig) {
				c.SequencingChainRPCURL = "invalid sequencing chain address"
			},
			expectedErrors: []string{"invalid URL for sequencing chain address"},
		},
		{
			name: "Invalid settlementChainAddr",
			configChangeUnderTest: func(c *CLIConfig) {
				c.SettlementChainRPCURL = "invalid settlement chain address"
			},
			expectedErrors: []string{"invalid URL for settlement chain address"},
		},
		{
			name: "Invalid metaBasedChainAddr",
			configChangeUnderTest: func(c *CLIConfig) {
				c.MetaBasedChainRPCURL = "invalid meta based chain address"
			},
			expectedErrors: []string{"invalid URL for meta based chain address"},
		},
		{
			name: "Invalid log level",
			configChangeUnderTest: func(c *CLIConfig) {
				c.LogLevel = "OTHER"
			},
			expectedErrors: []string{"invalid log level"},
		},
		{
			name: "Invalid settlementStartBlock",
			configChangeUnderTest: func(c *CLIConfig) {
				c.SettlementStartBlock = 0
			},
			expectedErrors: []string{"settlementStartBlock must be a positive number"},
		},
		{
			name: "Invalid sequencingContractAddress",
			configChangeUnderTest: func(c *CLIConfig) {
				c.SequencingContractAddress = "invalid sequencing contract address"
			},
			expectedErrors: []string{"sequencingContractAddress must be a valid hex address"},
		},
		{
			name: "Invalid batchInboxAddress",
			configChangeUnderTest: func(c *CLIConfig) {
				c.BatchInboxAddress = "invalid batch inbox address"
			},
			expectedErrors: []string{"batchInboxAddress must be a valid hex address"},
		},
		{
			name: "Multiple invalid fields",
			configChangeUnderTest: func(c *CLIConfig) {
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
			configChangeUnderTest: func(c *CLIConfig) {
				c.BatcherPrivateKey = ""
			},
			expectedErrors: []string{"batcherPrivateKey cannot be blank"},
		},
		{
			name: "Invalid Settlement Chain ID",
			configChangeUnderTest: func(c *CLIConfig) {
				c.SettlementChainID = 0
			},
			expectedErrors: []string{"settlementChainID must be a positive number: 0"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := validConfig()
			tt.configChangeUnderTest(&cfg)
			result := cfg.Check()

			if tt.expectedErrors == nil {
				assert.NoError(t, result, "Expected no error, but got: %v", result)
			} else {
				assert.Error(t, result, "Expected an error, but got none")

				joinedErrors, ok := result.(interface{ Unwrap() []error })
				assert.True(t, ok, "Expected a errors.joinError, but got: %T", result)
				errs := joinedErrors.Unwrap()
				if ok {
					assert.Equal(t, len(tt.expectedErrors), len(errs), "Expected %d errors, but got %d", len(tt.expectedErrors), len(errs))

					for _, expectedErr := range tt.expectedErrors {
						found := false
						for _, err := range errs {
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
