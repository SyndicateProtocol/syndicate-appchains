package translator_test

import (
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
)

func TestNewSigner(t *testing.T) {
	signer, err := translator.NewSigner(mocks.TestingBatcherPrivateKey, big.NewInt(mocks.TestingSettlementChainID))
	assert.NoError(t, err)

	assert.NotNil(t, signer, "Signer should be created successfully")
	assert.Equal(t, int64(84532), signer.ChainID(), "Signer chain ID should be set correctly")
}

func TestSignSuccess(t *testing.T) {
	signer, err := translator.NewSigner(mocks.TestingBatcherPrivateKey, big.NewInt(mocks.TestingSettlementChainID))
	assert.NoError(t, err)

	tx := types.NewTransaction(
		0,
		common.HexToAddress("0x0000000000000000000000000000000000000000"),
		big.NewInt(1000),
		21000,
		big.NewInt(1),
		nil,
	)

	signedTx, err := signer.Sign(tx)

	assert.NoError(t, err, "Signing the transaction should not produce an error")
	assert.NotNil(t, signedTx, "Signed transaction should not be nil")
}

func TestSignWithInvalidTransaction(t *testing.T) {
	signer, err := translator.NewSigner(mocks.TestingBatcherPrivateKey, big.NewInt(mocks.TestingSettlementChainID))
	assert.NoError(t, err)

	var tx *types.Transaction = nil
	signedTx, err := signer.Sign(tx)

	assert.Error(t, err, "Expected error when signing a nil transaction")
	assert.Nil(t, signedTx, "Signed transaction should be nil when there is an error")
}
