package main

import (
	"context"
	"os"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/flags"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/translator"
	"github.com/urfave/cli/v2"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum-optimism/optimism/op-service/ctxinterrupt"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/ethereum/go-ethereum/log"
)

var (
	Version   = "v0.0.1"
	GitCommit = ""
	GitDate   = ""
)

func main() {
	oplog.SetupDefaults()

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
	err := app.RunContext(ctx, os.Args)
	if err != nil {
		log.Crit("Application failed", "message", err)
	}
}
