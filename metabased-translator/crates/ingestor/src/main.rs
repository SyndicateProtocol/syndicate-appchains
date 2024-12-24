use env_logger;
use ingestor;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let rpc_url = "https://base.llamarpc.com"; //"https://eth.llamarpc.com";
    let start_block = 19486923;
    let polling_interval = Duration::from_secs(1);

    // Initialize the logger
    env_logger::init();

    let (mut ingestor, mut receiver) =
        ingestor::ingestor::Ingestor::new(rpc_url, start_block, 100, polling_interval)
            .await
            .expect("Failed to create ingestor");

    // Spawn a task to log what the receiver receives
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            log::info!(
                "[Ingestor] Received block number: {:?}",
                message.header.inner.number
            );
        }
    });

    ingestor
        .start_polling()
        .await
        .expect("Failed to start polling");
}
