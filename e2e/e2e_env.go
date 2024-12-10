//nolint:mnd // lots of "magic numbers" for testing in this file
package e2e_test

import (
	"context"
	"crypto/ecdsa"
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"testing"
	"time"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/mocks"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/ethereum-optimism/optimism/op-chain-ops/foundry"
	"github.com/ethereum-optimism/optimism/op-chain-ops/genesis"
	"github.com/ethereum-optimism/optimism/op-e2e/actions/helpers"
	"github.com/ethereum-optimism/optimism/op-e2e/config"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/geth"
	"github.com/ethereum-optimism/optimism/op-service/testlog"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	tracelogger "github.com/ethereum/go-ethereum/eth/tracers/logger"
	"github.com/ethereum/go-ethereum/ethclient"
	gethlog "github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/params"
	"github.com/stretchr/testify/require"
)

func TestMain(m *testing.M) {
	// TODO parse env
	os.Exit(m.Run())
}

// - spawn a settlement chain with the bridge contract
// - spawn a sequencing chain with the sequencing contract
// - have a tool to send txs to the sequencing chain
// - send sequenced batches and deposits to the L2 chains
// - use translator code to derive the L2.5 block
// - hook up an execution engine to the output of the translator, make assertions on the output

type FakeChain struct {
	t       helpers.StatefulTesting
	genesis *core.Genesis
	miner   *helpers.L1Miner
}

var _ SourceChain = (*FakeChain)(nil)

func NewFakeChain(t helpers.StatefulTesting, deployParams *e2eutils.DeployParams, log gethlog.Logger, chainID *big.Int) *FakeChain {
	chainGenesis, err := genesis.BuildL1DeveloperGenesis(
		deployParams.DeployConfig.Copy(),
		config.L1Allocs(deployParams.AllocType),
		config.L1Deployments(deployParams.AllocType),
	)
	require.NoError(t, err, "failed to create genesis")

	// override the default with a random chain ID
	chainGenesis.Config.ChainID = chainID

	// NOTE: we're setting up the L2 chains as "L1s" here - doesn't particularly matter, we just want to be able to emulate the behavior of a chain
	miner := helpers.NewL1Miner(t, log, chainGenesis)

	// TODO this is just producing blocks ad-infinitum, but we can expand the logic to be able to stop/resume block production for certain test cases
	go func() {
		for {
			if t.Ctx().Err() != nil {
				return
			}
			miner.ActL1StartBlock(1)(t) // TODO right now we have a 1s block time, but we can make this configurable / variable on command
			// include all pending txs
			runnableTxs, _ := miner.Eth.TxPool().Content()
			for _, txs := range runnableTxs {
				for _, tx := range txs {
					miner.ActL1IncludeTxByHash(tx.Hash())(t)
				}
			}
			b := miner.ActL1EndBlock(t)
			log.Info("new block mined", "number", b.NumberU64(), "hash", b.Hash().Hex())
			miner.ActL1SafeNext(t)
			miner.ActL1FinalizeNext(t)
			time.Sleep(100 * time.Millisecond) // 100ms block time
		}
	}()

	return &FakeChain{
		t:       t,
		genesis: chainGenesis,
		miner:   miner,
		// node:    gethNode,
	}
}

func (c *FakeChain) Client() *ethclient.Client {
	return c.miner.EthClient()
}

func (c *FakeChain) Signer() types.Signer {
	return types.LatestSigner(c.genesis.Config)
}

/// ------------------------------------------------------------------------------------------------

// SourceChain is a generic interface for a chain that can be used as a source of transactions
// it is meant to be filled with either a fake miner, or a real chain connection
type SourceChain interface {
	Client() *ethclient.Client
	Signer() types.Signer
}

type MetabasedChainTestEnv struct {
	T helpers.StatefulTesting

	SequencingChain SourceChain
	SettlementChain SourceChain
	log             gethlog.Logger
	translator      *translator.OPTranslator
	// l3                   *helpers.L3

	MetabasedFactoryArtifact        *foundry.Artifact
	MetabasedSequencerChainArtifact *foundry.Artifact
	AlwaysAllowModuleArtifact       *foundry.Artifact

	MetabasedFactoryContractAddress    common.Address
	AlwaysAllowedModuleContractAddress common.Address
	L3SequencingContractAddress        common.Address
	BatcherInboxAddress                common.Address // TODO not filled

	Secrets   *e2eutils.Secrets
	Addresses *e2eutils.Addresses

	SequencingChainID *big.Int
	SettlementChainID *big.Int
	L3ChainID         *big.Int
}

