package translator

import (
	"crypto/ecdsa"
	"math/big"

	"github.com/SyndicateProtocol/op-translator/internal/constants"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
)

type Signer struct {
	privateKey *ecdsa.PrivateKey
	signer     ethtypes.Signer
}

// TODO [SEQ-162]: Refactor this and put values in config, and use tests
func NewSigner() *Signer {
	key, _ := crypto.HexToECDSA("fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d")

	return &Signer{
		signer:     ethtypes.NewCancunSigner(big.NewInt(constants.ConfigChainID)),
		privateKey: key,
	}
}

func (s *Signer) Sign(tx *ethtypes.Transaction) (*ethtypes.Transaction, error) {
	txHash := s.signer.Hash(tx)

	signature, err := crypto.Sign(txHash.Bytes(), s.privateKey)
	if err != nil {
		return nil, err
	}

	return tx.WithSignature(s.signer, signature)
}
