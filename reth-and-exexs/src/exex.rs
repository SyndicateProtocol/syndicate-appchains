use dotenv::dotenv;
use std::env;

use reth_exex::{ExExContext, ExExEvent};
use reth_node_api::FullNodeComponents;
use reth_primitives::Address;

use crate::{
    engine::{Engine, EngineApi},
    manager::Manager,
};

pub struct SynExEx<Node: FullNodeComponents, E: Engine> {
    ctx: ExExContext<Node>,
    manager: Manager<E>,
}

impl<Node: FullNodeComponents> SynExEx<Node, EngineApi> {
    pub fn new(ctx: ExExContext<Node>) -> eyre::Result<Self> {
        // TODO [SEQ-47]: Do this in a mire robust/smarter way
        dotenv().ok();
        let sequencer_address = env::var("CONTRACT_ADDRESS")
            .unwrap_or("0x0000000000000000000000000000000000000000".to_string())
            .parse::<Address>()
            .unwrap();

        let manager = Manager::new(sequencer_address);
        Ok(Self { ctx, manager })
    }

    pub async fn start(mut self) -> eyre::Result<()> {
        // Process all new chain state notifications
        while let Some(notification) = self.ctx.notifications.recv().await {
            if let Some(reverted_chain) = notification.reverted_chain() {
                self.manager.revert(&reverted_chain)?;
            }

            if let Some(committed_chain) = notification.committed_chain() {
                self.manager.commit(&committed_chain).await?;
                self.ctx
                    .events
                    .send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_exex_test_utils::{test_exex_context, PollOnce};
    use reth_primitives::{Block, Header, Transaction, TxEip4844};
    use reth_provider::{Chain, ExecutionOutcome};
    use reth_testing_utils::generators::sign_tx_with_random_key_pair;
    use std::pin::{pin, Pin};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_syn_exex() -> eyre::Result<()> {
        dotenv().ok();

        let (ctx, handle) = test_exex_context().await?;
        let head = ctx.head;

        let sequencer_address = "0x0000000000000000000000000000000000000000"
            .to_string()
            .parse::<Address>()
            .unwrap();

        let transaction = Transaction::Eip4844(TxEip4844 {
            to: sequencer_address,
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

        let mut exex: Pin<&mut _> = pin!(super::SynExEx::new(ctx)?.start());
        handle.assert_events_empty();

        // Wait a bit to ensure the notification is processed
        sleep(Duration::from_millis(100)).await;

        exex.poll_once().await?;

        Ok(())
    }
}
