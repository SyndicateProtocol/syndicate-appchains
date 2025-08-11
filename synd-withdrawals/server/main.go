package main

import (
	"encoding/json"
	"io"
	"log/slog"
	"net/http"
	"sync"

	"github.com/mdlayher/vsock"
)

// small HTTP proxy that forwards requests to a vsock service
func main() {
	pool := sync.Pool{
		New: func() any {
			conn, err := vsock.Dial(16, 1234, &vsock.Config{})
			if err != nil {
				slog.Error("Error dialing vsock", "error", err)
				return nil
			}
			return conn
		},
	}

	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ok"))
	})

	handler := func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			w.WriteHeader(http.StatusMethodNotAllowed)
			return
		}
		req, err := io.ReadAll(r.Body)
		if err != nil {
			slog.Error("Error reading request body", "error", err)
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		_ = r.Body.Close()

		conn := pool.Get().(*vsock.Conn)
		if conn == nil {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		defer pool.Put(conn)

		_, err = conn.Write(req)
		if err != nil {
			slog.Error("Error writing to vsock", "error", err)
			w.WriteHeader(http.StatusInternalServerError)
			return
		}

		dec := json.NewDecoder(conn)
		dec.UseNumber()

		var raw json.RawMessage
		if err := dec.Decode(&raw); err != nil {
			slog.Error("Error decoding response", "error", err)
			w.WriteHeader(http.StatusInternalServerError)
			return
		}

		_, _ = w.Write(raw)
	}

	http.HandleFunc("/", handler)

	err := http.ListenAndServe(":7333", nil)
	if err != nil {
		slog.Error("Error starting server", "error", err)
	}
}
