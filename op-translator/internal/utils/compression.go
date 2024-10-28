package utils

import (
	"bytes"
	"compress/zlib"
	"io"

	"github.com/andybalholm/brotli"
)

const (
	ZlibCM8            = 8
	ZlibCM15           = 15
	VersionBrotli byte = 0x1
	NoCompression byte = 0x0
)

type CompressionType string

const (
	CompressionTypeZlib    CompressionType = "zlib"
	CompressionTypeBrotli  CompressionType = "brotli"
	CompressionTypeNone    CompressionType = "none"
	CompressionTypeUnknown CompressionType = "unknown"
)

func DecompressZlib(compressedData []byte) ([]byte, error) {
	reader, err := zlib.NewReader(bytes.NewReader(compressedData))
	if err != nil {
		return nil, err
	}
	defer reader.Close()
	return io.ReadAll(reader)
}

func DecompressBrotli(compressedData []byte) ([]byte, error) {
	reader := brotli.NewReader(bytes.NewReader(compressedData))
	return io.ReadAll(reader)
}

func GetCompressionType(versionByte byte) CompressionType {
	switch {
	case versionByte == NoCompression:
		return CompressionTypeNone

	case versionByte&0x0F == ZlibCM8 || versionByte&0x0F == ZlibCM15:
		return CompressionTypeZlib

	case versionByte == VersionBrotli:
		return CompressionTypeBrotli

	default:
		return CompressionTypeUnknown
	}
}
