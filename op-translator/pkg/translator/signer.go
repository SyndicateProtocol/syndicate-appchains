package translator

import (
	"crypto/ecdsa"
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
)

type Signer struct {
	privateKey *ecdsa.PrivateKey
	signer     ethtypes.Signer
}

func NewSigner(privateKey string, chainID *big.Int) (*Signer, error) {
	key, err := crypto.HexToECDSA(privateKey)
	if err != nil {
		return nil, err
	}

	return &Signer{
		signer:     ethtypes.NewCancunSigner(chainID),
		privateKey: key,
	}, nil
}

func (s *Signer) Sign(tx *ethtypes.Transaction) (*ethtypes.Transaction, error) {
	if tx == nil {
		return nil, fmt.Errorf("transaction is nil")
	}

	txHash := s.signer.Hash(tx)

	signature, err := crypto.Sign(txHash.Bytes(), s.privateKey)
	if err != nil {
		return nil, err
	}

	return tx.WithSignature(s.signer, signature)
}

func (s *Signer) ChainID() int64 {
	return s.signer.ChainID().Int64()
}

func (s *Signer) Address() common.Address {
	return crypto.PubkeyToAddress(s.privateKey.PublicKey)
}
