package translator

import (
	"math/big"
	"testing"

	"github.com/SyndicateProtocol/op-translator/mocks"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/stretchr/testify/assert"
)

func TestNewSigner(t *testing.T) {
	mockConfig := mocks.InitMockConfig()
	mockConfig.On("BatcherPrivateKey").Return("fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d")
	mockConfig.On("SettlementChainID").Return(int64(84532))

	signer := NewSigner(mockConfig)

	assert.NotNil(t, signer, "Signer should be created successfully")
	assert.NotNil(t, signer.privateKey, "Signer private key should be initialized")
	assert.NotNil(t, signer.signer, "Signer instance should be initialized")
	assert.Equal(t, int64(84532), signer.ChainID(), "Signer chain ID should be set correctly")
}

func TestSignSuccess(t *testing.T) {
	mockConfig := mocks.InitMockConfig()

	signer := NewSigner(mockConfig)

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
	mockConfig := mocks.InitMockConfig()

	signer := NewSigner(mockConfig)

	var tx *types.Transaction = nil
	signedTx, err := signer.Sign(tx)

	assert.Error(t, err, "Expected error when signing a nil transaction")
	assert.Nil(t, signedTx, "Signed transaction should be nil when there is an error")
}
