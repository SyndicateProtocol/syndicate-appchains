package utils

import (
	"encoding/hex"
	"errors"
	"math/big"
	"strconv"
	"strings"
)

const MillisecondsPerSecond = 1000

func DecodeHexString(hexStr string) ([]byte, error) {
	return hex.DecodeString(strings.TrimPrefix(hexStr, "0x"))
}

func HexToInt(hexStr string) (int, error) {
	if !strings.HasPrefix(hexStr, "0x") {
		return 0, errors.New("invalid hex string, must start with 0x")
	}
	hexStr = strings.TrimPrefix(hexStr, "0x")
	result, err := strconv.ParseInt(hexStr, 16, 64)
	if err != nil {
		return 0, err
	}

	return int(result), nil
}

func HexToBigInt(hexStr string) (*big.Int, error) {
	num, err := HexToInt(hexStr)
	if err != nil {
		return nil, err
	}
	return big.NewInt(int64(num)), nil
}

func HexToUInt64(hexStr string) (uint64, error) {
	if !strings.HasPrefix(hexStr, "0x") {
		return 0, errors.New("invalid hex string, must start with 0x")
	}
	hexStr = strings.TrimPrefix(hexStr, "0x")
	return strconv.ParseUint(hexStr, 16, 64)
}

func IntToHex(num int) string {
	return "0x" + strconv.FormatInt(int64(num), 16)
}

func UInt64ToHex(num uint64) string {
	return "0x" + strconv.FormatUint(num, 16)
}
