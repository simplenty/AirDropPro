use crate::config::Config;
use crate::utils::{
    base64_decode, base64_encode, clean_path_string, create_unique_file_path,
    encode_image_to_base64_png, url_encode,
};
use anyhow::{Context, Result};
use arboard::Clipboard;
use log::{error, info};
use notify_rust::Notification;
use notify_rust::Timeout;
use regex::Regex;
use rouille::{Request, Response, router};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;

fn success(msg: &str) {
    info!("\u{2570} Response: {}.", msg);
    Notification::new()
        .appname("app.airdroppro.desktop")
        .summary("AirDropPro Success")
        .body(msg)
        .timeout(Timeout::Milliseconds(5000))
        .show()
        .unwrap();
}

fn failed(err: anyhow::Error) -> Response {
    let user_msg = err.to_string();
    let json_msg = format!(r#"{{"success": false, "msg": "{}."}}"#, user_msg);

    error!("Error: {:?}", err);

    Notification::new()
        .appname("app.airdroppro.desktop")
        .summary("AirDropPro Error")
        .body(&user_msg)
        .timeout(Timeout::Milliseconds(5000))
        .show()
        .unwrap();

    Response::json(&json_msg).with_status_code(500)
}

fn get_file_handler(_request: &Request, encoded_filepath: String) -> Result<Response> {
    let filepath_str = base64_decode(&encoded_filepath).context("Failed to decode the URL path")?;

    let filepath = PathBuf::from(&filepath_str);
    let filename = filepath
        .file_name()
        .and_then(|name| name.to_str())
        .context(format!(
            "Failed to extract filename from path: {:?}",
            filepath
        ))?;

    let file = File::open(&filepath)
        .context(format!("Failed to open the file at path: {:?}", filepath))?;

    let mime_type = mime_guess::from_path(&filepath)
        .first_or_octet_stream()
        .to_string();
    let content_disposition_header =
        format!("attachment; filename*=UTF-8''{}", url_encode(filename));

    success(&format!(
        "Successfully served file from path: {:?}",
        filepath
    ));
    Ok(Response::from_file(mime_type, file)
        .with_additional_header("Content-Disposition", content_disposition_header))
}

fn post_file_handler(request: &Request) -> Result<Response> {
    let mut multipart_data = rouille::input::multipart::get_multipart_input(request)
        .context("Failed to parse multipart input")?;
    let destination_path = &Config::get().path;

    while let Some(mut field) = multipart_data.next() {
        if let Some(original_filename) = field.headers.filename {
            let unique_filepath = create_unique_file_path(destination_path, &original_filename)
                .context(format!(
                    "Failed to create unique filepath in directory: {:?}",
                    destination_path
                ))?;

            let mut file_buffer: Vec<u8> = Vec::new();
            field
                .data
                .read_to_end(&mut file_buffer)
                .context("Failed to read the uploaded file content")?;

            let mut new_file = File::create(&unique_filepath).context(format!(
                "Failed to create a new file at: {:?}",
                unique_filepath
            ))?;

            new_file.write_all(&file_buffer).context(format!(
                "Failed to write data to file: {:?}",
                unique_filepath
            ))?;

            success(&format!(
                "Successfully uploaded file '{:?}' to path: {:?}",
                original_filename, unique_filepath
            ));
        }
    }
    Ok(Response::json(&r#"{"success": true}"#))
}

fn get_clipboard_handler(_request: &Request) -> Result<Response> {
    let mut clipboard = Clipboard::new().context("Failed to initialize clipboard")?;

    if let Ok(image) = clipboard.get().image() {
        let base64_image_data =
            encode_image_to_base64_png(image).context("Failed to encode image to base64")?;

        success("Successfully served clipboard content as an image");
        return Ok(Response::json(&format!(
            r#"{{"success": true, "data": {{"type": "img", "data": {:?}}}}}"#,
            base64_image_data
        )));
    }

    if let Ok(file_list) = clipboard.get().file_list() {
        let mut encoded_file_paths = Vec::new();
        for path in file_list {
            if let Some(path_str) = path.to_str() {
                let encoded_path = base64_encode(clean_path_string(path_str))
                    .context("Failed to encode file path")?;
                encoded_file_paths.push(format!("{:?}", encoded_path));
            }
        }

        if !encoded_file_paths.is_empty() {
            let paths_json_string = encoded_file_paths.join(",");

            success(&format!(
                "Successfully served clipboard content as a file list with {} items",
                encoded_file_paths.len()
            ));
            return Ok(Response::json(&format!(
                r#"{{"success": true, "data": {{"type": "file", "data": [{}]}}}}"#,
                paths_json_string
            )));
        }
    }

    if let Ok(html) = clipboard.get().html() {
        let regex_for_file_urls = Regex::new(r#"src="(?P<path>file:///[^"]+)""#)?;
        let mut encoded_file_paths = Vec::new();
        for captures in regex_for_file_urls.captures_iter(&html) {
            if let Some(path_match) = captures.name("path") {
                let mut path_str = clean_path_string(path_match.as_str());
                if path_str.starts_with("file:///") {
                    path_str = &path_str[7..];
                }
                #[cfg(target_os = "windows")]
                {
                    if path_str.starts_with('/') {
                        path_str = &path_str[1..];
                    }
                }
                let encoded = base64_encode(path_str)
                    .context(format!("Failed to encode path: {}", path_str))?;
                encoded_file_paths.push(format!("{:?}", encoded));
            }
        }

        if !encoded_file_paths.is_empty() {
            let paths_json_string = encoded_file_paths.join(",");

            success(&format!(
                "Successfully served clipboard content from HTML with {} file links",
                encoded_file_paths.len()
            ));
            return Ok(Response::json(&format!(
                r#"{{"success": true, "data": {{"type": "file", "data": [{}]}}}}"#,
                paths_json_string
            )));
        }
    }

    if let Ok(text) = clipboard.get().text() {
        success(&format!(
            "Successfully served clipboard content as text: {:?}",
            text
        ));
        return Ok(Response::json(&format!(
            r#"{{"success": true, "data": {{"type": "text", "data": {:?}}}}}"#,
            text
        )));
    }

    anyhow::bail!("Unsupported clipboard format");
}

fn post_clipboard_handler(request: &Request) -> Result<Response> {
    let post_data =
        rouille::post_input!(request, {clipboard: String}).context("Failed to parse POST input")?;

    let text_to_set = post_data.clipboard.as_str();
    let mut clipboard = Clipboard::new().context("Failed to initialize clipboard")?;

    clipboard
        .set_text(text_to_set)
        .context("Failed to set clipboard contents")?;

    thread::sleep(std::time::Duration::from_millis(200));

    success(&format!(
        "Successfully set clipboard content with text: {:?}",
        text_to_set
    ));
    Ok(Response::json(&r#"{"success": true}"#))
}

fn page_not_found_handler() -> Response {
    Response::empty_404()
}

pub fn publish_server() -> Result<()> {
    let port = Config::get().port;
    let address = format!("0.0.0.0:{}", port);

    info!("\u{256D} Starting server on {}", address);
    thread::spawn(move || {
        rouille::start_server(address, move |request| {
            info!(
                "\u{256D} Received request: {} {} from {:?}",
                request.method(),
                request.url(),
                request.remote_addr()
            );

            router!(
                request,
                (GET) (/) => {
                    Response::text("Hello World!")
                },
                (GET) (/file/{path}) => {
                    get_file_handler(request, path).unwrap_or_else(failed)
                },
                (POST) (/file) => {
                    post_file_handler(request).unwrap_or_else(failed)
                },
                (GET) (/clipboard) => {
                    get_clipboard_handler(request).unwrap_or_else(failed)
                },
                (POST) (/clipboard) => {
                    post_clipboard_handler(request).unwrap_or_else(failed)
                },
                _ => {
                    page_not_found_handler()
                }
            )
        });
    });

    info!("\u{2570} Server has been started successfully!");
    Ok(())
}
