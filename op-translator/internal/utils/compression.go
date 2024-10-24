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

func DecompressZlib(compressedData []byte) ([]byte, error) {
	reader, err := zlib.NewReader(bytes.NewReader(compressedData))
	if err != nil {
		return nil, err
	}
	defer reader.Close()

	decompressedData, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	return decompressedData, nil
}

func DecompressBrotli(compressedData []byte) ([]byte, error) {
	reader := brotli.NewReader(bytes.NewReader(compressedData))
	decompressedData, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	return decompressedData, nil
}
