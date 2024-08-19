package server

import (
	"fmt"
	"net/http"

	"github.com/SyndicateProtocol/op-translator-proxy/internal/router"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/rs/zerolog/log"
)

func Init() {
	// local client
	clientPort := 8546
	client, err := rpc.Dial(fmt.Sprintf("ws://127.0.0.1:%d", clientPort))
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to connect to the local RPC client")
	}

	server := rpc.NewServer()
	service := &router.ProxyService{Client: client}

	r := router.Routes(server, service)
	serverPort := 8544
	log.Debug().Msgf("Starting JSON-RPC server on port %d", serverPort)
	// TODO probably more performant libs than `http` here
	err = http.ListenAndServe(fmt.Sprintf(":%d", serverPort), r)
	if err != nil {
		log.Error().
			Err(err).
			Msg("RPC server error")
	}
}
