package flags

import (
	"fmt"
	"time"

	"github.com/urfave/cli/v2"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	"github.com/ethereum-optimism/optimism/op-service/oppprof"
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

	// AltDA
	AltDAURL = &cli.StringFlag{
		Name:    "alt-da-url",
		Usage:   "URL for the AltDA service",
		EnvVars: []string{"ALT_DA_URL"},
	}

	//////////////////////////
	// Optional flags
	//////////////////////////

	PollInterval = &cli.DurationFlag{
		Name:    "poll-interval",
		Usage:   "Interval at which the service will poll the source chains for new data",
		EnvVars: prefixEnvVars("POLL_INTERVAL"),
		Value:   time.Second * 10,
	}
)

var requiredFlags = []cli.Flag{
	SettlementChainRPCURL,
	SequencingChainRPCURL,
	SequencingContractAddress,
	L3RPCURL,
	AltDAURL,
}

var optionalFlags = []cli.Flag{
	PollInterval,
}

func init() {
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
