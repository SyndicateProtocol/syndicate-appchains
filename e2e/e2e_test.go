package e2e_test

import (
	"math/big"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-e2e/actions/helpers"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/geth"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/stretchr/testify/require"
)

var counterContractArtifact = MustParseFoundryArtifact("./CounterContractArtifact.json")

func TestE2ESimpleCounterContract(t *testing.T) {
	env := NewMetabasedChainTestEnv(t)

	//
	// build a tx for Bob to deploy a counter contract
	deployCounterContractTxData, err := GenerateContractDeploymentCallData(counterContractArtifact)
	require.NoError(t, err, "failed to deploy counter contract")
	deployCounterContractTx := BuildGenericDataTx(
		env.T.Ctx(),
		env.L3Client, // NOTE: bob targets the L3 chain
		env.L3Signer,
		env.Secrets.Bob,
		deployCounterContractTxData,
	)
	deployCounterContractTxRaw, err := deployCounterContractTx.MarshalBinary()
	require.NoError(t, err, "failed to marshal transaction")

	//
	// Alice will act as the sequencer and sequence Bob's transaction
	env.SequenceL3TransactionRaw(env.Secrets.Alice, deployCounterContractTxRaw)

	//
	// contract should be deployed
	// counter should be 0
	receipt, err := geth.WaitForTransaction(deployCounterContractTx.Hash(), env.L3Client, 10*time.Second)
	require.NoError(t, err, "failed to get receipt")
	require.Equal(t, receipt.Status, types.ReceiptStatusSuccessful, "transaction should be successful")

	counterContractAddress := receipt.ContractAddress
	code, err := env.L3Client.CodeAt(env.T.Ctx(), counterContractAddress, nil)
	require.NoError(t, err, "failed to get code")
	require.Equal(t, code, counterContractArtifact.Bytecode, "code should be the counter contract")

	counterValue := getCounterValue(env.T, env.L3Client, counterContractAddress)
	require.Zero(t, counterValue, "counter should be 0")

	block, err := env.L3Client.BlockByNumber(env.T.Ctx(), nil)
	require.NoError(t, err, "failed to get block")
	require.GreaterOrEqual(t, block.NumberU64(), uint64(1), "block number should be greater than 1")

	//
	// TODO sequence a transaction to increment the counter
	// assert that the counter has been incremented
}

func getCounterValue(t helpers.StatefulTesting, client *ethclient.Client, counterContractAddress common.Address) *big.Int {
	counterValueBytes, err := client.CallContract(t.Ctx(), ethereum.CallMsg{
		To:   &counterContractAddress,
		Data: must(counterContractArtifact.ABI.Pack("number")),
	}, nil)
	require.NoError(t, err, "failed to get counter value")
	counterValue := new(big.Int)
	err = counterContractArtifact.ABI.UnpackIntoInterface(counterValue, "number", counterValueBytes)
	require.NoError(t, err, "failed to unpack counter value")
	return counterValue
}

// TODO write bridge test, see helpers.CrossLayerUser
