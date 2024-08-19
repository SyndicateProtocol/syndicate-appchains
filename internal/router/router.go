package router

import (
	"context"
	"encoding/json"
	"net/http"

	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

type ProxyService struct {
	Client *rpc.Client
}

func (s *ProxyService) GetBlockByNumber(ctx context.Context, blockNumber string, fullTx bool) (map[string]interface{}, error) {
	var result map[string]interface{}
	log.Info().Msg("-- HIT eth_getBlockByNumber")
	err := s.Client.CallContext(ctx, &result, "eth_getBlockByNumber", blockNumber, fullTx)
	return result, err
}

func (s *ProxyService) GetTransactionReceipt(ctx context.Context, txHash string) (map[string]interface{}, error) {
	var result map[string]interface{}
	log.Info().Msg("-- HIT eth_getTransactionReceipt")
	err := s.Client.CallContext(ctx, &result, "eth_getTransactionReceipt", txHash)
	return result, err
}

func (s *ProxyService) GetBlockByHash(ctx context.Context, blockHash string, fullTx bool) (map[string]interface{}, error) {
	var result map[string]interface{}
	log.Info().Msg("-- HIT eth_getBlockByHash")
	err := s.Client.CallContext(ctx, &result, "eth_getBlockByHash", blockHash, fullTx)
	return result, err
}

func (s *ProxyService) HealthCheck() (string, error) {
	return "Healthy", nil
}

func Routes(server *rpc.Server, service *ProxyService) *http.ServeMux {
	if err := server.RegisterName("eth", service); err != nil {
		log.Fatal().Err(err).Msg("Error registering service")
	}

	router := http.NewServeMux()

	router.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			server.ServeHTTP(w, r)
		} else {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	})

	router.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		status, err := service.HealthCheck()
		log.Info().Msg("-- HIT /health")
		if err != nil {
			w.WriteHeader(http.StatusServiceUnavailable)
			err = json.NewEncoder(w).Encode(map[string]string{"status": "Unhealthy", "error": err.Error()})
			if err != nil {
				return
			}
			return
		}
		w.WriteHeader(http.StatusOK)
		err = json.NewEncoder(w).Encode(map[string]string{"status": status})
		if err != nil {
			return
		}
	})

	return router
}
