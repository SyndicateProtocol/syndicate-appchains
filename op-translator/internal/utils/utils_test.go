package utils_test

import (
	"bytes"
	"compress/zlib"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/utils"
	"github.com/andybalholm/brotli"
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

func TestGetCompressionType(t *testing.T) {
	tests := []struct { //nolint:govet // test struct
		name         string
		versionByte  byte
		expectedType utils.CompressionType
	}{
		{"NoCompression", utils.NoCompression, utils.CompressionTypeNone},
		{"ZlibCM8", utils.ZlibCM8, utils.CompressionTypeZlib},
		{"ZlibCM15", utils.ZlibCM15, utils.CompressionTypeZlib},
		{"VersionBrotli", utils.VersionBrotli, utils.CompressionTypeBrotli},
		{"UnknownCompressionType", 0x66, utils.CompressionTypeUnknown},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			compressionType := utils.GetCompressionType(tt.versionByte)
			if compressionType != tt.expectedType {
				t.Errorf("GetCompressionType(%v) = %v, want %v", tt.versionByte, compressionType, tt.expectedType)
			}
		})
	}
}

func TestDecompressionFunctions(t *testing.T) {
	originalData := []byte("test compression data")
	invalidData := []byte("invalid data")

	tests := []struct { //nolint:govet // test struct
		name           string
		compressedData []byte
		expectedError  bool
		decompressFunc func([]byte) ([]byte, error)
	}{
		{"ValidZlibData", compressZlib(originalData), false, utils.DecompressZlib},
		{"InvalidZlibData", invalidData, true, utils.DecompressZlib},
		{"ValidBrotliData", compressBrotli(originalData), false, utils.DecompressBrotli},
		{"InvalidBrotliData", invalidData, true, utils.DecompressBrotli},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			decompressedData, err := tt.decompressFunc(tt.compressedData)
			if (err != nil) != tt.expectedError {
				t.Errorf("Expected error: %v, got: %v", tt.expectedError, err)
			}
			if !tt.expectedError && !bytes.Equal(decompressedData, originalData) {
				t.Errorf("Expected decompressed data: %v, got: %v", originalData, decompressedData)
			}
		})
	}
}

// Compression functions
func compressZlib(data []byte) []byte {
	var b bytes.Buffer
	w := zlib.NewWriter(&b)
	w.Write(data) //nolint:errcheck // just for testing
	w.Close()
	return b.Bytes()
}

func compressBrotli(data []byte) []byte {
	var b bytes.Buffer
	w := brotli.NewWriter(&b)
	w.Write(data) //nolint:errcheck // just for testing
	w.Close()
	return b.Bytes()
}
