use clap_builder::Parser;
use reth::cli::Cli;
use reth_node_optimism::{args::RollupArgs, rpc::SequencerClient, OptimismNode};
use std::sync::Arc;

mod engine;
mod exex;
mod l3_block;
mod manager;

use exex::SynExEx;

fn main() {
    reth::sigsegv_handler::install();

    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    if let Err(err) = Cli::<RollupArgs>::parse().run(|builder, rollup_args| async move {
        let handle = builder
            .node(OptimismNode::new(rollup_args.clone()))
            .extend_rpc_modules(move |ctx| {
                // register sequencer tx forwarder
                if let Some(sequencer_http) = rollup_args.sequencer_http {
                    ctx.registry
                        .set_eth_raw_transaction_forwarder(Arc::new(SequencerClient::new(
                            sequencer_http,
                        )));
                }

                Ok(())
            })
            .install_exex("SynExEx", move |ctx| async {
                Ok(SynExEx::new(ctx)?.start())
            })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    }) {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
