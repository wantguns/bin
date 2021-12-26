use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::{Path, PathBuf};

#[get("/<file..>", rank = 3)]
pub async fn static_files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    NamedFile::open(Path::new("static/").join(file))
        .await
        .map_err(|e| NotFound(e.to_string()))
}
