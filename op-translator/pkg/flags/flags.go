package flags

import (
	"fmt"
	"time"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/urfave/cli/v2"
)

const EnvVarPrefix = "OP_TRANSLATOR"

const (
	MaxFrameSize = 1_000_000
)

func prefixEnvVars(name string) []string {
	return opservice.PrefixEnvVar(EnvVarPrefix, name)
}

var (
	//////////////////////////
	// Required flags
	//////////////////////////

	SettlementChainRPCURL = &cli.StringFlag{
		Name:    "settlement_chain_rpc_url",
		Usage:   "Settlement chain address",
		EnvVars: []string{"SETTLEMENT_CHAIN_RPC_URL"},
	}

	SequencingChainRPCURL = &cli.StringFlag{
		Name:    "sequencing_chain_rpc_url",
		Usage:   "Sequencing chain address",
		EnvVars: []string{"SEQUENCING_CHAIN_RPC_URL"},
	}

	MetaBasedChainRPCURL = &cli.StringFlag{
		Name:    "meta_based_chain_rpc_url",
		Usage:   "Meta based chain address",
		EnvVars: []string{"META_BASED_CHAIN_RPC_URL"},
	}

	MetafillerURL = &cli.StringFlag{
		Name:    "metafiller_url",
		Usage:   "Metafiller URL",
		EnvVars: []string{"METAFILLER_URL"},
	}

	SequencingContractAddress = &cli.StringFlag{
		Name:    "sequencing_contract_address",
		Usage:   "Sequencing contract address",
		EnvVars: []string{"SEQUENCING_CONTRACT_ADDRESS"},
	}

	BatchInboxAddress = &cli.StringFlag{
		Name:    "batch_inbox_address",
		Usage:   "Batch inbox address",
		EnvVars: []string{"BATCH_INBOX_ADDRESS"},
	}

	BatcherPrivateKey = &cli.StringFlag{
		Name:    "batcher_private_key",
		Usage:   "Batcher private key",
		EnvVars: []string{"BATCHER_PRIVATE_KEY"},
	}

	SettlementChainID = &cli.Uint64Flag{
		Name:    "settlement_chain_id",
		Usage:   "Settlement chain id",
		EnvVars: []string{"SETTLEMENT_CHAIN_ID"},
	}

	CutoverEpochBlock = &cli.Uint64Flag{
		Name:    "cutover_epoch_block",
		Usage:   "Cutover epoch block",
		EnvVars: []string{"CUTOVER_EPOCH_BLOCK"},
	}

	SettlementChainBlockTime = &cli.DurationFlag{
		Name:    "settlement_chain_block_time",
		Usage:   "Settlement chain block time",
		EnvVars: []string{"SETTLEMENT_CHAIN_BLOCK_TIME"},
	}

	SettlementStartBlock = &cli.Uint64Flag{
		Name:    "settlement_start_block",
		Usage:   "Settlement chain start block",
		EnvVars: []string{"SETTLEMENT_START_BLOCK"},
	}

	//////////////////////////
	// Optional flags
	//////////////////////////

	FrameSize = &cli.IntFlag{
		Name:    "frame_size",
		Usage:   "Size of each frame in bytes. Max is 1,000,000",
		EnvVars: []string{"FRAME_SIZE"},
		Value:   1024, //nolint:mnd // default
	}

	Port = &cli.IntFlag{
		Name:    "port",
		Usage:   "Server port number for the app",
		EnvVars: prefixEnvVars("PORT"),
		Value:   8546, //nolint:mnd // default
	}

	ReadTimeout = &cli.DurationFlag{
		Name:    "server_read_timeout",
		Usage:   "Server read timeout",
		EnvVars: prefixEnvVars("SERVER_READ_TIMEOUT"),
		Value:   5 * time.Second, //nolint:mnd // default
	}

	WriteTimeout = &cli.DurationFlag{
		Name:    "server_write_timeout",
		Usage:   "Server write timeout",
		EnvVars: prefixEnvVars("SERVER_WRITE_TIMEOUT"),
		Value:   10 * time.Second, //nolint:mnd // default
	}
)

var requiredFlags = []cli.Flag{
	SettlementChainRPCURL,
	SequencingChainRPCURL,
	MetaBasedChainRPCURL,
	MetafillerURL,
	SequencingContractAddress,
	BatchInboxAddress,
	BatcherPrivateKey,
	SettlementChainID,
	CutoverEpochBlock,
	SettlementChainBlockTime,
	SettlementStartBlock,
}

var optionalFlags = []cli.Flag{
	FrameSize,
	Port,
	ReadTimeout,
	WriteTimeout,
}

func init() {
	optionalFlags = append(optionalFlags, oplog.CLIFlags(EnvVarPrefix)...)
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
