package main

import (
	"flag"
	"net/http"
	"os"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/mdlayher/vsock"
)

func main() {
	log.SetDefault(log.NewLogger(log.LogfmtHandlerWithLevel(os.Stdout, log.LevelDebug)))
	log.Info("Starting Enclave")
	flag.Parse()

	s := rpc.NewServer()
	serv, err := enclave.NewServer()
	if err != nil {
		log.Crit("Error creating API server", "error", err)
	}
	err = s.RegisterName("enclave", serv)
	if err != nil {
		log.Crit("Error registering API", "error", err)
	}

	listener, err := vsock.Listen(1234, &vsock.Config{})
	if err != nil {
		log.Warn("Error opening vsock listener, running in HTTP mode", "error", err)
		err = http.ListenAndServe(":1234", s)
	} else {
		err = s.ServeListener(listener)
	}
	if err != nil {
		log.Crit("Error starting server", "error", err)
	}
}
