use anyhow::{Context, Result};
use std::path::{PathBuf};

pub fn resolve_base_directory(dir_name: &str) -> Result<PathBuf> {
    match dir_name {
        "video" => dirs::video_dir(),
        "picture" => dirs::picture_dir(),
        "desktop" => dirs::desktop_dir(),
        "download" => dirs::download_dir(),
        "document" => dirs::document_dir(),
        _ => return Ok(PathBuf::from(dir_name)),
    }
        .with_context(|| format!("Could not find the {:?} directory.", dir_name))
}