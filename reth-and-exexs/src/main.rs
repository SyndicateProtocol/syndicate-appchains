use dotenv::dotenv;
use std::env;

use futures::Future;
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_primitives::Address;
use reth_tracing::tracing::{debug, info, warn};
use serde_json::json;

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(filter_block_containing_based_sequencer_chain_address_txs(
        ctx,
    ))
}

async fn filter_block_containing_based_sequencer_chain_address_txs<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
) -> eyre::Result<()> {
    dotenv().ok();

    let based_sequencer_address = env::var("CONTRACT_ADDRESS")
        .unwrap_or("0x0000000000000000000000000000000000000000".to_string())
        .parse::<Address>()
        .unwrap();

    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                for block in new.blocks_iter() {
                    let block_number = block.header.number;
                    let mut tx_found = false;
                    if let Some(tx) = block.transactions().find(|tx| {
                        tx.to() == Some(based_sequencer_address)
                        // TODO: Add check against function signature as well
                    }) {
                        tx_found = true;
                        info!(
                            target: "based_sequencer_chain",
                            event = "tx_found",
                            tx_hash = ?tx.hash(),
                            block_number = ?block_number,
                            address = ?based_sequencer_address,
                            "Block contains a tx to the BasedSequencerChain address"
                        );

                        // TODO [SEQ-28]: Parse L3 block from TX

                        // TODO [SEQ-29]: L3 Block -> ExecutionPayload

                        // TODO: Call engine_NewPayload

                        // TODO: Call engine_ForkchoiceUpdated
                    } else {
                        // We can avoid logging this in production since it will
                        // be very high frequency. We can also get it implicitly
                        // by looking at block numbers that do not have an
                        // `info` log associated with them
                        debug!(
                            target: "based_sequencer_chain",
                            event = "no_tx_found",
                            block_number = ?block_number,
                            address = ?based_sequencer_address,
                            "Block does not contain a tx to the BasedSequencerChain address"
                        );
                    }

                    // Log stats in a structured format
                    info!(
                        target: "based_sequencer_chain_stats",
                        json = json!({
                            "block_number": block_number,
                            "contains_tx": tx_found,
                            "address": based_sequencer_address,
                        }).to_string()
                    );
                }
                if let Some(committed_chain) = notification.committed_chain() {
                    ctx.events
                        .send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
                }
            }
            ExExNotification::ChainReorged { old, new } => {
                warn!(
                    target: "based_sequencer_chain",
                    event = "chain_reorg",
                    from_chain = ?old.range(),
                    to_chain = ?new.range(),
                    "Chain reorg occurred"
                );
            }
            ExExNotification::ChainReverted { old } => {
                warn!(
                    target: "based_sequencer_chain",
                    event = "chain_revert",
                    reverted_chain = ?old.range(),
                    "Chain reverted"
                );
            }
        };
    }
    Ok(())
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("BlockFilterExEx", exex_init)
            .launch()
            .await?;
        handle.wait_for_node_exit().await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_exex_test_utils::{test_exex_context, PollOnce};
    use reth_primitives::{Block, Header, Transaction, TxEip4844};
    use reth_provider::{Chain, ExecutionOutcome};
    use reth_testing_utils::generators::sign_tx_with_random_key_pair;
    use std::pin::pin;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_filter_block_containing_based_sequencer_chain_address_txs() -> eyre::Result<()> {
        dotenv().ok();

        let (ctx, mut handle) = test_exex_context().await?;
        let head = ctx.head;

        let based_sequencer_address = "0x0000000000000000000000000000000000000000"
            .to_string()
            .parse::<Address>()
            .unwrap();

        let transaction = Transaction::Eip4844(TxEip4844 {
            to: based_sequencer_address,
            ..Default::default()
        });

        // Sign the transaction
        let transaction_signed = sign_tx_with_random_key_pair(&mut rand::thread_rng(), transaction);

        let block = Block {
            header: Header {
                number: head.number + 1,
                ..Default::default()
            },
            body: vec![transaction_signed],
            ..Default::default()
        }
        .seal_slow()
        .seal_with_senders()
        .ok_or_else(|| eyre::eyre!("failed to recover senders"))?;

        handle
            .send_notification_chain_committed(Chain::from_block(
                block,
                ExecutionOutcome::default(),
                None,
            ))
            .await?;

        let mut exex = pin!(super::exex_init(ctx).await?);
        handle.assert_events_empty();

        // Wait a bit to ensure the notification is processed
        sleep(Duration::from_millis(100)).await;

        exex.poll_once().await?;
        handle.assert_event_finished_height(head.number + 1)?;

        Ok(())
    }
}
