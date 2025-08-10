package logger

import (
	"encoding/json"
	"os"
	"strings"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teemodule"
	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teetypes"

	"github.com/ethereum/go-ethereum/common"
	glog "github.com/ethereum/go-ethereum/log"
	"github.com/pkg/errors"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"

	"github.com/rs/zerolog/pkgerrors"
	"github.com/spf13/viper"
)

// Init starts the default JSON structured logger
// NOTE: For .Stack() to work we need to use `github.com/pkg/errors` everywhere instead of the standard `errors` package
// https://github.com/rs/zerolog#error-logging-with-stacktrace
func Init() {
	// Use GCP logging format
	zerolog.LevelFieldName = "severity"
	zerolog.TimestampFieldName = "timestamp"
	zerolog.TimeFieldFormat = time.RFC3339Nano
	// Recommended way to do this - https://github.com/rs/zerolog#error-logging-with-stacktrace
	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack

	// If used before config initialization, then these settings will not be changed
	logLevel := viper.GetString("log-level")
	switch strings.ToLower(logLevel) {
	case "debug":
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
		glog.SetDefault(glog.NewLogger(glog.JSONHandlerWithLevel(os.Stdout, glog.LevelDebug)))
	case "warn":
		zerolog.SetGlobalLevel(zerolog.WarnLevel)
		glog.SetDefault(glog.NewLogger(glog.JSONHandlerWithLevel(os.Stdout, glog.LevelWarn)))
	case "error":
		zerolog.SetGlobalLevel(zerolog.ErrorLevel)
		glog.SetDefault(glog.NewLogger(glog.JSONHandlerWithLevel(os.Stdout, glog.LevelError)))
	default:
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
		glog.SetDefault(glog.NewLogger(glog.JSONHandlerWithLevel(os.Stdout, glog.LevelInfo)))
	}

	// When running locally, use console writer
	if viper.GetString("DEPLOY_ENV") == "development" {
		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout}).With().Caller().Stack().Logger()
	}

}

// WrapErrorWithMsg util function
// Need to wrap std lib errors into `pkg/errors` to get the stack trace in `zerolog` logger
func WrapErrorWithMsg(msg string, err error) (string, error) {
	return msg, errors.Wrap(err, msg)
}

// ToHexForLogsPendingAssertion converts Pending assertion to a hex-encoded version
func ToHexForLogsPendingAssertion(t teemodule.PendingAssertion) string {
	hexInput := PendingAssertionHex{
		AppBlockHash: common.Hash(t.AppBlockHash).Hex(),
		AppSendRoot:  common.Hash(t.AppSendRoot).Hex(),
		SeqBlockHash: common.Hash(t.SeqBlockHash).Hex(),
		L1BatchAcc:   common.Hash(t.L1BatchAcc).Hex(),
	}
	jsonInput, _ := json.Marshal(hexInput)

	return string(jsonInput)
}

// ToHexForLogsTrustedInput  converts TeeTrustedInput to a hex-encoded version
func ToHexForLogsTrustedInput(t teetypes.TrustedInput) string {
	hexInput := TeeTrustedInputHex{
		ConfigHash:           common.Hash(t.ConfigHash).Hex(),
		AppStartBlockHash:    common.Hash(t.AppStartBlockHash).Hex(),
		SeqStartBlockHash:    common.Hash(t.SeqStartBlockHash).Hex(),
		SetDelayedMessageAcc: common.Hash(t.SetDelayedMessageAcc).Hex(),
		L1StartBatchAcc:      common.Hash(t.L1StartBatchAcc).Hex(),
		L1EndHash:            common.Hash(t.L1EndHash).Hex(),
	}
	jsonInput, _ := json.Marshal(hexInput)

	return string(jsonInput)
}

// PendingAssertionHex is a hex-encoded version for logging
type PendingAssertionHex struct {
	AppBlockHash string `json:"appBlockHash"`
	AppSendRoot  string `json:"appSendRoot"`
	SeqBlockHash string `json:"seqBlockHash"`
	L1BatchAcc   string `json:"l1BatchAcc"`
}

// TeeTrustedInputHex is a hex-encoded version for logging
type TeeTrustedInputHex struct {
	ConfigHash           string `json:"configHash"`
	AppStartBlockHash    string `json:"appStartBlockHash"`
	SeqStartBlockHash    string `json:"seqStartBlockHash"`
	SetDelayedMessageAcc string `json:"setDelayedMessageAcc"`
	L1StartBatchAcc      string `json:"l1StartBatchAcc"`
	L1EndHash            string `json:"l1EndHash"`
}
