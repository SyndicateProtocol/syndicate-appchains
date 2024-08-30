package main

import (
	server "github.com/SyndicateProtocol/op-translator/internal/api"
	config "github.com/SyndicateProtocol/op-translator/internal/config"
	translator "github.com/SyndicateProtocol/op-translator/internal/translator"
)

func initService() {
	cfg := config.Init()
	opTranslator := translator.Init(cfg)

	s, err := server.Init(cfg, opTranslator)
	if err != nil {
		panic(err)
	}
	server.Start(cfg, s)
}

func main() {

	// Initialize services and configurations
	initService()
}
