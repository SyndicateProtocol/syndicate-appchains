package server

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httputil"
	"net/url"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/constants"
	t "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

func TranslatorHandler(cfg config.IConfig, translator any) (*http.ServeMux, error) {
	// Setup proxy
	parsedURL, err := url.Parse(cfg.SettlementChainAddr())
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
	logLevel := constants.ToLogLevel(cfg.LogLevel())
	router := TranslatorRouter(logLevel, translatorRPC, parsedURL, proxy)

	return router, nil
}

func TranslatorRouter(logLevel constants.LogLevel, translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy) *http.ServeMux {
	loggingMiddleware := NoOpMiddleware
	if logLevel == constants.Debug {
		loggingMiddleware = VerboseLoggingMiddleware
	}

	router := http.NewServeMux()
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

func rpcEndpointsHandler(translatorRPC *rpc.Server, parsedURL *url.URL, proxy *httputil.ReverseProxy) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Ethereum JSON-RPC uses POST requests", http.StatusMethodNotAllowed)
			return
		}

		// Parse out the method
		method := parseMethod(r)
		log.Debug().Msgf("Method: %s", method)
		if t.ShouldTranslate(method) {
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

func Start(cfg *config.Config, router *http.ServeMux) {
	log.Debug().Msgf("Starting JSON-RPC server on port %d", cfg.Port())
	err := http.ListenAndServe(fmt.Sprintf(":%d", cfg.Port()), router) //nolint:gosec // TODO refactor for performance anyway
	if err != nil {
		log.Error().
			Err(err).
			Msg("RPC server error")
	}
}

func parseMethod(request *http.Request) string {
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
