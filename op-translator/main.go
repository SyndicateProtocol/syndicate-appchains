package main

import (
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/logger"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/server"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
)

func main() {
	cfg := config.Init()

	opTranslator := translator.Init(cfg)
	defer opTranslator.Close()

	s, err := server.TranslatorHandler(cfg, opTranslator)
	if err != nil {
		panic(err)
	}

	logger.Init(cfg)

	server.Start(&cfg, s)
}
