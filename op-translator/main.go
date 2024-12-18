package main

import (
	"context"
	"fmt"
	"os"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/urfave/cli/v2"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum-optimism/optimism/op-service/ctxinterrupt"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/log"
)

var (
	Version   = "v0.0.1"
	GitCommit = ""
	GitDate   = ""
)

func main() {
	oplog.SetupDefaults()

	batch, err := types.NewBatch("0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233", "0x0", "0x103ac73bf5b87545625259521c3c53c9f51f08c782831a5eb216c6383ddb201d", "0x6612AD22", []hexutil.Bytes{})
	if err != nil {
		log.Crit("Failed to create batch", "message", err)
	}

	frames, err := batch.GetFrames(flags.MaxFrameSize)
	for _, frame := range frames {
		fmt.Println("i", frame)
	}

	if err != nil {
		log.Crit("Failed to get frames", "message", err)
	}

	data, err := types.ToData(frames)
	if err != nil {
		log.Crit("Failed to convert frames to data", "message", err)
	}

	// print out data as hex
	fmt.Println(hexutil.Encode(data))

	app := cli.NewApp()
	app.Flags = cliapp.ProtectFlags(flags.Flags)
	app.Version = opservice.FormatVersion(Version, GitCommit, GitDate, "")
	app.Name = "op-translator"
	app.Usage = "op-translator Service"
	app.Description = "The op-translator service connects the sequencing chain and the settlement chain into a translated block that is used to derive the metabased L3 chain"
	app.Action = cliapp.LifecycleCmd(translator.Main(Version))
	app.Commands = []*cli.Command{
		{
			Name: "doc",
			// Subcommands: doc.NewSubcommands(metrics.NewMetrics("default")),
		},
	}

	ctx := ctxinterrupt.WithSignalWaiterMain(context.Background())
	err = app.RunContext(ctx, os.Args)
	if err != nil {
		log.Crit("Application failed", "message", err)
	}
}
