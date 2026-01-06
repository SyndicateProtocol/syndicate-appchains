//! End-to-end tests for snapshot functionality

#[cfg(feature = "rocksdb")]
use {
    eyre::Result,
    rocksdb::DB,
    std::time::Duration,
    std::{fs, path::PathBuf},
    synd_mchain::config::load_snapshot,
    synd_mchain::db::{ArbitrumDB, Block, State},
    test_utils::{
        port_manager::PortManager,
        tar::{create_tar, create_tar_gz, start_file_server},
        utils::test_path,
    },
    tokio::time::sleep,
};

#[cfg(feature = "rocksdb")]
struct SnapshotTestSetup {
    source_db_dir: PathBuf,
    restore_db_dir: PathBuf,
    snapshot_file: PathBuf,
    source_state: State,
    port: u16,
}

#[cfg(feature = "rocksdb")]
impl Drop for SnapshotTestSetup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.source_db_dir);
        let _ = fs::remove_dir_all(&self.restore_db_dir);
        let _ = fs::remove_file(&self.snapshot_file);
    }
}

/// Creates a test `RocksDB` database with some sample data
#[cfg(feature = "rocksdb")]
fn create_test_database(datadir: &std::path::Path) -> Result<DB> {
    let db = DB::open_default(datadir)?;

    // Create some test blocks
    let test_block = Block {
        timestamp: 1000,
        batch: vec![1, 2, 3, 4, 5].into(),
        after_batch_acc: [1u8; 32].into(),
        messages: vec![],
        before_batch_acc: [0u8; 32].into(),
        before_message_acc: [0u8; 32].into(),
        before_message_count: 0,
        slot: Default::default(),
    };

    db.put_block(1, &test_block);

    // Set some `State`
    let state = State {
        batch_count: 1,
        batch_acc: [1u8; 32].into(),
        message_count: 0,
        message_acc: [0u8; 32].into(),
        timestamp: 1000,
        slot: Default::default(),
    };
    db.put_state(&state);

    Ok(db)
}

/// Sets up a test database and prepares directories for snapshot testing
#[cfg(feature = "rocksdb")]
async fn setup_snapshot_test(test_name: &str, file_ext: &str) -> Result<SnapshotTestSetup> {
    let port = PortManager::instance().next_port().await;
    let source_db_dir = test_path(&format!("{test_name}_source"), None);
    let restore_db_dir = test_path(&format!("{test_name}_restore"), None);
    let snapshot_file =
        std::env::temp_dir().join(format!("{}_{}.{}", test_name, std::process::id(), file_ext));

    let source_db = create_test_database(&source_db_dir)?;
    let source_state = source_db.get_state();
    drop(source_db);

    Ok(SnapshotTestSetup { source_db_dir, restore_db_dir, snapshot_file, source_state, port })
}

#[tokio::test]
#[cfg(feature = "rocksdb")]
async fn test_snapshot_load_from_url() -> Result<()> {
    let setup = setup_snapshot_test("snapshot_test_url", "tar.gz").await?;

    // Create snapshot
    create_tar_gz(&setup.source_db_dir, &setup.snapshot_file)?;

    // Verify snapshot is a valid gzip file
    let original_bytes = fs::read(&setup.snapshot_file)?;
    assert!(
        original_bytes.len() >= 2 && original_bytes[0] == 0x1f && original_bytes[1] == 0x8b,
        "Original file should be a valid gzip file"
    );

    // Start HTTP server
    let server_handle = start_file_server(&setup.snapshot_file, setup.port).await?;
    sleep(Duration::from_millis(200)).await;

    // Verify download matches original
    let snapshot_url = format!("http://127.0.0.1:{}/snapshot.tar.gz", setup.port);
    let client = reqwest::Client::new();
    let response = client.get(&snapshot_url).send().await?;
    let downloaded_bytes = response.bytes().await?;
    assert_eq!(downloaded_bytes.as_ref(), original_bytes.as_slice());

    // Load snapshot and verify
    load_snapshot(&snapshot_url, &setup.restore_db_dir.to_string_lossy()).await?;
    server_handle.abort();

    let restored_db = DB::open_default(&setup.restore_db_dir)?;
    let restored_state = restored_db.get_state();
    assert_eq!(restored_state.batch_count, setup.source_state.batch_count);
    assert_eq!(restored_state.timestamp, setup.source_state.timestamp);
    assert_eq!(restored_state.batch_acc, setup.source_state.batch_acc);

    let restored_block = restored_db.get_block(1)?;
    assert_eq!(restored_block.timestamp, 1000);
    assert_eq!(restored_block.batch.len(), 5);

    Ok(())
}

#[tokio::test]
#[cfg(feature = "rocksdb")]
async fn test_snapshot_load_uncompressed_tar() -> Result<()> {
    let setup = setup_snapshot_test("snapshot_test_tar", "tar").await?;

    sleep(Duration::from_secs(2)).await;

    // Create uncompressed tar
    create_tar(&setup.source_db_dir, &setup.snapshot_file)?;

    // Start HTTP server
    let server_handle = start_file_server(&setup.snapshot_file, setup.port).await?;
    sleep(Duration::from_millis(100)).await;

    // Load snapshot and verify
    let snapshot_url = format!("http://127.0.0.1:{}/snapshot.tar", setup.port);
    load_snapshot(&snapshot_url, &setup.restore_db_dir.to_string_lossy()).await?;
    server_handle.abort();

    let restored_db = DB::open_default(&setup.restore_db_dir)?;
    let restored_state = restored_db.get_state();
    assert_eq!(restored_state.batch_count, setup.source_state.batch_count);

    Ok(())
}
