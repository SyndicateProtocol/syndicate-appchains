package utils

import (
	"encoding/hex"
	"errors"
	"fmt"
	"math/big"
	"strconv"
	"strings"

	"github.com/rs/zerolog/log"
)

const MillisecondsPerSecond = 1000
const HexBase = 16

func DecodeHexString(hexStr string) ([]byte, error) {
	return hex.DecodeString(strings.TrimPrefix(hexStr, "0x"))
}

func HexToInt(hexStr string) (int, error) {
	if !strings.HasPrefix(hexStr, "0x") {
		return 0, errors.New("invalid hex string, must start with 0x")
	}
	hexStr = strings.TrimPrefix(hexStr, "0x")
	result, err := strconv.ParseInt(hexStr, HexBase, 64)
	if err != nil {
		return 0, err
	}

	return int(result), nil
}

func HexToBigInt(hexStr string) (*big.Int, error) {
	num := new(big.Int)
	if !strings.HasPrefix(hexStr, "0x") {
		return nil, errors.New("invalid hex string, must start with 0x")
	}
	if hexStr == "0x" {
		return big.NewInt(0), nil
	}
	hexStr = strings.TrimPrefix(hexStr, "0x")
	_, ok := num.SetString(hexStr, HexBase)
	if !ok {
		return nil, fmt.Errorf("invalid hexadecimal value: %s", hexStr)
	}
	return num, nil
}

func MustHexToBigInt(hexStr string) *big.Int {
	value, err := HexToBigInt(hexStr)
	if err != nil {
		log.Warn().Err(err).Msgf("Failed to convert hex to big int: %v", hexStr)
		return big.NewInt(0)
	}
	return value
}

func HexToUInt64(hexStr string) (uint64, error) {
	if !strings.HasPrefix(hexStr, "0x") {
		return 0, errors.New("invalid hex string, must start with 0x")
	}
	hexStr = strings.TrimPrefix(hexStr, "0x")
	return strconv.ParseUint(hexStr, HexBase, 64)
}

func IntToHex(num int) string {
	return "0x" + strconv.FormatInt(int64(num), HexBase)
}

func UInt64ToHex(num uint64) string {
	return "0x" + strconv.FormatUint(num, HexBase)
}

func CloneBigIntPtr(value *big.Int) *big.Int {
	if value == nil {
		return nil
	}
	return new(big.Int).Set(value)
}
