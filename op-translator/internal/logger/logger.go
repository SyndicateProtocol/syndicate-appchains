package logger

import (
	"bytes"
	"encoding/json"
	"io"
	"os"
	"strings"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/rs/zerolog/pkgerrors"
)

func Init(cfg config.Config) {
	log.Debug().Msg("Logger initializing")
	// Add these back if GCP logging is necessary
	// zerolog.LevelFieldName = "severity"
	// zerolog.TimestampFieldName = "timestamp"
	// zerolog.TimeFieldFormat = time.RFC3339Nano

	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack //nolint:reassign // Appears to be working and recommended way to do this
	// See https://github.com/rs/zerolog#error-logging-with-stacktrace

	if cfg.LogLevel == constants.Debug.String() {
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	} else {
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	}

	// Configure logger output
	if cfg.Pretty {
		output := &prettifiedJSONWriter{out: os.Stdout}
		log.Logger = zerolog.New(output).With().Timestamp().Caller().Logger()
	} else {
		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout}).With().Caller().Stack().Logger()
	}
	log.Debug().Msg("Logger initialized")
}

type prettifiedJSONWriter struct {
	out io.Writer
}

func (w *prettifiedJSONWriter) Write(p []byte) (n int, err error) {
	var logEntry map[string]any

	err = json.Unmarshal(p, &logEntry)
	if err != nil {
		return 0, err
	}

	// Check if the 'body' field exists and is a string
	if body, exists := logEntry["body"].(string); exists {
		// Remove newlines and escaped characters from the 'body' field
		sanitizedBody := sanitizeBody(body)

		// Attempt to unmarshal the sanitized body into a JSON object
		var bodyObj any
		if err = json.Unmarshal([]byte(sanitizedBody), &bodyObj); err == nil {
			logEntry["body"] = bodyObj
		} else {
			// Fallback to sanitized string if unmarshaling fails
			logEntry["body"] = sanitizedBody
		}
	}

	sanitizedJSON, err := json.Marshal(logEntry)
	if err != nil {
		log.Info().Msg("sanitizedJSON")
		log.Info().Msg(string(sanitizedJSON))
		return 0, err
	}

	var out bytes.Buffer
	// Use tab ("\t") for indentation instead of spaces
	err = json.Indent(&out, sanitizedJSON, "", "\t")
	if err != nil {
		return 0, err
	}

	// Retain a newline between log entries for readability
	out.WriteByte('\n')
	to, err := out.WriteTo(w.out)
	return int(to), err
}

// sanitizeBody removes newlines and escaped characters from the 'body' field.
func sanitizeBody(body string) string {
	// Remove all newline characters
	sanitized := strings.ReplaceAll(body, "\n", " ")
	sanitized = strings.ReplaceAll(sanitized, "\r", " ")

	// Optionally, remove other escaped characters if necessary
	// For example, remove tab characters
	sanitized = strings.ReplaceAll(sanitized, "\t", " ")

	return sanitized
}
