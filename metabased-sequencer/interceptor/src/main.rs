use interceptor::presentation::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::init_tracing_subscriber();
    cli::run().await
}
