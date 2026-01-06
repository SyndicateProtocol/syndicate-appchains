//! Shared snapshot testing utilities

use eyre::Result;
use flate2::{write::GzEncoder, Compression};
use std::{fs, io::Write, path::Path};
use tar::Builder;

/// Creates a tar.gz archive from a directory
/// Files are added with just their filenames so they extract directly into the target directory
pub fn create_tar_gz(source_dir: &Path, output_path: &Path) -> Result<()> {
    let file = fs::File::create(output_path)?;
    let gz = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(gz);

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid filename: {:?}", path))?;

        let mut file = fs::File::open(&path)?;
        tar.append_file(file_name, &mut file)?;
    }

    tar.finish()?;
    Ok(())
}

/// Creates an uncompressed tar archive from a directory
/// Files are added with just their filenames so they extract directly into the target directory
pub fn create_tar(source_dir: &Path, output_path: &Path) -> Result<()> {
    let file = fs::File::create(output_path)?;
    let mut tar = Builder::new(file);

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid filename: {:?}", path))?;

        let mut source_file = fs::File::open(&path)?;
        tar.append_file(file_name, &mut source_file)?;
    }

    let mut file = tar.into_inner()?;
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
