package main

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"log/slog"
	"net/http"
	"os"
	"runtime"
	"sort"
	"strings"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/jellydator/ttlcache/v3"
	"github.com/mdlayher/vsock"
)

const TTL = time.Hour
const RequestID = "1"

var cache = ttlcache.New(
	ttlcache.WithTTL[common.Hash, json.RawMessage](TTL),
	// in case a cached response is invalid, e.g. the signer key changes, expire it after the timeout
	ttlcache.WithDisableTouchOnHit[common.Hash, json.RawMessage](),
)

var pool = sync.Pool{
	New: func() any {
		conn, err := vsock.Dial(16, 1234, &vsock.Config{})
		if err != nil {
			slog.Error("Error dialing vsock", "error", err)
		}
		return conn
	},
}

type EnclaveRequest struct {
	request  []byte
	cacheKey common.Hash
	ctx      context.Context
	response chan<- json.RawMessage
}

type RequestData struct {
	Jsonrpc string          `json:"jsonrpc"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params,omitempty"`
	Id      json.RawMessage `json:"id,omitempty"`
}

var requestChan = make(chan EnclaveRequest)

func processEnclaveRequest(req []byte) json.RawMessage {
	conn := pool.Get().(*vsock.Conn)
	if conn == nil {
		slog.Error("Error getting vsock connection from pool")
		return nil
	}

	defer func() {
		if conn != nil {
			conn.Close()
		}
	}()

	_, err := conn.Write(req)
	if err != nil {
		slog.Error("Error writing to vsock", "error", err)
		return nil
	}

	dec := json.NewDecoder(conn)
	dec.UseNumber()

	var raw json.RawMessage
	if err := dec.Decode(&raw); err != nil {
		slog.Error("Error decoding response", "error", err)
		return nil
	}

	pool.Put(conn)
	conn = nil
	return raw
}

