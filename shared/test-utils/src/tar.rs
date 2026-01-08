//! Shared snapshot testing utilities

use axum::Router;
use eyre::Result;
use flate2::{write::GzEncoder, Compression};
use std::{fs, io::Write, path::Path};
use tar::Builder;
use tower_http::services::ServeFile;

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

pub async fn start_file_server(file_path: &Path, port: u16) -> Result<tokio::task::JoinHandle<()>> {
    let app = Router::new().fallback_service(ServeFile::new(file_path));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    Ok(handle)
}
