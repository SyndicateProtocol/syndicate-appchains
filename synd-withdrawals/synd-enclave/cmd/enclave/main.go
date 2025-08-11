package main

import (
	_ "embed"
	"flag"
	"math"
	"net/http"
	"os"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave/wavmio"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/mdlayher/vsock"
	"github.com/offchainlabs/nitro/execution/gethexec"
	"github.com/offchainlabs/nitro/gethhook"
)

//go:embed g1.point
var g1Data []byte

func main() {
	log.SetDefault(log.NewLogger(log.LogfmtHandlerWithLevel(os.Stdout, log.LevelDebug)))
	log.Info("Starting Enclave")
	flag.Parse()

	// RequireHookedGeth does nothing, but forces an import to let the init function run
	gethhook.RequireHookedGeth()

	// set the wasm compilation target to the native machine architecture (rawdb.LocalTarget())
	{
		targets := gethexec.StylusTargetConfig{}
		if err := targets.Validate(); err != nil {
			panic(err)
		}
		if err := gethexec.PopulateStylusTargetCache(&targets); err != nil {
			panic(err)
		}
	}

	wavmio.Init(g1Data)
	g1Data = nil

	s := rpc.NewServer()
	s.SetHTTPBodyLimit(math.MaxInt32)
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
