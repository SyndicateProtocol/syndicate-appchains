//! The `synd-migration` CLI tool for migrating Nitro appchain databases.
//!
//! This tool provides functionality to update chain configuration parameters
//! in a Nitro node's database, such as the `DataAvailabilityCommittee` flag.

use clap::Parser;
use std::path::PathBuf;
use tracing::error;

#[derive(Parser, Debug)]
#[command(
    name = "synd-migration",
    version,
    about = "Migration tool for Syndicate appchains",
    long_about = "CLI tool for inspecting existing Nitro rollups DBs in preparation to migrate to the syndicate appchains stack"
)]
#[allow(missing_docs)]
struct Args {
    /// Path to the Nitro database directory (parent of l2chaindata, e.g., /data/nitro)
    #[arg(short = 'd', long, env = "NITRO_DB_PATH")]
    nitro_db_path: PathBuf,
}

#[tokio::main]
#[cfg(feature = "rocksdb")]
async fn main() {
    use synd_migration_cli::migration::get_migration_data;

    let args = Args::parse();

    shared::tracing::setup_global_logging()
        .unwrap_or_else(|e| panic!("failed to setup logging: {e}"));

    if let Err(e) = get_migration_data(&args.nitro_db_path).await {
        error!("\nMigration failed. {e}");
        std::process::exit(1);
    }
}

#[tokio::main]
#[cfg(not(feature = "rocksdb"))]
async fn main() -> eyre::Result<()> {
    Ok(())
}
