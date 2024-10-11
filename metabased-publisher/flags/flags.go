package flags

import (
	"fmt"

	"github.com/urfave/cli/v2"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
	"github.com/ethereum-optimism/optimism/op-service/txmgr"
)

const EnvVarPrefix = "MB_PUBLISHER"

func prefixEnvVars(name string) []string {
	return opservice.PrefixEnvVar(EnvVarPrefix, name)
}

var (
	//////////////////////////
	// Required flags
	//////////////////////////

	// settlement chain
	SettlementChainRPCURL = &cli.StringFlag{
		Name:    "settlement-chain-rpc",
		Usage:   "JSON-RPC URL for the settlement chain",
		EnvVars: []string{"SETTLEMENT_CHAIN_ADDR"},
	}
	// state commitments
	// L2OutputOracleAddr = &cli.StringFlag{
	// 	Name:    "l2-output-oracle-address",
	// 	Usage:   "Address of the L2OutputOracle contract",
	// 	EnvVars: prefixEnvVars("L2_OUTPUT_ORACLE_ADDRESS"),
	// }
	// DisputeGameFactoryAddress = &cli.StringFlag{
	// 	Name:    "game-factory-address",
	// 	Usage:   "Address of the DisputeGameFactory contract",
	// 	EnvVars: prefixEnvVars("GAME_FACTORY_ADDRESS"),
	// }
	// ProposalInterval = &cli.DurationFlag{
	// 	Name:    "proposal-interval",
	// 	Usage:   "Interval between submitting L3 output proposals when the dispute game factory address is set",
	// 	EnvVars: prefixEnvVars("PROPOSAL_INTERVAL"),
	// }
	// DisputeGameType = &cli.UintFlag{
	// 	Name:    "game-type",
	// 	Usage:   "Dispute game type to create via the configured DisputeGameFactory",
	// 	Value:   0,
	// 	EnvVars: prefixEnvVars("GAME_TYPE"),
	// }

	// DA commitments
	BatchInboxAddress = &cli.StringFlag{
		Name:    "batch-inbox-address",
		Usage:   "Address of the BatchInbox contract",
		EnvVars: []string{"BATCH_INBOX_ADDRESS"},
	}

	// sequencing chain
	SequencingChainRPCURL = &cli.StringFlag{
		Name:    "sequencing-chain-rpc",
		Usage:   "JSON-RPC URL for the Sequencing chain",
		EnvVars: []string{"SEQUENCING_CHAIN_ADDR"},
	}
	SequencingContractAddress = &cli.StringFlag{
		Name:    "sequencing-contract-address",
		Usage:   "Address of the Sequencing contract",
		EnvVars: []string{"SEQUENCING_CONTRACT_ADDRESS"},
	}

	// L3 Metabased Chain
	L3RPCURL = &cli.StringFlag{
		Name:    "l3-rpc",
		Usage:   "JSON-RPC URL for the L3 Metabased Chain Execution Client",
		EnvVars: []string{"META_BASED_CHAIN_ADDR"},
	}

	//////////////////////////
	// Optional flags
	//////////////////////////

	PollInterval = &cli.DurationFlag{
		Name:    "poll-interval",
		Usage:   "Interval at which the service will poll the source chains for new data",
		EnvVars: prefixEnvVars("POLL_INTERVAL"),
	}

// TODO add if necessary
//
//	MaxPendingTransactions = &cli.Uint64Flag{
//		Name:    "max-pending-tx",
//		Usage:   "The maximum number of pending transactions. 0 for no limit.",
//		Value:   1,
//		EnvVars: prefixEnvVars("MAX_PENDING_TX"),
//	}
)

var requiredFlags = []cli.Flag{
	SettlementChainRPCURL,
	BatchInboxAddress,
	SequencingChainRPCURL,
	SequencingContractAddress,
	L3RPCURL,
}

var optionalFlags = []cli.Flag{
	PollInterval,
}

func init() {
	optionalFlags = append(optionalFlags, txmgr.CLIFlags(EnvVarPrefix)...)
	optionalFlags = append(optionalFlags, oplog.CLIFlags(EnvVarPrefix)...)
	optionalFlags = append(optionalFlags, opmetrics.CLIFlags(EnvVarPrefix)...)
	optionalFlags = append(optionalFlags, oppprof.CLIFlags(EnvVarPrefix)...)

	Flags = append(requiredFlags, optionalFlags...) //nolint:gocritic // false positive
}

// Flags contains the list of configuration options available to the binary.
var Flags []cli.Flag

func CheckRequired(ctx *cli.Context) error {
	for _, f := range requiredFlags {
		if !ctx.IsSet(f.Names()[0]) {
			return fmt.Errorf("flag %s is required", f.Names()[0])
		}
	}
	return nil
}