func NewMetabasedChainTestEnv(gt *testing.T) *MetabasedChainTestEnv { //nolint:thelper // t is named gt and wrapped into a helpers.StatefulTesting object which is named t
	gt.Helper()
	t := helpers.NewDefaultTesting(gt)
	s := &MetabasedChainTestEnv{
		SequencingChainID: big.NewInt(1001),
		SettlementChainID: big.NewInt(1002),
		L3ChainID:         big.NewInt(1003),
	}
	s.T = t
	s.log = testlog.Logger(t, gethlog.LvlDebug)

	// NOTE: these files are generated by running "forge build" in the metabased-contracts dir
	// read contract artifacts
	s.MetabasedFactoryArtifact = MustParseFoundryArtifact("../metabased-contracts/out/MetabasedFactory.sol/MetabasedFactory.json")
	s.AlwaysAllowModuleArtifact = MustParseFoundryArtifact("../metabased-contracts/out/AlwaysAllowedModule.sol/AlwaysAllowedModule.json")
	s.MetabasedSequencerChainArtifact = MustParseFoundryArtifact("../metabased-contracts/out/MetabasedSequencerChain.sol/MetabasedSequencerChain.json")

	alwaysAllowModuleJSON, err := os.ReadFile("../metabased-contracts/out/AlwaysAllowedModule.sol/AlwaysAllowedModule.json")
	require.NoError(t, err, "failed to read  alwaysAllowModule json")
	require.NoError(t, json.Unmarshal(alwaysAllowModuleJSON, &s.AlwaysAllowModuleArtifact), "failed to unmarshal alwaysAllowModule json")

	// TODO swap for real chains here if configured (Note: and real wallets)
	{
		s.Secrets = must(e2eutils.DefaultMnemonicConfig.Secrets())
		s.Addresses = s.Secrets.Addresses()

		testParams := &e2eutils.TestParams{
			MaxSequencerDrift:   40,
			SequencerWindowSize: 120,
			ChannelTimeout:      120,
			L1BlockTime:         2, // 2s block time for the source chains
			UseAltDA:            false,
			AllocType:           config.AllocTypeStandard,
		}

		deployParams := e2eutils.MakeDeployParams(t, testParams)
		// setupData := e2eutils.Setup(t, deployParams, &e2eutils.AllocParams{PrefundTestUsers: true})

		sequencingChain := NewFakeChain(t, deployParams, s.log.New("role", "sequencing-chain-miner"), s.SequencingChainID)
		settlementChain := NewFakeChain(t, deployParams, s.log.New("role", "settlement-chain-miner"), s.SettlementChainID)
		s.SequencingChain = sequencingChain
		s.SettlementChain = settlementChain

	}

	s.deployNewMetabasedChain()

	// TODO deploy rollup contrants on the settlement layer

	// assert non-zero balance for all test actors on the source chains
	s.AssertNonZeroBalance(s.SequencingChain.Client(), s.Addresses.Alice)
	s.AssertNonZeroBalance(s.SequencingChain.Client(), s.Addresses.Bob)
	s.AssertNonZeroBalance(s.SequencingChain.Client(), s.Addresses.Mallory)

	s.AssertNonZeroBalance(s.SettlementChain.Client(), s.Addresses.Alice)
	s.AssertNonZeroBalance(s.SettlementChain.Client(), s.Addresses.Bob)
	s.AssertNonZeroBalance(s.SettlementChain.Client(), s.Addresses.Mallory)

	//
	// init op-translator

	batchProvider := translator.NewMetaBasedBatchProvider(
		rpc.NewRPCClient(s.SettlementChain.Client(), s.SettlementChain.Client().Client(), nil), // TODO is the receipt fetcher necessary?
		rpc.NewRPCClient(s.SequencingChain.Client(), s.SequencingChain.Client().Client(), nil), // TODO is the receipt fetcher necessary?
		s.L3SequencingContractAddress,
		0, // TODO we currently only handle "fully metabased chains", this should be changed if we want to support backfilled chains
		1, // TODO this should be deprecated to handle variable block time chains
		&mocks.MockMetrics{},
		s.log.New("role", "batch-provider"),
	)

	s.translator = translator.NewOPTranslator(
		rpc.NewRPCClient(s.SettlementChain.Client(), s.SettlementChain.Client().Client(), nil), // TODO is the receipt fetcher necessary?
		batchProvider,
		nil, // TODO backfill provider is currently nil
		must(translator.NewSigner(hex.EncodeToString(crypto.FromECDSA(s.Secrets.Bob)), s.SettlementChainID)), // TODO Bob is hardcoded as the signer - define what address should be used here
		&s.BatcherInboxAddress,
		&mocks.MockMetrics{},
		s.log.New("role", "translator"),
	)

	// TODO continue
	// TODO use deriver to obtain L3 state from the translator output

	return s
}

