use anyhow::{Context, Result};
use arboard::ImageData;
use base64::Engine;
use base64::engine::general_purpose;
use env::current_exe;
use image::{ImageBuffer, ImageFormat, Rgba};
use native_dialog::{DialogBuilder, MessageLevel};
use rouille::percent_encoding::percent_encode;
use single_instance::SingleInstance;
use std::env;
use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::{Path, PathBuf};

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

pub fn create_unique_file_path<P: AsRef<Path>>(base_dir: P, file_name: &str) -> Result<PathBuf> {
    create_dir_all(&base_dir)?;

    let original_path = Path::new(file_name);
    let mut unique_path = base_dir.as_ref().join(original_path);

    if unique_path.exists() {
        let name_stem = original_path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid file name")?;
        let extension = original_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        for i in 1.. {
            let new_name = format!("{}({}).{}", name_stem, i, extension);
            let temp_path = base_dir.as_ref().join(new_name);

            if !temp_path.exists() {
                unique_path = temp_path;
                break;
            }
        }
    }

    Ok(unique_path)
}

pub fn get_config_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().context("Failed to get standard config directory")?;
    config_path.push("AirDropPro");
    create_dir_all(&config_path)
        .with_context(|| format!("Failed to create config path: {:?}", config_path))?;
    config_path.push("config.ini");
    Ok(config_path)
}

pub fn base64_encode(input: &str) -> Result<String> {
    Ok(general_purpose::URL_SAFE.encode(input.as_bytes()))
}

pub fn base64_decode(input: &str) -> Result<String> {
    let decoded_bytes = general_purpose::URL_SAFE
        .decode(input)
        .context("Failed to decode base64 code.")?;
    let decoded_string =
        String::from_utf8(decoded_bytes).context("Failed to convert base64 string to UTF-8.")?;
    Ok(decoded_string)
}

pub fn encode_image_to_base64_png(image_data: ImageData) -> Result<String> {
    let img_buf = ImageBuffer::<Rgba<u8>, _>::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.as_ref(),
    )
    .context("Failed to create image buffer from raw data. The dimensions might be wrong.")?;
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    img_buf
        .write_to(&mut cursor, ImageFormat::Png)
        .context("Failed to encode image to PNG format.")?;
    let encoded_base64 = general_purpose::STANDARD.encode(&bytes);
    Ok(encoded_base64)
}

pub fn clean_path_string(path: &str) -> &str {
    path.trim_end_matches(&['\r', '\n', '\u{0020}'][..])
}

pub fn url_encode(input: &str) -> String {
    percent_encode(input.as_bytes(), rouille::DEFAULT_ENCODE_SET).to_string()
}

pub fn set_auto_startup(status: bool) -> Result<()> {
    let app_path = current_exe().context("Failed to get current executable path")?;
    let app_path = app_path.to_str().context("Failed to get absolute path")?;
    let auto_launch = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("AirDropPro")
        .set_app_path(app_path)
        .build()
        .context("Failed to build auto launch manager")?;
    let current_status = auto_launch
        .is_enabled()
        .context("Failed to get autolaunch status")?;
    if status != current_status {
        if status {
            auto_launch
                .enable()
                .context("Failed to enable autolaunch")?;
        } else {
            auto_launch
                .disable()
                .context("Failed to disable autolaunch")?;
        }
    }
    Ok(())
}

pub fn ensure_single_instance(instance_name: &str) -> Result<()> {
    let instance = SingleInstance::new(instance_name).context("Failed to register an instance")?;
    if !instance.is_single() {
        let message = format!(
            "An instance of '{}' is already running. The new instance will exit immediately.",
            instance_name
        );
        DialogBuilder::message()
            .set_level(MessageLevel::Info)
            .set_title(instance_name)
            .set_text(message)
            .alert()
            .show()
            .context("Failed to show dialog")?;
        std::process::exit(1);
    }
    let _lock: &'static SingleInstance = Box::leak(Box::new(instance));
    Ok(())
}
