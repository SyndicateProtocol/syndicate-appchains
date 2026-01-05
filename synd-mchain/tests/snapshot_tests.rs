//! End-to-end tests for snapshot functionality

#[cfg(feature = "rocksdb")]
use {
    eyre::Result,
    rocksdb::DB,
    std::fs,
    synd_mchain::config::load_snapshot,
    synd_mchain::db::{ArbitrumDB, Block, State},
    test_utils::{
        port_manager::PortManager,
        snapshot::{create_tar, create_tar_gz, start_file_server},
        utils::test_path,
    },
};

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

#[tokio::test]
#[cfg(feature = "rocksdb")]
async fn test_snapshot_load_from_url() -> Result<()> {
    // Find an available port and generate unique test ID
    let port = PortManager::instance().next_port().await;

    // Create temporary directories
    let source_db_dir = test_path("snapshot_test_url_source", None);
    let restore_db_dir = test_path("snapshot_test_url_restore", None);
    let snapshot_file =
        std::env::temp_dir().join(format!("snapshot_test_url_{}.tar.gz", std::process::id()));

    // Clean up any existing directories
    let _ = fs::remove_dir_all(&source_db_dir);
    let _ = fs::remove_dir_all(&restore_db_dir);

    // Create source database with test data
    fs::create_dir_all(&source_db_dir)?;
    let source_db = create_test_database(&source_db_dir)?;

    // Verify source database has data
    let source_state = source_db.get_state();
    assert_eq!(source_state.batch_count, 1);
    assert_eq!(source_state.timestamp, 1000);

    // Close the database
    drop(source_db);

    // Create snapshot tar.gz from the database directory
    // RocksDB creates multiple files, so we need to copy the entire directory structure
    create_tar_gz(&source_db_dir, &snapshot_file)?;

    // Verify snapshot file exists and is not empty
    assert!(snapshot_file.exists());
    let snapshot_size = fs::metadata(&snapshot_file)?.len();
    assert!(snapshot_size > 0);

    // Verify the original file is a valid gzip file
    let original_bytes = fs::read(&snapshot_file)?;
    assert!(
        original_bytes.len() >= 2 && original_bytes[0] == 0x1f && original_bytes[1] == 0x8b,
        "Original file should be a valid gzip file"
    );

    // Start HTTP server to serve the snapshot
    let server_handle = start_file_server(&snapshot_file, port).await?;

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Create restore directory
    fs::create_dir_all(&restore_db_dir)?;

    // Load snapshot from URL
    let snapshot_url = format!("http://127.0.0.1:{port}/snapshot.tar.gz");

    // Verify the downloaded file matches the original
    let client = reqwest::Client::new();
    let response = client.get(&snapshot_url).send().await?;
    let downloaded_bytes = response.bytes().await?;

    assert_eq!(
        downloaded_bytes.len(),
        original_bytes.len(),
        "Downloaded file size should match original"
    );
    assert_eq!(
        downloaded_bytes.as_ref(),
        original_bytes.as_slice(),
        "Downloaded file content should match original"
    );

    // Now load the snapshot
    load_snapshot(&snapshot_url, &restore_db_dir.to_string_lossy()).await?;

    // Stop the server
    server_handle.abort();

    // Verify the restored database
    let restored_db = DB::open_default(&restore_db_dir)?;
    let restored_state = restored_db.get_state();

    // Verify the state matches
    assert_eq!(restored_state.batch_count, source_state.batch_count);
    assert_eq!(restored_state.timestamp, source_state.timestamp);
    assert_eq!(restored_state.batch_acc, source_state.batch_acc);

    // Verify we can read the block
    let restored_block = restored_db.get_block(1)?;
    assert_eq!(restored_block.timestamp, 1000);
    assert_eq!(restored_block.batch.len(), 5);

    // Cleanup
    let _ = fs::remove_dir_all(&source_db_dir);
    let _ = fs::remove_dir_all(&restore_db_dir);
    let _ = fs::remove_file(&snapshot_file);

    Ok(())
}

#[tokio::test]
#[cfg(feature = "rocksdb")]
async fn test_snapshot_load_uncompressed_tar() -> Result<()> {
    // Find an available port and generate unique test ID
    let port = PortManager::instance().next_port().await;

    // Create temporary directories
    let source_db_dir = test_path("snapshot_test_tar_source", None);
    let restore_db_dir = test_path("snapshot_test_tar_restore", None);
    let snapshot_file =
        std::env::temp_dir().join(format!("snapshot_test_tar_{}.tar", std::process::id()));

    // Clean up any existing directories
    let _ = fs::remove_dir_all(&source_db_dir);
    let _ = fs::remove_dir_all(&restore_db_dir);

    // Create source database with test data
    fs::create_dir_all(&source_db_dir)?;
    let source_db = create_test_database(&source_db_dir)?;
    let source_state = source_db.get_state();

    // Explicitly close and flush the database
    drop(source_db);

    // Give RocksDB time to release all file handles
    // This is critical to ensure all files are flushed and closed before we create the tar
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Create uncompressed tar file
    create_tar(&source_db_dir, &snapshot_file)?;

    // Start HTTP server
    let server_handle = start_file_server(&snapshot_file, port).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Load snapshot
    fs::create_dir_all(&restore_db_dir)?;
    let snapshot_url = format!("http://127.0.0.1:{port}/snapshot.tar");
    load_snapshot(&snapshot_url, &restore_db_dir.to_string_lossy()).await?;

    server_handle.abort();

    // Verify restored database
    let restored_db = DB::open_default(&restore_db_dir)?;
    let restored_state = restored_db.get_state();
    assert_eq!(restored_state.batch_count, source_state.batch_count);

    // Cleanup
    let _ = fs::remove_dir_all(&source_db_dir);
    let _ = fs::remove_dir_all(&restore_db_dir);
    let _ = fs::remove_file(&snapshot_file);

    Ok(())
}
