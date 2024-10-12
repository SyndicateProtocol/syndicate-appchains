package main

import (
	"context"
	"os"

	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/flags"
	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/metrics"
	"github.com/SyndicateProtocol/metabased-rollup/metabased-publisher/publisher"
	"github.com/urfave/cli/v2"

	opservice "github.com/ethereum-optimism/optimism/op-service"
	"github.com/ethereum-optimism/optimism/op-service/cliapp"
	"github.com/ethereum-optimism/optimism/op-service/ctxinterrupt"
	oplog "github.com/ethereum-optimism/optimism/op-service/log"
	"github.com/ethereum-optimism/optimism/op-service/metrics/doc"
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
	app.Name = "metabased-publisher"
	app.Usage = "Metabase Publisher Service"
	app.Description = "Service for publishing metabased chains commitments to the settlement chain"
	app.Action = cliapp.LifecycleCmd(publisher.Main(Version))
	app.Commands = []*cli.Command{
		{
			Name:        "doc",
			Subcommands: doc.NewSubcommands(metrics.NewMetrics("default")),
		},
	}

	ctx := ctxinterrupt.WithSignalWaiterMain(context.Background())
	err := app.RunContext(ctx, os.Args)
	if err != nil {
		log.Crit("Application failed", "message", err)
	}
}
