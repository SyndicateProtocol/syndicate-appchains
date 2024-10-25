package utils_test

import (
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/stretchr/testify/assert"
)

func TestHexToInt(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    int
		wantErr bool
	}{
		{"Valid hex", "0x1A", 26, false},
		{"Zero", "0x0", 0, false},
		{"Invalid hex without prefix", "1A", 0, true},
		{"Invalid hex", "0xG", 0, true},
		{"Empty string", "", 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := utils.HexToInt(tt.input)
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.want, got)
			}
		})
	}
}

func TestHexToUInt64(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    uint64
		wantErr bool
	}{
		{"Valid hex", "0x1A", 26, false},
		{"Zero", "0x0", 0, false},
		{"Max uint64", "0xFFFFFFFFFFFFFFFF", 18446744073709551615, false},
		{"Invalid hex without prefix", "1A", 0, true},
		{"Invalid hex", "0xG", 0, true},
		{"Empty string", "", 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := utils.HexToUInt64(tt.input)
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.want, got)
			}
		})
	}
}

func TestIntToHex(t *testing.T) {
	tests := []struct {
		name  string
		want  string
		input int
	}{
		{"Zero", "0x0", 0},
		{"Positive number", "0x1a", 26},
		{"Large number", "0xf4240", 1000000},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, utils.IntToHex(tt.input))
		})
	}
}

func TestSecondsToMilliseconds(t *testing.T) {
	assert.Equal(t, 1000, utils.SecondsToMilliseconds(1))
	assert.Equal(t, 2000, utils.SecondsToMilliseconds(2))
}
