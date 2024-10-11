package server

import (
	"bytes"
	"io"
	"net/http"

	"github.com/rs/zerolog/log"
)

func NoOpMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		next.ServeHTTP(w, r)
	}
}

// VerboseLoggingMiddleware logs the incoming request and the outgoing response
func VerboseLoggingMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Log the request
		logRequest(r)

		// Create a custom ResponseWriter to capture the response
		crw := &customResponseWriter{
			ResponseWriter: w,
			body:           &bytes.Buffer{},
			statusCode:     http.StatusOK, // Default status code
		}

		// Call the next handler
		next.ServeHTTP(crw, r)

		// Log the response
		logResponse(r, crw)
	}
}

// customResponseWriter is a custom ResponseWriter that captures the response body and status code
type customResponseWriter struct {
	http.ResponseWriter
	body       *bytes.Buffer
	statusCode int
}

func (crw *customResponseWriter) WriteHeader(statusCode int) {
	crw.statusCode = statusCode
	crw.ResponseWriter.WriteHeader(statusCode)
}

func (crw *customResponseWriter) Write(b []byte) (int, error) {
	crw.body.Write(b)
	return crw.ResponseWriter.Write(b)
}

// logRequest logs the details of the incoming request
func logRequest(r *http.Request) {
	var body []byte
	if r.Body != nil {
		body, _ = io.ReadAll(r.Body)
		// Restore the body for further processing
		r.Body = io.NopCloser(bytes.NewBuffer(body))
	}

	log.Info().
		Str("method", r.Method).
		Str("url", r.URL.String()).
		Str("remote_addr", r.RemoteAddr).
		Interface("headers", r.Header).
		Str("body", string(body)).
		Msg("Incoming request")
}

// logResponse logs the details of the outgoing response
func logResponse(r *http.Request, crw *customResponseWriter) {
	log.Info().
		Str("method", r.Method).
		Str("url", r.URL.String()).
		Int("status", crw.statusCode).
		Str("body", crw.body.String()).
		Msg("Outgoing response")
}
