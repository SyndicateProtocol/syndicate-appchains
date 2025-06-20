package pkg

import (
	"crypto/ecdsa"
	"fmt"

	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rpc"
)

func NewProvider(rpcURL string) (*rpc.Client, error) {
	client, err := rpc.Dial(rpcURL)
	if err != nil {
		return nil, fmt.Errorf("failed to dial RPC at %s: %w", rpcURL, err)
	}
	return client, nil
}

func NewSignerFromPrivateKey(hexKey string) (*ecdsa.PrivateKey, error) {
	privateKey, err := crypto.HexToECDSA(hexKey)
	if err != nil {
		return nil, fmt.Errorf("invalid private key: %w", err)
	}
	return privateKey, nil
}
