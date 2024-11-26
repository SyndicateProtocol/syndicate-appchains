package server

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"sync"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/rs/zerolog/log"
)

func TranslatorHandler(settlementChainRPCURL, logLevel string, translator any) (*http.ServeMux, error) {
	// Setup proxy
	parsedURL, err := url.Parse(settlementChainRPCURL)
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
	router := TranslatorRouter(constants.ToLogLevel(logLevel), translatorRPC, parsedURL, proxy)

	return router, nil
}

func TranslatorRouter(logLevel constants.LogLevel, translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy) *http.ServeMux {
	loggingMiddleware := NoOpMiddleware
	if logLevel == constants.Debug {
		loggingMiddleware = VerboseLoggingMiddleware
	}

	router := http.NewServeMux()
	router.Handle("/metrics", promhttp.Handler())
	router.HandleFunc("/", loggingMiddleware(
		rpcEndpointsHandler(translatorRPC, parsedURL, proxy)))
	router.HandleFunc("/health", loggingMiddleware(
		healthcheckHandler()))
	return router
}

func healthcheckHandler() func(w http.ResponseWriter, _ *http.Request) {
	return func(w http.ResponseWriter, _ *http.Request) {
		log.Info().Msg("-- HIT /health")
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

func rpcEndpointsHandler(translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Ethereum JSON-RPC uses POST requests", http.StatusMethodNotAllowed)
			return
		}

		// Parse out the method
		method := ParseMethod(r)
		log.Debug().Msgf("Method: %s", method)
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
	srv    *http.Server
	ctx    context.Context
	cancel context.CancelFunc
	wg     sync.WaitGroup
}

func NewServer(
	port int,
	readTimeout,
	writeTimeout time.Duration,
	settlementChainRPCURL string,
	opTranslator any,
	logLevel string,
) (*Server, error) {
	router, err := TranslatorHandler(settlementChainRPCURL, logLevel, opTranslator)
	if err != nil {
		return nil, err
	}
	srv := &http.Server{
		Addr:         fmt.Sprintf(":%d", port),
		Handler:      router,
		ReadTimeout:  readTimeout,
		WriteTimeout: writeTimeout,
	}
	return &Server{srv: srv}, nil
}

func (s *Server) Start(ctx context.Context) {
	log.Debug().Msgf("Starting JSON-RPC server on address %s", s.srv.Addr)
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
			log.Error().Err(err).Msg("RPC server error")
			panic(err)
		}
	}()
}

func (s *Server) Stop(ctx context.Context) {
	log.Info().Msg("Stopping JSON-RPC server")
	s.cancel()
	if err := s.srv.Shutdown(ctx); err != nil {
		log.Error().Err(err).Msg("Failed to shutdown JSON-RPC server")
		panic(err)
	}
	s.wg.Wait()
	log.Info().Msg("JSON-RPC server stopped")
}

func ParseMethod(request *http.Request) string {
	// Read body to buffer
	body, err := io.ReadAll(request.Body)
	if err != nil {
		log.Printf("Error reading body: %v", err)
		panic(err)
	}

	request.Body = io.NopCloser(bytes.NewBuffer(body))

	var result map[string]any

	// Unmarshal the JSON from the buffer into the map
	if err := json.NewDecoder(bytes.NewBuffer(body)).Decode(&result); err != nil {
		log.Warn().Msgf("Error decoding JSON: %v", err)
		return ""
	}

	method, ok := result["method"].(string)
	if !ok {
		log.Warn().Msg("Invalid method type")
		return ""
	}

	return method
}
