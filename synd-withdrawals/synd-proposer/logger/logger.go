package logger

import (
	"os"
	"time"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/config"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/rs/zerolog/pkgerrors"
)

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
	if config.Get("LOG_LEVEL") == "debug" {
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	} else {
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	}

	// When running locally, use console writer
	if config.Get("DEPLOY_ENV") == "development" {
		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout}).With().Caller().Stack().Logger()
	}
}
