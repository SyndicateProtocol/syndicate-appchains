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

	// op-translator
	OpTranslatorRPCURL = &cli.StringFlag{
		Name:    "op-translator-rpc",
		Usage:   "JSON-RPC URL for the op-translator service",
		EnvVars: []string{"OP_TRANSLATOR_RPC_URL"},
	}

	BatcherAddress = &cli.StringFlag{
		Name:    "batcher-address",
		Usage:   "Address of the Batcher",
		EnvVars: []string{"BATHER_ADDRESS"},
	}
	BatchInboxAddress = &cli.StringFlag{
		Name:    "batch-inbox-address",
		Usage:   "Address of the Batch Inbox",
		EnvVars: []string{"BATCH_INBOX_ADDRESS"},
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
		Value:   time.Second * 60, //nolint:mnd // default value
	}

	NetworkTimeout = &cli.DurationFlag{
		Name:    "network-timeout",
		Usage:   "Timeout for network operations",
		EnvVars: prefixEnvVars("NETWORK_TIMEOUT"),
		Value:   time.Second * 20, //nolint:mnd // default value
	}

	BlobUploadTimeout = &cli.DurationFlag{
		Name:    "blob-upload-timeout",
		Usage:   "Timeout for blob upload operations",
		EnvVars: prefixEnvVars("BLOB_UPLOAD_TIMEOUT"),
		Value:   time.Minute * 10, //nolint:mnd // default value
	}
)

var requiredFlags = []cli.Flag{
	OpTranslatorRPCURL,
	BatcherAddress,
	BatchInboxAddress,
	AltDAURL,
}

var optionalFlags = []cli.Flag{
	PollInterval,
	NetworkTimeout,
	BlobUploadTimeout,
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
