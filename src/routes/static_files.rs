use crate::models::response_wrapper::ResponseWrapper;
use rocket::http::ContentType;
use rust_embed::RustEmbed;
use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

#[get("/static/<file..>")]
pub fn static_files(
    file: PathBuf,
) -> ResponseWrapper<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = match Static::get(&filename) {
        Some(v) => v,
        None => return ResponseWrapper::not_found(&file.to_string_lossy()),
    };

    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    ResponseWrapper::meta_response((content_type, asset.data))
}