func (s *MetabasedChainTestEnv) AssertNonZeroBalance(client *ethclient.Client, addr common.Address) {
	bal, err2 := client.BalanceAt(s.T.Ctx(), s.Addresses.Alice, nil)
	require.NoError(s.T, err2, "failed to get alice balance")
	require.True(s.T, bal.Cmp(e2eutils.Ether(0)) > 0, "alice balance is too low")
}

func (s *MetabasedChainTestEnv) deployNewMetabasedChain() {
	seqClient := s.SequencingChain.Client()

	// TODO skip if factory contract is already provided
	//
	// deploy the metabased factory
	{
		tx := BuildGenericDataTx(s.T.Ctx(), seqClient, s.SequencingChain.Signer(), s.Secrets.Alice, must(DeployContractCallData(s.MetabasedFactoryArtifact)))
		receipt := s.sendTxWaitReceipt(seqClient, tx)
		require.Equal(s.T, receipt.Status, types.ReceiptStatusSuccessful)
		s.MetabasedFactoryContractAddress = receipt.ContractAddress
	}

	// TODO skip if a sequencing module address is already provided

	//
	// deploy the always allowed module
	{
		tx := BuildGenericDataTx(s.T.Ctx(), seqClient, s.SequencingChain.Signer(), s.Secrets.Alice, must(DeployContractCallData(s.AlwaysAllowModuleArtifact)))
		receipt := s.sendTxWaitReceipt(seqClient, tx)
		require.Equal(s.T, receipt.Status, types.ReceiptStatusSuccessful)
		s.AlwaysAllowedModuleContractAddress = receipt.ContractAddress
	}

	// NOTE: for now, we're assuming all tests will be performed on a freshly minted metabased chain.
	// it should be possible to run tests on a chain that has already been deployed, but the sequencer keys would have to be provided.
	//
	// create the metabased chain
	{
		tx := types.MustSignNewTx(s.Secrets.Alice, s.SequencingChain.Signer(), &types.DynamicFeeTx{
			ChainID:   must(seqClient.ChainID(s.T.Ctx())),
			Nonce:     must(seqClient.PendingNonceAt(s.T.Ctx(), s.Addresses.Alice)),
			GasFeeCap: new(big.Int).Add(must(seqClient.HeaderByNumber(s.T.Ctx(), nil)).BaseFee, big.NewInt(2*params.GWei)),
			Gas:       10_000_000,
			Value:     e2eutils.Ether(0),
			To:        &s.MetabasedFactoryContractAddress,
			Data:      must(s.MetabasedFactoryArtifact.ABI.Pack("createMetabasedSequencerChain", s.L3ChainID, s.Addresses.Alice, s.AlwaysAllowedModuleContractAddress)),
		})
		receipt := s.sendTxWaitReceipt(seqClient, tx)
		require.Equal(s.T, receipt.Status, types.ReceiptStatusSuccessful)

		event := s.MetabasedFactoryArtifact.ABI.Events["MetabasedSequencerChainCreated"]
		argIndex := -1
		for i := 0; i < len(event.Inputs); i++ {
			if event.Inputs[i].Name == "metabasedSequencerChainAddress" {
				argIndex = i
				break
			}
		}
		require.NotEqual(s.T, argIndex, -1, "metabasedSequencerChainAddress argument not found")

		for _, log := range receipt.Logs {
			if log.Topics[0] == event.ID {
				s.L3SequencingContractAddress = common.BytesToAddress(log.Topics[argIndex+1].Bytes())
				break
			}
		}
		require.NotEqual(s.T, s.L3SequencingContractAddress, (common.Address{}), "MetabasedSequencerChainCreated event not found")

		// sanity check
		code, err := seqClient.CodeAt(s.T.Ctx(), s.L3SequencingContractAddress, nil)
		require.NoError(s.T, err, "failed to get code")
		require.NotEqual(s.T, len(code), 0, "sequencing contract not deployed")
	}
}

