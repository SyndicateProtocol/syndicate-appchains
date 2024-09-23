package utils

import (
	"testing"
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
			got, err := HexToInt(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("HexToInt() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("HexToInt() = %v, want %v", got, tt.want)
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
			got, err := HexToUInt64(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("HexToUInt64() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("HexToUInt64() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIntToHex(t *testing.T) {
	tests := []struct { //nolint:govet // Test struct
		name  string
		input int
		want  string
	}{
		{"Zero", 0, "0x0"},
		{"Positive number", 26, "0x1a"},
		{"Large number", 1000000, "0xf4240"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IntToHex(tt.input); got != tt.want {
				t.Errorf("IntToHex() = %v, want %v", got, tt.want)
			}
		})
	}
}
