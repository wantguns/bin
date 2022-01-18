use rocket::http::ContentType;
use rust_embed::RustEmbed;
use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

#[get("/static/<file..>")]
pub fn static_files(
    file: PathBuf,
) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Static::get(&filename)?;

    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}
