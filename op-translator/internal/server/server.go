package server

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"sync"
	"time"

	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

func TranslatorHandler(targetProxyURL string, translator any, log gethlog.Logger) (*http.ServeMux, error) {
	// Setup proxy
	parsedURL, err := url.Parse(targetProxyURL)
	if err != nil {
		return nil, err
	}
	proxy := httputil.NewSingleHostReverseProxy(parsedURL)

	// Setup translator
	translatorRPC := rpc.NewServer()
	err = translatorRPC.RegisterName("eth", translator)
	if err != nil {
		return nil, err
	}

	// Setup routing
	router := TranslatorRouter(translatorRPC, parsedURL, proxy, log)

	return router, nil
}

func TranslatorRouter(translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy, log gethlog.Logger) *http.ServeMux {
	loggingMiddleware := NoOpMiddleware
	if log.Enabled(context.Background(), slog.LevelDebug) {
		loggingMiddleware = VerboseLoggingMiddleware
	}

	router := http.NewServeMux()
	router.Handle("/metrics", promhttp.Handler())
	router.HandleFunc("/",
		loggingMiddleware(
			rpcEndpointsHandler(translatorRPC, parsedURL, proxy, log),
			log,
		),
	)
	router.HandleFunc("/health",
		loggingMiddleware(
			healthcheckHandler(log),
			log,
		),
	)
	return router
}

func healthcheckHandler(log gethlog.Logger) func(w http.ResponseWriter, _ *http.Request) {
	return func(w http.ResponseWriter, _ *http.Request) {
		log.Info("-- HIT /health")
		status := "Healthy"
		w.WriteHeader(http.StatusOK)
		err := json.NewEncoder(w).Encode(map[string]string{"status": status})
		if err != nil {
			return
		}
	}
}

func ShouldTranslate(method string) bool {
	switch method {
	case "eth_getBlockByNumber":
		return true
	case "eth_getBlockByHash":
		return true
	}
	return false
}

func rpcEndpointsHandler(translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy, log gethlog.Logger) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Ethereum JSON-RPC uses POST requests", http.StatusMethodNotAllowed)
			return
		}

		// Parse out the method
		method, err := ParseMethod(r)
		if err != nil {
			log.Warn("parseMethod failed", "error", err)
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		log.Debug("method to translate", "method", method)
		if ShouldTranslate(method) {
			// Translate
			translatorRPC.ServeHTTP(w, r)
			return
		}

		// Proxy
		r.URL.Host = parsedURL.Host
		r.URL.Scheme = parsedURL.Scheme
		r.Header.Set("X-Forwarded-Host", r.Header.Get("Host"))
		r.Host = parsedURL.Host
		proxy.ServeHTTP(w, r)
	}
}

type Server struct {
	ctx    context.Context
	log    gethlog.Logger
	srv    *http.Server
	cancel context.CancelFunc
	wg     sync.WaitGroup
}

func NewServer(
	port int,
	readTimeout,
	writeTimeout time.Duration,
	targetProxyURL string,
	opTranslator any,
	log gethlog.Logger,
) (*Server, error) {
	router, err := TranslatorHandler(targetProxyURL, opTranslator, log)
	if err != nil {
		return nil, err
	}
	srv := &http.Server{
		Addr:         fmt.Sprintf(":%d", port),
		Handler:      router,
		ReadTimeout:  readTimeout,
		WriteTimeout: writeTimeout,
	}
	return &Server{srv: srv, log: log}, nil
}

func (s *Server) Start(ctx context.Context) {
	s.log.Debug("Starting op-translator JSON-RPC server", "address", s.srv.Addr)
	ctx, cancel := context.WithCancel(ctx)
	s.ctx = ctx
	s.cancel = cancel
	s.srv.BaseContext = func(net.Listener) context.Context {
		return s.ctx
	}
	s.wg.Add(1)
	go func() {
		defer s.wg.Done()
		// ErrServerClosed == Graceful shutdown
		if err := s.srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			s.log.Error("op-translator: RPC server error", "error", err)
			panic(err)
		}
	}()
}

func (s *Server) Stop(ctx context.Context) {
	s.log.Info("Stopping JSON-RPC server")
	s.cancel()
	if err := s.srv.Shutdown(ctx); err != nil {
		s.log.Error("failed to shutdown JSON-RPC server", "error", err)
		panic(err)
	}
	s.wg.Wait()
	s.log.Info("JSON-RPC server stopped")
}

func ParseMethod(request *http.Request) (string, error) {
	// Read body to buffer
	body, err := io.ReadAll(request.Body)
	if err != nil {
		return "", fmt.Errorf("error reading body: %w", err)
	}

	request.Body = io.NopCloser(bytes.NewBuffer(body))

	var result map[string]any

	// Unmarshal the JSON from the buffer into the map
	if err := json.NewDecoder(bytes.NewBuffer(body)).Decode(&result); err != nil {
		return "", fmt.Errorf("error decoding JSON: %w", err)
	}

	method, ok := result["method"].(string)
	if !ok {
		return "", fmt.Errorf("invalid method type")
	}

	return method, nil
}
