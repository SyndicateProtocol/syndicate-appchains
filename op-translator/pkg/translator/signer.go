package translator

import (
	"crypto/ecdsa"
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/interfaces"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/rs/zerolog/log"
)

type Signer struct {
	privateKey *ecdsa.PrivateKey
	signer     ethtypes.Signer
}

func NewSigner(cfg interfaces.IConfig) *Signer {
	key, err := crypto.HexToECDSA(cfg.BatcherPrivateKey())

	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize signer")
	}

	chainID := big.NewInt(cfg.SettlementChainID())
	return &Signer{
		signer:     ethtypes.NewCancunSigner(chainID),
		privateKey: key,
	}
}

func (s *Signer) Sign(tx *ethtypes.Transaction) (*ethtypes.Transaction, error) {
	if tx == nil {
		return nil, fmt.Errorf("transaction is nil")
	}

	txHash := s.signer.Hash(tx)

	signature, err := crypto.Sign(txHash.Bytes(), s.privateKey)
	if err != nil {
		log.Error().Err(err).Msg("Failed to sign the transaction")
		return nil, err
	}

	return tx.WithSignature(s.signer, signature)
}

func (s *Signer) ChainID() int64 {
	return s.signer.ChainID().Int64()
}
