package main

import (
	server "github.com/SyndicateProtocol/op-translator-proxy/internal/api"
)

func initService() {
	server.Init()

}

func main() {

	// Initialize services and configurations
	initService()
}
