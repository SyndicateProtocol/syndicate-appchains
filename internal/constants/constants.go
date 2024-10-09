package constants

import "slices"

type LogLevel string

func (l LogLevel) String() string {
	return string(l)
}

const (
	Info  LogLevel = "info"
	Debug LogLevel = "debug"
)

func IsValidLogLevel(logLevel string) bool {
	return slices.Contains(
		[]string{Info.String(), Debug.String()},
		logLevel)
}

func ToLogLevel(logLevel string) LogLevel {
	switch logLevel {
	case Info.String():
		return Info
	case Debug.String():
		return Debug
	default:
		return Info
	}
}

const ZeroHash = "0x0000000000000000000000000000000000000000000000000000000000000000"
