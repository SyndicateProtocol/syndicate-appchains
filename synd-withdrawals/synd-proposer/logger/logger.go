package logger

import (
	"os"
	"strings"
	"time"

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
