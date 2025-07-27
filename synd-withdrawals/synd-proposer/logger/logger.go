package logger

import (
	"os"
	"strings"
	"time"

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
	logLevel := viper.GetString("LOG_LEVEL")
	switch strings.ToLower(logLevel) {
	case "debug":
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	case "info":
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	case "warn":
		zerolog.SetGlobalLevel(zerolog.WarnLevel)
	case "error":
		zerolog.SetGlobalLevel(zerolog.ErrorLevel)
	default:
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
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
