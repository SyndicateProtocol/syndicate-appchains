package e2e_test

import (
	"testing"

	"github.com/stretchr/testify/require"
)

var counterContractArtifact = MustParseFoundryArtifact("./CounterContractArtifact.json")

func TestE2EBasic(t *testing.T) {
	env := NewMetabasedChainTestEnv(t)

	deployCounterContractTxData, err := DeployContractCallData(counterContractArtifact)
	require.NoError(t, err, "failed to deploy counter contract")

	deployCounterContractTx := BuildGenericDataTx(
		env.T.Ctx(),
		env.SequencingChain.Client(),
		env.SequencingChain.Signer(),
		env.Secrets.Bob,
		deployCounterContractTxData,
	)
	deployCounterContractTxRaw, err := deployCounterContractTx.MarshalBinary()
	require.NoError(t, err, "failed to marshal transaction")

	// Alice will act as the sequencer and send a transaction signed by Bob to deploy a counter contract
	env.SequenceL3TransactionRaw(env.Secrets.Alice, deployCounterContractTxRaw)

	// TODO assert L3 state
}
