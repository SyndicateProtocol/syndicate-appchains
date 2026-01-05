//! Shared snapshot testing utilities

use eyre::Result;
use flate2::{write::GzEncoder, Compression};
use std::{fs, path::Path};
use tar::Builder;
use tracing::{debug, error};

/// Creates a tar.gz archive from a directory
/// Files are added with just their filenames so they extract directly into the target directory
///
/// For RocksDB directories, this function handles locked files by first creating a checkpoint copy
pub fn create_tar_gz(source_dir: &Path, output_path: &Path) -> Result<()> {
    create_tar_gz_internal(source_dir, output_path, false)
}

/// Creates a tar.gz archive from a directory, optionally using a temporary copy for locked files
fn create_tar_gz_internal(
    source_dir: &Path,
    output_path: &Path,
    use_temp_copy: bool,
) -> Result<()> {
    let actual_source = if use_temp_copy {
        // Create a temporary copy of the directory to avoid file locking issues
        let temp_dir = std::env::temp_dir().join(format!("snapshot_temp_{}", std::process::id()));

        // Remove if exists from previous run
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir)?;

        // Copy all files from source to temp directory
        // Use Rust's fs operations which work reliably across platforms
        let mut files_copied = 0;
        let mut total_files = 0;

        let entries: Vec<_> = fs::read_dir(source_dir)?.collect::<Result<Vec<_>, _>>()?;

        debug!("Copying snapshot from {:?}, found {} entries", source_dir, entries.len());

        for entry in entries {
            let source_path = entry.path();
            let dest_path = temp_dir.join(entry.file_name());

            if source_path.is_file() {
                total_files += 1;
                let file_size = fs::metadata(&source_path)?.len();
                debug!("Copying file: {:?} ({} bytes)", entry.file_name(), file_size);

                // Copy the file - RocksDB files should be readable even if the DB is open
                match fs::copy(&source_path, &dest_path) {
                    Ok(bytes) => {
                        debug!("Copied {} bytes", bytes);
                        files_copied += 1;
                    }
                    Err(e) => {
                        // Log the error but continue - some files might be transient
                        error!("Failed to copy: {}", e);
                    }
                }
            }
        }

        debug!("Copy complete: {}/{} files copied successfully", files_copied, total_files);

        if files_copied == 0 {
            return Err(eyre::eyre!(
                "No files were copied from {:?} to temp directory (found {} total files)",
                source_dir,
                total_files
            ));
        }

        temp_dir
    } else {
        source_dir.to_path_buf()
    };

    let file = fs::File::create(output_path)?;
    let gz = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(gz);

    // RocksDB stores all files in the root of the datadir, so we only need filenames
    for entry in fs::read_dir(&actual_source)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a file
        if !path.is_file() {
            continue;
        }

        // Use just the filename as a string to avoid path issues
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid filename: {:?}", path))?;

        let mut file = fs::File::open(&path)?;
        tar.append_file(file_name, &mut file)?;
    }

    tar.finish()?;

    // Clean up temp directory if we used one
    if use_temp_copy {
        let _ = fs::remove_dir_all(&actual_source);
    }

    Ok(())
}

/// Creates a tar.gz archive from a potentially locked RocksDB directory
/// This uses a temporary copy to avoid file locking issues with active databases
pub fn create_tar_gz_from_rocksdb(source_dir: &Path, output_path: &Path) -> Result<()> {
    create_tar_gz_internal(source_dir, output_path, true)
}

/// Creates an uncompressed tar archive from a directory
/// Files are added with just their filenames so they extract directly into the target directory
pub fn create_tar(source_dir: &Path, output_path: &Path) -> Result<()> {
    use std::io::Write;

    // Collect all files first to ensure they're all accessible
    let mut files_to_archive = Vec::new();
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            files_to_archive.push(path);
        }
    }

    // Sort for deterministic ordering
    files_to_archive.sort();

    let file = fs::File::create(output_path)?;
    let mut tar = Builder::new(file);

    for path in &files_to_archive {
        // Use just the filename as a string to avoid path issues
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid filename: {:?}", path))?;

        let mut source_file = fs::File::open(path)?;
        tar.append_file(file_name, &mut source_file)?;
    }

    // Finish the tar and get back the underlying file
    let mut file = tar.into_inner()?;
    // Explicitly flush and sync the file to disk
    file.flush()?;
    file.sync_all()?;

    Ok(())
}

/// Simple HTTP server that serves a file
pub async fn start_file_server(file_path: &Path, port: u16) -> Result<tokio::task::JoinHandle<()>> {
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    let file_path = file_path.to_path_buf();

    let handle = tokio::spawn(async move {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        while let Ok((stream, _)) = listener.accept().await {
            let file_path = file_path.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stream);

                // Read the entire HTTP request (until we see the empty line)
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            if line.trim().is_empty() {
                                break;
                            }
                        }
                        Err(_) => return,
                    }
                }

                // Read the entire file into memory
                let file_bytes = match tokio::fs::read(&file_path).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let mut stream = reader.into_inner();
                        let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").await;
                        return;
                    }
                };

                let content_length = file_bytes.len();

                // Determine content type based on file extension
                let content_type = if file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == "gz")
                    .unwrap_or(false)
                {
                    "application/gzip"
                } else {
                    "application/x-tar"
                };

                let headers = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {content_length}\r\nConnection: close\r\n\r\n",
                );

                let mut stream = reader.into_inner();
                if stream.write_all(headers.as_bytes()).await.is_err() {
                    return;
                }

                // Write the file content
                if stream.write_all(&file_bytes).await.is_err() {
                    return;
                }
                let _ = stream.flush().await;
            });
        }
    });

    Ok(handle)
}