func handler(w http.ResponseWriter, r *http.Request) {
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

	var reqData RequestData

	if err := json.Unmarshal(req, &reqData); err != nil || reqData.Jsonrpc != "2.0" || len(reqData.Method) == 0 || len(reqData.Id) == 0 {
		slog.Error("Error unmarshalling request", "error", err, "request", string(req))
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var reqId json.RawMessage
	var cacheKey common.Hash
	var raw json.RawMessage
	ctx := r.Context()
	// Cache resource intensive proof generation requests that time out
	if reqData.Method == "enclave_verifyAppchain" || reqData.Method == "enclave_verifySequencingChain" {
		params := [1]map[string]json.RawMessage{}
		if err := json.Unmarshal(reqData.Params, &params); err != nil {
			slog.Error("Error unmarshalling enclave request params", "error", err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// sort preimage data
		{
			preimageData := params[0]["preimageData"]
			if preimageData != nil {
				var data map[uint8][][]byte
				if err := json.Unmarshal(preimageData, &data); err != nil {
					slog.Error("Error unmarshalling preimage data")
				}
				for _, entry := range data {
					sort.Slice(entry, func(i, j int) bool {
						return bytes.Compare(entry[i], entry[j]) < 0
					})
				}
				params[0]["preimageData"], err = json.Marshal(data)
				if err != nil {
					panic("failed to marshal preimage data object")
				}
			}
		}
		if reqData.Params, err = json.Marshal(params); err != nil {
			panic("failed to marshal request params")
		}
		cacheKey = crypto.Keccak256Hash(crypto.Keccak256([]byte(reqData.Method)), []byte(reqData.Params))
		if entry := cache.Get(cacheKey); entry != nil {
			slog.Warn("Found entry in cache", "cacheKey", "cacheKey")
			raw = entry.Value()
		} else {
			// remove extra fields from the request & normalize the id
			reqData.Id = json.RawMessage(RequestID)
			req, err = json.Marshal(reqData)
			if err != nil {
				panic("failed to marshal request data object")
			}
			responseChan := make(chan json.RawMessage)
			requestChan <- EnclaveRequest{request: req, ctx: ctx, cacheKey: cacheKey, response: responseChan}
			raw = <-responseChan
		}
	} else {
		raw = processEnclaveRequest(req)
	}

	if err := ctx.Err(); err != nil {
		if raw != nil && cacheKey != (common.Hash{}) {
			slog.Warn("Request timed out: storing to cache instead", "error", err, "cacheKey", cacheKey)
			cache.Set(cacheKey, raw, ttlcache.DefaultTTL)
		} else {
			slog.Warn("Request timed out", "error", err, "cacheKey", cacheKey)
		}
		w.WriteHeader(http.StatusRequestTimeout)
		return
	}

	if raw == nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	response := raw
	if cacheKey != (common.Hash{}) {
		var m map[string]json.RawMessage
		if err := json.Unmarshal(raw, &m); err != nil {
			slog.Error("Failed to unmarshal response", "error", err, "response", string(raw))
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		if !bytes.Equal(m["id"], json.RawMessage(RequestID)) {
			slog.Error("Unexpected response id", "response_id", string(m["id"]), "request_id", string(RequestID))
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		m["id"] = reqId
		response, err = json.Marshal(m)
		if err != nil {
			panic("failed to marshal response")
		}
	}

	if _, err = w.Write(response); err != nil {
		if cacheKey != (common.Hash{}) {
			slog.Warn("Response failed: storing to cache instead", "error", err)
			cache.Set(cacheKey, raw, ttlcache.DefaultTTL)
		} else {
			slog.Warn("Response failed", "error", err)
		}
	}

}

func worker() {
	for req := range requestChan {
		if entry := cache.Get(req.cacheKey); entry != nil {
			slog.Info("Found entry in cache", "cacheKey", req.cacheKey)
			req.response <- entry.Value()
		} else if err := req.ctx.Err(); err != nil {
			slog.Warn("Request timed out in queue", "error", err, "cacheKey", req.cacheKey)
			req.response <- nil
		} else {
			req.response <- processEnclaveRequest(req.request)
		}
		close(req.response)
	}
	panic("request channel is closed")
}

// small HTTP proxy that forwards requests to a vsock service
func main() {
	programLevel := new(slog.LevelVar) // Info by default
	switch strings.ToLower(os.Getenv("GO_LOG")) {
	case "debug":
		programLevel.Set(slog.LevelDebug)
	case "warn":
		programLevel.Set(slog.LevelWarn)
	case "error":
		programLevel.Set(slog.LevelError)
	}
	slog.SetDefault(slog.New(slog.NewJSONHandler(os.Stderr, &slog.HandlerOptions{Level: programLevel})))

	// Automatically remove expired items from the ttl cache
	go cache.Start()

	// Process a resource intensive request on each CPU core
	for range runtime.NumCPU() {
		go worker()
	}

	go func() {
		for req := range requestChan {
			if entry := cache.Get(req.cacheKey); entry != nil {
				slog.Warn("Found entry in cache", "cacheKey", req.cacheKey)
				req.response <- entry.Value()
			} else if err := req.ctx.Err(); err != nil {
				slog.Warn("Request timed out in queue", "error", err, "cacheKey", req.cacheKey)
				req.response <- nil
			} else {
				req.response <- processEnclaveRequest(req.request)
			}
			close(req.response)
		}
		panic("request channel is closed")
	}()

	// Get bind address from environment variable, default to all interfaces if unset
	// Use BIND_ADDRESS=127.0.0.1:7333 for localhost only
	bindAddr := os.Getenv("BIND_ADDRESS")
	if bindAddr == "" {
		bindAddr = "0.0.0.0:7333" // Default to all interfaces
	}

	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("ok"))
	})

	http.HandleFunc("/", handler)

	slog.Info("Starting server", "bind_address", bindAddr)
	err := http.ListenAndServe(bindAddr, nil)
	if err != nil {
		slog.Error("Error starting server", "error", err)
	}
}
