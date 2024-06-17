use futures::Future;
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_primitives::{address, Address};
use reth_tracing::tracing::info;

const BASED_SEQUENCER_CHAIN: Address = address!("61164FeB6e29682dEbcD1c9de07f6726838FDd4e");

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
    while let Some(notification) = ctx.notifications.recv().await {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                for (_, block) in new.blocks().iter() {
                    if block
                        .body
                        .iter()
                        .any(|tx| tx.to() == Some(BASED_SEQUENCER_CHAIN))
                    {
                        info!(
                            "Block {:?} contains txs from/to BasedSequencerChain address",
                            block.header.number
                        );
                        ctx.events
                            .send(ExExEvent::FinishedHeight(block.header.number))?;
                    }
                }
            }
            ExExNotification::ChainReorged { old, new } => {
                info!(from_chain = ?old.range(), to_chain = ?new.range(), "Reorg from {:?} to {:?}", old.range(), new.range());
            }
            ExExNotification::ChainReverted { old } => {
                info!("Reverted chain from {:?}", old.range());
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

    #[tokio::test]
    async fn test_filter_block_containing_based_sequencer_chain_address_txs() -> eyre::Result<()> {
        let (ctx, mut handle) = test_exex_context().await?;
        let head = ctx.head;

        let transaction = Transaction::Eip4844(TxEip4844 {
            to: BASED_SEQUENCER_CHAIN,
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

        exex.poll_once().await?;
        handle.assert_event_finished_height(head.number + 1)?;

        Ok(())
    }
}
