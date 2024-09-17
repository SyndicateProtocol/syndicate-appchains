package main

import (
	"github.com/SyndicateProtocol/op-translator/internal/config"
	"github.com/SyndicateProtocol/op-translator/internal/logger"
	"github.com/SyndicateProtocol/op-translator/internal/server"
	"github.com/SyndicateProtocol/op-translator/internal/translator"
)

func initService() {
	cfg := config.Init()
	opTranslator := translator.Init(cfg)
	s, err := server.TranslatorHandler(cfg, opTranslator)
	if err != nil {
		panic(err)
	}

	logger.Init(cfg)
	server.Start(cfg, s)
}

func main() {
	// Initialize services and configurations
	initService()
}