func (s *MetabasedChainTestEnv) sendTxWaitReceipt(client *ethclient.Client, tx *types.Transaction) *types.Receipt {
	err := client.SendTransaction(context.Background(), tx)
	require.NoError(s.T, err, "failed to send tx")
	receipt, err := geth.WaitForTransaction(tx.Hash(), client, 5*time.Second) // TODO make this timeout configurable
	require.NoError(s.T, err, "failed to get receipt")
	return receipt
}

// ------------------------------------------------------------------------------------------------
// public methods

func (s *MetabasedChainTestEnv) SequenceL3TransactionRaw(sender *ecdsa.PrivateKey, data []byte) {
	seqClient := s.SequencingChain.Client()
	txData, err := s.MetabasedSequencerChainArtifact.ABI.Pack("processTransactionRaw", data)
	require.NoError(s.T, err, "failed to pack tx data")
	tx := BuildGenericDataTx(s.T.Ctx(), seqClient, s.SequencingChain.Signer(), sender, txData, s.L3SequencingContractAddress)
	receipt := s.sendTxWaitReceipt(seqClient, tx)
	require.Equal(s.T, receipt.Status, types.ReceiptStatusSuccessful)
}

// ------------------------------------------------------------------------------------------------
// utils

func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

func DeployContractCallData(contractArtifact *foundry.Artifact, args ...any) ([]byte, error) {
	constructorArgs, err := contractArtifact.ABI.Pack("", args...)
	if err != nil {
		return nil, err
	}
	data := []byte{}
	data = append(data, contractArtifact.Bytecode.Object...)
	data = append(data, constructorArgs...)
	return data, nil
}

func MustParseFoundryArtifact(path string) *foundry.Artifact {
	jsonContent, err := os.ReadFile(path)
	if err != nil {
		panic(fmt.Errorf("failed to read metabased factory json: %w", err))
	}
	var artifact foundry.Artifact
	err = json.Unmarshal(jsonContent, &artifact)
	if err != nil {
		panic(fmt.Errorf("failed to unmarshal metabased factory json: %w", err))
	}
	return &artifact
}

func BuildGenericDataTx(ctx context.Context, client *ethclient.Client, signer types.Signer, sender *ecdsa.PrivateKey, data []byte, to ...common.Address) *types.Transaction {
	var txTo *common.Address
	if len(to) > 0 {
		txTo = &to[0]
	}
	return types.MustSignNewTx(sender, signer, &types.DynamicFeeTx{
		ChainID:   must(client.ChainID(ctx)),
		Nonce:     must(client.PendingNonceAt(ctx, crypto.PubkeyToAddress(sender.PublicKey))),
		GasFeeCap: must(client.HeaderByNumber(ctx, nil)).BaseFee,
		Gas:       10_000_000,
		Value:     e2eutils.Ether(0),
		Data:      data,
		To:        txTo,
	})
}

type traceConfig struct {
	*tracelogger.Config
	Tracer  string  `json:"tracer"`
	Timeout *string `json:"timeout"`
	// Config specific to given tracer. Note struct logger
	// config are historically embedded in main object.
	TracerConfig json.RawMessage
}

// TraceTransaction is just an useful debugging tool
func TraceTransaction(client *ethclient.Client, txHash common.Hash) {
	var result json.RawMessage
	err := client.Client().CallContext(context.Background(), &result, "debug_traceTransaction", txHash, traceConfig{
		Config: &tracelogger.Config{
			EnableMemory:     true,
			DisableStack:     true,
			DisableStorage:   true,
			EnableReturnData: false,
			Debug:            true,
			Limit:            0,
			Overrides:        nil,
		},
		Tracer:       "callTracer",
		Timeout:      nil,
		TracerConfig: nil,
	})
	if err != nil {
		panic(err)
	}
	println(string(result))
}
