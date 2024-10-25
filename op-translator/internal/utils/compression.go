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
	return io.ReadAll(reader)
}

func DecompressBrotli(compressedData []byte) ([]byte, error) {
	reader := brotli.NewReader(bytes.NewReader(compressedData))
	return io.ReadAll(reader)
}
